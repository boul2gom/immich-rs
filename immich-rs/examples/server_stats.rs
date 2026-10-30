//! Query server version compatibility, storage metrics, and asset statistics.
//!
//! # Usage
//! ```sh
//! export IMMICH_INSTANCE_URL="https://photos.example.com/api"
//! export IMMICH_API_KEY="your-api-key"
//! cargo run -p immich-rs --example server_stats
//! ```

#![allow(clippy::print_stdout)]

use anyhow::{Context, Result};
use immich_rs::{Compatibility, ImmichClient, TARGET_IMMICH_VERSION};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let client = ImmichClient::from_env()
        .context("Failed to initialize ImmichClient from IMMICH_INSTANCE_URL and IMMICH_API_KEY")?;

    println!("=======================================================");
    println!("          IMMICH SERVER DASHBOARD & HEALTH             ");
    println!("=======================================================");
    println!("Target URL: {}", client.base_url());

    // 1. Version Handshake & Compatibility
    println!("\n🔍 Running Version Handshake...");
    let compat = client
        .check_compatibility()
        .await
        .context("Failed to evaluate server version compatibility")?;

    let version_dto = client
        .server()
        .get_server_version()
        .await
        .context("Failed to fetch server version")?;

    let server_ver = format!("{}.{}.{}", version_dto.major, version_dto.minor, version_dto.patch);

    println!("   SDK Built-in Target: v{TARGET_IMMICH_VERSION}");
    println!("   Remote Immich:       v{server_ver}");

    match compat {
        Compatibility::ExactMatch { .. } => {
            println!("   Status:              ✅ Exact Match (100% verified compatibility)");
        }
        Compatibility::Compatible { .. } => {
            println!("   Status:              🟢 Compatible (minor/patch revision difference)");
        }
        Compatibility::ServerOlder { .. } => {
            println!("   Status:              ⚠️ Server is older than SDK target; newer endpoints may 404");
        }
        Compatibility::ServerNewer { .. } => {
            println!("   Status:              ⚠️ Server is newer than SDK target; deprecated fields may differ");
        }
        Compatibility::Unparsable { server, target } => {
            println!("   Status:              ❓ Unparsable version format: server={server}, target={target}");
        }
        _ => {
            println!("   Status:              Unknown compatibility state");
        }
    }

    // 2. Storage Metrics
    println!("\n💾 Querying Disk & Storage Health...");
    match client.server().get_storage().await {
        Ok(storage) => {
            println!("   Total Disk Size:     {}", storage.disk_size);
            println!("   Available Disk:      {}", storage.disk_available);
            println!("   Used Disk:           {}", storage.disk_use);
            println!("   Usage Percentage:    {:.1}%", storage.disk_usage_percentage);
        }
        Err(e) => {
            println!("   ⚠️ Storage details unavailable (insufficient admin privileges: {e})");
        }
    }

    // 3. Asset & Library Statistics
    println!("\n📊 Querying Media Library Statistics...");
    match client.server().get_server_statistics().await {
        Ok(stats) => {
            println!("   Total Photos:        {}", stats.photos);
            println!("   Total Videos:        {}", stats.videos);
            #[allow(clippy::cast_precision_loss)]
            let total_gb = (stats.usage as f64) / (1024.0 * 1024.0 * 1024.0);
            #[allow(clippy::cast_precision_loss)]
            let photo_gb = (stats.usage_photos as f64) / (1024.0 * 1024.0 * 1024.0);
            #[allow(clippy::cast_precision_loss)]
            let video_gb = (stats.usage_videos as f64) / (1024.0 * 1024.0 * 1024.0);

            let usage_bytes = stats.usage;
            println!("   Total Storage Used:  {total_gb:.2} GB ({usage_bytes} bytes)");
            println!("     ├─ Photos:         {photo_gb:.2} GB");
            println!("     └─ Videos:         {video_gb:.2} GB");
            println!("   Active Users:        {}", stats.usage_by_user.len());
        }
        Err(e) => {
            println!("   ⚠️ Statistics unavailable (requires admin permissions: {e})");
        }
    }

    println!("\n=======================================================");
    println!("Dashboard query completed successfully.");
    Ok(())
}
