<h2 align="center">🖼️ An asynchronous, type-safe, and resilient Rust SDK for Immich</h2>

<div align="center">
  A high-performance Rust client and typed domain models for <a href="https://immich.app/"><strong>Immich</strong></a> self-hosted photo &amp; video management, generated strictly from official OpenAPI specifications.
</div>
<div align="center">
  Features typed domain IDs, nested sub-clients, atomic file streaming, real-time events over Socket.IO, and WebAssembly compatibility.
</div>

<br>
<div align="center">💬 The project is actively maintained. If you encounter any bugs or have feature requests, please feel free to open an issue or discussion.</div>
<br>

<div align="center">
  <a href="https://docs.rs/immich-rs">Documentation</a>
  ·
  <a href="https://github.com/boul2gom/immich-rs/issues/new/choose">Report a Bug</a>
  ·
  <a href="https://github.com/boul2gom/immich-rs/discussions/new?category=ideas">Request a Feature</a>
  ·
  <a href="https://github.com/boul2gom/immich-rs/discussions">Discussions</a>
</div>

---

<p align="center">
  <a href="https://github.com/boul2gom/immich-rs/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/boul2gom/immich-rs/ci.yml?branch=main&label=CI&logo=github" alt="CI"/>
  </a>
  <a href="https://crates.io/crates/immich-rs">
    <img src="https://img.shields.io/crates/v/immich-rs?label=Release&logo=rust" alt="Release"/>
  </a>
  <a href="https://docs.rs/immich-rs">
    <img src="https://img.shields.io/docsrs/immich-rs?label=docs.rs&logo=docs.rs" alt="Documentation"/>
  </a>
  <a href="https://crates.io/crates/immich-rs">
    <img src="https://img.shields.io/crates/d/immich-rs?label=Downloads&logo=rust" alt="Downloads"/>
  </a>
</p>
<p align="center">
  <a href="https://github.com/boul2gom/immich-rs/discussions">
    <img src="https://img.shields.io/github/discussions/boul2gom/immich-rs?label=Discussions&logo=github" alt="Discussions">
  </a>
  <a href="https://github.com/boul2gom/immich-rs/issues">
    <img src="https://img.shields.io/github/issues-raw/boul2gom/immich-rs?label=Issues&logo=github" alt="Issues">
  </a>
  <a href="https://github.com/boul2gom/immich-rs/pulls">
    <img src="https://img.shields.io/github/issues-pr-raw/boul2gom/immich-rs?label=Pull%20requests&logo=github" alt="Pull requests">
  </a>
</p>
<p align="center">
  <a href="https://github.com/boul2gom/immich-rs#license">
    <img src="https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-green" alt="License">
  </a>
  <a href="https://github.com/boul2gom/immich-rs/stargazers">
    <img src="https://img.shields.io/github/stars/boul2gom/immich-rs?label=Stars&logo=github" alt="Stars">
  </a>
  <a href="https://github.com/boul2gom/immich-rs/fork">
    <img src="https://img.shields.io/github/forks/boul2gom/immich-rs?label=Forks&logo=github" alt="Forks">
  </a>
  <a href="https://webassembly.org/">
    <img src="https://img.shields.io/badge/WASM-browser%20ready-purple?logo=webassembly" alt="WASM Ready">
  </a>
</p>

---

## 📑 Table of Contents

