//! # `immich-rs`
//!
//! An asynchronous, type-safe Rust SDK for the [Immich](https://immich.app/) self-hosted photo
//! and video backup server.
//!
//! ## Overview
//!
//! - **Full API Coverage**: Auto-generated strongly typed clients and models derived from the official Immich `OpenAPI` specification.
//! - **Fluent API**: Request builders with chained parameters and direct `.await` support via [`std::future::IntoFuture`].
//! - **Resilient**: Exponential backoff with jitter and automated retry of transient network and 5xx errors.
//! - **Streaming Transfers**: Direct-to-disk atomic downloads and memory-efficient streaming uploads.
//! - **Observability**: Zero credentials leakage in `Debug` output and structured `tracing` integration.
//!
//! ## Quick Start
//!
//! ```no_run
//! use immich_rs::ImmichClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), immich_rs::Error> {
//!     // Initialize via base URL and API key
//!     let client = ImmichClient::new("https://photos.example.com/api", "your-api-key")?;
//!
//!     // Fetch all albums
//!     let albums = client.albums().get_all_albums().send().await?;
//!     for album in albums {
//!         println!("Album: {} (id: {})", album.album_name, album.id);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Environment-Based Initialization
//!
//! You can initialize the client directly from environment variables (`IMMICH_INSTANCE_URL`, `IMMICH_API_KEY`):
//!
//! ```no_run
//! use immich_rs::ImmichClient;
//!
//! # fn main() -> Result<(), immich_rs::Error> {
//! let client = ImmichClient::from_env()?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod client;
pub mod compatibility;
pub mod error;
#[cfg(feature = "socketio")]
pub mod events;
pub mod middleware;
pub mod progress;

#[cfg(feature = "fs")]
mod bulk;
mod core;
mod download;
mod upload;
mod utils;

pub use core::RequestConfig;

#[cfg(feature = "fs")]
pub use bulk::{BulkUploadBuilder, BulkUploadItem, BulkUploadResult};
pub use client::{ImmichClient, ImmichClientBuilder};
pub use compatibility::Compatibility;
pub use error::{ApiErrorDetails, Error, ErrorKind, ImmichErrorCode, Result};
#[cfg(feature = "socketio")]
pub use events::{EventSubscription, ServerEvent};

/// A boxed pinned future that is `Send` on native targets and single-threaded on WebAssembly targets.
#[cfg(not(target_arch = "wasm32"))]
pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

/// A boxed pinned future that is `Send` on native targets and single-threaded on WebAssembly targets.
#[cfg(target_arch = "wasm32")]
pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + 'a>>;

/// Marker trait representing `Send` on multi-threaded targets and an unconstrained marker on single-threaded WebAssembly targets.
#[cfg(not(target_arch = "wasm32"))]
pub trait MaybeSend: Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send + ?Sized> MaybeSend for T {}

/// Marker trait representing `Send` on multi-threaded targets and an unconstrained marker on single-threaded WebAssembly targets.
#[cfg(target_arch = "wasm32")]
pub trait MaybeSend {}
#[cfg(target_arch = "wasm32")]
impl<T: ?Sized> MaybeSend for T {}
/// Re-export of all Immich API data models.
///
/// Allows consumers to use `immich_rs::models::albums::CreateAlbumDTO`
/// without adding `immich-models` as a direct dependency.
pub use immich_models as models;
pub use immich_models::TARGET_IMMICH_VERSION;
pub use immich_models::id::*;
pub use middleware::RateLimitLayer;
pub use progress::{ProgressSender, TransferProgress};
pub use upload::UploadBytesOptions;
#[cfg(feature = "fs")]
pub use upload::UploadOptions;
pub use utils::checksum::calculate_bytes_checksum;
#[cfg(feature = "fs")]
pub use utils::checksum::calculate_file_checksum;

pub mod prelude {
    pub use immich_models::id::*;

    #[cfg(feature = "fs")]
    pub use crate::bulk::{BulkUploadBuilder, BulkUploadItem, BulkUploadResult};
    pub use crate::client::{ImmichClient, ImmichClientBuilder};
    pub use crate::compatibility::Compatibility;
    pub use crate::error::{ApiErrorDetails, Error, ErrorKind, ImmichErrorCode, Result};
    #[cfg(feature = "socketio")]
    pub use crate::events::{EventSubscription, ServerEvent};
    pub use crate::middleware::RateLimitLayer;
    pub use crate::progress::{ProgressSender, TransferProgress};
    pub use crate::upload::UploadBytesOptions;
    #[cfg(feature = "fs")]
    pub use crate::upload::UploadOptions;
}
