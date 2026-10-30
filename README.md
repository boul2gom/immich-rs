# 🖼️ immich-rs

<div align="center">

**An asynchronous, type-safe, and resilient Rust SDK for [Immich](https://immich.app/) self-hosted photo & video management.**

[![CI](https://img.shields.io/github/actions/workflow/status/boul2gom/immich-rs/ci.yml?branch=main&label=CI&logo=github&style=flat-square)](https://github.com/boul2gom/immich-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/immich-rs?logo=rust&style=flat-square)](https://crates.io/crates/immich-rs)
[![docs.rs](https://img.shields.io/docsrs/immich-rs?logo=docs.rs&style=flat-square)](https://docs.rs/immich-rs)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-blue?logo=rust&style=flat-square)](https://www.rust-lang.org)
[![Edition](https://img.shields.io/badge/Edition-2024-orange?style=flat-square)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-green?style=flat-square)](https://github.com/boul2gom/immich-rs#license)
[![WASM Ready](https://img.shields.io/badge/WASM-browser%20ready-purple?logo=webassembly&style=flat-square)](https://webassembly.org/)

[Documentation](https://docs.rs/immich-rs) · [Report Bug](https://github.com/boul2gom/immich-rs/issues/new/choose) · [Request Feature](https://github.com/boul2gom/immich-rs/discussions/new?category=ideas) · [Discussions](https://github.com/boul2gom/immich-rs/discussions)

</div>

---

## 📑 Table of Contents

- [✨ Key Highlights](#-key-highlights)
- [📦 Installation & Feature Flags](#-installation--feature-flags)
- [🚀 Quick Start](#-quick-start)
  - [API Key Authentication](#api-key-authentication)
  - [Bearer Token Authentication](#bearer-token-authentication)
  - [Automatic Environment Resolution](#automatic-environment-resolution)
- [📖 Core Concepts & Guides](#-core-concepts--guides)
  - [1. Ergonomic Nested Sub-Clients](#1-ergonomic-nested-sub-clients)
  - [2. Strongly Typed Domain IDs](#2-strongly-typed-domain-ids)
  - [3. Fluent Request Builders & `.await`](#3-fluent-request-builders--await)
  - [4. Resilient Retries & Rate Limiting](#4-resilient-retries--rate-limiting)
  - [5. Memory-Efficient Streaming & Progress](#5-memory-efficient-streaming--progress)
  - [6. High-Throughput Bulk Uploads](#6-high-throughput-bulk-uploads)
  - [7. Real-time Events with Socket.IO](#7-real-time-events-with-socketio)
  - [8. Safe `OpenAPI` Forward Compatibility](#8-safe-openapi-forward-compatibility)
- [🌐 WebAssembly (WASM) & Browser Support](#-webassembly-wasm--browser-support)
- [🛠️ Examples Walkthrough](#️-examples-walkthrough)
- [💡 Features coming soon](#-features-coming-soon)
- [🤝 Contributing & Development](#-contributing--development)
- [📄 License & Acknowledgements](#-license--acknowledgements)

---

## ✨ Key Highlights

- **Complete `OpenAPI` Coverage**: 100% typed bindings generated strictly from the official Immich `OpenAPI` specification (`v3.3.5`).
- **20 `#[repr(transparent)]` Domain IDs**: Compile-time prevention of ID mixups (`AssetId`, `AlbumId`, `UserId`, `TagId`, etc.) with zero runtime overhead.
- **Hierarchical API Client**: Intuitive namespacing mirrorring Immich domains: `client.assets().faces()`, `client.users().admin()`, `client.system().config()`.
- **Atomic Direct-to-Disk Transfers**: Downloads stream directly to temporary files before atomic filesystem rename, ensuring partial downloads never corrupt your disk.
- **High-Throughput Ingestion**: Parallel bulk uploads with concurrent bounded SHA-1 checksum calculation and upstream deduplication check.
- **Enterprise-Grade Network Layer**: Idempotence-aware retry policies (500 retried only on idempotent methods), RFC 9110 `Retry-After` parsing, and token-bucket rate limiting.
- **Real-Time Event Streaming**: Asynchronous event streams over Socket.IO delivering asset uploads, deletions, facial updates, and custom plugin events.
- **Zero Credential Leaks**: API keys, bearer tokens, and secrets are strictly redacted in all `Debug` implementations and tracing instrumentation.
- **Universal Multi-Platform**: First-class support for native Tokio platforms as well as browser environments via `wasm32-unknown-unknown`.

---

## 📦 Installation & Feature Flags

Add `immich-rs` to your `Cargo.toml`:

```toml
[dependencies]
immich-rs = "0.1.0"
```

If you only need the `OpenAPI` data models and DTOs without network or HTTP dependencies (e.g. for event workers, webhooks, or custom clients):

```toml
[dependencies]
immich-models = "0.1.0"
```

### Feature Flags

| Feature | Description | Default |
|---|---|:---:|
| `default` | Enables `fs` and `rustls-tls`. | **Yes** |
| `fs` | Local filesystem operations: streaming file uploads, atomic downloads, and SHA-1 checksum helpers. | **Yes** |
| `rustls-tls` | Modern, secure TLS powered by `rustls` (pure Rust, no OpenSSL or C dependency). | **Yes** |
| `native-tls` | Platform-native TLS using system certificate stores (`SChannel` / Security Framework / OpenSSL). | No |
| `socketio` | Real-time event notifications via Socket.IO (`rust_socketio`). Native targets only. | No |
| `opentelemetry` | Metrics integration via OpenTelemetry. | No |

To use `native-tls` instead of `rustls-tls`:

```toml
[dependencies]
immich-rs = { version = "0.1.0", default-features = false, features = ["fs", "native-tls"] }
```

---

## 🚀 Quick Start

### API Key Authentication

```rust,no_run
use immich_rs::ImmichClient;

#[tokio::main]
async fn main() -> Result<(), immich_rs::Error> {
    let client = ImmichClient::builder()
        .base_url("https://photos.example.com/api")
        .api_key("your-api-key")
        .build()?;

    let albums = client.albums().get_all_albums().await?;
    println!("Found {} albums!", albums.len());
    for album in albums {
        println!("- {} (ID: {})", album.album_name, album.id);
    }

    Ok(())
}
```

### Bearer Token Authentication

```rust,no_run
use immich_models::authentication::LoginCredentialDTO;
use immich_rs::ImmichClient;

#[tokio::main]
async fn main() -> Result<(), immich_rs::Error> {
    // 1. Log in with an unauthenticated client
    let unauth = ImmichClient::builder()
        .base_url("https://photos.example.com/api")
        .build()?;

    let login_dto = LoginCredentialDTO::new("user@example.com", "secret-password");
    let login_resp = unauth.authentication().login(login_dto).await?;

    // 2. Build authenticated client with the received access token
    let client = ImmichClient::builder()
        .base_url("https://photos.example.com/api")
        .bearer_token(login_resp.access_token)
        .build()?;

    let me = client.users().get_my_user().await?;
    println!("Logged in as: {} ({})", me.name, me.email);

    Ok(())
}
```

### Automatic Environment Resolution

```rust,no_run
use immich_rs::ImmichClient;

#[tokio::main]
async fn main() -> Result<(), immich_rs::Error> {
    // Automatically resolves IMMICH_INSTANCE_URL (or IMMICH_URL, IMMICH_BASE_URL)
    // and IMMICH_API_KEY (or IMMICH_API_TOKEN, IMMICH_BEARER_TOKEN).
    let client = ImmichClient::from_env()?;

    let about = client.server().get_about_info().await?;
    println!("Connected to Immich v{}", about.version);

    Ok(())
}
```

---

## 📖 Core Concepts & Guides

### 1. Ergonomic Nested Sub-Clients

Rather than exposing hundreds of flat methods on a single monolithic struct, `immich-rs` groups APIs naturally into cohesive domain namespaces:

```rust,no_run
# use immich_rs::ImmichClient;
# async fn example(client: &ImmichClient) -> Result<(), immich_rs::Error> {
// Assets domain
let _faces = client.assets().faces();
let _stacks = client.assets().stacks();
let _downloads = client.assets().downloads();

// System administration
let _config = client.system().config();
let _maintenance = client.admin().maintenance();
let _user_admin = client.users().admin();
# Ok(())
# }
```

### 2. Strongly Typed Domain IDs

To eliminate accidental ID swapping at compile-time, all 20 Immich entity IDs are transparent newtypes:

```rust,no_run
use immich_models::id::{AlbumId, AssetId, UserId};
use uuid::Uuid;

let asset_id = AssetId::new(Uuid::new_v4());
let album_id = AlbumId::new(Uuid::new_v4());

// asset_id cannot be accidentally passed to an API expecting an album_id:
// client.albums().get_album_info(asset_id); // ❌ Compile error!
```

### 3. Fluent Request Builders & `.await`

All request builders implement [`std::future::IntoFuture`], allowing you to chain optional parameters fluently and directly `.await` the builder without calling `.send()`:

```rust,no_run
# use immich_rs::ImmichClient;
# async fn example(client: &ImmichClient) -> Result<(), immich_rs::Error> {
let people = client
    .people()
    .get_all_people()
    .with_hidden(true)
    .await?; // Direct .await!
# Ok(())
# }
```

### 4. Resilient Retries & Rate Limiting

The client automatically handles server backpressure, rate-limiting, and temporary network disruptions:

```rust,no_run
use std::time::Duration;
use immich_rs::ImmichClient;

let client = ImmichClient::builder()
    .base_url("https://photos.example.com/api")
    .api_key("secret")
    .max_retries(5)
    .rate_limit_per_second(10) // Smooth token-bucket rate limiter
    .connect_timeout(Duration::from_secs(5))
    .global_timeout(Duration::from_secs(30))
    .build()?;
# Ok::<(), immich_rs::Error>(())
```

- **Idempotence-aware**: HTTP `500 Internal Server Error` is safely retried on idempotent methods (`GET`, `PUT`, `DELETE`), but **never** on mutating operations (`POST`).
- **RFC 9110 Compliant**: Respects `Retry-After` headers expressed either in whole seconds or RFC 9110 HTTP dates.
- **Full Jitter**: Exponential backoff uses randomized decorrelated jitter to prevent thundering herd problems.

### 5. Memory-Efficient Streaming & Progress

Download full-resolution raw photos and videos directly to disk without buffering in RAM:

```rust,no_run
# use immich_rs::ImmichClient;
# use immich_models::id::AssetId;
# use std::path::Path;
# async fn example(client: &ImmichClient, asset_id: AssetId) -> Result<(), immich_rs::Error> {
use std::sync::Arc;
use immich_rs::ProgressSender;

let progress = ProgressSender::Callback(Arc::new(|p| {
    if let Some(pct) = p.percentage() {
        println!("Progress: {pct:.1}% ({}/{} bytes)", p.transferred, p.total.unwrap_or(0));
    }
}));

client.download_with_progress(asset_id, Path::new("photo.jpg"), progress).await?;
# Ok(())
# }
```

> [!NOTE]
> Atomic safety: downloads are written to `.photo.jpg.<asset_id>.tmp`. If the download is cancelled or fails, the temporary file is immediately removed via RAII guard.

### 6. High-Throughput Bulk Uploads

Effortlessly upload large photo libraries with automatic SHA-1 checksumming, upstream duplicate rejection, and bounded concurrency:

```rust,no_run
# use immich_rs::ImmichClient;
# async fn example(client: &ImmichClient) -> Result<(), Box<dyn std::error::Error>> {
use std::path::PathBuf;
use std::pin::pin;
use futures_util::StreamExt;
use immich_rs::{BulkUploadResult, UploadOptions};

let f1 = PathBuf::from("IMG_0001.JPG");
let f2 = PathBuf::from("IMG_0002.JPG");
let files: Vec<UploadOptions<'_>> = vec![
    UploadOptions::new(&f1),
    UploadOptions::new(&f2),
];

let mut stream = pin!(
    client
        .upload_bulk(files)
        .concurrency(4)
        .check_duplicates(true)
        .into_stream()
);

while let Some(res) = stream.next().await {
    match res? {
        BulkUploadResult::Uploaded(asset) => println!("✓ Uploaded: {}", asset.id),
        BulkUploadResult::Duplicate { path, .. } => println!("↷ Skipped duplicate: {path:?}"),
        BulkUploadResult::Rejected { path, reason } => println!("✗ Rejected: {path:?} ({reason:?})"),
        _ => {}
    }
}
# Ok(())
# }
```

### 7. Real-time Events with Socket.IO

Listen to live server events such as upload completions, asset deletions, and background jobs:

```rust,no_run
# #[cfg(feature = "socketio")]
# async fn example(client: &immich_rs::ImmichClient) -> Result<(), immich_rs::Error> {
use immich_rs::ServerEvent;

let mut sub = client.subscribe_events().await?;
println!("Subscribed to real-time events!");

while let Some(event) = sub.next_event().await {
    match event {
        ServerEvent::UploadSuccess(val) => println!("📸 New asset uploaded: {val}"),
        ServerEvent::AssetDelete(val) => println!("🗑️ Asset deleted: {val}"),
        ServerEvent::PersonThumbnail(val) => println!("👤 Facial recognition updated: {val}"),
        ServerEvent::Other { event, payload } => println!("🔔 [{event}] {payload}"),
        _ => {}
    }
}
# Ok(())
# }
```

### 8. Safe `OpenAPI` Forward Compatibility

All generated enums support **lossless forward compatibility**:
- If a future Immich server version introduces a new enum variant (e.g. `"live_photo_v2"`), deserialization will **never panic or fail**.
- The string is stored losslessly in `MyEnum::Unknown(String)` and re-serialized identically.
- Structs and enums are marked `#[non_exhaustive]` to ensure backward compatibility across SDK updates.

---

## 🌐 WebAssembly (WASM) & Browser Support

`immich-rs` fully supports the `wasm32-unknown-unknown` target for frontend web applications (e.g., Yew, Dioxus, Leptos):

```bash
cargo check -p immich-rs --target wasm32-unknown-unknown --no-default-features --features rustls-tls
```

In WASM environments:
- Filesystem-specific features (`download`, `upload_bulk`) are conditionally disabled.
- In-memory uploads are supported via [`ImmichClient::upload_asset_bytes`](https://docs.rs/immich-rs/latest/immich_rs/struct.ImmichClient.html#method.upload_asset_bytes).
- High-resolution timers and async sleeps automatically use `web-time` and `gloo-timers`.

---

## 🛠️ Examples Walkthrough

Explore complete executable scripts in the [`immich-rs/examples`](immich-rs/examples) directory:

| Example | Command | Description |
|---|---|---|
| `ping_server` | `cargo run -p immich-rs --example ping_server` | Test server connection and response latency. |
| `list_albums` | `cargo run -p immich-rs --example list_albums` | Fetch and list all accessible albums. |
| `paginate_people` | `cargo run -p immich-rs --example paginate_people` | Stream people entities using page-based pagination. |
| `upload_asset` | `cargo run -p immich-rs --example upload_asset` | Upload a single photo with metadata and MIME detection. |
| `bulk_upload` | `cargo run -p immich-rs --example bulk_upload` | High-concurrency bulk ingestion with SHA-1 deduplication. |
| `cleanup_duplicates` | `cargo run -p immich-rs --example cleanup_duplicates` | Inspect server-detected duplicate asset groups. |
| `export_album` | `cargo run -p immich-rs --example export_album` | Download an entire album to disk with live progress callbacks. |
| `live_events` | `cargo run -p immich-rs --example live_events --features socketio` | Subscribe to live Socket.IO server events. |
| `server_stats` | `cargo run -p immich-rs --example server_stats` | Display server version compatibility, storage, and statistics. |

---

## 💡 Features coming soon

- [ ] **Resumable Chunked Uploads**: TUS or custom chunked upload protocol support for huge multi-gigabyte video files.
- [ ] **CLI Tool (`immich-cli`)**: Standalone binary for command-line backup, sync, and album management.
- [ ] **Client Caching**: Optional cache middleware for thumbnail images and repeated immutable asset queries.
- [ ] **OpenTelemetry Tracing Middleware**: Distributed tracing propagation via W3C `TraceContext` headers.

---

## 🤝 Contributing & Development

Contributions are welcome! Please check out [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md) before opening a pull request.

### Local Workflow & Codegen

The codebase uses an internal `xtask` orchestrator to generate and sanitize API models directly from the Immich `OpenAPI` specification:

```bash
# Regenerate models and client bindings
cargo xtask generate

# Run the complete test suite
cargo test --workspace --all-features

# Verify code formatting and strict pedantic lints
cargo +nightly fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Verify WASM build compatibility
cargo check -p immich-rs --target wasm32-unknown-unknown --no-default-features
```

---

## 📄 License & Acknowledgements

Dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)

at your option.

Special thanks to the [Immich Team](https://github.com/immich-app/immich) for building an exceptional self-hosted photo management platform.
