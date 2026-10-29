// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `sessions`.

use immich_models::sessions::{SessionCreateDTO, SessionCreateResponseDTO, SessionResponseDTO, SessionUpdateDTO};

/// Fluent request builder for [`Sessions::create_session`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateSessionBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a SessionCreateDTO,
}

/// Type alias for [`CreateSessionBuilder`].
pub type CreateSessionArgs<'a> = CreateSessionBuilder<'a>;
impl CreateSessionBuilder<'_> {
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
    pub async fn send(self) -> Result<SessionCreateResponseDTO, crate::Error> {
        tracing::trace!(module = "Sessions", method = "POST", path = "/sessions", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/sessions")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<SessionCreateResponseDTO>("sessions", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateSessionBuilder<'a> {
    type Output = Result<SessionCreateResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Sessions::delete_all_sessions`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteAllSessionsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`DeleteAllSessionsBuilder`].
pub type DeleteAllSessionsArgs<'a> = DeleteAllSessionsBuilder<'a>;
impl DeleteAllSessionsBuilder<'_> {
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
            module = "Sessions",
            method = "DELETE",
            path = "/sessions",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/sessions")?;
        request = self.request_config.apply(request);
        self.client.execute_unit("sessions", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteAllSessionsBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Sessions::delete_session`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteSessionBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`DeleteSessionBuilder`].
pub type DeleteSessionArgs<'a> = DeleteSessionBuilder<'a>;
impl DeleteSessionBuilder<'_> {
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
            module = "Sessions",
            method = "DELETE",
            path = "/sessions/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/sessions/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("sessions", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteSessionBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Sessions::get_sessions`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetSessionsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetSessionsBuilder`].
pub type GetSessionsArgs<'a> = GetSessionsBuilder<'a>;
impl GetSessionsBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<SessionResponseDTO>, crate::Error> {
        tracing::trace!(module = "Sessions", method = "GET", path = "/sessions", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/sessions")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<Vec<SessionResponseDTO>>("sessions", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetSessionsBuilder<'a> {
    type Output = Result<Vec<SessionResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Sessions::lock_session`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct LockSessionBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`LockSessionBuilder`].
pub type LockSessionArgs<'a> = LockSessionBuilder<'a>;
impl LockSessionBuilder<'_> {
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
            module = "Sessions",
            method = "POST",
            path = "/sessions/{id}/lock",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, &format!("/sessions/{}/lock", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("sessions", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for LockSessionBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Sessions::update_session`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateSessionBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a SessionUpdateDTO,
}

/// Type alias for [`UpdateSessionBuilder`].
pub type UpdateSessionArgs<'a> = UpdateSessionBuilder<'a>;
impl UpdateSessionBuilder<'_> {
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
    pub async fn send(self) -> Result<SessionResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Sessions",
            method = "PUT",
            path = "/sessions/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/sessions/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<SessionResponseDTO>("sessions", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateSessionBuilder<'a> {
    type Output = Result<SessionResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Sessions` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Sessions<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Sessions<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create a session
    /// Create a session as a child to the current session. This endpoint is used for casting.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `SessionCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `SessionCreateResponseDTO`
    pub fn create_session(&self, body: &'a SessionCreateDTO) -> CreateSessionBuilder<'a> {
        CreateSessionBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete all sessions
    /// Delete all sessions for the user. This will not delete the current session.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_all_sessions(&self) -> DeleteAllSessionsBuilder<'a> {
        DeleteAllSessionsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Delete a session
    /// Delete a specific session by id.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_session(&self, id: uuid::Uuid) -> DeleteSessionBuilder<'a> {
        DeleteSessionBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Retrieve sessions
    /// Retrieve a list of sessions for the user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<SessionResponseDTO>`
    pub fn get_sessions(&self) -> GetSessionsBuilder<'a> {
        GetSessionsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Lock a session
    /// Lock a specific session by id.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn lock_session(&self, id: uuid::Uuid) -> LockSessionBuilder<'a> {
        LockSessionBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Update a session
    /// Update a specific session identified by id.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `SessionUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `SessionResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_session(&self, id: uuid::Uuid, body: &'a SessionUpdateDTO) -> UpdateSessionBuilder<'a> {
        UpdateSessionBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }
}
