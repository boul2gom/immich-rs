//! Paginates through people in the Immich server using `.into_paginator().pages()`.
//!
//! # Usage
//! ```bash
//! IMMICH_URL=http://localhost:2283/api/ IMMICH_API_KEY=your_key cargo run -p examples --bin paginate_people
//! ```

use std::pin::pin;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use immich_rs::ImmichClient;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let base_url = std::env::var("IMMICH_URL").unwrap_or_else(|_| "http://localhost:2283/api/".to_owned());
    let api_key = std::env::var("IMMICH_API_KEY").context("IMMICH_API_KEY environment variable is not set")?;

    let client = ImmichClient::builder()
        .base_url(base_url)
        .api_key(api_key)
        .build()
        .context("failed to build Immich client")?;

    let mut pages = pin!(
        client
            .people()
            .get_all_people()
            .with_hidden(true)
            .into_paginator()
            .pages()
    );

    let mut total = 0;
    while let Some(page) = pages.next().await {
        let people = page.context("failed to fetch page of people")?;
        total += people.len();
        for person in people {
            println!("- Person ID: {}, Name: {}", person.id, person.name);
        }
    }

    println!("Found {total} people via pagination stream!");
    Ok(())
}
