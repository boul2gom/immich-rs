// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `queues`.

use immich_models::jobs::QueueName;
use immich_models::queues::{QueueDeleteDTO, QueueJobResponseDTO, QueueJobStatus, QueueResponseDTO, QueueUpdateDTO};

/// Fluent request builder for [`Queues::empty_queue`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct EmptyQueueBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    name: QueueName,
    body: QueueDeleteDTO,
}

impl EmptyQueueBuilder<'_> {
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
            module = "queues",
            method = "DELETE",
            path = "/queues/{name}/jobs",
            "HTTP request"
        );
        let __seg_1 = self.name.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("queues"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("jobs"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("queues", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for EmptyQueueBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Queues::get_queue`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetQueueBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    name: QueueName,
}

impl GetQueueBuilder<'_> {
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
    pub async fn send(self) -> Result<QueueResponseDTO, crate::Error> {
        tracing::trace!(
            module = "queues",
            method = "GET",
            path = "/queues/{name}",
            "HTTP request"
        );
        let __seg_1 = self.name.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("queues"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self.client.execute::<QueueResponseDTO>("queues", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetQueueBuilder<'a> {
    type Output = Result<QueueResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Queues::get_queue_jobs`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetQueueJobsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    name: QueueName,
    status: Option<Vec<QueueJobStatus>>,
}

impl GetQueueJobsBuilder<'_> {
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
    /// Filter jobs by status
    pub fn status(mut self, value: impl IntoIterator<Item = QueueJobStatus>) -> Self {
        self.status = Some(value.into_iter().collect());
        self
    }
    /// Sets or clears `status`.
    pub fn set_status(mut self, value: Option<Vec<QueueJobStatus>>) -> Self {
        self.status = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<QueueJobResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "queues",
            method = "GET",
            path = "/queues/{name}/jobs",
            "HTTP request"
        );
        let __seg_1 = self.name.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("queues"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("jobs"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        if let Some(items) = &self.status {
            for item in items {
                request = request.query(&[("status", item)]);
            }
        }
        let response = self
            .client
            .execute::<Vec<QueueJobResponseDTO>>("queues", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetQueueJobsBuilder<'a> {
    type Output = Result<Vec<QueueJobResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Queues::get_queues`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetQueuesBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetQueuesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<QueueResponseDTO>, crate::Error> {
        tracing::trace!(module = "queues", method = "GET", path = "/queues", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/queues")?;
        request = self.request_config.apply(request)?;
        let response = self.client.execute::<Vec<QueueResponseDTO>>("queues", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetQueuesBuilder<'a> {
    type Output = Result<Vec<QueueResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Queues::update_queue`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateQueueBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    name: QueueName,
    body: QueueUpdateDTO,
}

impl UpdateQueueBuilder<'_> {
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
    pub async fn send(self) -> Result<QueueResponseDTO, crate::Error> {
        tracing::trace!(
            module = "queues",
            method = "PUT",
            path = "/queues/{name}",
            "HTTP request"
        );
        let __seg_1 = self.name.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("queues"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<QueueResponseDTO>("queues", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateQueueBuilder<'a> {
    type Output = Result<QueueResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Queues` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Queues<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Queues<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Empty a queue
    ///
    /// Removes all jobs from the specified queue.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `name` | `QueueName` | ✓ |  |
    /// | `body` | `QueueDeleteDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn empty_queue(&self, name: QueueName, body: impl Into<QueueDeleteDTO>) -> EmptyQueueBuilder<'a> {
        EmptyQueueBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name,
            body: body.into(),
        }
    }
    /// Retrieve a queue
    ///
    /// Retrieves a specific queue by its name.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `name` | `QueueName` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `QueueResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_queue(&self, name: QueueName) -> GetQueueBuilder<'a> {
        GetQueueBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name,
        }
    }
    /// Retrieve queue jobs
    ///
    /// Retrieves a list of queue jobs from the specified queue.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `name` | `QueueName` | ✓ |  |
    /// | `status` | `Vec<QueueJobStatus>` |  | Filter jobs by status |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<QueueJobResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_queue_jobs(&self, name: QueueName) -> GetQueueJobsBuilder<'a> {
        GetQueueJobsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name,
            status: None,
        }
    }
    /// List all queues
    ///
    /// Retrieves a list of queues.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<QueueResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_queues(&self) -> GetQueuesBuilder<'a> {
        GetQueuesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Update a queue
    ///
    /// Change the paused status of a specific queue.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `name` | `QueueName` | ✓ |  |
    /// | `body` | `QueueUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `QueueResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_queue(&self, name: QueueName, body: impl Into<QueueUpdateDTO>) -> UpdateQueueBuilder<'a> {
        UpdateQueueBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name,
            body: body.into(),
        }
    }
}
