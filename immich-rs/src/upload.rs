//! High-level upload helper that streams a file to Immich without loading it
//! entirely into memory.
//!
//! This is a thin wrapper around the generated [`Assets::upload_asset`](crate::api::assets::Assets::upload_asset) endpoint
//! that handles file I/O, MIME detection, and SHA-1 checksum computation.

#[cfg(feature = "fs")]
use std::borrow::Cow;
#[cfg(feature = "fs")]
use std::path::Path;

#[cfg(feature = "fs")]
use chrono::DateTime;
use chrono::Utc;
use immich_models::assets::AssetMediaResponseDTO;
#[cfg(feature = "fs")]
use immich_models::users::CreateProfileImageResponseDTO;
use reqwest::multipart::{Form, Part};
#[cfg(feature = "fs")]
use tokio::fs::File;

use crate::client::ImmichClient;
use crate::error::{Error, Result};
#[cfg(feature = "fs")]
use crate::progress::{ProgressReader, ProgressSender, TransferProgress};
#[cfg(feature = "fs")]
use crate::utils::mime::get_mime_type_for_file;

/// Options for uploading an in-memory asset (bytes).
#[derive(Debug, Clone)]
pub struct UploadBytesOptions<'a> {
    /// File name with extension (e.g. "photo.jpg").
    pub file_name: &'a str,
    /// Device identifier. Defaults to "immich-rs".
    pub device_id: &'a str,
    /// Unique device asset ID (used for deduplication).
    pub device_asset_id: &'a str,
    /// File creation date (RFC 3339). Defaults to current time if empty.
    pub file_created_at: &'a str,
    /// File modification date (RFC 3339). Defaults to current time if empty.
    pub file_modified_at: &'a str,
    /// Is the file marked as favorite.
    pub is_favorite: bool,
    /// MIME type. If None, guessed from `file_name`.
    pub mime_type: Option<&'a str>,
    /// Optional SHA-1 checksum header (`x-immich-checksum`).
    pub checksum: Option<&'a str>,
}

impl<'a> UploadBytesOptions<'a> {
    /// Creates a new `UploadBytesOptions` for the specified file name.
    #[must_use]
    pub fn new(file_name: &'a str) -> Self {
        Self {
            file_name,
            device_id: "immich-rs",
            device_asset_id: "",
            file_created_at: "",
            file_modified_at: "",
            is_favorite: false,
            mime_type: None,
            checksum: None,
        }
    }

    /// Sets the device identifier.
    #[must_use]
    pub fn device_id(mut self, device_id: &'a str) -> Self {
        self.device_id = device_id;
        self
    }

    /// Sets the unique device asset ID.
    #[must_use]
    pub fn device_asset_id(mut self, device_asset_id: &'a str) -> Self {
        self.device_asset_id = device_asset_id;
        self
    }

    /// Sets the file creation timestamp (RFC 3339).
    #[must_use]
    pub fn file_created_at(mut self, file_created_at: &'a str) -> Self {
        self.file_created_at = file_created_at;
        self
    }

    /// Sets the file modification timestamp (RFC 3339).
    #[must_use]
    pub fn file_modified_at(mut self, file_modified_at: &'a str) -> Self {
        self.file_modified_at = file_modified_at;
        self
    }

    /// Sets whether the uploaded asset should be marked as favorite.
    #[must_use]
    pub fn is_favorite(mut self, is_favorite: bool) -> Self {
        self.is_favorite = is_favorite;
        self
    }

    /// Sets the explicit MIME type (e.g. "image/jpeg").
    #[must_use]
    pub fn mime_type(mut self, mime_type: &'a str) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    /// Sets the optional SHA-1 checksum header (`x-immich-checksum`).
    #[must_use]
    pub fn checksum(mut self, checksum: &'a str) -> Self {
        self.checksum = Some(checksum);
        self
    }
}

/// Options for uploading a single asset.
#[cfg(feature = "fs")]
#[derive(Debug, Clone)]
pub struct UploadOptions<'a> {
    /// Path to the file to upload.
    pub path: &'a Path,
    /// Unique device asset ID (used for deduplication).
    pub device_asset_id: &'a str,
    /// Device identifier.
    pub device_id: &'a str,
    /// File creation date.
    pub file_created_at: &'a str,
    /// File modification date.
    pub file_modified_at: &'a str,
    /// Is the file marked as favorite.
    pub is_favorite: bool,
    /// SHA1 checksum for deduplication.
    pub checksum: Option<Cow<'a, str>>,
    /// Optional progress notification listener.
    pub progress: Option<ProgressSender>,
    /// Optional path to an XMP sidecar file.
    pub sidecar_path: Option<&'a Path>,
    /// Optional companion video ID for a Live Photo.
    pub live_photo_video_id: Option<immich_models::id::AssetId>,
}

#[cfg(feature = "fs")]
impl<'a> UploadOptions<'a> {
    /// Creates a new `UploadOptions` with the given file path and empty metadata.
    ///
    /// Any metadata fields left empty (`""`) will be automatically resolved from the file's
    /// OS metadata when [`ImmichClient::upload_asset`] is called.
    #[must_use]
    pub fn new(path: &'a Path) -> Self {
        Self {
            path,
            device_asset_id: "",
            device_id: "",
            file_created_at: "",
            file_modified_at: "",
            is_favorite: false,
            checksum: None,
            progress: None,
            sidecar_path: None,
            live_photo_video_id: None,
        }
    }

    /// Sets the unique device asset ID.
    #[must_use]
    pub fn device_asset_id(mut self, device_asset_id: &'a str) -> Self {
        self.device_asset_id = device_asset_id;
        self
    }

    /// Sets the device identifier.
    #[must_use]
    pub fn device_id(mut self, device_id: &'a str) -> Self {
        self.device_id = device_id;
        self
    }

