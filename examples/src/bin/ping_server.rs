//! Pings the Immich server and prints the response.
//!
//! # Usage
//! ```bash
//! IMMICH_URL=http://localhost:2283/api/ cargo run -p examples --bin ping_server
//! ```

use anyhow::{Context, Result};
use immich_rs::ImmichClient;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let base_url = std::env::var("IMMICH_URL").unwrap_or_else(|_| "http://localhost:2283/api/".to_owned());

    let mut builder = ImmichClient::builder().base_url(base_url);
    if let Ok(key) = std::env::var("IMMICH_API_KEY")
        && !key.trim().is_empty()
    {
        builder = builder.api_key(key);
    }

    let client = builder.build().context("failed to build Immich client")?;

    let ping = client.server().ping_server().await.context("failed to ping server")?;

    println!("Server response: {ping:?}");
    Ok(())
}
