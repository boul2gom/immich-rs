// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `views`.

use immich_models::assets::AssetResponseDTO;

/// Fluent request builder for [`Views::get_assets_by_original_path`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetsByOriginalPathBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    path: String,
}

impl GetAssetsByOriginalPathBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<AssetResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "path")]
            path: &'b String,
        }
        tracing::trace!(module = "views", method = "GET", path = "/view/folder", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/view/folder")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams { path: &self.path };
        request = request.query(&query);
        let response = self.client.execute::<Vec<AssetResponseDTO>>("views", request).await?;
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

/// Fluent request builder for [`Views::get_unique_original_paths`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUniqueOriginalPathsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetUniqueOriginalPathsBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<String>, crate::Error> {
        tracing::trace!(
            module = "views",
            method = "GET",
            path = "/view/folder/unique-paths",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/view/folder/unique-paths")?;
        request = self.request_config.apply(request)?;
        let response = self.client.execute::<Vec<String>>("views", request).await?;
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

/// Client handle for `Views` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Views<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Views<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Retrieve assets by original path
    ///
    /// Retrieve assets that are children of a specific folder.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `path` | `String` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_assets_by_original_path(&self, path: impl Into<String>) -> GetAssetsByOriginalPathBuilder<'a> {
        GetAssetsByOriginalPathBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            path: path.into(),
        }
    }
    /// Retrieve unique paths
    ///
    /// Retrieve a list of unique folder paths from asset original paths.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<String>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_unique_original_paths(&self) -> GetUniqueOriginalPathsBuilder<'a> {
        GetUniqueOriginalPathsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
}
