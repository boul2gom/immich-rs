// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `trash`.

use immich_models::albums::BulkIdsDTO;
use immich_models::trash::TrashResponseDTO;

/// Fluent request builder for [`Trash::empty_trash`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct EmptyTrashBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`EmptyTrashBuilder`].
pub type EmptyTrashArgs<'a> = EmptyTrashBuilder<'a>;
impl EmptyTrashBuilder<'_> {
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
    pub async fn send(self) -> Result<TrashResponseDTO, crate::Error> {
        tracing::trace!(module = "Trash", method = "POST", path = "/trash/empty", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/trash/empty")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<TrashResponseDTO>("trash", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for EmptyTrashBuilder<'a> {
    type Output = Result<TrashResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Trash::restore_assets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RestoreAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a BulkIdsDTO,
}

/// Type alias for [`RestoreAssetsBuilder`].
pub type RestoreAssetsArgs<'a> = RestoreAssetsBuilder<'a>;
impl RestoreAssetsBuilder<'_> {
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
    pub async fn send(self) -> Result<TrashResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Trash",
            method = "POST",
            path = "/trash/restore/assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/trash/restore/assets")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<TrashResponseDTO>("trash", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for RestoreAssetsBuilder<'a> {
    type Output = Result<TrashResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Trash::restore_trash`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RestoreTrashBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`RestoreTrashBuilder`].
pub type RestoreTrashArgs<'a> = RestoreTrashBuilder<'a>;
impl RestoreTrashBuilder<'_> {
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
    pub async fn send(self) -> Result<TrashResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Trash",
            method = "POST",
            path = "/trash/restore",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/trash/restore")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<TrashResponseDTO>("trash", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for RestoreTrashBuilder<'a> {
    type Output = Result<TrashResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Trash` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Trash<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Trash<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Empty trash
    /// Permanently delete all items in the trash.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `TrashResponseDTO`
    pub fn empty_trash(&self) -> EmptyTrashBuilder<'a> {
        EmptyTrashBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Restore assets
    /// Restore specific assets from the trash.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `BulkIdsDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `TrashResponseDTO`
    pub fn restore_assets(&self, body: &'a BulkIdsDTO) -> RestoreAssetsBuilder<'a> {
        RestoreAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Restore trash
    /// Restore all items in the trash.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `TrashResponseDTO`
    pub fn restore_trash(&self) -> RestoreTrashBuilder<'a> {
        RestoreTrashBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
}
