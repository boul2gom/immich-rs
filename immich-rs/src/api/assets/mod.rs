// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `assets`.

pub mod downloads;
pub mod faces;
pub mod files;
pub mod stacks;

use immich_models::assets::{
    AssetBulkDeleteDTO, AssetBulkUpdateDTO, AssetBulkUploadCheckDTO, AssetBulkUploadCheckResponseDTO, AssetCopyDTO,
    AssetEditsCreateDTO, AssetEditsResponseDTO, AssetJobsDTO, AssetMediaResponseDTO, AssetMediaSize,
    AssetMetadataBulkDeleteDTO, AssetMetadataBulkResponseDTO, AssetMetadataBulkUpsertDTO, AssetMetadataResponseDTO,
    AssetMetadataUpsertDTO, AssetOcrResponseDTO, AssetResponseDTO, AssetStatsResponseDTO, AssetVisibility,
    UpdateAssetDTO,
};
use immich_models::id::{AssetId, SessionId};

/// Fluent request builder for [`Assets::check_bulk_upload`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CheckBulkUploadBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AssetBulkUploadCheckDTO,
}

impl CheckBulkUploadBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AssetBulkUploadCheckResponseDTO, crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "POST",
            path = "/assets/bulk-upload-check",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/assets/bulk-upload-check")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<AssetBulkUploadCheckResponseDTO>("assets", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CheckBulkUploadBuilder<'a> {
    type Output = Result<AssetBulkUploadCheckResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::copy_asset`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CopyAssetBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AssetCopyDTO,
}

impl CopyAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(module = "assets", method = "PUT", path = "/assets/copy", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/assets/copy")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("assets", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for CopyAssetBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::delete_asset_metadata`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteAssetMetadataBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    key: String,
}

impl DeleteAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "DELETE",
            path = "/assets/{id}/metadata/{key}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("metadata"),
                crate::core::PathSegment::Param(self.key.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("assets", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteAssetMetadataBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::delete_assets`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteAssetsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AssetBulkDeleteDTO,
}

impl DeleteAssetsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(module = "assets", method = "DELETE", path = "/assets", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/assets")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("assets", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteAssetsBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::delete_bulk_asset_metadata`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteBulkAssetMetadataBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AssetMetadataBulkDeleteDTO,
}

impl DeleteBulkAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "DELETE",
            path = "/assets/metadata",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, "/assets/metadata")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("assets", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteBulkAssetMetadataBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::download_asset`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DownloadAssetBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    edited: Option<bool>,
    id: AssetId,
    key: Option<String>,
    slug: Option<String>,
}

impl DownloadAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Return edited asset if available
    pub fn edited(mut self, value: bool) -> Self {
        self.edited = Some(value);
        self
    }
    /// Sets or clears `edited`.
    pub fn set_edited(mut self, value: Option<bool>) -> Self {
        self.edited = value;
        self
    }
    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }
    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "edited", skip_serializing_if = "Option::is_none")]
            edited: Option<&'b bool>,
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/original",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("original"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            edited: self.edited.as_ref(),
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let raw_response = self.client.execute_stream("assets", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for DownloadAssetBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::edit_asset`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct EditAssetBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    body: AssetEditsCreateDTO,
}

impl EditAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AssetEditsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "PUT",
            path = "/assets/{id}/edits",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("edits"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<AssetEditsResponseDTO>("assets", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for EditAssetBuilder<'a> {
    type Output = Result<AssetEditsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::end_session`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct EndSessionBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    key: Option<String>,
    session_id: SessionId,
    slug: Option<String>,
}

impl EndSessionBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }
    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "assets",
            method = "DELETE",
            path = "/assets/{id}/video/stream/{sessionId}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let __seg_4 = self.session_id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("video"),
                crate::core::PathSegment::Static("stream"),
                crate::core::PathSegment::Param(__seg_4.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        self.client.execute_unit("assets", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for EndSessionBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_asset_edits`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetEditsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
}

impl GetAssetEditsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AssetEditsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/edits",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("edits"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self.client.execute::<AssetEditsResponseDTO>("assets", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetEditsBuilder<'a> {
    type Output = Result<AssetEditsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_asset_info`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetInfoBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    key: Option<String>,
    slug: Option<String>,
}

impl GetAssetInfoBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }
    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AssetResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(module = "assets", method = "GET", path = "/assets/{id}", "HTTP request");
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let response = self.client.execute::<AssetResponseDTO>("assets", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetInfoBuilder<'a> {
    type Output = Result<AssetResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_asset_metadata`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetMetadataBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
}

