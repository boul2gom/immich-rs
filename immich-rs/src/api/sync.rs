// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `sync`.

use immich_models::sync::{SyncAckDTO, SyncAckDeleteDTO, SyncAckSetDTO, SyncStreamDTO};

/// Fluent request builder for [`Sync::delete_sync_ack`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteSyncAckBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a SyncAckDeleteDTO,
}

/// Type alias for [`DeleteSyncAckBuilder`].
pub type DeleteSyncAckArgs<'a> = DeleteSyncAckBuilder<'a>;
impl DeleteSyncAckBuilder<'_> {
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
        tracing::trace!(module = "Sync", method = "DELETE", path = "/sync/ack", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/sync/ack")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("sync", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteSyncAckBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Sync::get_sync_ack`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetSyncAckBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetSyncAckBuilder`].
pub type GetSyncAckArgs<'a> = GetSyncAckBuilder<'a>;
impl GetSyncAckBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<SyncAckDTO>, crate::Error> {
        tracing::trace!(module = "Sync", method = "GET", path = "/sync/ack", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/sync/ack")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<Vec<SyncAckDTO>>("sync", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetSyncAckBuilder<'a> {
    type Output = Result<Vec<SyncAckDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Sync::get_sync_stream`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetSyncStreamBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a SyncStreamDTO,
}

/// Type alias for [`GetSyncStreamBuilder`].
pub type GetSyncStreamArgs<'a> = GetSyncStreamBuilder<'a>;
impl GetSyncStreamBuilder<'_> {
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
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(module = "Sync", method = "POST", path = "/sync/stream", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/sync/stream")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        let raw_response = self.client.execute_stream("sync", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for GetSyncStreamBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Sync::send_sync_ack`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SendSyncAckBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a SyncAckSetDTO,
}

/// Type alias for [`SendSyncAckBuilder`].
pub type SendSyncAckArgs<'a> = SendSyncAckBuilder<'a>;
impl SendSyncAckBuilder<'_> {
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
        tracing::trace!(module = "Sync", method = "POST", path = "/sync/ack", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/sync/ack")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("sync", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for SendSyncAckBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Sync` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Sync<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Sync<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Delete acknowledgements
    /// Delete specific synchronization acknowledgments.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `SyncAckDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_sync_ack(&self, body: &'a SyncAckDeleteDTO) -> DeleteSyncAckBuilder<'a> {
        DeleteSyncAckBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Retrieve acknowledgements
    /// Retrieve the synchronization acknowledgments for the current session.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<SyncAckDTO>`
    pub fn get_sync_ack(&self) -> GetSyncAckBuilder<'a> {
        GetSyncAckBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Stream sync changes
    /// Retrieve a JSON lines streamed response of changes for synchronization. This endpoint is used by the mobile app to efficiently stay up to date with changes.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `SyncStreamDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    pub fn get_sync_stream(&self, body: &'a SyncStreamDTO) -> GetSyncStreamBuilder<'a> {
        GetSyncStreamBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Acknowledge changes
    /// Send a list of synchronization acknowledgements to confirm that the latest changes have been received.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `SyncAckSetDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn send_sync_ack(&self, body: &'a SyncAckSetDTO) -> SendSyncAckBuilder<'a> {
        SendSyncAckBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
