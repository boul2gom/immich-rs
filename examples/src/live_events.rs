//! Listen to real-time Immich events via Socket.IO.
//!
//! # Usage
//! ```sh
//! export IMMICH_INSTANCE_URL="https://photos.example.com/api"
//! export IMMICH_API_KEY="your-api-key"
//! cargo run -p examples --bin live_events
//! ```

use anyhow::{Context, Result};
use immich_rs::{ImmichClient, ServerEvent};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let client = ImmichClient::from_env()
        .context("Failed to initialize ImmichClient from IMMICH_INSTANCE_URL and IMMICH_API_KEY")?;

    println!("⚡ Connecting to Immich real-time events at {}", client.base_url());
    let mut subscription = client
        .subscribe_events()
        .await
        .context("Failed to subscribe to Immich Socket.IO event stream")?;

    println!("🟢 Connected! Listening for real-time events (Press Ctrl+C to stop)...\n");

    tokio::select! {
        () = async {
            while let Some(event) = subscription.next_event().await {
                match event {
                    ServerEvent::UploadSuccess(val) => {
                        println!("📸 [UPLOAD SUCCESS] New asset added: {val}");
                    }
                    ServerEvent::AssetDelete(val) => {
                        println!("🗑️ [ASSET DELETE] Asset removed: {val}");
                    }
                    ServerEvent::AssetTrash(val) => {
                        println!("🚮 [ASSET TRASH] Asset trashed: {val}");
                    }
                    ServerEvent::PersonThumbnail(val) => {
                        println!("👤 [PERSON UPDATE] Facial recognition update: {val}");
                    }
                    ServerEvent::ServerVersion(val) => {
                        println!("ℹ️ [SERVER VERSION] Version notification: {val}");
                    }
                    ServerEvent::ConfigUpdate(val) => {
                        println!("⚙️ [CONFIG UPDATE] Configuration updated: {val}");
                    }
                    ServerEvent::Other { event, payload } => {
                        println!("🔔 [{event}] Custom event received: {payload}");
                    }
                }
            }
        } => {}
        _ = tokio::signal::ctrl_c() => {
            println!("\n🛑 Interrupted by user. Disconnecting...");
            subscription.disconnect().await?;
        }
    }

    println!("👋 Disconnected cleanly.");
    Ok(())
}
