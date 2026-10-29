// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `queues`.

use immich_models::jobs::QueueName;
use immich_models::queues::{QueueDeleteDTO, QueueJobResponseDTO, QueueJobStatus, QueueResponseDTO, QueueUpdateDTO};

/// Fluent request builder for [`Queues::empty_queue`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct EmptyQueueBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub name: QueueName,
    #[serde(skip)]
    pub body: &'a QueueDeleteDTO,
}

/// Type alias for [`EmptyQueueBuilder`].
pub type EmptyQueueArgs<'a> = EmptyQueueBuilder<'a>;
impl EmptyQueueBuilder<'_> {
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
            module = "Queues",
            method = "DELETE",
            path = "/queues/{name}/jobs",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/queues/{}/jobs", self.name))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetQueueBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub name: QueueName,
}

/// Type alias for [`GetQueueBuilder`].
pub type GetQueueArgs<'a> = GetQueueBuilder<'a>;
impl GetQueueBuilder<'_> {
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
    pub async fn send(self) -> Result<QueueResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Queues",
            method = "GET",
            path = "/queues/{name}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/queues/{}", self.name))?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetQueueJobsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub name: QueueName,
    #[serde(skip)]
    pub status: Option<Vec<QueueJobStatus>>,
}

/// Type alias for [`GetQueueJobsBuilder`].
pub type GetQueueJobsArgs<'a> = GetQueueJobsBuilder<'a>;
impl GetQueueJobsBuilder<'_> {
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

    /// Filter jobs by status
    pub fn status(mut self, value: Vec<QueueJobStatus>) -> Self {
        self.status = Some(value);
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<QueueJobResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Queues",
            method = "GET",
            path = "/queues/{name}/jobs",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/queues/{}/jobs", self.name))?;
        request = self.request_config.apply(request);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetQueuesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetQueuesBuilder`].
pub type GetQueuesArgs<'a> = GetQueuesBuilder<'a>;
impl GetQueuesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<QueueResponseDTO>, crate::Error> {
        tracing::trace!(module = "Queues", method = "GET", path = "/queues", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/queues")?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateQueueBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub name: QueueName,
    #[serde(skip)]
    pub body: &'a QueueUpdateDTO,
}

/// Type alias for [`UpdateQueueBuilder`].
pub type UpdateQueueArgs<'a> = UpdateQueueBuilder<'a>;
impl UpdateQueueBuilder<'_> {
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
    pub async fn send(self) -> Result<QueueResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Queues",
            method = "PUT",
            path = "/queues/{name}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/queues/{}", self.name))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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

///Client handle for `Queues` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Queues<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Queues<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Empty a queue
    /// Removes all jobs from the specified queue.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `name` | `QueueName` | ✓ |  |
    /// | `body` | `QueueDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn empty_queue(&self, name: QueueName, body: &'a QueueDeleteDTO) -> EmptyQueueBuilder<'a> {
        EmptyQueueBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name,
            body,
        }
    }

    /// Retrieve a queue
    /// Retrieves a specific queue by its name.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `name` | `QueueName` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `QueueResponseDTO`
    pub fn get_queue(&self, name: QueueName) -> GetQueueBuilder<'a> {
        GetQueueBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name,
        }
    }

    /// Retrieve queue jobs
    /// Retrieves a list of queue jobs from the specified queue.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `name` | `QueueName` | ✓ |  |
    /// | `status` | `Vec<QueueJobStatus>` |  | Filter jobs by status |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<QueueJobResponseDTO>`
    pub fn get_queue_jobs(&self, name: QueueName) -> GetQueueJobsBuilder<'a> {
        GetQueueJobsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name,
            status: None,
        }
    }

    /// List all queues
    /// Retrieves a list of queues.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<QueueResponseDTO>`
    pub fn get_queues(&self) -> GetQueuesBuilder<'a> {
        GetQueuesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Update a queue
    /// Change the paused status of a specific queue.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `name` | `QueueName` | ✓ |  |
    /// | `body` | `QueueUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `QueueResponseDTO`
    pub fn update_queue(&self, name: QueueName, body: &'a QueueUpdateDTO) -> UpdateQueueBuilder<'a> {
        UpdateQueueBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name,
            body,
        }
    }
}
