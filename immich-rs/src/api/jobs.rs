// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `jobs`.

use immich_models::jobs::{JobCreateDTO, QueueCommandDTO, QueueName, QueueResponseLegacyDTO, QueuesResponseLegacyDTO};

/// Fluent request builder for [`Jobs::create_job`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateJobBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a JobCreateDTO,
}

/// Type alias for [`CreateJobBuilder`].
pub type CreateJobArgs<'a> = CreateJobBuilder<'a>;
impl CreateJobBuilder<'_> {
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
        tracing::trace!(module = "Jobs", method = "POST", path = "/jobs", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/jobs")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("jobs", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for CreateJobBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Jobs::get_queues_legacy`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetQueuesLegacyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetQueuesLegacyBuilder`].
pub type GetQueuesLegacyArgs<'a> = GetQueuesLegacyBuilder<'a>;
impl GetQueuesLegacyBuilder<'_> {
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
    pub async fn send(self) -> Result<QueuesResponseLegacyDTO, crate::Error> {
        tracing::trace!(module = "Jobs", method = "GET", path = "/jobs", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/jobs")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<QueuesResponseLegacyDTO>("jobs", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetQueuesLegacyBuilder<'a> {
    type Output = Result<QueuesResponseLegacyDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Jobs::run_queue_command_legacy`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RunQueueCommandLegacyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub name: QueueName,
    #[serde(skip)]
    pub body: &'a QueueCommandDTO,
}

/// Type alias for [`RunQueueCommandLegacyBuilder`].
pub type RunQueueCommandLegacyArgs<'a> = RunQueueCommandLegacyBuilder<'a>;
impl RunQueueCommandLegacyBuilder<'_> {
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
    pub async fn send(self) -> Result<QueueResponseLegacyDTO, crate::Error> {
        tracing::trace!(module = "Jobs", method = "PUT", path = "/jobs/{name}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/jobs/{}", self.name))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<QueueResponseLegacyDTO>("jobs", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for RunQueueCommandLegacyBuilder<'a> {
    type Output = Result<QueueResponseLegacyDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Jobs` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Jobs<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Jobs<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create a manual job
    /// Run a specific job. Most jobs are queued automatically, but this endpoint allows for manual creation of a handful of jobs, including various cleanup tasks, as well as creating a new database backup.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `JobCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn create_job(&self, body: &'a JobCreateDTO) -> CreateJobBuilder<'a> {
        CreateJobBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Retrieve queue counts and status
    /// Retrieve the counts of the current queue, as well as the current status.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `QueuesResponseLegacyDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn get_queues_legacy(&self) -> GetQueuesLegacyBuilder<'a> {
        GetQueuesLegacyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Run jobs
    /// Queue all assets for a specific job type. Defaults to only queueing assets that have not yet been processed, but the force command can be used to re-process all assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `name` | `QueueName` | ✓ |  |
    /// | `body` | `QueueCommandDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `QueueResponseLegacyDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn run_queue_command_legacy(
        &self,
        name: QueueName,
        body: &'a QueueCommandDTO,
    ) -> RunQueueCommandLegacyBuilder<'a> {
        RunQueueCommandLegacyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name,
            body,
        }
    }
}
