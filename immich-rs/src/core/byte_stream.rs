use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Bytes, BytesMut};
use futures_core::Stream;
use futures_util::StreamExt;

use crate::error::{Error, Result};

enum StreamInner {
    Response(reqwest::Response),
    Stream(crate::BoxStream<'static, Result<Bytes>>),
    Consumed,
}

/// An opaque, asynchronous stream of binary data returned by download and media endpoints.
///
/// Wraps an HTTP response, exposing response status, headers, and media metadata
/// (`content_type`, `content_length`, `filename`), as well as high-level consumption methods
/// ([`.bytes()`](Self::bytes), [`.save_to_path()`](Self::save_to_path), [`.into_async_read()`](Self::into_async_read)).
///
/// Implements [`Stream<Item = Result<Bytes>>`](Stream) and provides an inherent
/// [`.next()`](Self::next) method for sequential chunk consumption without requiring `StreamExt`.
#[must_use = "streams do nothing unless consumed"]
pub struct ByteStream {
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    content_length: Option<u64>,
    inner: StreamInner,
}

impl ByteStream {
    /// Creates a new `ByteStream` wrapping an underlying [`reqwest::Response`].
    pub fn new(response: reqwest::Response) -> Self {
        let status = response.status();
        let headers = response.headers().clone();
        let content_length = response.content_length();
        Self {
            status,
            headers,
            content_length,
            inner: StreamInner::Response(response),
        }
    }

    /// Returns the HTTP status code of the response.
    #[must_use]
    pub const fn status(&self) -> reqwest::StatusCode {
        self.status
    }

    /// Returns the response headers.
    #[must_use]
    pub const fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    /// Returns the MIME content type indicated in the `Content-Type` header, if present.
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.headers
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
    }

    /// Returns the expected payload length in bytes from the `Content-Length` header, if known.
    #[must_use]
    pub const fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    /// Extracts the suggested filename from the `Content-Disposition` header, if present.
    ///
    /// Supports both standard `filename="name.ext"` and RFC 5987 / RFC 6266 `filename*=UTF-8''name.ext`
    /// attributes with automatic percent-decoding.
    #[must_use]
    pub fn filename(&self) -> Option<String> {
        let val = self.headers.get(reqwest::header::CONTENT_DISPOSITION)?.to_str().ok()?;
        parse_filename_from_disposition(val)
    }

    /// Unwraps the underlying [`reqwest::Response`], if this stream has not yet been polled or consumed.
    ///
    /// Returns `None` if the response body has already transitioned to a chunk stream.
    #[must_use]
    pub fn into_inner(self) -> Option<reqwest::Response> {
        match self.inner {
            StreamInner::Response(resp) => Some(resp),
            _ => None,
        }
    }

    fn ensure_stream(&mut self) -> &mut crate::BoxStream<'static, Result<Bytes>> {
        if let StreamInner::Response(_) = self.inner {
            let StreamInner::Response(resp) = std::mem::replace(&mut self.inner, StreamInner::Consumed) else {
                unreachable!()
            };
            let stream = resp.bytes_stream().map(|res| res.map_err(Error::Transport));
            self.inner = StreamInner::Stream(Box::pin(stream));
        }

