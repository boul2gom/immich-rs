# Changelog

All notable changes to `immich-rs` and `immich-models` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-10-30

### Added
- **Seamless stream pagination (`PaginationStream`)**: Concrete, `Unpin` stream wrapping paginated results with inherent `.try_next().await?` and `.collect().await?` methods, removing the need for `futures_util::StreamExt` imports or `tokio::pin!`.
- **Unified binary streaming (`ByteStream`)**: Opaque response stream for all binary payloads (`view_asset`, `download_archive`, `download_database_backup`), providing RFC 5987 / RFC 6266 filename resolution, atomic `.save_to_path(&path).await`, `.bytes().await`, and `.into_async_read()`, eliminating direct `reqwest` SemVer leakage in public signatures.
- **Low-level HTTP escape hatch**: Generic raw request dispatch via `client.request()`, `client.get()`, `post()`, `put()`, `delete()`, running through authentication, retry policies, and rate limiters with `.send()` and `.send_json::<T>()`.
- **Live authentication & dynamic token rotation**: Atomic lock-free auth updates via `ArcSwap<Auth>` (`client.set_bearer_token()`, `client.set_api_key()`) and streamlined login helpers (`ImmichClient::login_to()`, `client.login()`).
- **CLI configuration auto-discovery (`cli-config` feature)**: Automatic credentials resolution from official `@immich/cli` (`auth.yml`), `immich-go` (`immich-go.toml`, `.yaml`, `.json`), or environment variables with cascaded fallback (`ImmichClient::from_env_or_config()`).
- **Zero-network test mock utilities (`test-util` feature)**: In-memory client support (`ImmichClientBuilder::connector()`) and high-level mock helper (`test_util::mock_client()`) for fast unit and integration testing without network sockets.
- **End-to-end strongly typed IDs**: `AssetId`, `AlbumId`, `UserId`, `PersonId`, `TagId`, `LibraryId`, and all domain identifiers (`#[repr(transparent)]`) are now used directly across `immich-models` DTOs and `immich-rs` endpoints (`impl Into<AssetId>`).
- **Lossless forward-compatible enums**: Generated string enums now feature `Unknown(String)` with lossless round-trip `Serialize`/`Deserialize` (`GET -> PUT` safe against newer Immich servers), plus `#[serde(untagged)] Unknown(serde_json::Value)` fallbacks on `oneOf` unions and `#[non_exhaustive]` on generated structs and enums.
- **Nested sub-client hierarchy**: Clean domain grouping for sub-APIs (`client.users().admin()`, `client.assets().faces()`, `client.assets().stacks()`, `client.assets().downloads()`, `client.assets().files()`, `client.system().config()`, `client.system().metadata()`, `client.admin().database()`, `client.admin().maintenance()`, `client.admin().notifications()`).
- **First-class WebAssembly (`wasm32-unknown-unknown`) support**: Works out of the box even with default features enabled; retries, token-bucket rate limiting, and OpenTelemetry latency metrics run natively in browsers via `web-time` and `gloo-timers`.
- **Idempotence-aware HTTP retry policy**: Retries `429 | 502 | 503 | 504` across replayable requests and `500` exclusively on idempotent HTTP methods (`GET`, `HEAD`, `PUT`, `DELETE`, `OPTIONS`), supports RFC 9110 HTTP-date `Retry-After` headers, and retries `upload_asset_bytes`.
- **Bulk upload server rejections**: Added `BulkUploadResult::Rejected { path, reason }` so non-duplicate rejections (e.g., `UnsupportedFormat`) are reported cleanly instead of uploading rejected files, alongside concurrent SHA-1 checksum calculation.
- **`application/x-www-form-urlencoded` endpoint support**: Repaired `POST /oauth/backchannel-logout` (`client.oauth().logout_oauth(...)`).
- **Socket.IO enhancements**: Wildcard `on_any` routing for `ServerEvent::Other`, `Auth::Bearer` opening header support, `futures_core::Stream` implementation on `EventSubscription`, and dedicated `Error::SocketIo` variant.

### Changed
- **Dual license**: Relicensed under permissive **`MIT OR Apache-2.0`**.
- **Idiomatic Rust acronym casing (`C-CASE`)**: Standardized generated type and method names (`Dto` instead of `DTO`, `oauth` instead of `o_auth`).
- **Encapsulated request builders**: Request builder fields are now private, accept owned request body DTOs (`impl Into<BodyDto>`), and accept `impl IntoIterator<Item = impl Into<T>>` on list parameters.
- **Zero `regex` runtime dependency in `immich-models`**: Removed client-side regex validation during server JSON deserialization, shrinking WASM bundle size and preventing crashes on self-hosted non-RFC values (e.g., `admin@localhost`).

### Fixed
- **Tokio multi-threaded `tracing` span safety**: Replaced `span.enter()` across `.await` points with `.instrument(span)`.
- **REST URL path-segment safety**: Prevented `.` and `..` path parameters from collapsing URL segments via `Url::path_segments_mut`.
- **Header validation in `RequestConfig`**: Invalid header names or values now return `Error::Validation` instead of being silently dropped.
- **Search paginator state preservation**: `SearchAssetsPaginator` and `SearchSmartPaginator` now preserve `key`, `slug`, and per-request `RequestConfig` across all pages.
- **SemVer pre-release compatibility parsing**: Pre-release tags (e.g., `v2.0.0-rc.1`) are parsed accurately and unparsable versions return `Compatibility::Unparsable` instead of `Compatible`.

## [0.1.0] - 2026-10-23

### Added
- Initial release of `immich-rs` and `immich-models` targeting Immich `v3.3.5`.
