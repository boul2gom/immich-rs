#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../../README.md")]

pub mod api;
/// Client builder, authentication configuration, and HTTP transport setup.
pub mod client;
pub mod compatibility;
/// Structured API, transport, validation, and real-time error types.
pub mod error;
#[cfg(all(feature = "socketio", not(target_arch = "wasm32")))]
pub mod events;
pub mod middleware;
pub mod progress;

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
mod bulk;
mod core;
mod download;
mod upload;
mod utils;

pub use core::{BoxCloneSyncService, RequestConfig};

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
pub use bulk::{BulkUploadBuilder, BulkUploadItem, BulkUploadResult};
pub use client::{BoxService, ImmichClient, ImmichClientBuilder};
pub use compatibility::Compatibility;
pub use error::{ApiErrorDetails, Error, ErrorKind, ImmichErrorCode, Result};
#[cfg(all(feature = "socketio", not(target_arch = "wasm32")))]
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

/// Re-export of all Immich `OpenAPI` data models.
///
/// Allows consumers to use `immich_rs::models::albums::CreateAlbumDTO`
/// without adding `immich-models` as a direct dependency.
pub use immich_models as models;
pub use immich_models::TARGET_IMMICH_VERSION;
pub use immich_models::id::*;
pub use middleware::RateLimitLayer;
pub use progress::{ProgressSender, TransferProgress};
pub use upload::UploadBytesOptions;
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
pub use upload::UploadOptions;
pub use utils::checksum::calculate_bytes_checksum;
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
pub use utils::checksum::calculate_file_checksum;

/// Convenient glob re-export of core client types, domain IDs, error types, and transfer options.
pub mod prelude {
    pub use immich_models::id::*;

    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub use crate::bulk::{BulkUploadBuilder, BulkUploadItem, BulkUploadResult};
    pub use crate::client::{ImmichClient, ImmichClientBuilder};
    pub use crate::compatibility::Compatibility;
    pub use crate::error::{ApiErrorDetails, Error, ErrorKind, ImmichErrorCode, Result};
    #[cfg(all(feature = "socketio", not(target_arch = "wasm32")))]
    pub use crate::events::{EventSubscription, ServerEvent};
    pub use crate::middleware::RateLimitLayer;
    pub use crate::progress::{ProgressSender, TransferProgress};
    pub use crate::upload::UploadBytesOptions;
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub use crate::upload::UploadOptions;
}