- [💭 Why immich-rs?](#-why-immich-rs)
- [✨ Key Highlights](#-key-highlights)
- [📥 How to get it](#-how-to-get-it)
- [🔌 Optional features](#-optional-features)
- [🚀 Quick Start](#-quick-start)
  - [API Key Authentication](#api-key-authentication)
  - [Bearer Token Authentication & Live Login](#bearer-token-authentication--live-login)
  - [Automatic Environment & CLI Discovery](#automatic-environment--cli-discovery)
- [📖 Core Concepts & Guides](#-core-concepts--guides)
  - [1. Ergonomic Nested Sub-Clients](#1-ergonomic-nested-sub-clients)
  - [2. Strongly Typed Domain IDs](#2-strongly-typed-domain-ids)
  - [3. Fluent Request Builders & `.await`](#3-fluent-request-builders--await)
  - [4. Seamless Stream Pagination (`PaginationStream`)](#4-seamless-stream-pagination-paginationstream)
  - [5. Unified Binary Streaming (`ByteStream`)](#5-unified-binary-streaming-bytestream)
  - [6. Resilient Retries & Rate Limiting](#6-resilient-retries--rate-limiting)
  - [7. Dynamic Token Rotation & Live Auth](#7-dynamic-token-rotation--live-auth)
  - [8. Low-Level HTTP Escape Hatch](#8-low-level-http-escape-hatch)
  - [9. Memory-Efficient Streaming & Progress](#9-memory-efficient-streaming--progress)
  - [10. High-Throughput Bulk Uploads](#10-high-throughput-bulk-uploads)
  - [11. Real-time Events with Socket.IO](#11-real-time-events-with-socketio)
  - [12. Safe `OpenAPI` Forward Compatibility](#12-safe-openapi-forward-compatibility)
  - [13. Zero-Network Test Mocking](#13-zero-network-test-mocking)
- [🌐 WebAssembly (WASM) & Browser Support](#-webassembly-wasm--browser-support)
- [🛠️ Examples Walkthrough](#️-examples-walkthrough)
- [💡 Features coming soon](#-features-coming-soon)
- [🤝 Contributing & Development](#-contributing--development)
- [📄 License & Acknowledgements](#-license--acknowledgements)

---

## 💭 Why immich-rs?

[Immich](https://immich.app/) is a rapidly evolving photo and video self-hosting solution with an extensive REST and WebSocket API. Writing raw HTTP calls or hand-crafting JSON payloads quickly leads to brittle scripts that break on subtle API changes.

`immich-rs` solves this by providing:
- **Zero API drift**: All models and client bindings are generated from official `OpenAPI` specifications with lossless unknown enum handling (`Unknown(String)`).
- **Compile-time safety**: Transparent newtypes for 20 domain IDs (`AssetId`, `AlbumId`, `UserId`, etc.) ensure you never pass an asset ID where an album ID was expected.
- **Enterprise-grade resilience**: Token-bucket rate limiting, full-jitter exponential backoff, RFC 9110 `Retry-After` parsing, and method-idempotence checks.
- **Zero credential leaks**: All keys and tokens are strictly masked in `Debug` implementations and OpenTelemetry/tracing spans.
- **Universal deployment**: Native multi-threaded Tokio support and complete browser WASM support (`wasm32-unknown-unknown`).

---

## ✨ Key Highlights

- **Complete `OpenAPI` Coverage**: 100% typed bindings generated strictly from the official Immich `OpenAPI` specification (`v3.3.5`).
- **20 `#[repr(transparent)]` Domain IDs**: Compile-time prevention of ID mixups (`AssetId`, `AlbumId`, `UserId`, `TagId`, etc.) with zero runtime overhead.
- **Hierarchical API Client**: Intuitive namespacing mirroring Immich domains: `client.assets().faces()`, `client.users().admin()`, `client.system().config()`.
- **Atomic Direct-to-Disk Transfers**: Downloads stream directly to temporary files before atomic filesystem rename, ensuring partial downloads never corrupt your disk.
- **High-Throughput Ingestion**: Parallel bulk uploads with concurrent bounded SHA-1 checksum calculation and upstream deduplication check.
- **Enterprise-Grade Network Layer**: Idempotence-aware retry policies (500 retried only on idempotent methods), RFC 9110 `Retry-After` parsing, and token-bucket rate limiting.
- **Real-Time Event Streaming**: Asynchronous event streams over Socket.IO delivering asset uploads, deletions, facial updates, and custom plugin events.
- **Zero Credential Leaks**: API keys, bearer tokens, and secrets are strictly redacted in all `Debug` implementations and tracing instrumentation.
- **Universal Multi-Platform**: First-class support for native Tokio platforms as well as browser environments via `wasm32-unknown-unknown`.

---

## 📥 How to get it

Add `immich-rs` to your `Cargo.toml`:

```toml
[dependencies]
immich-rs = "0.1.0"
```

If you only need the `OpenAPI` data models and DTOs without network or HTTP dependencies (e.g. for event workers, webhooks, or custom clients):

```toml
[dependencies]
immich-models = "1.0.0"
```

---

## 🔌 Optional features

This library puts advanced functionality behind optional Cargo features to optimize compile times and dependency trees for minimal binaries:

- 💾 **`fs`** *(enabled by default)* — Local filesystem operations: streaming file uploads, atomic downloads, and SHA-1 checksum helpers.
- 🔒 **`rustls-tls`** *(enabled by default)* — Modern, memory-safe TLS powered by `rustls` (pure Rust, no OpenSSL or C dependency).
- 🛡️ **`native-tls`** — Platform-native TLS using system certificate stores (`SChannel` / Security Framework / OpenSSL).
- ⚙️ **`cli-config`** — Automatic configuration discovery from `@immich/cli` (`auth.yml`) and `immich-go` files (`immich-go.toml/yaml/json`). Native targets only.
- 🧪 **`test-util`** — First-class mock client utilities (`test_util::mock_client`) for zero-network testing.
- 📡 **`socketio`** — Real-time event notifications via Socket.IO (`rust_socketio`). Native targets only.
- 📊 **`opentelemetry`** — Request latency and metrics integration via OpenTelemetry.

### Feature Summary Table

| Feature | Description | Default |
|---|---|:---:|
| `default` | Enables `fs` and `rustls-tls`. | **Yes** |
| `fs` | Local filesystem operations: streaming file uploads, atomic downloads, and SHA-1 checksum helpers. | **Yes** |
| `rustls-tls` | Modern, secure TLS powered by `rustls` (pure Rust, no OpenSSL or C dependency). | **Yes** |
| `native-tls` | Platform-native TLS using system certificate stores (`SChannel` / Security Framework / OpenSSL). | No |
| `cli-config` | Automatic configuration discovery from `@immich/cli` (`auth.yml`) and `immich-go` files. Native targets only. | No |
| `test-util` | First-class mock client utilities (`test_util::mock_client`) for zero-network testing. | No |
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

### Bearer Token Authentication & Live Login

You can log in directly using email credentials in a single step:

```rust,no_run
use immich_rs::ImmichClient;

#[tokio::main]
async fn main() -> Result<(), immich_rs::Error> {
    // Directly authenticate and receive an authenticated ImmichClient
    let client = ImmichClient::login_to(
        "https://photos.example.com/api",
        "user@example.com",
        "secret-password",
    )
    .await?;

    let me = client.users().get_my_user().await?;
    println!("Logged in as: {} ({})", me.name, me.email);

    Ok(())
}
```

Or configure a known bearer token manually:

```rust,no_run
# use immich_rs::ImmichClient;
# fn example() -> Result<(), immich_rs::Error> {
let client = ImmichClient::builder()
    .base_url("https://photos.example.com/api")
    .bearer_token("eyJhbGciOi...")
    .build()?;
# Ok(())
# }
```

### Automatic Environment & CLI Discovery

When writing CLI tools or automation scripts, `immich-rs` can automatically discover server and credential configuration from environment variables or existing CLI tools:

```rust,no_run
use immich_rs::ImmichClient;

#[tokio::main]
async fn main() -> Result<(), immich_rs::Error> {
    // 1. From standard environment variables (IMMICH_INSTANCE_URL and IMMICH_API_KEY)
    let client = ImmichClient::from_env()?;

    // 2. Or cascading through environment, local `immich-go` files, and global `@immich/cli` auth
    #[cfg(feature = "cli-config")]
    let client = ImmichClient::from_env_or_config()?;

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

### 4. Seamless Stream Pagination (`PaginationStream`)

All paginated endpoints return a concrete, `Unpin` [`PaginationStream`](https://docs.rs/immich-rs/latest/immich_rs/struct.PaginationStream.html) with inherent asynchronous `.try_next()` and `.collect()` methods. You can consume paginated items sequentially with **zero imports of `StreamExt` and zero `tokio::pin!`**:

```rust,no_run
# use immich_rs::ImmichClient;
# async fn example(client: &ImmichClient) -> Result<(), immich_rs::Error> {
// Stream individual items sequentially
let mut people = client.people().get_all_people().into_paginator().items();
while let Some(person) = people.try_next().await? {
    println!("Person: {:?}", person.name);
}

// Or collect all paginated items into a single Vec in one step
let all_people = client
    .people()
    .get_all_people()
    .into_paginator()
    .items()
    .collect()
    .await?;
println!("Fetched {} people", all_people.len());
# Ok(())
# }
```

### 5. Unified Binary Streaming (`ByteStream`)

Binary endpoints (asset viewing, database backups, media downloads) return an opaque [`ByteStream`](https://docs.rs/immich-rs/latest/immich_rs/struct.ByteStream.html). It encapsulates HTTP responses cleanly, preventing direct leaky dependencies on `reqwest`:

```rust,no_run
# use immich_rs::ImmichClient;
# use immich_models::id::AssetId;
# async fn example(client: &ImmichClient, asset_id: AssetId) -> Result<(), immich_rs::Error> {
let mut stream = client.assets().view_asset(asset_id).await?;

// Inspect response headers and RFC 5987 / RFC 6266 filenames
println!("Status: {}", stream.status());
println!("Filename: {:?}", stream.filename());
println!("Content length: {:?}", stream.content_length());

// Stream directly to a local file atomically with automatic temp-file cleanup
stream.save_to_path("downloaded_photo.jpg").await?;
# Ok(())
# }
```

### 6. Resilient Retries & Rate Limiting

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

### 7. Dynamic Token Rotation & Live Auth

Update authentication tokens lock-free at runtime across all concurrent operations without rebuilding the client:

```rust,no_run
# use immich_rs::ImmichClient;
# async fn example(client: &ImmichClient) -> Result<(), immich_rs::Error> {
// Authenticate and update the client token in-place
client.login("user@example.com", "secret-password").await?;

// Or perform lock-free token swaps during periodic rotation
client.set_bearer_token("new-refreshed-jwt-token");
client.set_api_key("new-api-key");
# Ok(())
# }
```

### 8. Low-Level HTTP Escape Hatch

If Immich introduces a newly released endpoint before SDK generation, use raw requests while retaining full authentication, rate limiting, and retry policies:

```rust,no_run
# use immich_rs::ImmichClient;
# use serde::Deserialize;
# #[derive(Deserialize)] struct CustomStats { count: u64 }
# async fn example(client: &ImmichClient) -> Result<(), immich_rs::Error> {
let stats: CustomStats = client
    .get("/custom/stats")
    .header("X-Custom-Header", "custom-value")
    .send_json()
    .await?;
# Ok(())
# }
```

### 9. Memory-Efficient Streaming & Progress

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

### 10. High-Throughput Bulk Uploads

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

### 11. Real-time Events with Socket.IO

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

### 12. Safe `OpenAPI` Forward Compatibility

All generated enums support **lossless forward compatibility**:
- If a future Immich server version introduces a new enum variant (e.g. `"live_photo_v2"`), deserialization will **never panic or fail**.
- The string is stored losslessly in `MyEnum::Unknown(String)` and re-serialized identically.
- Structs and enums are marked `#[non_exhaustive]` to ensure backward compatibility across SDK updates.

### 13. Zero-Network Test Mocking

Test your application code against a mock Immich client without running a local HTTP server or network sockets:

```rust,no_run
# #[cfg(feature = "test-util")]
# async fn example() -> Result<(), immich_rs::Error> {
use immich_rs::test_util::mock_client;

let client = mock_client(|req| async move {
    assert_eq!(req.url().path(), "/api/server/version");
    http::Response::builder()
        .status(200)
        .body(reqwest::Body::from(r#"{"major":3,"minor":3,"patch":5}"#))
        .map(reqwest::Response::from)
        .unwrap()
});

let ver = client.server().get_server_version().await?;
assert_eq!(ver.major, 3);
# Ok(())
# }
```

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
