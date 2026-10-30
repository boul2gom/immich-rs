// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `workflows`.

use immich_models::id::WorkflowId;
use immich_models::plugins::WorkflowTrigger;
use immich_models::workflows::{
    WorkflowCreateDTO, WorkflowLogEntryDTO, WorkflowResponseDTO, WorkflowResult, WorkflowShareResponseDTO,
    WorkflowTriggerResponseDTO, WorkflowUpdateDTO,
};

/// Fluent request builder for [`Workflows::create_workflow`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateWorkflowBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: WorkflowCreateDTO,
}

impl CreateWorkflowBuilder<'_> {
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
    pub async fn send(self) -> Result<WorkflowResponseDTO, crate::Error> {
        tracing::trace!(
            module = "workflows",
            method = "POST",
            path = "/workflows",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/workflows")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<WorkflowResponseDTO>("workflows", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateWorkflowBuilder<'a> {
    type Output = Result<WorkflowResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Workflows::delete_workflow`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteWorkflowBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: WorkflowId,
}

impl DeleteWorkflowBuilder<'_> {
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
            module = "workflows",
            method = "DELETE",
            path = "/workflows/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("workflows"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("workflows", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteWorkflowBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Workflows::get_workflow`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetWorkflowBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: WorkflowId,
}

impl GetWorkflowBuilder<'_> {
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
    pub async fn send(self) -> Result<WorkflowResponseDTO, crate::Error> {
        tracing::trace!(
            module = "workflows",
            method = "GET",
            path = "/workflows/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("workflows"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self.client.execute::<WorkflowResponseDTO>("workflows", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetWorkflowBuilder<'a> {
    type Output = Result<WorkflowResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Workflows::get_workflow_for_share`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetWorkflowForShareBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: WorkflowId,
}

impl GetWorkflowForShareBuilder<'_> {
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
    pub async fn send(self) -> Result<WorkflowShareResponseDTO, crate::Error> {
        tracing::trace!(
            module = "workflows",
            method = "GET",
            path = "/workflows/{id}/share",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("workflows"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("share"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<WorkflowShareResponseDTO>("workflows", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetWorkflowForShareBuilder<'a> {
    type Output = Result<WorkflowShareResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Workflows::get_workflow_logs`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetWorkflowLogsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    before: Option<chrono::DateTime<chrono::Utc>>,
    id: WorkflowId,
    limit: Option<i64>,
    result: Option<WorkflowResult>,
}

impl GetWorkflowLogsBuilder<'_> {
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
    /// Filter by runs before a date/time
    pub fn before(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.before = Some(value);
        self
    }
    /// Sets or clears `before`.
    pub fn set_before(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.before = value;
        self
    }
    /// Maximum number of logs
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Sets or clears `limit`.
    pub fn set_limit(mut self, value: Option<i64>) -> Self {
        self.limit = value;
        self
    }
    /// Filter by run result
    pub fn result(mut self, value: WorkflowResult) -> Self {
        self.result = Some(value);
        self
    }
    /// Sets or clears `result`.
    pub fn set_result(mut self, value: Option<WorkflowResult>) -> Self {
        self.result = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<WorkflowLogEntryDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
            before: Option<&'b chrono::DateTime<chrono::Utc>>,
            #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
            limit: Option<&'b i64>,
            #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
            result: Option<&'b WorkflowResult>,
        }
        tracing::trace!(
            module = "workflows",
            method = "GET",
            path = "/workflows/{id}/logs",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("workflows"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("logs"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            before: self.before.as_ref(),
            limit: self.limit.as_ref(),
            result: self.result.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<Vec<WorkflowLogEntryDTO>>("workflows", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetWorkflowLogsBuilder<'a> {
    type Output = Result<Vec<WorkflowLogEntryDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Workflows::get_workflow_triggers`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetWorkflowTriggersBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetWorkflowTriggersBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<WorkflowTriggerResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "workflows",
            method = "GET",
            path = "/workflows/triggers",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/workflows/triggers")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<Vec<WorkflowTriggerResponseDTO>>("workflows", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetWorkflowTriggersBuilder<'a> {
    type Output = Result<Vec<WorkflowTriggerResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Workflows::search_workflows`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchWorkflowsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    description: Option<String>,
    enabled: Option<bool>,
    id: Option<WorkflowId>,
    logging: Option<bool>,
    name: Option<String>,
    trigger: Option<WorkflowTrigger>,
}

impl SearchWorkflowsBuilder<'_> {
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
    /// Workflow description
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Sets or clears `description`.
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Workflow enabled
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Sets or clears `enabled`.
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }
    /// Workflow ID
    pub fn id(mut self, value: impl Into<WorkflowId>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<WorkflowId>) -> Self {
        self.id = value;
        self
    }
    /// Workflow logs run results
    pub fn logging(mut self, value: bool) -> Self {
        self.logging = Some(value);
        self
    }
    /// Sets or clears `logging`.
    pub fn set_logging(mut self, value: Option<bool>) -> Self {
        self.logging = value;
        self
    }
    /// Workflow name
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Workflow trigger type
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = Some(value);
        self
    }
    /// Sets or clears `trigger`.
    pub fn set_trigger(mut self, value: Option<WorkflowTrigger>) -> Self {
        self.trigger = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<WorkflowResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
            description: Option<&'b String>,
            #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
            enabled: Option<&'b bool>,
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b WorkflowId>,
            #[serde(rename = "logging", skip_serializing_if = "Option::is_none")]
            logging: Option<&'b bool>,
            #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
            name: Option<&'b String>,
            #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
            trigger: Option<&'b WorkflowTrigger>,
        }
        tracing::trace!(
            module = "workflows",
            method = "GET",
            path = "/workflows",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/workflows")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            description: self.description.as_ref(),
            enabled: self.enabled.as_ref(),
            id: self.id.as_ref(),
            logging: self.logging.as_ref(),
            name: self.name.as_ref(),
            trigger: self.trigger.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<Vec<WorkflowResponseDTO>>("workflows", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchWorkflowsBuilder<'a> {
    type Output = Result<Vec<WorkflowResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Workflows::update_workflow`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateWorkflowBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: WorkflowId,
    body: WorkflowUpdateDTO,
}

impl UpdateWorkflowBuilder<'_> {
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
    pub async fn send(self) -> Result<WorkflowResponseDTO, crate::Error> {
        tracing::trace!(
            module = "workflows",
            method = "PUT",
            path = "/workflows/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("workflows"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<WorkflowResponseDTO>("workflows", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateWorkflowBuilder<'a> {
    type Output = Result<WorkflowResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Workflows` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Workflows<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Workflows<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Create a workflow
    ///
    /// Create a new workflow, the workflow can also be created with empty filters and actions.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `WorkflowCreateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `WorkflowResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn create_workflow(&self, body: impl Into<WorkflowCreateDTO>) -> CreateWorkflowBuilder<'a> {
        CreateWorkflowBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete a workflow
    ///
    /// Delete a workflow by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `WorkflowId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_workflow(&self, id: impl Into<WorkflowId>) -> DeleteWorkflowBuilder<'a> {
        DeleteWorkflowBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve a workflow
    ///
    /// Retrieve information about a specific workflow by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `WorkflowId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `WorkflowResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_workflow(&self, id: impl Into<WorkflowId>) -> GetWorkflowBuilder<'a> {
        GetWorkflowBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve a workflow
    ///
    /// Retrieve a workflow details without ids, default values, etc.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `WorkflowId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `WorkflowShareResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_workflow_for_share(&self, id: impl Into<WorkflowId>) -> GetWorkflowForShareBuilder<'a> {
        GetWorkflowForShareBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve workflow logs
    ///
    /// Retrieve logs of a workflows runs by ID
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `before` | `chrono::DateTime<chrono::Utc>` |  | Filter by runs before a date/time |
    /// | `id` | `WorkflowId` | ✓ |  |
    /// | `limit` | `i64` |  | Maximum number of logs |
    /// | `result` | `WorkflowResult` |  | Filter by run result |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<WorkflowLogEntryDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_workflow_logs(&self, id: impl Into<WorkflowId>) -> GetWorkflowLogsBuilder<'a> {
        GetWorkflowLogsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            before: None,
            id: id.into(),
            limit: None,
            result: None,
        }
    }
    /// List all workflow triggers
    ///
    /// Retrieve a list of all available workflow triggers.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<WorkflowTriggerResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_workflow_triggers(&self) -> GetWorkflowTriggersBuilder<'a> {
        GetWorkflowTriggersBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// List all workflows
    ///
    /// Retrieve a list of workflows available to the authenticated user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `description` | `String` |  | Workflow description |
    /// | `enabled` | `bool` |  | Workflow enabled |
    /// | `id` | `WorkflowId` |  | Workflow ID |
    /// | `logging` | `bool` |  | Workflow logs run results |
    /// | `name` | `String` |  | Workflow name |
    /// | `trigger` | `WorkflowTrigger` |  | Workflow trigger type |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<WorkflowResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn search_workflows(&self) -> SearchWorkflowsBuilder<'a> {
        SearchWorkflowsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            description: None,
            enabled: None,
            id: None,
            logging: None,
            name: None,
            trigger: None,
        }
    }
    /// Update a workflow
    ///
    /// Update the information of a specific workflow by its ID. This endpoint can be used to update the workflow name, description, trigger type, filters and actions order, etc.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `WorkflowId` | ✓ |  |
    /// | `body` | `WorkflowUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `WorkflowResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_workflow(
        &self,
        id: impl Into<WorkflowId>,
        body: impl Into<WorkflowUpdateDTO>,
    ) -> UpdateWorkflowBuilder<'a> {
        UpdateWorkflowBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
}
