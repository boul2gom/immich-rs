//! Export all assets from an Immich album to a local directory with live transfer progress.
//!
//! # Usage
//! ```sh
//! export IMMICH_INSTANCE_URL="https://photos.example.com/api"
//! export IMMICH_API_KEY="your-api-key"
//! cargo run -p examples --bin export_album
//! ```

use anyhow::{Context, Result};
use futures_util::StreamExt;
use immich_rs::ImmichClient;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let client = ImmichClient::from_env()
        .context("Failed to initialize ImmichClient from IMMICH_INSTANCE_URL and IMMICH_API_KEY")?;

    println!("🔍 Searching for albums on {}", client.base_url());
    let albums = client
        .albums()
        .get_all_albums()
        .send()
        .await
        .context("Failed to retrieve albums")?;

    if albums.is_empty() {
        println!("⚠️ No albums found on the server.");
        return Ok(());
    }

    let target_album = &albums[0];
    println!(
        "📁 Selected album: '{}' (id: {}, asset count: {})",
        target_album.album_name, target_album.id, target_album.asset_count
    );

    let album_details = client
        .albums()
        .get_album_info(target_album.id)
        .send()
        .await
        .context("Failed to fetch album details")?;

    let assets = album_details.assets;
    if assets.is_empty() {
        println!("⚠️ Album contains no downloadable assets.");
        return Ok(());
    }

    let out_dir = PathBuf::from("exported_albums").join(&target_album.album_name);
    tokio::fs::create_dir_all(&out_dir)
        .await
        .with_context(|| format!("Failed to create destination folder: {}", out_dir.display()))?;

    println!(
        "⬇️ Exporting {} assets to '{}' (concurrency: 3)...",
        assets.len(),
        out_dir.display()
    );

    let total = assets.len();
    let completed = Arc::new(AtomicUsize::new(0));

    let download_stream = futures_util::stream::iter(assets).map(|asset| {
        let client = client.clone();
        let out_dir = out_dir.clone();
        let completed = Arc::clone(&completed);

        async move {
            let file_name = format!("{}.bin", asset.id);
            let dest_path = out_dir.join(&file_name);

            let progress_sender = immich_rs::ProgressSender::Callback(Arc::new({
                let file_name = file_name.clone();
                move |progress| {
                    if let Some(pct) = progress.percentage() {
                        tracing::debug!(file = %file_name, percent = pct, "Download in progress");
                    }
                }
            }));

            client
                .download_with_progress(asset.id, &dest_path, progress_sender)
                .await
                .with_context(|| format!("Failed to download asset {}", asset.id))?;

            let count = completed.fetch_add(1, Ordering::Relaxed) + 1;
            println!("  ✓ [{count}/{total}] Downloaded {file_name}");

            Ok::<(), anyhow::Error>(())
        }
    });

    let results: Vec<Result<()>> = download_stream.buffer_unordered(3).collect().await;

    for res in results {
        res?;
    }

    println!("🎉 All assets successfully exported to {}", out_dir.display());
    Ok(())
}
