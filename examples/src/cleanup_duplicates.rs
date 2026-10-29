//! Inspect and analyze duplicate assets detected by Immich.
//!
//! # Usage
//! ```sh
//! export IMMICH_INSTANCE_URL="https://photos.example.com/api"
//! export IMMICH_API_KEY="your-api-key"
//! cargo run -p examples --bin cleanup_duplicates
//! ```

use anyhow::{Context, Result};
use immich_rs::ImmichClient;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let client = ImmichClient::from_env()
        .context("Failed to initialize ImmichClient from IMMICH_INSTANCE_URL and IMMICH_API_KEY")?;

    println!("🔍 Fetching duplicate asset groups from {}", client.base_url());
    let duplicate_groups = client
        .duplicates()
        .get_asset_duplicates()
        .await
        .context("Failed to retrieve duplicate asset groups")?;

    if duplicate_groups.is_empty() {
        println!("✨ No duplicate assets detected on the server! Everything is clean.");
        return Ok(());
    }

    println!(
        "⚠️ Found {} duplicate group(s) on your Immich instance:\n",
        duplicate_groups.len()
    );

    let mut total_redundant_assets = 0;

    for (idx, group) in duplicate_groups.iter().enumerate() {
        let count = group.assets.len();
        let redundant = count.saturating_sub(1);
        total_redundant_assets += redundant;

        println!(
            "📦 Group #{idx} (ID: {}) — {} copies ({} redundant):",
            group.duplicate_id, count, redundant
        );

        for asset in &group.assets {
            let is_suggested_keep = group.suggested_keep_asset_ids.contains(&asset.id);
            let marker = if is_suggested_keep {
                "🌟 KEEP"
            } else {
                "🗑️ TRASH"
            };

            println!(
                "   [{marker}] ID: {} | File: {}",
                asset.id, asset.original_file_name
            );
        }
        println!();
    }

    println!("📊 Summary:");
    println!("   Total duplicate groups:   {}", duplicate_groups.len());
    println!("   Total redundant copies:   {total_redundant_assets}");
    println!(
        "\n💡 Tip: You can resolve duplicate groups automatically via `client.duplicates().resolve_duplicates(&dto).await`"
    );

    Ok(())
}
