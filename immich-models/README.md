<h2 align="center">📦 Strongly-typed OpenAPI data models for Immich</h2>

<div align="center">
  Generated Rust data types, request/response DTOs, and schemas strictly derived from the official <a href="https://immich.app/">Immich</a> OpenAPI specification.<br>
  Completely decoupled from network and HTTP clients — built purely with Serde, Chrono, and Uuid.
</div>

<br>
<div align="center">⚠️ Part of the <a href="https://github.com/boul2gom/immich-rs">immich-rs</a> workspace. Published independently on <a href="https://crates.io/crates/immich-models">crates.io</a>.</div>
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
  <a href="https://crates.io/crates/immich-models">
    <img src="https://img.shields.io/crates/v/immich-models?label=crates.io&logo=Rust" alt="crates.io"/>
  </a>
  <a href="https://crates.io/crates/immich-models">
    <img src="https://img.shields.io/crates/d/immich-models?label=Downloads&logo=Rust" alt="Downloads"/>
  </a>
  <a href="https://docs.rs/immich-models">
    <img src="https://img.shields.io/docsrs/immich-models?label=docs.rs&logo=Rust" alt="docs.rs"/>
  </a>
</p>
<p align="center">
  <a href="https://github.com/boul2gom/immich-rs/blob/main/LICENSE-MIT">
    <img src="https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg" alt="License">
  </a>
  <a href="https://github.com/boul2gom/immich-rs/stargazers">
    <img src="https://img.shields.io/github/stars/boul2gom/immich-rs?label=Stars&logo=Github" alt="Stars">
  </a>
</p>

---

## 💭 Why `immich-models`?

If you are:
- Building an Immich webhook handler, event consumer, or custom integration.
- Writing a custom HTTP client (e.g. using `reqwest`, `surf`, `hyper`, or `ureq`).
- Processing Immich database exports, sync tools, or backup verification scripts.

You do **not** need the full `immich-rs` client and its async network dependencies. `immich-models` provides purely the type definitions, JSON serialization/deserialization logic, and strongly typed ID wrappers.

Targeted Immich version: **`v3.3.5`** (accessible at runtime via `immich_models::TARGET_IMMICH_VERSION`).

---

## 📥 Installation

Add `immich-models` to your `Cargo.toml`:

```toml
[dependencies]
immich-models = "0.1.0"
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.154"
```

---

## 🚀 Quickstart

### Deserializing an API Response

```rust,no_run
use immich_models::assets::AssetResponseDTO;

fn main() -> Result<(), serde_json::Error> {
    let json_data = r#"{
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "deviceAssetId": "IMG_1234",
        "ownerId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "deviceId": "iPhone",
        "type": "IMAGE",
        "originalPath": "upload/library/admin/IMG_1234.jpg",
        "originalFileName": "IMG_1234",
        "resized": false,
        "thumbhash": null,
        "fileCreatedAt": "2024-01-01T12:00:00.000Z",
        "fileModifiedAt": "2024-01-01T12:00:00.000Z",
        "updatedAt": "2024-01-01T12:00:00.000Z",
        "isFavorite": false,
        "isArchived": false,
        "isTrashed": false,
        "duration": "0:00:00.000000"
    }"#;

    let asset: AssetResponseDTO = serde_json::from_str(json_data)?;
    println!("Asset ID: {}, Type: {:?}", asset.id, asset.type_);

    Ok(())
}
```

### Strongly-Typed IDs

`immich-models` provides typed newtypes for IDs to prevent confusing different entities:

```rust
use immich_models::id::{AssetId, AlbumId, UserId};
use uuid::Uuid;

let asset_id = AssetId::from(Uuid::new_v4());
let album_id = AlbumId::from(Uuid::new_v4());
let user_id = UserId::from(Uuid::new_v4());

// Display and Serde implementations work transparently:
println!("Asset ID: {asset_id}");
```

---

## 📂 Model Modules

`immich-models` organizes schemas into domain-specific modules matching the Immich endpoints:

| Module | Schema Coverage |
|---|---|
| `activities` | Activity feed, comments, reactions |
| `admin` | Admin operations (database, maintenance, notifications) |
| `albums` | Album metadata, asset mapping, user sharing |
| `api_keys` | API keys, secret hashing, permissions |
| `assets` | Upload DTOs, metadata, jobs, EXIF data, faces, stacks |
| `authentication` | Login credentials, auth tokens, logout requests |
| `clusters` | People and face cluster groups |
| `duplicates` | Duplicate asset detection and merging |
| `jobs` | Job statuses, command queues, job parameters |
| `libraries` | Storage libraries, scanning options |
| `map` | Geo-reverse coding, markers, map bounds |
| `memories` | Time-based memories and highlights |
| `notifications` | In-app notifications and alerts |
| `partners` | Partner sharing and permissions |
| `people` | Person recognition, face crops, naming |
| `plugins` | Server plugins and integrations |
| `queues` | Background task queues |
| `search` | Search filters, vector smart search, exploration |
| `server` | Server configuration, stats, ping |
| `sessions` | Active sessions and devices |
| `shared_links` | Public and password-protected sharing links |
| `sync` | Delta sync tokens, mobile sync state |
| `system` | Storage statistics, hardware acceleration profiles, config |
| `tags` | Custom tags and entity tags |
| `timeline` | Bucket timeline, month/year aggregations |
| `trash` | Soft deletion and trash cleanup |
| `users` | User profiles, preferences, quotas, admin user management |
| `workflows` | Workflow definitions and triggers |

---

## 🛠️ Code Generation & Synchronization

All models in this crate are generated automatically from Immich's official `OpenAPI` specification.

To regenerate or sync with a new Immich release, use the `xtask` CLI within the `immich-rs` workspace:

```bash
# Fetch latest OpenAPI spec from Immich upstream and regenerate models:
cargo xtask generate
```

---

## 🤝 Contributing

Contributions are welcome! Please feel free to open an issue or submit a Pull Request on the main repository: [https://github.com/boul2gom/immich-rs](https://github.com/boul2gom/immich-rs).

---

## 📄 License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/boul2gom/immich-rs/blob/main/LICENSE-APACHE))
* MIT license ([LICENSE-MIT](https://github.com/boul2gom/immich-rs/blob/main/LICENSE-MIT))

at your option.
