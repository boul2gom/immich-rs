<h2 align="center">🖼️ An asynchronous, type-safe Rust SDK for Immich</h2>

<div align="center">
  A strongly typed, high-performance client for the <a href="https://immich.app/">Immich</a> self-hosted photo & video backup server.<br>
  Built on Tokio and Reqwest, featuring fluent builders, chunked streaming uploads, direct-to-disk downloads, automatic retries with jitter, and real-time events.
</div>

<br>
<div align="center">⚠️ Part of the <a href="https://github.com/boul2gom/immich-rs">immich-rs</a> workspace. Published on <a href="https://crates.io/crates/immich-rs">crates.io</a>.</div>
<br>

<div align="center">
  <a href="https://github.com/boul2gom/immich-rs/issues/new/choose">Report a Bug</a>
  ·
  <a href="https://github.com/boul2gom/immich-rs/discussions/new?category=ideas">Request a Feature</a>
  ·
  <a href="https://github.com/boul2gom/immich-rs/discussions/new?category=q-a">Ask a Question</a>
</div>

---

<p align="center">
  <a href="https://github.com/boul2gom/immich-rs/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/boul2gom/immich-rs/ci.yml?branch=main&label=CI&logo=Github" alt="CI"/>
  </a>
  <a href="https://crates.io/crates/immich-rs">
    <img src="https://img.shields.io/crates/v/immich-rs?label=crates.io&logo=Rust" alt="crates.io"/>
  </a>
  <a href="https://crates.io/crates/immich-rs">
    <img src="https://img.shields.io/crates/d/immich-rs?label=Downloads&logo=Rust" alt="Downloads"/>
  </a>
  <a href="https://docs.rs/immich-rs">
    <img src="https://img.shields.io/docsrs/immich-rs?label=docs.rs&logo=Rust" alt="docs.rs"/>
  </a>
</p>
<p align="center">
  <a href="https://github.com/boul2gom/immich-rs/discussions">
    <img src="https://img.shields.io/github/discussions/boul2gom/immich-rs?label=Discussions&logo=Github" alt="Discussions">
  </a>
  <a href="https://github.com/boul2gom/immich-rs/issues">
    <img src="https://img.shields.io/github/issues-raw/boul2gom/immich-rs?label=Issues&logo=Github" alt="Issues">
  </a>
  <a href="https://github.com/boul2gom/immich-rs/pulls">
    <img src="https://img.shields.io/github/issues-pr-raw/boul2gom/immich-rs?label=Pull%20requests&logo=Github" alt="Pull requests">
  </a>
</p>
<p align="center">
  <a href="https://github.com/boul2gom/immich-rs/blob/main/LICENSE.md">
    <img src="https://img.shields.io/github/license/boul2gom/immich-rs?label=License&logo=Github" alt="License">
  </a>
  <a href="https://github.com/boul2gom/immich-rs/stargazers">
    <img src="https://img.shields.io/github/stars/boul2gom/immich-rs?label=Stars&logo=Github" alt="Stars">
  </a>
  <a href="https://github.com/boul2gom/immich-rs/fork">
    <img src="https://img.shields.io/github/forks/boul2gom/immich-rs?label=Forks&logo=Github" alt="Forks">
  </a>
</p>

---

## ⚡ Features

- **Full OpenAPI Coverage**: Models and typed API endpoints strictly derived from the official Immich OpenAPI specification (`v3.3.5`).
- **Fluent & Ergonomic API**: Chained request builders supporting both `.send().await` and direct `.await` via `IntoFuture`.
- **Resilient Network Layer**: Automated retry of transient network errors (HTTP `429`, `500`, `502`, `503`, `504`) with exponential backoff and jitter.
- **Streaming Uploads & Atomic Downloads**:
  - Memory-efficient streaming uploads with SHA-1 checksum calculation and transfer progress hooks.
  - Direct-to-disk atomic downloads (writes to temporary files before atomic rename to avoid corrupted partial files).
  - High-performance parallel uploads with `BulkUploadBuilder`.