        match &mut self.inner {
            StreamInner::Stream(stream) => stream,
            StreamInner::Consumed | StreamInner::Response(_) => unreachable!(),
        }
    }

    /// Yields the next chunk of binary data from the stream, or `None` when finished.
    ///
    /// # Errors
    ///
    /// Returns an error if reading the next network chunk fails.
    pub async fn next(&mut self) -> Option<Result<Bytes>> {
        use futures_util::StreamExt;
        self.ensure_stream().next().await
    }

    /// Buffers and returns the full binary payload as [`Bytes`].
    ///
    /// # Errors
    ///
    /// Returns an error if reading the response body fails.
    pub async fn bytes(mut self) -> Result<Bytes> {
        match std::mem::replace(&mut self.inner, StreamInner::Consumed) {
            StreamInner::Response(resp) => resp.bytes().await.map_err(Error::Transport),
            StreamInner::Stream(mut stream) => {
                use futures_util::StreamExt;
                let mut buf = match self.content_length {
                    Some(len) if len <= 1024 * 1024 * 512 => BytesMut::with_capacity(usize::try_from(len).unwrap_or(0)),
                    _ => BytesMut::new(),
                };
                while let Some(chunk_res) = stream.next().await {
                    let chunk = chunk_res?;
                    buf.extend_from_slice(&chunk);
                }
                Ok(buf.freeze())
            }
            StreamInner::Consumed => Ok(Bytes::new()),
        }
    }

    /// Streams the binary payload directly into a file on disk without buffering it entirely in memory.
    ///
    /// Writes to a temporary `.tmp` file in the destination folder first and atomically renames it
    /// upon completion. If writing is cancelled or fails, the partial file is automatically removed.
    ///
    /// Returns the total number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error if creating the destination directory, writing to the file, or the network stream fails.
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub async fn save_to_path(&mut self, path: impl AsRef<std::path::Path>) -> Result<u64> {
        use tokio::io::AsyncWriteExt;

        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut tmp_path = path.as_os_str().to_os_string();
        tmp_path.push(format!(".tmp.{}", rand::random::<u64>()));
        let tmp_path = std::path::PathBuf::from(tmp_path);

        let cleanup_guard = scopeguard::guard(tmp_path.clone(), |p| {
            let _ = std::fs::remove_file(p);
        });

        let file = tokio::fs::File::create(&tmp_path).await?;
        let mut writer = tokio::io::BufWriter::with_capacity(64 * 1024, file);
        let mut total_bytes = 0u64;

        while let Some(chunk_res) = self.next().await {
            let chunk = chunk_res?;
            writer.write_all(&chunk).await?;
            total_bytes += chunk.len() as u64;
        }

        writer.flush().await?;
        let file = writer.into_inner();
        file.sync_all().await?;
        drop(file);

        tokio::fs::rename(&tmp_path, path).await?;
        scopeguard::ScopeGuard::into_inner(cleanup_guard);
        Ok(total_bytes)
    }

    /// Converts this `ByteStream` into a type implementing [`tokio::io::AsyncRead`].
    #[must_use]
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub fn into_async_read(mut self) -> impl tokio::io::AsyncRead + Unpin {
        self.ensure_stream();
        let StreamInner::Stream(stream) = std::mem::replace(&mut self.inner, StreamInner::Consumed) else {
            unreachable!()
        };
        let io_stream = stream.map(|res| res.map_err(|e| std::io::Error::other(e.to_string())));
        tokio_util::io::StreamReader::new(io_stream)
    }
}

impl Stream for ByteStream {
    type Item = Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.ensure_stream().as_mut().poll_next(cx)
    }
}

impl std::fmt::Debug for ByteStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ByteStream")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("content_length", &self.content_length)
            .finish_non_exhaustive()
    }
}

fn parse_filename_from_disposition(header: &str) -> Option<String> {
    // 1. Look for filename*=
    for part in header.split(';') {
        let trimmed = part.trim();
        if let Some(rest) = trimmed.strip_prefix("filename*=") {
            let rest = rest.trim().trim_matches('"');
            if let Some(pos) = rest.rfind("''") {
                let encoded = &rest[pos + 2..];
                return Some(percent_decode(encoded));
            }
        }
    }
    // 2. Look for regular filename=
    for part in header.split(';') {
        let trimmed = part.trim();
        if let Some(rest) = trimmed.strip_prefix("filename=") {
            let filename = rest.trim().trim_matches('"').trim_matches('\'').trim();
            if !filename.is_empty() {
                return Some(filename.to_string());
            }
        }
    }
    None
}

fn percent_decode(s: &str) -> String {
    let mut bytes = Vec::with_capacity(s.len());
    let mut chars = s.as_bytes().iter().copied().peekable();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(c1), Some(c2)) = (h1, h2) {
                if let Ok(decoded) = hex::decode([c1, c2]) {
                    bytes.push(decoded[0]);
                    continue;
                }
                bytes.push(b'%');
                bytes.push(c1);
                bytes.push(c2);
                continue;
            }
        }
        bytes.push(b);
    }
    String::from_utf8(bytes).unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned())
}
