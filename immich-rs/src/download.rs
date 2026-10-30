//! High-level download helpers that stream assets from Immich without buffering in RAM.
//!
//! Wraps the generated [`Assets::download_asset`](crate::api::assets::Assets::download_asset) endpoint.

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
use std::path::Path;

use immich_models::id::AssetId;
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::client::ImmichClient;
use crate::error::Result;
use crate::progress::ProgressSender;

impl ImmichClient {
    /// Downloads an asset directly to a file on disk without loading it into RAM.
    ///
    /// Writes to a temporary `<dest>.<asset_id>.tmp` file first and atomically renames it
    /// to `dest` upon completion. If the transfer fails or the future is cancelled,
    /// the partial temporary file is automatically removed while leaving any existing `dest` untouched.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if writing to disk fails.
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub async fn download<P: AsRef<Path>>(&self, asset_id: impl Into<AssetId>, dest: P) -> Result<()> {
        self.download_internal(asset_id.into(), dest.as_ref(), None).await
    }

    /// Downloads an asset directly to a file on disk while reporting transfer progress.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if writing to disk fails.
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub async fn download_with_progress<P: AsRef<Path>>(
        &self,
        asset_id: impl Into<AssetId>,
        dest: P,
        progress: ProgressSender,
    ) -> Result<()> {
        self.download_internal(asset_id.into(), dest.as_ref(), Some(progress))
            .await
    }

    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    async fn download_internal(&self, asset_id: AssetId, dest: &Path, progress: Option<ProgressSender>) -> Result<()> {
        let mut stream = self.assets().download_asset(asset_id).await?;
        let total = stream.content_length();

        let mut tmp_os = dest.as_os_str().to_os_string();
        tmp_os.push(format!(".{asset_id}.tmp"));
        let tmp_dest = std::path::PathBuf::from(tmp_os);

        let cleanup_guard = scopeguard::guard(tmp_dest.clone(), |path| {
            let _ = std::fs::remove_file(path);
        });

        let file = File::create(&tmp_dest).await?;
        let mut writer = tokio::io::BufWriter::with_capacity(64 * 1024, file);
        let mut transferred = 0u64;

        if let Some(ref p) = progress {
            p.report(0, total);
        }

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
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
    /// Returns the total number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if writing to the destination fails.
    pub async fn download_to_writer<W>(&self, asset_id: impl Into<AssetId>, writer: W) -> Result<u64>
    where
        W: tokio::io::AsyncWrite + Unpin,
    {
        self.download_to_writer_internal(asset_id.into(), writer, None).await
    }

    /// Streams an asset directly into any asynchronous writer while reporting transfer progress.
    ///
    /// Returns the total number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if writing to the destination fails.
    pub async fn download_to_writer_with_progress<W>(
        &self,
        asset_id: impl Into<AssetId>,
        writer: W,
        progress: ProgressSender,
    ) -> Result<u64>
    where
        W: tokio::io::AsyncWrite + Unpin,
    {
        self.download_to_writer_internal(asset_id.into(), writer, Some(progress))
            .await
    }

    async fn download_to_writer_internal<W>(
        &self,
        asset_id: AssetId,
        mut writer: W,
        progress: Option<ProgressSender>,
    ) -> Result<u64>
    where
        W: tokio::io::AsyncWrite + Unpin,
    {
        let mut stream = self.assets().download_asset(asset_id).await?;
        let total = stream.content_length();
        let mut total_bytes = 0u64;

        if let Some(ref p) = progress {
            p.report(0, total);
        }

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
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