- **Real-time Event Streaming**: Listen to Immich server events and background jobs via Socket.io.
- **Observability & Security**:
  - Structured logging with [`tracing`](https://crates.io/crates/tracing) and optional OpenTelemetry metrics.
  - Zero credential leakage: API keys and sensitive tokens are strictly redacted in `Debug` implementations.
- **WASM Support**: Compatible with `wasm32-unknown-unknown` targets.

---

## 📥 Installation

Add `immich-rs` to your `Cargo.toml`:

```toml
[dependencies]
immich-rs = "0.1.0"
```

### Feature Flags

| Feature | Description | Default |
|---|---|---|
| `fs` | Enables local file system helpers (file upload, checksum calculation, atomic downloads). | **Yes** |
| `rustls-tls` | Enables modern TLS using `rustls` (pure Rust, no OpenSSL dependency). | **Yes** |
| `native-tls` | Enables platform-native TLS using system certificates (`OpenSSL` / SChannel / Security Framework). | No |
| `socketio` | Enables the real-time event client via Socket.io / WebSocket (`rust_socketio`). | No |
| `opentelemetry` | Exposes client metrics via OpenTelemetry integration. | No |

To use `native-tls` instead of `rustls-tls`:

```toml
[dependencies]
immich-rs = { version = "0.1.0", default-features = false, features = ["fs", "native-tls"] }
```

---

## 🚀 Quickstart

### 1. Initialize the Client

```rust,no_run
use immich_rs::ImmichClient;

#[tokio::main]
async fn main() -> Result<(), immich_rs::Error> {
    // Direct initialization
    let client = ImmichClient::new("https://photos.example.com/api", "YOUR_API_KEY")?;

    // Or load automatically from IMMICH_INSTANCE_URL and IMMICH_API_KEY
    // let client = ImmichClient::from_env()?;

    Ok(())
}
```

Or configure custom retry policies, rate limits, or timeouts with `ImmichClientBuilder`:

```rust,no_run
use std::time::Duration;
use immich_rs::ImmichClientBuilder;

#[tokio::main]
async fn main() -> Result<(), immich_rs::Error> {
    let client = ImmichClientBuilder::new("https://photos.example.com/api")
        .api_key("YOUR_API_KEY")
        .timeout(Duration::from_secs(30))
        .max_retries(5)
        .build()?;

    Ok(())
}
```

### 2. Query Albums and Assets

```rust,no_run
use immich_rs::ImmichClient;

#[tokio::main]
async fn main() -> Result<(), immich_rs::Error> {
    let client = ImmichClient::from_env()?;

    // Fetch all albums
    let albums = client.albums().get_all_albums().send().await?;
    for album in albums {
        println!("Album: {} (ID: {})", album.album_name, album.id);
    }

    // Search assets
    let results = client.search()
        .search_assets()
        .query("sunset")
        .send()
        .await?;

    println!("Found {} assets matching search.", results.assets.items.len());
    Ok(())
}
```

### 3. Upload Media with Progress Tracking

```rust,no_run
use std::path::Path;
use immich_rs::{ImmichClient, UploadOptions, ProgressSender};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ImmichClient::from_env()?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            println!("Uploaded {} / {} bytes", progress.bytes_sent, progress.total_bytes);
        }
    });

    let options = UploadOptions::default()
        .progress(ProgressSender::from_unbounded(tx));

    let asset = client.upload_asset(Path::new("photo.jpg"), options).await?;
    println!("Uploaded asset successfully: {}", asset.id);

    Ok(())
}
```

### 4. Bulk Uploads

For high-throughput asset ingestion:

```rust,no_run
use std::path::PathBuf;
use immich_rs::{ImmichClient, BulkUploadBuilder, BulkUploadItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ImmichClient::from_env()?;

    let items = vec![
        BulkUploadItem::new(PathBuf::from("IMG_0001.JPG")),
        BulkUploadItem::new(PathBuf::from("IMG_0002.JPG")),
    ];

    let result = BulkUploadBuilder::new(&client)
        .concurrency(4)
        .items(items)
        .run()
        .await?;

    println!("Bulk upload completed: {} succeeded, {} failed", result.succeeded.len(), result.failed.len());
    Ok(())
}
```

### 5. Real-time Events (Socket.io)

Enable the `socketio` feature flag to subscribe to real-time events:

```rust,no_run
#[cfg(feature = "socketio")]
use immich_rs::{ImmichClient, ServerEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "socketio")]
    {
        let client = ImmichClient::from_env()?;
        let mut subscription = client.events().subscribe().await?;

        while let Some(event) = subscription.next().await {
            match event {
                ServerEvent::AssetUploaded { asset_id } => println!("New asset: {asset_id}"),
                ServerEvent::JobStatus { job, status } => println!("Job {job:?}: {status:?}"),
                _ => {}
            }
        }
    }
    Ok(())
}
```

---

## 🏛️ Workspace Architecture

The `immich-rs` repository is organized as a Cargo workspace:

| Crate | Purpose | Crates.io |
|---|---|---|
| [`immich-rs`](./immich-rs) | High-level ergonomic async SDK with retry, uploads, downloads, and events. | [![crates.io](https://img.shields.io/crates/v/immich-rs)](https://crates.io/crates/immich-rs) |
| [`immich-models`](./immich-models) | Standalone DTOs and OpenAPI data models (decoupled from any HTTP client). | [![crates.io](https://img.shields.io/crates/v/immich-models)](https://crates.io/crates/immich-models) |
| `xtask` | Internal CLI used to fetch Immich OpenAPI specs and generate models. | *Internal* |
| `examples` | Ready-to-run code samples demonstrating real workflows. | *Internal* |

If you only need standalone OpenAPI data transfer objects (DTOs) without an HTTP client:

```toml
[dependencies]
immich-models = "0.1.0"
```

---

## 🗺️ API Coverage

The SDK provides organized access to the full Immich API surface:

| Module | Accessor | Description |
|---|---|---|
| `activities` | `client.activities()` | Likes, comments, and activity feeds. |
| `albums` | `client.albums()` | Album creation, asset assignment, sharing, and management. |
| `api_keys` | `client.api_keys()` | API key lifecycle and permissions. |
| `assets` | `client.assets()` | Asset upload, download, metadata, and deletion. |
| `authentication` | `client.authentication()` | Login, logout, OAuth, and credential exchange. |
| `jobs` | `client.jobs()` | Server background jobs (thumbnail generation, face detection, transcode). |
| `libraries` | `client.libraries()` | External and internal library paths and scans. |
| `people` | `client.people()` | Facial recognition, person naming, and thumbnail management. |
| `search` | `client.search()` | Smart search, metadata search, and person search. |
| `server` | `client.server()` | Server version, ping, stats, and configuration. |
| `system` | `client.system()` | System storage, hardware acceleration, and diagnostics. |
| `tags` | `client.tags()` | Tag taxonomy and asset association. |
| `trash` | `client.trash()` | Emptying trash and restoring soft-deleted items. |
| `users` | `client.users()` | User management, quotas, and profiles. |

---

## 🤝 Contributing

Contributions are warmly welcome!
1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/amazing-feature`).
3. Commit your changes (`git commit -m 'feat: add amazing feature'`).
4. Push to your branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

Please make sure `cargo check --workspace`, `cargo test --workspace`, and `cargo clippy --workspace` pass cleanly.

---

## 📄 License

This project is licensed under the [GNU General Public License v3.0](LICENSE.md).
