//! High-level upload helpers that stream files or in-memory bytes to Immich.
//!
//! Wraps the generated [`Assets::upload_asset`](crate::api::assets::Assets::upload_asset) endpoint
//! with MIME detection, SHA-1 checksum headers, progress reporting, and idempotence-aware retries.

use std::borrow::Cow;
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
use std::path::Path;

use chrono::{DateTime, Utc};
use immich_models::assets::AssetMediaResponseDTO;
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
use immich_models::users::CreateProfileImageResponseDTO;
use reqwest::Method;
use reqwest::multipart::{Form, Part};
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
use tokio::fs::File;

use crate::client::ImmichClient;
use crate::error::{Error, Result};
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
use crate::progress::{ProgressReader, ProgressSender, TransferProgress};
use crate::utils::mime::get_mime_type_for_file;

/// Options for uploading an in-memory asset (`Vec<u8>`).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UploadBytesOptions<'a> {
    /// File name with extension (e.g. `"photo.jpg"`).
    pub file_name: Cow<'a, str>,
    /// Device identifier (`None` defaults to `"immich-rs"`).
    pub device_id: Option<Cow<'a, str>>,
    /// Unique device asset ID (`None` defaults to `"{file_name}-{byte_len}"`).
    pub device_asset_id: Option<Cow<'a, str>>,
    /// File creation timestamp (`None` defaults to current UTC time).
    pub file_created_at: Option<DateTime<Utc>>,
    /// File modification timestamp (`None` defaults to `file_created_at` or current UTC time).
    pub file_modified_at: Option<DateTime<Utc>>,
    /// Whether the uploaded asset should be marked as favorite.
    pub is_favorite: bool,
    /// Explicit MIME type (`None` guesses from `file_name`).
    pub mime_type: Option<Cow<'a, str>>,
    /// Optional SHA-1 checksum header (`x-immich-checksum`).
    pub checksum: Option<Cow<'a, str>>,
}

