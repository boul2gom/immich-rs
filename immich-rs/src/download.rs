//! High-level download helper that streams a file from Immich without loading it
//! entirely into memory.
//!
//! This is a thin wrapper around the generated [`Assets::download_asset`](crate::api::assets::Assets::download_asset) endpoint.

#[cfg(feature = "fs")]
use std::path::Path;

use futures_util::StreamExt;
#[cfg(feature = "fs")]
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::client::ImmichClient;
use crate::error::{Error, Result};
use crate::progress::ProgressSender;

impl ImmichClient {
    /// Download an asset directly to a file on disk without loading it into RAM.
    ///
    /// Writes to a temporary `.<asset_id>.tmp` file first and atomically renames it
    /// to `dest` upon completion. If the transfer fails or the future is cancelled,
    /// the partial temporary file is automatically removed.
    ///
    /// # Errors
    /// Returns an error if the request fails or if writing to disk fails.
    #[cfg(feature = "fs")]
    pub async fn download<P: AsRef<Path>>(&self, asset_id: uuid::Uuid, dest: P) -> Result<()> {
        self.download_internal(asset_id, dest.as_ref(), None).await
    }

    /// Download an asset directly to a file on disk while reporting progress.
    ///
    /// # Errors
    /// Returns an error if the request fails or if writing to disk fails.
    #[cfg(feature = "fs")]
    pub async fn download_with_progress<P: AsRef<Path>>(
        &self,
        asset_id: uuid::Uuid,
        dest: P,
        progress: ProgressSender,
    ) -> Result<()> {
        self.download_internal(asset_id, dest.as_ref(), Some(progress)).await
    }

    #[cfg(feature = "fs")]
    async fn download_internal(
        &self,
        asset_id: uuid::Uuid,
        dest: &Path,
        progress: Option<ProgressSender>,
    ) -> Result<()> {
        let resp = self.assets().download_asset(asset_id).await?;
        let total = resp.content_length();

        let mut tmp_os = dest.as_os_str().to_os_string();
        tmp_os.push(format!(".{asset_id}.tmp"));
        let tmp_dest = std::path::PathBuf::from(tmp_os);

        let cleanup_guard = scopeguard::guard(tmp_dest.clone(), |path| {
            let _ = std::fs::remove_file(path);
        });

        let file = File::create(&tmp_dest).await?;
        let mut writer = tokio::io::BufWriter::with_capacity(64 * 1024, file);
        let mut stream = resp.bytes_stream();
        let mut transferred = 0u64;

        if let Some(ref p) = progress {
            p.report(0, total);
        }

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(Error::Transport)?;
            writer.write_all(&chunk).await?;
            transferred += chunk.len() as u64;
            if let Some(ref p) = progress {
                p.report(transferred, total);
            }
        }

        writer.flush().await?;
        let file = writer.into_inner();
        file.sync_all().await?;
        drop(file);
        tokio::fs::rename(&tmp_dest, dest).await?;

        scopeguard::ScopeGuard::into_inner(cleanup_guard);
        Ok(())
    }

    /// Streams an asset directly into any asynchronous writer without loading it entirely into RAM.
    ///
    /// This is useful for streaming directly to an HTTP response, an arbitrary pipe, a network socket,
    /// or a custom stream sink.
    ///
    /// Returns the total number of bytes written.
    ///
    /// # Errors
    /// Returns an error if the request fails or if writing to the destination fails.
    pub async fn download_to_writer<W>(&self, asset_id: uuid::Uuid, writer: W) -> Result<u64>
    where
        W: tokio::io::AsyncWrite + Unpin,
    {
        self.download_to_writer_internal(asset_id, writer, None).await
    }

    /// Streams an asset directly into any asynchronous writer while reporting transfer progress.
    ///
    /// Returns the total number of bytes written.
    ///
    /// # Errors
    /// Returns an error if the request fails or if writing to the destination fails.
    pub async fn download_to_writer_with_progress<W>(
        &self,
        asset_id: uuid::Uuid,
        writer: W,
        progress: ProgressSender,
    ) -> Result<u64>
    where
        W: tokio::io::AsyncWrite + Unpin,
    {
        self.download_to_writer_internal(asset_id, writer, Some(progress)).await
    }

    async fn download_to_writer_internal<W>(
        &self,
        asset_id: uuid::Uuid,
        mut writer: W,
        progress: Option<ProgressSender>,
    ) -> Result<u64>
    where
        W: tokio::io::AsyncWrite + Unpin,
    {
        let resp = self.assets().download_asset(asset_id).await?;
        let total = resp.content_length();
        let mut stream = resp.bytes_stream();
        let mut total_bytes = 0u64;

        if let Some(ref p) = progress {
            p.report(0, total);
        }

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(Error::Transport)?;
            writer.write_all(&chunk).await?;
            total_bytes += chunk.len() as u64;
            if let Some(ref p) = progress {
                p.report(total_bytes, total);
            }
        }

        writer.flush().await?;
        Ok(total_bytes)
    }
}
