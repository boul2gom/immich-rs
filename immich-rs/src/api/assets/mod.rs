// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `assets`.

pub mod download;
pub mod faces;
pub mod stacks;

use immich_models::assets::{
    AssetBulkDeleteDTO, AssetBulkUpdateDTO, AssetBulkUploadCheckDTO, AssetBulkUploadCheckResponseDTO, AssetCopyDTO,
    AssetEditsCreateDTO, AssetEditsResponseDTO, AssetJobsDTO, AssetMediaResponseDTO, AssetMediaSize,
    AssetMetadataBulkDeleteDTO, AssetMetadataBulkResponseDTO, AssetMetadataBulkUpsertDTO, AssetMetadataResponseDTO,
    AssetMetadataUpsertDTO, AssetOcrResponseDTO, AssetResponseDTO, AssetStatsResponseDTO, AssetVisibility,
    UpdateAssetDTO,
};

/// Fluent request builder for [`Assets::check_bulk_upload`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CheckBulkUploadBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AssetBulkUploadCheckDTO,
}

/// Type alias for [`CheckBulkUploadBuilder`].
pub type CheckBulkUploadArgs<'a> = CheckBulkUploadBuilder<'a>;
impl CheckBulkUploadBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AssetBulkUploadCheckResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "POST",
            path = "/assets/bulk-upload-check",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/assets/bulk-upload-check")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CopyAssetBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AssetCopyDTO,
}

/// Type alias for [`CopyAssetBuilder`].
pub type CopyAssetArgs<'a> = CopyAssetBuilder<'a>;
impl CopyAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(module = "Assets", method = "PUT", path = "/assets/copy", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/assets/copy")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteAssetMetadataBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(skip)]
    pub key: String,
}

/// Type alias for [`DeleteAssetMetadataBuilder`].
pub type DeleteAssetMetadataArgs<'a> = DeleteAssetMetadataBuilder<'a>;
impl DeleteAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "DELETE",
            path = "/assets/{id}/metadata/{key}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::DELETE,
            &format!("/assets/{}/metadata/{}", self.id, ::urlencoding::encode(&self.key)),
        )?;
        request = self.request_config.apply(request);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AssetBulkDeleteDTO,
}

/// Type alias for [`DeleteAssetsBuilder`].
pub type DeleteAssetsArgs<'a> = DeleteAssetsBuilder<'a>;
impl DeleteAssetsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(module = "Assets", method = "DELETE", path = "/assets", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/assets")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteBulkAssetMetadataBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AssetMetadataBulkDeleteDTO,
}

/// Type alias for [`DeleteBulkAssetMetadataBuilder`].
pub type DeleteBulkAssetMetadataArgs<'a> = DeleteBulkAssetMetadataBuilder<'a>;
impl DeleteBulkAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "DELETE",
            path = "/assets/metadata",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, "/assets/metadata")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DownloadAssetBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "edited", skip_serializing_if = "Option::is_none")]
    pub edited: Option<bool>,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Type alias for [`DownloadAssetBuilder`].
pub type DownloadAssetArgs<'a> = DownloadAssetBuilder<'a>;
impl DownloadAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/original",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/assets/{}/original", self.id))?;
        request = self.request_config.apply(request);
        request = request.query(&self);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct EditAssetBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(skip)]
    pub body: &'a AssetEditsCreateDTO,
}

/// Type alias for [`EditAssetBuilder`].
pub type EditAssetArgs<'a> = EditAssetBuilder<'a>;
impl EditAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AssetEditsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "PUT",
            path = "/assets/{id}/edits",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/assets/{}/edits", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct EndSessionBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip)]
    pub session_id: uuid::Uuid,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Type alias for [`EndSessionBuilder`].
pub type EndSessionArgs<'a> = EndSessionBuilder<'a>;
impl EndSessionBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "DELETE",
            path = "/assets/{id}/video/stream/{sessionId}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::DELETE,
            &format!("/assets/{}/video/stream/{}", self.id, self.session_id),
        )?;
        request = self.request_config.apply(request);
        request = request.query(&self);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetEditsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
}

/// Type alias for [`GetAssetEditsBuilder`].
pub type GetAssetEditsArgs<'a> = GetAssetEditsBuilder<'a>;
impl GetAssetEditsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AssetEditsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/edits",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/assets/{}/edits", self.id))?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetInfoBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Type alias for [`GetAssetInfoBuilder`].
pub type GetAssetInfoArgs<'a> = GetAssetInfoBuilder<'a>;
impl GetAssetInfoBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AssetResponseDTO, crate::Error> {
        tracing::trace!(module = "Assets", method = "GET", path = "/assets/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/assets/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetMetadataBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
}

/// Type alias for [`GetAssetMetadataBuilder`].
pub type GetAssetMetadataArgs<'a> = GetAssetMetadataBuilder<'a>;
impl GetAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetMetadataResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/metadata",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/assets/{}/metadata", self.id))?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetMetadataByKeyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(skip)]
    pub key: String,
}