impl GetAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetMetadataResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/metadata",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("metadata"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<Vec<AssetMetadataResponseDTO>>("assets", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetMetadataBuilder<'a> {
    type Output = Result<Vec<AssetMetadataResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_asset_metadata_by_key`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetMetadataByKeyBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    key: String,
}

impl GetAssetMetadataByKeyBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AssetMetadataResponseDTO, crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/metadata/{key}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("metadata"),
                crate::core::PathSegment::Param(self.key.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<AssetMetadataResponseDTO>("assets", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetMetadataByKeyBuilder<'a> {
    type Output = Result<AssetMetadataResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_asset_ocr`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetOcrBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
}

impl GetAssetOcrBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetOcrResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/ocr",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("ocr"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<Vec<AssetOcrResponseDTO>>("assets", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetOcrBuilder<'a> {
    type Output = Result<Vec<AssetOcrResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_asset_statistics`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetStatisticsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    is_favorite: Option<bool>,
    is_trashed: Option<bool>,
    visibility: Option<AssetVisibility>,
}

impl GetAssetStatisticsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Filter by favorite status
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }
    /// Sets or clears `isFavorite`.
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }
    /// Filter by trash status
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = Some(value);
        self
    }
    /// Sets or clears `isTrashed`.
    pub fn set_is_trashed(mut self, value: Option<bool>) -> Self {
        self.is_trashed = value;
        self
    }
    /// Sets `visibility`.
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }
    /// Sets or clears `visibility`.
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AssetStatsResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
            is_favorite: Option<&'b bool>,
            #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
            is_trashed: Option<&'b bool>,
            #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
            visibility: Option<&'b AssetVisibility>,
        }
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/statistics",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/assets/statistics")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            is_favorite: self.is_favorite.as_ref(),
            is_trashed: self.is_trashed.as_ref(),
            visibility: self.visibility.as_ref(),
        };
        request = request.query(&query);
        let response = self.client.execute::<AssetStatsResponseDTO>("assets", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetStatisticsBuilder<'a> {
    type Output = Result<AssetStatsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_main_playlist`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMainPlaylistBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    key: Option<String>,
    slug: Option<String>,
}

