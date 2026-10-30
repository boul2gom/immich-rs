// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `downloads`.

use immich_models::assets::downloads::{DownloadArchiveDTO, DownloadInfoDTO, DownloadResponseDTO};

/// Fluent request builder for [`Downloads::download_archive`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DownloadArchiveBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    key: Option<String>,
    slug: Option<String>,
    body: DownloadArchiveDTO,
}

impl DownloadArchiveBuilder<'_> {
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
    pub async fn send(self) -> Result<crate::ByteStream, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "assets/downloads",
            method = "POST",
            path = "/download/archive",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/download/archive")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        request = request.json(&self.body);
        let raw_response = self.client.execute_stream("assets/downloads", request).await?;
        Ok(crate::ByteStream::new(raw_response))
    }
}

impl<'a> std::future::IntoFuture for DownloadArchiveBuilder<'a> {
    type Output = Result<crate::ByteStream, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Downloads::get_download_info`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetDownloadInfoBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    key: Option<String>,
    slug: Option<String>,
    body: DownloadInfoDTO,
}

impl GetDownloadInfoBuilder<'_> {
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
    pub async fn send(self) -> Result<DownloadResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "assets/downloads",
            method = "POST",
            path = "/download/info",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/download/info")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<DownloadResponseDTO>("assets/downloads", request)
            .await?;
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

/// Client handle for `Downloads` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Downloads<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Downloads<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Download asset archive
    ///
    /// Download a ZIP archive containing the specified assets. The assets must have been previously requested via the "`getDownloadInfo`" endpoint.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    /// | `body` | `DownloadArchiveDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `crate::ByteStream`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn download_archive(&self, body: impl Into<DownloadArchiveDTO>) -> DownloadArchiveBuilder<'a> {
        DownloadArchiveBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
            body: body.into(),
        }
    }
    /// Retrieve download information
    ///
    /// Retrieve information about how to request a download for the specified assets or album. The response includes groups of assets that can be downloaded together.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    /// | `body` | `DownloadInfoDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `DownloadResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_download_info(&self, body: impl Into<DownloadInfoDTO>) -> GetDownloadInfoBuilder<'a> {
        GetDownloadInfoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
            body: body.into(),
        }
    }
}
