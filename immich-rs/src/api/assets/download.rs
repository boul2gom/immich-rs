// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `download`.

use immich_models::assets::download::{DownloadArchiveDTO, DownloadInfoDTO, DownloadResponseDTO};

/// Fluent request builder for [`Download::download_archive`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DownloadArchiveBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip)]
    pub body: &'a DownloadArchiveDTO,
}

/// Type alias for [`DownloadArchiveBuilder`].
pub type DownloadArchiveArgs<'a> = DownloadArchiveBuilder<'a>;
impl DownloadArchiveBuilder<'_> {
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
            module = "Download",
            method = "POST",
            path = "/download/archive",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/download/archive")?;
        request = self.request_config.apply(request);
        request = request.query(&self);
        request = request.json(self.body);
        let raw_response = self.client.execute_stream("download", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for DownloadArchiveBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Download::get_download_info`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetDownloadInfoBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip)]
    pub body: &'a DownloadInfoDTO,
}

/// Type alias for [`GetDownloadInfoBuilder`].
pub type GetDownloadInfoArgs<'a> = GetDownloadInfoBuilder<'a>;
impl GetDownloadInfoBuilder<'_> {
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
    pub async fn send(self) -> Result<DownloadResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Download",
            method = "POST",
            path = "/download/info",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/download/info")?;
        request = self.request_config.apply(request);
        request = request.query(&self);
        request = request.json(self.body);

        let response = self.client.execute::<DownloadResponseDTO>("download", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetDownloadInfoBuilder<'a> {
    type Output = Result<DownloadResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Download` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Download<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Download<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Download asset archive
    /// Download a ZIP archive containing the specified assets. The assets must have been previously requested via the "getDownloadInfo" endpoint.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    /// | `body` | `DownloadArchiveDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn download_archive(&self, body: &'a DownloadArchiveDTO) -> DownloadArchiveBuilder<'a> {
        DownloadArchiveBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
            body,
        }
    }

    /// Retrieve download information
    /// Retrieve information about how to request a download for the specified assets or album. The response includes groups of assets that can be downloaded together.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    /// | `body` | `DownloadInfoDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `DownloadResponseDTO`
    pub fn get_download_info(&self, body: &'a DownloadInfoDTO) -> GetDownloadInfoBuilder<'a> {
        GetDownloadInfoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
            body,
        }
    }
}