/// Type alias for [`GetAssetMetadataByKeyBuilder`].
pub type GetAssetMetadataByKeyArgs<'a> = GetAssetMetadataByKeyBuilder<'a>;
impl GetAssetMetadataByKeyBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AssetMetadataResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/metadata/{key}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::GET,
            &format!("/assets/{}/metadata/{}", self.id, ::urlencoding::encode(&self.key)),
        )?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetOcrBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
}

/// Type alias for [`GetAssetOcrBuilder`].
pub type GetAssetOcrArgs<'a> = GetAssetOcrBuilder<'a>;
impl GetAssetOcrBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetOcrResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/ocr",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/assets/{}/ocr", self.id))?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetStatisticsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
    pub is_trashed: Option<bool>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AssetVisibility>,
}

/// Type alias for [`GetAssetStatisticsBuilder`].
pub type GetAssetStatisticsArgs<'a> = GetAssetStatisticsBuilder<'a>;
impl GetAssetStatisticsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AssetStatsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/statistics",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/assets/statistics")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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

/// Fluent request builder for [`Assets::get_assets_by_original_path`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetsByOriginalPathBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "path")]
    pub path: String,
}

/// Type alias for [`GetAssetsByOriginalPathBuilder`].
pub type GetAssetsByOriginalPathArgs<'a> = GetAssetsByOriginalPathBuilder<'a>;
impl GetAssetsByOriginalPathBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetResponseDTO>, crate::Error> {
        tracing::trace!(module = "Assets", method = "GET", path = "/view/folder", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/view/folder")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self.client.execute::<Vec<AssetResponseDTO>>("assets", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetsByOriginalPathBuilder<'a> {
    type Output = Result<Vec<AssetResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::get_main_playlist`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMainPlaylistBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Type alias for [`GetMainPlaylistBuilder`].
pub type GetMainPlaylistArgs<'a> = GetMainPlaylistBuilder<'a>;
impl GetMainPlaylistBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/video/stream/main.m3u8",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::GET,
            &format!("/assets/{}/video/stream/main.m3u8", self.id),
        )?;
        request = self.request_config.apply(request);
        request = request.query(&self);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMediaPlaylistBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip)]
    pub session_id: uuid::Uuid,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip)]
    pub variant_index: i64,
    #[serde(skip)]
    pub x_immich_hls_pos: Option<f64>,
}

/// Type alias for [`GetMediaPlaylistBuilder`].
pub type GetMediaPlaylistArgs<'a> = GetMediaPlaylistBuilder<'a>;
impl GetMediaPlaylistBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/video/stream/{sessionId}/{variantIndex}/playlist.m3u8",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::GET,
            &format!(
                "/assets/{}/video/stream/{}/{}/playlist.m3u8",
                self.id, self.session_id, self.variant_index
            ),
        )?;
        request = self.request_config.apply(request);
        if let Some(v) = &self.x_immich_hls_pos {
            request = request.header("x-immich-hls-pos", v.to_string());
        }
        request = request.query(&self);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetSegmentBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub filename: String,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip)]
    pub session_id: uuid::Uuid,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip)]
    pub variant_index: i64,
    #[serde(skip)]
    pub x_immich_hls_msn: Option<i64>,
}

/// Type alias for [`GetSegmentBuilder`].
pub type GetSegmentArgs<'a> = GetSegmentBuilder<'a>;
impl GetSegmentBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/video/stream/{sessionId}/{variantIndex}/{filename}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::GET,
            &format!(
                "/assets/{}/video/stream/{}/{}/{}",
                self.id,
                self.session_id,
                self.variant_index,
                ::urlencoding::encode(&self.filename)
            ),
        )?;
        request = self.request_config.apply(request);
        if let Some(v) = &self.x_immich_hls_msn {
            request = request.header("x-immich-hls-msn", v.to_string());
        }
        request = request.query(&self);
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

/// Fluent request builder for [`Assets::get_unique_original_paths`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUniqueOriginalPathsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetUniqueOriginalPathsBuilder`].
pub type GetUniqueOriginalPathsArgs<'a> = GetUniqueOriginalPathsBuilder<'a>;
impl GetUniqueOriginalPathsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<String>, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/view/folder/unique-paths",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/view/folder/unique-paths")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<Vec<String>>("assets", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUniqueOriginalPathsBuilder<'a> {
    type Output = Result<Vec<String>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Assets::play_asset_video`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct PlayAssetVideoBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Type alias for [`PlayAssetVideoBuilder`].
pub type PlayAssetVideoArgs<'a> = PlayAssetVideoBuilder<'a>;
impl PlayAssetVideoBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/video/playback",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/assets/{}/video/playback", self.id))?;
        request = self.request_config.apply(request);
        request = request.query(&self);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveAssetEditsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
}

/// Type alias for [`RemoveAssetEditsBuilder`].
pub type RemoveAssetEditsArgs<'a> = RemoveAssetEditsBuilder<'a>;
impl RemoveAssetEditsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "DELETE",
            path = "/assets/{id}/edits",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/assets/{}/edits", self.id))?;
        request = self.request_config.apply(request);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RunAssetJobsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AssetJobsDTO,
}

/// Type alias for [`RunAssetJobsBuilder`].
pub type RunAssetJobsArgs<'a> = RunAssetJobsBuilder<'a>;
impl RunAssetJobsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "POST",
            path = "/assets/jobs",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/assets/jobs")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAssetBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(skip)]
    pub body: &'a UpdateAssetDTO,
}

/// Type alias for [`UpdateAssetBuilder`].
pub type UpdateAssetArgs<'a> = UpdateAssetBuilder<'a>;
impl UpdateAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AssetResponseDTO, crate::Error> {
        tracing::trace!(module = "Assets", method = "PUT", path = "/assets/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/assets/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAssetMetadataBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(skip)]
    pub body: &'a AssetMetadataUpsertDTO,
}

/// Type alias for [`UpdateAssetMetadataBuilder`].
pub type UpdateAssetMetadataArgs<'a> = UpdateAssetMetadataBuilder<'a>;
impl UpdateAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetMetadataResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "PUT",
            path = "/assets/{id}/metadata",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/assets/{}/metadata", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AssetBulkUpdateDTO,
}

/// Type alias for [`UpdateAssetsBuilder`].
pub type UpdateAssetsArgs<'a> = UpdateAssetsBuilder<'a>;
impl UpdateAssetsBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<(), crate::Error> {
        tracing::trace!(module = "Assets", method = "PUT", path = "/assets", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/assets")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateBulkAssetMetadataBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AssetMetadataBulkUpsertDTO,
}

/// Type alias for [`UpdateBulkAssetMetadataBuilder`].
pub type UpdateBulkAssetMetadataArgs<'a> = UpdateBulkAssetMetadataBuilder<'a>;
impl UpdateBulkAssetMetadataBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetMetadataBulkResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "PUT",
            path = "/assets/metadata",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/assets/metadata")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
    pub request_config: crate::core::RequestConfig,
    pub key: Option<String>,
    pub slug: Option<String>,
    pub x_immich_checksum: Option<String>,
    pub form: reqwest::multipart::Form,
}

/// Type alias for [`UploadAssetBuilder`].
pub type UploadAssetArgs<'a> = UploadAssetBuilder<'a>;
impl UploadAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AssetMediaResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(module = "Assets", method = "POST", path = "/assets", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/assets")?;
        request = self.request_config.apply(request);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ViewAssetBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "edited", skip_serializing_if = "Option::is_none")]
    pub edited: Option<bool>,
    #[serde(skip)]
    pub id: immich_models::id::AssetId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<AssetMediaSize>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Type alias for [`ViewAssetBuilder`].
pub type ViewAssetArgs<'a> = ViewAssetBuilder<'a>;
impl ViewAssetBuilder<'_> {
    /// Overrides or sets the timeout for this specific request.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_config.set_timeout(timeout);
        self
    }

    /// Adds a custom HTTP header to this specific request.
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "Assets",
            method = "GET",
            path = "/assets/{id}/thumbnail",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/assets/{}/thumbnail", self.id))?;
        request = self.request_config.apply(request);
        request = request.query(&self);
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

///Client handle for `Assets` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Assets<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Assets<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Check bulk upload
    /// Determine which assets have already been uploaded to the server based on their SHA1 checksums.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AssetBulkUploadCheckDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetBulkUploadCheckResponseDTO`
    pub fn check_bulk_upload(&self, body: &'a AssetBulkUploadCheckDTO) -> CheckBulkUploadBuilder<'a> {
        CheckBulkUploadBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Copy asset
    /// Copy asset information like albums, tags, etc. from one asset to another.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AssetCopyDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn copy_asset(&self, body: &'a AssetCopyDTO) -> CopyAssetBuilder<'a> {
        CopyAssetBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete asset metadata by key
    /// Delete a specific metadata key-value pair associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ | Asset ID |
    /// | `key` | `String` | ✓ | Metadata key |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_asset_metadata(
        &self,
        id: impl Into<immich_models::id::AssetId>,
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
    /// Deletes multiple assets at the same time.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AssetBulkDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_assets(&self, body: &'a AssetBulkDeleteDTO) -> DeleteAssetsBuilder<'a> {
        DeleteAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete asset metadata
    /// Delete metadata key-value pairs for multiple assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AssetMetadataBulkDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_bulk_asset_metadata(
        &self,
        body: &'a AssetMetadataBulkDeleteDTO,
    ) -> DeleteBulkAssetMetadataBuilder<'a> {
        DeleteBulkAssetMetadataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Download original asset
    /// Downloads the original file of the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `edited` | `bool` |  | Return edited asset if available |
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn download_asset(&self, id: impl Into<immich_models::id::AssetId>) -> DownloadAssetBuilder<'a> {
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
    /// Apply a series of edit actions (crop, rotate, mirror) to the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `AssetEditsCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetEditsResponseDTO`
    pub fn edit_asset(
        &self,
        id: impl Into<immich_models::id::AssetId>,
        body: &'a AssetEditsCreateDTO,
    ) -> EditAssetBuilder<'a> {
        EditAssetBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// End HLS streaming session
    /// Releases server resources for the streaming session.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `sessionId` | `uuid::Uuid` | ✓ |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn end_session(
        &self,
        id: impl Into<immich_models::id::AssetId>,
        session_id: uuid::Uuid,
    ) -> EndSessionBuilder<'a> {
        EndSessionBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            session_id,
            slug: None,
        }
    }

    /// Retrieve edits for an existing asset
    /// Retrieve a series of edit actions (crop, rotate, mirror) associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetEditsResponseDTO`
    pub fn get_asset_edits(&self, id: impl Into<immich_models::id::AssetId>) -> GetAssetEditsBuilder<'a> {
        GetAssetEditsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve an asset
    /// Retrieve detailed information about a specific asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetResponseDTO`
    pub fn get_asset_info(&self, id: impl Into<immich_models::id::AssetId>) -> GetAssetInfoBuilder<'a> {
        GetAssetInfoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }

    /// Get asset metadata
    /// Retrieve all metadata key-value pairs associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetMetadataResponseDTO>`
    pub fn get_asset_metadata(&self, id: impl Into<immich_models::id::AssetId>) -> GetAssetMetadataBuilder<'a> {
        GetAssetMetadataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve asset metadata by key
    /// Retrieve the value of a specific metadata key associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ | Asset ID |
    /// | `key` | `String` | ✓ | Metadata key |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetMetadataResponseDTO`
    pub fn get_asset_metadata_by_key(
        &self,
        id: impl Into<immich_models::id::AssetId>,
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
    /// Retrieve all OCR (Optical Character Recognition) data associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetOcrResponseDTO>`
    pub fn get_asset_ocr(&self, id: impl Into<immich_models::id::AssetId>) -> GetAssetOcrBuilder<'a> {
        GetAssetOcrBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Get asset statistics
    /// Retrieve various statistics about the assets owned by the authenticated user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `isFavorite` | `bool` |  | Filter by favorite status |
    /// | `isTrashed` | `bool` |  | Filter by trash status |
    /// | `visibility` | `AssetVisibility` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetStatsResponseDTO`
    pub fn get_asset_statistics(&self) -> GetAssetStatisticsBuilder<'a> {
        GetAssetStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            is_favorite: None,
            is_trashed: None,
            visibility: None,
        }
    }

    /// Retrieve assets by original path
    /// Retrieve assets that are children of a specific folder.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `path` | `String` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetResponseDTO>`
    pub fn get_assets_by_original_path(&self, path: impl Into<String>) -> GetAssetsByOriginalPathBuilder<'a> {
        GetAssetsByOriginalPathBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            path: path.into(),
        }
    }

    /// Get HLS main playlist
    /// Returns an HLS main playlist with all available variants for the asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn get_main_playlist(&self, id: impl Into<immich_models::id::AssetId>) -> GetMainPlaylistBuilder<'a> {
        GetMainPlaylistBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }

    /// Get HLS media playlist
    /// Returns an HLS media playlist for one variant of the streaming session.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `sessionId` | `uuid::Uuid` | ✓ |  |
    /// | `slug` | `String` |  |  |
    /// | `variantIndex` | `i64` | ✓ |  |
    /// | `x-immich-hls-pos` | `f64` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn get_media_playlist(
        &self,
        id: impl Into<immich_models::id::AssetId>,
        session_id: uuid::Uuid,
        variant_index: i64,
    ) -> GetMediaPlaylistBuilder<'a> {
        GetMediaPlaylistBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            session_id,
            slug: None,
            variant_index,
            x_immich_hls_pos: None,
        }
    }

    /// Get HLS segment or init file
    /// Streams an HLS init segment (init.mp4) or media segment (`seg_N.m4s`).
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `filename` | `String` | ✓ |  |
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `sessionId` | `uuid::Uuid` | ✓ |  |
    /// | `slug` | `String` |  |  |
    /// | `variantIndex` | `i64` | ✓ |  |
    /// | `x-immich-hls-msn` | `i64` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn get_segment(
        &self,
        filename: impl Into<String>,
        id: impl Into<immich_models::id::AssetId>,
        session_id: uuid::Uuid,
        variant_index: i64,
    ) -> GetSegmentBuilder<'a> {
        GetSegmentBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            filename: filename.into(),
            id: id.into(),
            key: None,
            session_id,
            slug: None,
            variant_index,
            x_immich_hls_msn: None,
        }
    }

    /// Retrieve unique paths
    /// Retrieve a list of unique folder paths from asset original paths.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<String>`
    pub fn get_unique_original_paths(&self) -> GetUniqueOriginalPathsBuilder<'a> {
        GetUniqueOriginalPathsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Play asset video
    /// Streams the video file for the specified asset. This endpoint also supports byte range requests.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn play_asset_video(&self, id: impl Into<immich_models::id::AssetId>) -> PlayAssetVideoBuilder<'a> {
        PlayAssetVideoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }

    /// Remove edits from an existing asset
    /// Removes all edit actions (crop, rotate, mirror) associated with the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn remove_asset_edits(&self, id: impl Into<immich_models::id::AssetId>) -> RemoveAssetEditsBuilder<'a> {
        RemoveAssetEditsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Run an asset job
    /// Run a specific job on a set of assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AssetJobsDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn run_asset_jobs(&self, body: &'a AssetJobsDTO) -> RunAssetJobsBuilder<'a> {
        RunAssetJobsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Update an asset
    /// Update information of a specific asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `UpdateAssetDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_asset(
        &self,
        id: impl Into<immich_models::id::AssetId>,
        body: &'a UpdateAssetDTO,
    ) -> UpdateAssetBuilder<'a> {
        UpdateAssetBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Update asset metadata
    /// Update or add metadata key-value pairs for the specified asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `AssetMetadataUpsertDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetMetadataResponseDTO>`
    pub fn update_asset_metadata(
        &self,
        id: impl Into<immich_models::id::AssetId>,
        body: &'a AssetMetadataUpsertDTO,
    ) -> UpdateAssetMetadataBuilder<'a> {
        UpdateAssetMetadataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Update assets
    /// Updates multiple assets at the same time.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AssetBulkUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_assets(&self, body: &'a AssetBulkUpdateDTO) -> UpdateAssetsBuilder<'a> {
        UpdateAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Upsert asset metadata
    /// Upsert metadata key-value pairs for multiple assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AssetMetadataBulkUpsertDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetMetadataBulkResponseDTO>`
    pub fn update_bulk_asset_metadata(
        &self,
        body: &'a AssetMetadataBulkUpsertDTO,
    ) -> UpdateBulkAssetMetadataBuilder<'a> {
        UpdateBulkAssetMetadataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Upload asset
    /// Uploads a new asset to the server.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    /// | `x-immich-checksum` | `String` |  | sha1 checksum that can be used for duplicate detection before the file is uploaded |
    /// | `body` | `AssetMediaCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` | Asset is a duplicate |
    /// | `201` | Asset uploaded successfully |
    ///
    /// **Returns:** `AssetMediaResponseDTO`
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
    /// Retrieve the thumbnail image for the specified asset. Viewing the fullsize thumbnail might redirect to downloadAsset, which requires a different permission.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `edited` | `bool` |  | Return edited asset if available |
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `size` | `AssetMediaSize` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn view_asset(&self, id: impl Into<immich_models::id::AssetId>) -> ViewAssetBuilder<'a> {
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