impl<'a> UploadBytesOptions<'a> {
    /// Creates a new `UploadBytesOptions` for the specified file name.
    #[must_use]
    pub fn new(file_name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            file_name: file_name.into(),
            device_id: None,
            device_asset_id: None,
            file_created_at: None,
            file_modified_at: None,
            is_favorite: false,
            mime_type: None,
            checksum: None,
        }
    }

    /// Sets the device identifier.
    #[must_use]
    pub fn device_id(mut self, device_id: impl Into<Cow<'a, str>>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// Sets the unique device asset ID.
    #[must_use]
    pub fn device_asset_id(mut self, device_asset_id: impl Into<Cow<'a, str>>) -> Self {
        self.device_asset_id = Some(device_asset_id.into());
        self
    }

    /// Sets the file creation timestamp.
    #[must_use]
    pub fn file_created_at(mut self, file_created_at: DateTime<Utc>) -> Self {
        self.file_created_at = Some(file_created_at);
        self
    }

    /// Sets the file modification timestamp.
    #[must_use]
    pub fn file_modified_at(mut self, file_modified_at: DateTime<Utc>) -> Self {
        self.file_modified_at = Some(file_modified_at);
        self
    }

    /// Sets whether the uploaded asset should be marked as favorite.
    #[must_use]
    pub fn is_favorite(mut self, is_favorite: bool) -> Self {
        self.is_favorite = is_favorite;
        self
    }

    /// Sets the explicit MIME type (e.g. `"image/jpeg"`).
    #[must_use]
    pub fn mime_type(mut self, mime_type: impl Into<Cow<'a, str>>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }

    /// Sets the optional SHA-1 checksum header (`x-immich-checksum`).
    #[must_use]
    pub fn checksum(mut self, checksum: impl Into<Cow<'a, str>>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }
}

/// Options for streaming a single file asset upload from disk.
#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UploadOptions<'a> {
    /// Path to the file to upload.
    pub path: &'a Path,
    /// Unique device asset ID (`None` defaults to `"{file_name}-{byte_len}"`).
    pub device_asset_id: Option<Cow<'a, str>>,
    /// Device identifier (`None` defaults to `"immich-rs"`).
    pub device_id: Option<Cow<'a, str>>,
    /// File creation timestamp (`None` resolves from OS metadata).
    pub file_created_at: Option<DateTime<Utc>>,
    /// File modification timestamp (`None` resolves from OS metadata).
    pub file_modified_at: Option<DateTime<Utc>>,
    /// Whether the uploaded asset should be marked as favorite.
    pub is_favorite: bool,
    /// Optional SHA-1 checksum header (`x-immich-checksum`).
    pub checksum: Option<Cow<'a, str>>,
    /// Optional progress notification listener.
    pub progress: Option<ProgressSender>,
    /// Optional path to an XMP sidecar file.
    pub sidecar_path: Option<&'a Path>,
    /// Optional companion video ID for a Live Photo.
    pub live_photo_video_id: Option<immich_models::id::AssetId>,
}

#[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
impl<'a> UploadOptions<'a> {
    /// Creates a new `UploadOptions` for `path`.
    ///
    /// Unset metadata fields (`None`) are automatically resolved from the file's OS metadata
    /// when [`ImmichClient::upload_asset`] is called.
    #[must_use]
    pub const fn new(path: &'a Path) -> Self {
        Self {
            path,
            device_asset_id: None,
            device_id: None,
            file_created_at: None,
            file_modified_at: None,
            is_favorite: false,
            checksum: None,
            progress: None,
            sidecar_path: None,
            live_photo_video_id: None,
        }
    }

    /// Sets the unique device asset ID.
    #[must_use]
    pub fn device_asset_id(mut self, device_asset_id: impl Into<Cow<'a, str>>) -> Self {
        self.device_asset_id = Some(device_asset_id.into());
        self
    }

    /// Sets the device identifier.
    #[must_use]
    pub fn device_id(mut self, device_id: impl Into<Cow<'a, str>>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// Sets the file creation timestamp.
    #[must_use]
    pub fn file_created_at(mut self, file_created_at: DateTime<Utc>) -> Self {
        self.file_created_at = Some(file_created_at);
        self
    }

    /// Sets the file modification timestamp.
    #[must_use]
    pub fn file_modified_at(mut self, file_modified_at: DateTime<Utc>) -> Self {
        self.file_modified_at = Some(file_modified_at);
        self
    }

    /// Sets whether the uploaded asset should be marked as favorite.
    #[must_use]
    pub fn is_favorite(mut self, is_favorite: bool) -> Self {
        self.is_favorite = is_favorite;
        self
    }

    /// Sets the optional SHA-1 checksum header (`x-immich-checksum`).
    #[must_use]
    pub fn checksum(mut self, checksum: impl Into<Cow<'a, str>>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    /// Sets the path to an optional XMP sidecar file to upload alongside the asset.
    #[must_use]
    pub fn sidecar_path(mut self, sidecar_path: &'a Path) -> Self {
        self.sidecar_path = Some(sidecar_path);
        self
    }

    /// Sets the companion video asset ID for a Live Photo.
    #[must_use]
    pub fn live_photo_video_id(mut self, id: impl Into<immich_models::id::AssetId>) -> Self {
        self.live_photo_video_id = Some(id.into());
        self
    }

    /// Registers a synchronous callback to receive upload progress notifications.
    #[must_use]
    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(TransferProgress) + Send + Sync + 'static,
    {
        self.progress = Some(ProgressSender::Callback(std::sync::Arc::new(callback)));
        self
    }

    /// Connects an asynchronous `tokio::sync::watch` channel sender to receive upload progress notifications.
    #[must_use]
    pub fn progress_sender(mut self, sender: tokio::sync::watch::Sender<TransferProgress>) -> Self {
        self.progress = Some(ProgressSender::Watch(sender));
        self
    }
}

impl ImmichClient {
    /// Uploads an asset from in-memory bytes with automatic transient retry support.
    ///
    /// Usable in all environments including WebAssembly (`wasm32-unknown-unknown`), serverless, and native.
    ///
    /// # Errors
    ///
    /// Returns an error if the MIME type is invalid or if the API request fails after retries.
    pub async fn upload_asset_bytes(
        &self,
        bytes: impl Into<Vec<u8>>,
        options: UploadBytesOptions<'_>,
    ) -> Result<AssetMediaResponseDTO> {
        let bytes = bytes.into();
        let now = crate::core::time::utc_now();
        let created_dt = options.file_created_at.unwrap_or(now);
        let modified_dt = options.file_modified_at.unwrap_or(created_dt);
        let created_str = created_dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let modified_str = modified_dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        let bytes_len = bytes.len();
        let resolved_device_asset_id = options
            .device_asset_id
            .unwrap_or_else(|| Cow::Owned(format!("{}-{bytes_len}", options.file_name)));
        let resolved_device_id = options.device_id.unwrap_or(Cow::Borrowed("immich-rs"));

        let mime = options
            .mime_type
            .unwrap_or_else(|| Cow::Owned(get_mime_type_for_file(options.file_name.as_ref())));
        let is_fav_str = if options.is_favorite { "true" } else { "false" };
        let mut attempt = 0;

        loop {
            let part = Part::bytes(bytes.clone())
                .file_name(options.file_name.to_string())
                .mime_str(mime.as_ref())
                .map_err(|e| Error::Validation(format!("invalid MIME type: {e}")))?;

            let form = Form::new()
                .text("deviceAssetId", resolved_device_asset_id.to_string())
                .text("deviceId", resolved_device_id.to_string())
                .text("fileCreatedAt", created_str.clone())
                .text("fileModifiedAt", modified_str.clone())
                .text("isFavorite", is_fav_str)
                .part("assetData", part);

            let mut req = self.assets().upload_asset(form);
            if let Some(checksum) = &options.checksum {
                req = req.x_immich_checksum(checksum.as_ref());
            }

            match req.send().await {
                Ok(res) => return Ok(res),
                Err(e) if attempt < self.inner.retry_policy.max_attempts => {
                    if crate::core::retry::should_retry_for_method(&e, Some(&Method::POST)) {
                        attempt += 1;
                        let delay = e
                            .retry_after()
                            .unwrap_or_else(|| self.inner.retry_policy.delay(attempt));
                        tracing::warn!(
                            attempt = attempt,
                            max_attempts = self.inner.retry_policy.max_attempts,
                            ?delay,
                            error = %e,
                            "Transient error during upload_asset_bytes, retrying"
                        );
                        crate::core::time::sleep(delay).await;
                        continue;
                    }
                    return Err(e);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Uploads a single file asset to Immich by streaming directly from disk without full-RAM buffering.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened, read, or if the API request fails.
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub async fn upload_asset(&self, options: UploadOptions<'_>) -> Result<AssetMediaResponseDTO> {
        let file_name = options
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("upload.jpg")
            .to_string();

        let metadata = tokio::fs::metadata(options.path).await?;
        let default_modified: DateTime<Utc> = metadata
            .modified()
            .map_or_else(|_| crate::core::time::utc_now(), DateTime::<Utc>::from);
        let default_created: DateTime<Utc> = metadata.created().map_or(default_modified, DateTime::<Utc>::from);

        let resolved_modified = options
            .file_modified_at
            .unwrap_or(default_modified)
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let resolved_created = options
            .file_created_at
            .unwrap_or(default_created)
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let resolved_device_asset_id = options
            .device_asset_id
            .unwrap_or_else(|| Cow::Owned(format!("{file_name}-{}", metadata.len())));
        let resolved_device_id = options.device_id.unwrap_or(Cow::Borrowed("immich-rs"));

        let file_len = metadata.len();
        let mime_type = get_mime_type_for_file(options.path);
        let is_fav_str = if options.is_favorite { "true" } else { "false" };
        let mut attempt = 0;

        loop {
            let file = File::open(options.path).await?;
            let progress_reader = ProgressReader::new(file, Some(file_len), options.progress.clone());
            let stream = tokio_util::io::ReaderStream::new(progress_reader);
            let body = reqwest::Body::wrap_stream(stream);

            let part = Part::stream_with_length(body, file_len)
                .file_name(file_name.clone())
                .mime_str(&mime_type)
                .map_err(|e| Error::Validation(format!("invalid MIME type: {e}")))?;

            let mut form = Form::new()
                .text("deviceAssetId", resolved_device_asset_id.to_string())
                .text("deviceId", resolved_device_id.to_string())
                .text("fileCreatedAt", resolved_created.clone())
                .text("fileModifiedAt", resolved_modified.clone())
                .text("isFavorite", is_fav_str)
                .part("assetData", part);

            if let Some(sidecar_path) = options.sidecar_path {
                let sidecar_metadata = tokio::fs::metadata(sidecar_path).await?;
                let sidecar_len = sidecar_metadata.len();
                let sidecar_name = sidecar_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("sidecar.xmp")
                    .to_string();
                let sidecar_file = File::open(sidecar_path).await?;
                let sidecar_mime = get_mime_type_for_file(sidecar_path);
                let sidecar_part = Part::stream_with_length(reqwest::Body::from(sidecar_file), sidecar_len)
                    .file_name(sidecar_name)
                    .mime_str(&sidecar_mime)
                    .map_err(|e| Error::Validation(format!("invalid sidecar MIME type: {e}")))?;
                form = form.part("sidecarData", sidecar_part);
            }

            if let Some(video_id) = options.live_photo_video_id {
                form = form.text("livePhotoVideoId", video_id.to_string());
            }

            let mut req = self.assets().upload_asset(form);
            if let Some(checksum) = &options.checksum {
                req = req.x_immich_checksum(checksum.as_ref());
            }

            match req.send().await {
                Ok(res) => return Ok(res),
                Err(e) if attempt < self.inner.retry_policy.max_attempts => {
                    if crate::core::retry::should_retry_for_method(&e, Some(&Method::POST)) {
                        attempt += 1;
                        let delay = e
                            .retry_after()
                            .unwrap_or_else(|| self.inner.retry_policy.delay(attempt));
                        tracing::warn!(
                            attempt = attempt,
                            max_attempts = self.inner.retry_policy.max_attempts,
                            ?delay,
                            error = %e,
                            "Transient error during upload_asset, retrying"
                        );
                        crate::core::time::sleep(delay).await;
                        continue;
                    }
                    return Err(e);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Uploads a user profile image (`multipart/form-data`) from disk.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened, read, or if the API request fails.
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub async fn upload_profile_image<P: AsRef<Path>>(&self, file_path: P) -> Result<CreateProfileImageResponseDTO> {
        let path = file_path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("profile.jpg")
            .to_string();

        let file_len = tokio::fs::metadata(path).await?.len();
        let mime_type = get_mime_type_for_file(path);
        let mut attempt = 0;

        loop {
            let file = File::open(path).await?;
            let body = reqwest::Body::from(file);

            let part = Part::stream_with_length(body, file_len)
                .file_name(file_name.clone())
                .mime_str(&mime_type)
                .map_err(|e| Error::Validation(format!("invalid MIME type: {e}")))?;

            let form = Form::new().part("file", part);

            match self.users().create_profile_image(form).await {
                Ok(res) => return Ok(res),
                Err(e) if attempt < self.inner.retry_policy.max_attempts => {
                    if crate::core::retry::should_retry_for_method(&e, Some(&Method::POST)) {
                        attempt += 1;
                        let delay = e
                            .retry_after()
                            .unwrap_or_else(|| self.inner.retry_policy.delay(attempt));
                        tracing::warn!(
                            attempt = attempt,
                            max_attempts = self.inner.retry_policy.max_attempts,
                            ?delay,
                            error = %e,
                            "Transient error during upload_profile_image, retrying"
                        );
                        crate::core::time::sleep(delay).await;
                        continue;
                    }
                    return Err(e);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Uploads a Live Photo by first uploading its companion video, then uploading the photo linked to the video ID.
    ///
    /// # Errors
    ///
    /// Returns an error if either the video or photo upload fails.
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub async fn upload_live_photo(
        &self,
        photo_path: impl AsRef<Path>,
        video_path: impl AsRef<Path>,
    ) -> Result<AssetMediaResponseDTO> {
        let video_opts = UploadOptions::new(video_path.as_ref());
        let video_res = self.upload_asset(video_opts).await?;
        let photo_opts = UploadOptions::new(photo_path.as_ref()).live_photo_video_id(video_res.id);
        self.upload_asset(photo_opts).await
    }

    /// Uploads an asset along with its XMP sidecar file in a single multipart request.
    ///
    /// # Errors
    ///
    /// Returns an error if the asset or sidecar cannot be read, or if the API request fails.
    #[cfg(all(feature = "fs", not(target_arch = "wasm32")))]
    pub async fn upload_with_sidecar(
        &self,
        asset_path: impl AsRef<Path>,
        sidecar_path: impl AsRef<Path>,
    ) -> Result<AssetMediaResponseDTO> {
        let opts = UploadOptions::new(asset_path.as_ref()).sidecar_path(sidecar_path.as_ref());
        self.upload_asset(opts).await
    }
}