impl GetMainPlaylistBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }
    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/video/stream/main.m3u8",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("video"),
                crate::core::PathSegment::Static("stream"),
                crate::core::PathSegment::Static("main.m3u8"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let raw_response = self.client.execute_stream("assets", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for GetMainPlaylistBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_media_playlist`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMediaPlaylistBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    key: Option<String>,
    session_id: SessionId,
    slug: Option<String>,
    variant_index: i64,
    x_immich_hls_pos: Option<f64>,
}

impl GetMediaPlaylistBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }
    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sets `x-immich-hls-pos`.
    pub fn x_immich_hls_pos(mut self, value: f64) -> Self {
        self.x_immich_hls_pos = Some(value);
        self
    }
    /// Sets or clears `x-immich-hls-pos`.
    pub fn set_x_immich_hls_pos(mut self, value: Option<f64>) -> Self {
        self.x_immich_hls_pos = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/video/stream/{sessionId}/{variantIndex}/playlist.m3u8",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let __seg_4 = self.session_id.to_string();
        let __seg_5 = self.variant_index.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("video"),
                crate::core::PathSegment::Static("stream"),
                crate::core::PathSegment::Param(__seg_4.as_str()),
                crate::core::PathSegment::Param(__seg_5.as_str()),
                crate::core::PathSegment::Static("playlist.m3u8"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        if let Some(v) = &self.x_immich_hls_pos {
            request = request.header("x-immich-hls-pos", v.to_string());
        }
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let raw_response = self.client.execute_stream("assets", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for GetMediaPlaylistBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_segment`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetSegmentBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    filename: String,
    id: AssetId,
    key: Option<String>,
    session_id: SessionId,
    slug: Option<String>,
    variant_index: i64,
    x_immich_hls_msn: Option<i64>,
}

impl GetSegmentBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }
    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sets `x-immich-hls-msn`.
    pub fn x_immich_hls_msn(mut self, value: i64) -> Self {
        self.x_immich_hls_msn = Some(value);
        self
    }
    /// Sets or clears `x-immich-hls-msn`.
    pub fn set_x_immich_hls_msn(mut self, value: Option<i64>) -> Self {
        self.x_immich_hls_msn = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/video/stream/{sessionId}/{variantIndex}/{filename}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let __seg_4 = self.session_id.to_string();
        let __seg_5 = self.variant_index.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("video"),
                crate::core::PathSegment::Static("stream"),
                crate::core::PathSegment::Param(__seg_4.as_str()),
                crate::core::PathSegment::Param(__seg_5.as_str()),
                crate::core::PathSegment::Param(self.filename.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        if let Some(v) = &self.x_immich_hls_msn {
            request = request.header("x-immich-hls-msn", v.to_string());
        }
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let raw_response = self.client.execute_stream("assets", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for GetSegmentBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::play_asset_video`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct PlayAssetVideoBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    key: Option<String>,
    slug: Option<String>,
}

impl PlayAssetVideoBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }
    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/video/playback",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("video"),
                crate::core::PathSegment::Static("playback"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let raw_response = self.client.execute_stream("assets", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for PlayAssetVideoBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::remove_asset_edits`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveAssetEditsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
}

impl RemoveAssetEditsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "DELETE",
            path = "/assets/{id}/edits",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("edits"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("assets", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for RemoveAssetEditsBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::run_asset_jobs`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RunAssetJobsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AssetJobsDTO,
}

impl RunAssetJobsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "POST",
            path = "/assets/jobs",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/assets/jobs")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("assets", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for RunAssetJobsBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::update_asset`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAssetBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    body: UpdateAssetDTO,
}

impl UpdateAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AssetResponseDTO, crate::Error> {
        tracing::trace!(module = "assets", method = "PUT", path = "/assets/{id}", "HTTP request");
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<AssetResponseDTO>("assets", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateAssetBuilder<'a> {
    type Output = Result<AssetResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::update_asset_metadata`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAssetMetadataBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetId,
    body: AssetMetadataUpsertDTO,
}

impl UpdateAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetMetadataResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "PUT",
            path = "/assets/{id}/metadata",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("metadata"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<Vec<AssetMetadataResponseDTO>>("assets", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateAssetMetadataBuilder<'a> {
    type Output = Result<Vec<AssetMetadataResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::update_assets`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAssetsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AssetBulkUpdateDTO,
}

impl UpdateAssetsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(module = "assets", method = "PUT", path = "/assets", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/assets")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("assets", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for UpdateAssetsBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::update_bulk_asset_metadata`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateBulkAssetMetadataBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AssetMetadataBulkUpsertDTO,
}

impl UpdateBulkAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetMetadataBulkResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "assets",
            method = "PUT",
            path = "/assets/metadata",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/assets/metadata")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<Vec<AssetMetadataBulkResponseDTO>>("assets", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateBulkAssetMetadataBuilder<'a> {
    type Output = Result<Vec<AssetMetadataBulkResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::upload_asset`].
#[derive(Debug)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UploadAssetBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    key: Option<String>,
    slug: Option<String>,
    x_immich_checksum: Option<String>,
    form: reqwest::multipart::Form,
}

impl UploadAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }
    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// sha1 checksum that can be used for duplicate detection before the file is uploaded
    pub fn x_immich_checksum(mut self, value: impl Into<String>) -> Self {
        self.x_immich_checksum = Some(value.into());
        self
    }
    /// Sets or clears `x-immich-checksum`.
    pub fn set_x_immich_checksum(mut self, value: Option<String>) -> Self {
        self.x_immich_checksum = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AssetMediaResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(module = "assets", method = "POST", path = "/assets", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/assets")?;
        request = self.request_config.apply(request)?;
        if let Some(v) = &self.x_immich_checksum {
            request = request.header("x-immich-checksum", v.as_str());
        }
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        request = request.multipart(self.form);
        let response = self.client.execute::<AssetMediaResponseDTO>("assets", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UploadAssetBuilder<'a> {
    type Output = Result<AssetMediaResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::view_asset`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ViewAssetBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    edited: Option<bool>,
    id: AssetId,
    key: Option<String>,
    size: Option<AssetMediaSize>,
    slug: Option<String>,
}

impl ViewAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }
    /// Adds a custom HTTP header to this specific request.
    ///
    /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
    /// when the request is sent.
    pub fn header(
        mut self,
        key: impl TryInto<reqwest::header::HeaderName>,
        value: impl TryInto<reqwest::header::HeaderValue>,
    ) -> Self {
        self.request_config.insert_header(key, value);
        self
    }
    /// Return edited asset if available
    pub fn edited(mut self, value: bool) -> Self {
        self.edited = Some(value);
        self
    }
    /// Sets or clears `edited`.
    pub fn set_edited(mut self, value: Option<bool>) -> Self {
        self.edited = value;
        self
    }
    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }
    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }
    /// Sets `size`.
    pub fn size(mut self, value: AssetMediaSize) -> Self {
        self.size = Some(value);
        self
    }
    /// Sets or clears `size`.
    pub fn set_size(mut self, value: Option<AssetMediaSize>) -> Self {
        self.size = value;
        self
    }
    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "edited", skip_serializing_if = "Option::is_none")]
            edited: Option<&'b bool>,
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
            size: Option<&'b AssetMediaSize>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "assets",
            method = "GET",
            path = "/assets/{id}/thumbnail",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("assets"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("thumbnail"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            edited: self.edited.as_ref(),
            key: self.key.as_ref(),
            size: self.size.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let raw_response = self.client.execute_stream("assets", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for ViewAssetBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Assets` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Assets<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Assets<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Returns a sub-client handle for [`downloads::Downloads`] endpoints.
    #[must_use]
    pub const fn downloads(&self) -> downloads::Downloads<'a> {
        downloads::Downloads::new(self.client)
    }
    /// Returns a sub-client handle for [`faces::Faces`] endpoints.
    #[must_use]
    pub const fn faces(&self) -> faces::Faces<'a> {
        faces::Faces::new(self.client)
    }
    /// Returns a sub-client handle for [`files::Files`] endpoints.
    #[must_use]
    pub const fn files(&self) -> files::Files<'a> {
        files::Files::new(self.client)
    }
    /// Returns a sub-client handle for [`stacks::Stacks`] endpoints.
    #[must_use]
    pub const fn stacks(&self) -> stacks::Stacks<'a> {
        stacks::Stacks::new(self.client)
    }
    /// Check bulk upload
    ///
    /// Determine which assets have already been uploaded to the server based on their SHA1 checksums.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AssetBulkUploadCheckDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetBulkUploadCheckResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn check_bulk_upload(&self, body: impl Into<AssetBulkUploadCheckDTO>) -> CheckBulkUploadBuilder<'a> {
        CheckBulkUploadBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Copy asset
    ///
    /// Copy asset information like albums, tags, etc. from one asset to another.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AssetCopyDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn copy_asset(&self, body: impl Into<AssetCopyDTO>) -> CopyAssetBuilder<'a> {
        CopyAssetBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete asset metadata by key
    ///
    /// Delete a specific metadata key-value pair associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ | Asset ID |
    /// | `key` | `String` | ✓ | Metadata key |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_asset_metadata(
        &self,
        id: impl Into<AssetId>,
        key: impl Into<String>,
    ) -> DeleteAssetMetadataBuilder<'a> {
        DeleteAssetMetadataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: key.into(),
        }
    }
    /// Delete assets
    ///
    /// Deletes multiple assets at the same time.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AssetBulkDeleteDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_assets(&self, body: impl Into<AssetBulkDeleteDTO>) -> DeleteAssetsBuilder<'a> {
        DeleteAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete asset metadata
    ///
    /// Delete metadata key-value pairs for multiple assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AssetMetadataBulkDeleteDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_bulk_asset_metadata(
        &self,
        body: impl Into<AssetMetadataBulkDeleteDTO>,
    ) -> DeleteBulkAssetMetadataBuilder<'a> {
        DeleteBulkAssetMetadataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Download original asset
    ///
    /// Downloads the original file of the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `edited` | `bool` |  | Return edited asset if available |
    /// | `id` | `AssetId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn download_asset(&self, id: impl Into<AssetId>) -> DownloadAssetBuilder<'a> {
        DownloadAssetBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            edited: None,
            id: id.into(),
            key: None,
            slug: None,
        }
    }
    /// Apply edits to an existing asset
    ///
    /// Apply a series of edit actions (crop, rotate, mirror) to the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    /// | `body` | `AssetEditsCreateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetEditsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn edit_asset(&self, id: impl Into<AssetId>, body: impl Into<AssetEditsCreateDTO>) -> EditAssetBuilder<'a> {
        EditAssetBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// End HLS streaming session
    ///
    /// Releases server resources for the streaming session.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `session_id` | `SessionId` | ✓ |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn end_session(&self, id: impl Into<AssetId>, session_id: impl Into<SessionId>) -> EndSessionBuilder<'a> {
        EndSessionBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            session_id: session_id.into(),
            slug: None,
        }
    }
    /// Retrieve edits for an existing asset
    ///
    /// Retrieve a series of edit actions (crop, rotate, mirror) associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetEditsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_asset_edits(&self, id: impl Into<AssetId>) -> GetAssetEditsBuilder<'a> {
        GetAssetEditsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve an asset
    ///
    /// Retrieve detailed information about a specific asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_asset_info(&self, id: impl Into<AssetId>) -> GetAssetInfoBuilder<'a> {
        GetAssetInfoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }
    /// Get asset metadata
    ///
    /// Retrieve all metadata key-value pairs associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetMetadataResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_asset_metadata(&self, id: impl Into<AssetId>) -> GetAssetMetadataBuilder<'a> {
        GetAssetMetadataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve asset metadata by key
    ///
    /// Retrieve the value of a specific metadata key associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ | Asset ID |
    /// | `key` | `String` | ✓ | Metadata key |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetMetadataResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_asset_metadata_by_key(
        &self,
        id: impl Into<AssetId>,
        key: impl Into<String>,
    ) -> GetAssetMetadataByKeyBuilder<'a> {
        GetAssetMetadataByKeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: key.into(),
        }
    }
    /// Retrieve asset OCR data
    ///
    /// Retrieve all OCR (Optical Character Recognition) data associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetOcrResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_asset_ocr(&self, id: impl Into<AssetId>) -> GetAssetOcrBuilder<'a> {
        GetAssetOcrBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Get asset statistics
    ///
    /// Retrieve various statistics about the assets owned by the authenticated user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `is_favorite` | `bool` |  | Filter by favorite status |
    /// | `is_trashed` | `bool` |  | Filter by trash status |
    /// | `visibility` | `AssetVisibility` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetStatsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_asset_statistics(&self) -> GetAssetStatisticsBuilder<'a> {
        GetAssetStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            is_favorite: None,
            is_trashed: None,
            visibility: None,
        }
    }
    /// Get HLS main playlist
    ///
    /// Returns an HLS main playlist with all available variants for the asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_main_playlist(&self, id: impl Into<AssetId>) -> GetMainPlaylistBuilder<'a> {
        GetMainPlaylistBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }
    /// Get HLS media playlist
    ///
    /// Returns an HLS media playlist for one variant of the streaming session.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `session_id` | `SessionId` | ✓ |  |
    /// | `slug` | `String` |  |  |
    /// | `variant_index` | `i64` | ✓ |  |
    /// | `x_immich_hls_pos` | `f64` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_media_playlist(
        &self,
        id: impl Into<AssetId>,
        session_id: impl Into<SessionId>,
        variant_index: i64,
    ) -> GetMediaPlaylistBuilder<'a> {
        GetMediaPlaylistBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            session_id: session_id.into(),
            slug: None,
            variant_index,
            x_immich_hls_pos: None,
        }
    }
    /// Get HLS segment or init file
    ///
    /// Streams an HLS init segment (init.mp4) or media segment (`seg_N`.m4s).
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `filename` | `String` | ✓ |  |
    /// | `id` | `AssetId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `session_id` | `SessionId` | ✓ |  |
    /// | `slug` | `String` |  |  |
    /// | `variant_index` | `i64` | ✓ |  |
    /// | `x_immich_hls_msn` | `i64` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_segment(
        &self,
        filename: impl Into<String>,
        id: impl Into<AssetId>,
        session_id: impl Into<SessionId>,
        variant_index: i64,
    ) -> GetSegmentBuilder<'a> {
        GetSegmentBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            filename: filename.into(),
            id: id.into(),
            key: None,
            session_id: session_id.into(),
            slug: None,
            variant_index,
            x_immich_hls_msn: None,
        }
    }
    /// Play asset video
    ///
    /// Streams the video file for the specified asset. This endpoint also supports byte range requests.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn play_asset_video(&self, id: impl Into<AssetId>) -> PlayAssetVideoBuilder<'a> {
        PlayAssetVideoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }
    /// Remove edits from an existing asset
    ///
    /// Removes all edit actions (crop, rotate, mirror) associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn remove_asset_edits(&self, id: impl Into<AssetId>) -> RemoveAssetEditsBuilder<'a> {
        RemoveAssetEditsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Run an asset job
    ///
    /// Run a specific job on a set of assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AssetJobsDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn run_asset_jobs(&self, body: impl Into<AssetJobsDTO>) -> RunAssetJobsBuilder<'a> {
        RunAssetJobsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Update an asset
    ///
    /// Update information of a specific asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    /// | `body` | `UpdateAssetDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_asset(&self, id: impl Into<AssetId>, body: impl Into<UpdateAssetDTO>) -> UpdateAssetBuilder<'a> {
        UpdateAssetBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Update asset metadata
    ///
    /// Update or add metadata key-value pairs for the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetId` | ✓ |  |
    /// | `body` | `AssetMetadataUpsertDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetMetadataResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_asset_metadata(
        &self,
        id: impl Into<AssetId>,
        body: impl Into<AssetMetadataUpsertDTO>,
    ) -> UpdateAssetMetadataBuilder<'a> {
        UpdateAssetMetadataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Update assets
    ///
    /// Updates multiple assets at the same time.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AssetBulkUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_assets(&self, body: impl Into<AssetBulkUpdateDTO>) -> UpdateAssetsBuilder<'a> {
        UpdateAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Upsert asset metadata
    ///
    /// Upsert metadata key-value pairs for multiple assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AssetMetadataBulkUpsertDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetMetadataBulkResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_bulk_asset_metadata(
        &self,
        body: impl Into<AssetMetadataBulkUpsertDTO>,
    ) -> UpdateBulkAssetMetadataBuilder<'a> {
        UpdateBulkAssetMetadataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Upload asset
    ///
    /// Uploads a new asset to the server.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    /// | `x_immich_checksum` | `String` |  | sha1 checksum that can be used for duplicate detection before the file is uploaded |
    /// | `form` | `reqwest::multipart::Form` | ✓ | Multipart form payload |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` | Asset is a duplicate |
    /// | `201` | Asset uploaded successfully |
    ///
    /// **Returns:** `AssetMediaResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn upload_asset(&self, form: reqwest::multipart::Form) -> UploadAssetBuilder<'a> {
        UploadAssetBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
            x_immich_checksum: None,
            form,
        }
    }
    /// View asset thumbnail
    ///
    /// Retrieve the thumbnail image for the specified asset. Viewing the fullsize thumbnail might redirect to downloadAsset, which requires a different permission.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `edited` | `bool` |  | Return edited asset if available |
    /// | `id` | `AssetId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `size` | `AssetMediaSize` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn view_asset(&self, id: impl Into<AssetId>) -> ViewAssetBuilder<'a> {
        ViewAssetBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            edited: None,
            id: id.into(),
            key: None,
            size: None,
            slug: None,
        }
    }
}
