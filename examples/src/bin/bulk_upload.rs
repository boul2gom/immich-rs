//! Bulk uploads assets to the Immich server using `ImmichClient::upload_bulk`.
//!
//! # Usage
//! ```bash
//! IMMICH_URL=http://localhost:2283/api/ IMMICH_API_KEY=your_key cargo run -p examples --bin bulk_upload
//! ```

use std::env;
use std::path::PathBuf;
use std::pin::pin;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use immich_rs::prelude::*;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let directive = "immich_rs=trace".parse().context("failed to parse log directive")?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(directive))
        .init();

    let server_url = env::var("IMMICH_URL").unwrap_or_else(|_| "http://localhost:2283/api/".to_string());
    let api_key = env::var("IMMICH_API_KEY").context("IMMICH_API_KEY not set")?;

    let client = ImmichClient::builder()
        .base_url(server_url)
        .api_key(api_key)
        .build()
        .context("failed to build Immich client")?;

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.parent().unwrap_or(&manifest_dir).to_path_buf();

    let mut files = Vec::new();
    for i in 1..=10 {
        let path = workspace_root.join(format!("immich-rs/tests/assets/dummy_{i}.jpg"));
        if path.exists() {
            files.push(path);
        }
    }

    if files.is_empty() {
        tracing::error!("No test assets found in immich-rs/tests/assets/.");
        return Ok(());
    }

    tracing::info!("Starting bulk upload of {} files", files.len());

    let items: Vec<UploadOptions<'_>> = files
        .iter()
        .map(|p| UploadOptions::new(p).device_id("immich-rs-example-client"))
        .collect();

    let mut stream = pin!(client.upload_bulk(items).concurrency(4).into_stream());
    let mut success_count = 0usize;

    while let Some(res) = stream.next().await {
        match res {
            Ok(immich_rs::BulkUploadResult::Uploaded(media)) => {
                tracing::info!("Finished uploading asset (ID: {})", media.id);
                success_count += 1;
            }
            Ok(immich_rs::BulkUploadResult::Duplicate {
                path,
                existing_asset_id,
            }) => {
                tracing::info!(
                    "Skipped duplicate asset {:?} (Existing ID: {:?})",
                    path,
                    existing_asset_id
                );
                success_count += 1;
            }
            Err(e) => {
                tracing::warn!("Asset upload failed: {e:?}");
            }
        }
    }

    tracing::info!("Bulk upload completed: {success_count}/{} successful", files.len());
    Ok(())
}