    /// Sets the file creation timestamp (RFC 3339).
    #[must_use]
    pub fn file_created_at(mut self, file_created_at: &'a str) -> Self {
        self.file_created_at = file_created_at;
        self
    }

    /// Sets the file modification timestamp (RFC 3339).
    #[must_use]
    pub fn file_modified_at(mut self, file_modified_at: &'a str) -> Self {
        self.file_modified_at = file_modified_at;
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
    /// Uploads an asset from in-memory bytes.
    ///
    /// Usable in all environments including WebAssembly (browser), serverless, and native.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the MIME type is invalid.
    pub async fn upload_asset_bytes(
        &self,
        bytes: Vec<u8>,
        options: UploadBytesOptions<'_>,
    ) -> Result<AssetMediaResponseDTO> {
        let now_str = Utc::now().to_rfc3339();
        let resolved_created = if options.file_created_at.is_empty() {
            &now_str
        } else {
            options.file_created_at
        };
        let resolved_modified = if options.file_modified_at.is_empty() {
            &now_str
        } else {
            options.file_modified_at
        };
        let bytes_len = bytes.len();
        let auto_asset_id;
        let resolved_device_asset_id = if options.device_asset_id.is_empty() {
            auto_asset_id = format!("{}-{bytes_len}", options.file_name);
            &auto_asset_id
        } else {
            options.device_asset_id
        };

        let mime = options.mime_type.map_or_else(
            || crate::utils::mime::get_mime_type_for_file(options.file_name),
            ToString::to_string,
        );

        let part = Part::bytes(bytes)
            .file_name(options.file_name.to_owned())
            .mime_str(&mime)
            .map_err(|e| Error::Validation(format!("Invalid mime type: {e}")))?;

        let is_fav_str = if options.is_favorite { "true" } else { "false" };

        let form = Form::new()
            .text("deviceAssetId", resolved_device_asset_id.to_string())
            .text("deviceId", options.device_id.to_string())
            .text("fileCreatedAt", resolved_created.to_string())
            .text("fileModifiedAt", resolved_modified.to_string())
            .text("isFavorite", is_fav_str.to_string())
            .part("assetData", part);

        let mut req = self.assets().upload_asset(form);
        if let Some(checksum) = options.checksum {
            req = req.x_immich_checksum(checksum);
        }

        req.send().await
    }

    /// Upload a single asset to Immich with proper memory streaming (no full-RAM loading).
    ///
    /// This uses `multipart/form-data` to stream the file from disk directly to the network.
    /// If `device_asset_id`, `device_id`, `file_created_at`, or `file_modified_at` are empty,
    /// sensible defaults are automatically derived from the file's metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened, read, or if the API request fails.
    #[cfg(feature = "fs")]
    pub async fn upload_asset(&self, options: UploadOptions<'_>) -> Result<AssetMediaResponseDTO> {
        let file_name = options
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("upload.jpg")
            .to_string();

        let metadata = tokio::fs::metadata(options.path).await?;
        let modified_dt: DateTime<Utc> = metadata.modified().map_or_else(|_| Utc::now(), DateTime::<Utc>::from);
        let created_dt: DateTime<Utc> = metadata.created().map_or(modified_dt, DateTime::<Utc>::from);

        let resolved_modified: Cow<'_, str> = if options.file_modified_at.is_empty() {
            Cow::Owned(modified_dt.to_rfc3339())
        } else {
            Cow::Borrowed(options.file_modified_at)
        };
        let resolved_created: Cow<'_, str> = if options.file_created_at.is_empty() {
            Cow::Owned(created_dt.to_rfc3339())
        } else {
            Cow::Borrowed(options.file_created_at)
        };
        let resolved_device_asset_id: Cow<'_, str> = if options.device_asset_id.is_empty() {
            Cow::Owned(format!("{file_name}-{}", metadata.len()))
        } else {
            Cow::Borrowed(options.device_asset_id)
        };
        let resolved_device_id: &str = if options.device_id.is_empty() {
            "immich-rs"
        } else {
            options.device_id
        };

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
                .map_err(|e| Error::Validation(format!("Invalid mime type: {e}")))?;

            let mut form = Form::new()
                .text("deviceAssetId", resolved_device_asset_id.to_string())
                .text("deviceId", resolved_device_id.to_owned())
                .text("fileCreatedAt", resolved_created.to_string())
                .text("fileModifiedAt", resolved_modified.to_string())
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
                    .map_err(|e| Error::Validation(format!("Invalid sidecar mime type: {e}")))?;
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
                    if crate::core::retry::should_retry(&e) {
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
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Err(e);
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Upload a profile image (multipart/form-data).
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened, read, or if the API request fails.
    #[cfg(feature = "fs")]
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
                .map_err(|e| Error::Validation(format!("Invalid mime type: {e}")))?;

            let form = Form::new().part("file", part);

            match self.users().create_profile_image(form).await {
                Ok(res) => return Ok(res),
                Err(e) if attempt < self.inner.retry_policy.max_attempts => {
                    if crate::core::retry::should_retry(&e) {
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
                        tokio::time::sleep(delay).await;
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
    /// Returns an error if either the video or photo upload fails.
    #[cfg(feature = "fs")]
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
    /// Returns an error if the asset or sidecar cannot be read, or if the API request fails.
    #[cfg(feature = "fs")]
    pub async fn upload_with_sidecar(
        &self,
        asset_path: impl AsRef<Path>,
        sidecar_path: impl AsRef<Path>,
    ) -> Result<AssetMediaResponseDTO> {
        let opts = UploadOptions::new(asset_path.as_ref()).sidecar_path(sidecar_path.as_ref());
        self.upload_asset(opts).await
    }
}
