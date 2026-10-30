// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `files`.

use immich_models::assets::files::{AssetFileResponseDTO, AssetFileType};
use immich_models::id::{AssetFileId, AssetId};

/// Fluent request builder for [`Files::delete_asset_file`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteAssetFileBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetFileId,
}

impl DeleteAssetFileBuilder<'_> {
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
            module = "assets/files",
            method = "DELETE",
            path = "/asset-files/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("asset-files"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("assets/files", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteAssetFileBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Files::download_asset_file`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DownloadAssetFileBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetFileId,
}

impl DownloadAssetFileBuilder<'_> {
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
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "assets/files",
            method = "GET",
            path = "/asset-files/{id}/download",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("asset-files"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("download"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let raw_response = self.client.execute_stream("assets/files", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for DownloadAssetFileBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Files::get_asset_file`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetFileBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AssetFileId,
}

impl GetAssetFileBuilder<'_> {
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
    pub async fn send(self) -> Result<AssetFileResponseDTO, crate::Error> {
        tracing::trace!(
            module = "assets/files",
            method = "GET",
            path = "/asset-files/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("asset-files"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<AssetFileResponseDTO>("assets/files", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetFileBuilder<'a> {
    type Output = Result<AssetFileResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Files::search_asset_files`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchAssetFilesBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    asset_id: AssetId,
    is_edited: Option<bool>,
    is_progressive: Option<bool>,
    is_transparent: Option<bool>,
    type_: Option<AssetFileType>,
}

impl SearchAssetFilesBuilder<'_> {
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
    /// The file was generated from an edit
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = Some(value);
        self
    }
    /// Sets or clears `isEdited`.
    pub fn set_is_edited(mut self, value: Option<bool>) -> Self {
        self.is_edited = value;
        self
    }
    /// The file is a progressively encoded JPEG
    pub fn is_progressive(mut self, value: bool) -> Self {
        self.is_progressive = Some(value);
        self
    }
    /// Sets or clears `isProgressive`.
    pub fn set_is_progressive(mut self, value: Option<bool>) -> Self {
        self.is_progressive = value;
        self
    }
    /// The file is transparent
    pub fn is_transparent(mut self, value: bool) -> Self {
        self.is_transparent = Some(value);
        self
    }
    /// Sets or clears `isTransparent`.
    pub fn set_is_transparent(mut self, value: Option<bool>) -> Self {
        self.is_transparent = value;
        self
    }
    /// Filter by type of file
    pub fn type_(mut self, value: AssetFileType) -> Self {
        self.type_ = Some(value);
        self
    }
    /// Sets or clears `type`.
    pub fn set_type_(mut self, value: Option<AssetFileType>) -> Self {
        self.type_ = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetFileResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "assetId")]
            asset_id: &'b AssetId,
            #[serde(rename = "isEdited", skip_serializing_if = "Option::is_none")]
            is_edited: Option<&'b bool>,
            #[serde(rename = "isProgressive", skip_serializing_if = "Option::is_none")]
            is_progressive: Option<&'b bool>,
            #[serde(rename = "isTransparent", skip_serializing_if = "Option::is_none")]
            is_transparent: Option<&'b bool>,
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            type_: Option<&'b AssetFileType>,
        }
        tracing::trace!(
            module = "assets/files",
            method = "GET",
            path = "/asset-files",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/asset-files")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            asset_id: &self.asset_id,
            is_edited: self.is_edited.as_ref(),
            is_progressive: self.is_progressive.as_ref(),
            is_transparent: self.is_transparent.as_ref(),
            type_: self.type_.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<Vec<AssetFileResponseDTO>>("assets/files", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchAssetFilesBuilder<'a> {
    type Output = Result<Vec<AssetFileResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Files` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Files<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Files<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Delete an asset file
    ///
    /// Delete a file and remove it from the database.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetFileId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_asset_file(&self, id: impl Into<AssetFileId>) -> DeleteAssetFileBuilder<'a> {
        DeleteAssetFileBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Download an asset file
    ///
    /// Serve the contents of a specific asset file.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetFileId` | ✓ |  |
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
    pub fn download_asset_file(&self, id: impl Into<AssetFileId>) -> DownloadAssetFileBuilder<'a> {
        DownloadAssetFileBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve an asset file
    ///
    /// Returns metadata about a specific asset file.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AssetFileId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetFileResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_asset_file(&self, id: impl Into<AssetFileId>) -> GetAssetFileBuilder<'a> {
        GetAssetFileBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Search asset files
    ///
    /// Returns all matching asset files.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `asset_id` | `AssetId` | ✓ | Asset ID to filter files by |
    /// | `is_edited` | `bool` |  | The file was generated from an edit |
    /// | `is_progressive` | `bool` |  | The file is a progressively encoded JPEG |
    /// | `is_transparent` | `bool` |  | The file is transparent |
    /// | `type_` | `AssetFileType` |  | Filter by type of file |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetFileResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn search_asset_files(&self, asset_id: impl Into<AssetId>) -> SearchAssetFilesBuilder<'a> {
        SearchAssetFilesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            asset_id: asset_id.into(),
            is_edited: None,
            is_progressive: None,
            is_transparent: None,
            type_: None,
        }
    }
}
