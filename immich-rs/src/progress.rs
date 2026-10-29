//! Transfer progress tracking types and adapters.
//!
//! Provides a unified [`TransferProgress`] notification mechanism that supports both
//! synchronous closures (`Fn(TransferProgress)`) and asynchronous watch channels (`tokio::sync::watch`).

use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, ReadBuf};
use tokio::sync::watch;

/// Represents the current status of an ongoing file upload or download.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferProgress {
    /// Number of bytes transferred so far.
    pub transferred: u64,
    /// Total number of bytes expected, or `None` if the total size is unknown.
    pub total: Option<u64>,
}

impl TransferProgress {
    /// Creates a new `TransferProgress` report.
    #[must_use]
    pub const fn new(transferred: u64, total: Option<u64>) -> Self {
        Self { transferred, total }
    }

    /// Returns the progress as a normalized ratio between `0.0` and `1.0`, or `None` if total is unknown or zero.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn ratio(&self) -> Option<f64> {
        match self.total {
            Some(total) if total > 0 => Some((self.transferred as f64 / total as f64).min(1.0)),
            _ => None,
        }
    }

    /// Returns the progress percentage between `0.0` and `100.0`, or `None` if total is unknown or zero.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn percentage(&self) -> Option<f32> {
        self.ratio().map(|r| (r * 100.0) as f32)
    }

    /// Returns `true` if all expected bytes have been transferred.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        match self.total {
            Some(total) => self.transferred >= total,
            None => false,
        }
    }
}

/// Destination for progress reports, supporting closures or asynchronous watch channels.
#[derive(Clone)]
pub enum ProgressSender {
    /// A thread-safe synchronous callback closure.
    Callback(Arc<dyn Fn(TransferProgress) + Send + Sync + 'static>),
    /// A `tokio::sync::watch` channel sender.
    Watch(watch::Sender<TransferProgress>),
}

impl std::fmt::Debug for ProgressSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Callback(_) => write!(f, "ProgressSender::Callback(...)"),
            Self::Watch(_) => write!(f, "ProgressSender::Watch(...)"),
        }
    }
}

impl ProgressSender {
    /// Emits a new progress report to the registered listener.
    pub fn report(&self, transferred: u64, total: Option<u64>) {
        let progress = TransferProgress::new(transferred, total);
        match self {
            Self::Callback(cb) => cb(progress),
            Self::Watch(tx) => {
                let _ = tx.send(progress);
            }
        }
    }
}

/// An `AsyncRead` adapter that intercepts read operations to calculate and emit progress reports.
#[derive(Debug)]
pub struct ProgressReader<R> {
    inner: R,
    transferred: u64,
    total: Option<u64>,
    progress: Option<ProgressSender>,
}

impl<R> ProgressReader<R> {
    /// Wraps an existing reader with optional progress reporting.
    pub fn new(inner: R, total: Option<u64>, progress: Option<ProgressSender>) -> Self {
        if let Some(ref p) = progress {
            p.report(0, total);
        }
        Self {
            inner,
            transferred: 0,
            total,
            progress,
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for ProgressReader<R> {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        let prev_len = buf.filled().len();
        let poll_result = Pin::new(&mut self.inner).poll_read(cx, buf);

        if let Poll::Ready(Ok(())) = &poll_result {
            let newly_read = buf.filled().len() - prev_len;
            if newly_read > 0 {
                self.transferred += newly_read as u64;
                let transferred = self.transferred;
                let total = self.total;
                if let Some(ref p) = self.progress {
                    p.report(transferred, total);
                }
            }
        }

        poll_result
    }
}
