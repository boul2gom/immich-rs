//! Uploads a local file as an asset to the Immich server.
//!
//! # Usage
//! ```bash
//! IMMICH_URL=http://localhost:2283/api/ IMMICH_API_KEY=your_key IMMICH_FILE_PATH=/path/to/photo.jpg cargo run -p examples --bin upload_asset
//! ```

use anyhow::{Context, Result};
use immich_rs::{ImmichClient, UploadOptions};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let base_url = std::env::var("IMMICH_URL").unwrap_or_else(|_| "http://localhost:2283/api/".to_owned());
    let api_key = std::env::var("IMMICH_API_KEY").context("IMMICH_API_KEY environment variable is not set")?;
    let file_path = std::env::var("IMMICH_FILE_PATH").context("IMMICH_FILE_PATH environment variable is not set")?;

    let client = ImmichClient::builder()
        .base_url(base_url)
        .api_key(api_key)
        .build()
        .context("failed to build Immich client")?;

    let path = std::path::Path::new(&file_path);
    let options = UploadOptions::new(path).device_id("immich-rs-example");

    let response = client.upload_asset(options).await.context("failed to upload asset")?;

    println!("Uploaded asset: id={}", response.id);
    Ok(())
}
