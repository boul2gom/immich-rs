// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `api_keys`.

use immich_models::api_keys::{ApiKeyCreateDTO, ApiKeyCreateResponseDTO, ApiKeyResponseDTO, ApiKeyUpdateDTO};

/// Fluent request builder for [`ApiKeys::create_api_key`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateApiKeyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a ApiKeyCreateDTO,
}

/// Type alias for [`CreateApiKeyBuilder`].
pub type CreateApiKeyArgs<'a> = CreateApiKeyBuilder<'a>;
impl CreateApiKeyBuilder<'_> {
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
    pub async fn send(self) -> Result<ApiKeyCreateResponseDTO, crate::Error> {
        tracing::trace!(module = "ApiKeys", method = "POST", path = "/api-keys", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/api-keys")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<ApiKeyCreateResponseDTO>("api_keys", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateApiKeyBuilder<'a> {
    type Output = Result<ApiKeyCreateResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`ApiKeys::delete_api_key`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteApiKeyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::ApiKeyId,
}

/// Type alias for [`DeleteApiKeyBuilder`].
pub type DeleteApiKeyArgs<'a> = DeleteApiKeyBuilder<'a>;
impl DeleteApiKeyBuilder<'_> {
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
            module = "ApiKeys",
            method = "DELETE",
            path = "/api-keys/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/api-keys/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("api_keys", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteApiKeyBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`ApiKeys::get_api_key`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetApiKeyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::ApiKeyId,
}

/// Type alias for [`GetApiKeyBuilder`].
pub type GetApiKeyArgs<'a> = GetApiKeyBuilder<'a>;
impl GetApiKeyBuilder<'_> {
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
    pub async fn send(self) -> Result<ApiKeyResponseDTO, crate::Error> {
        tracing::trace!(
            module = "ApiKeys",
            method = "GET",
            path = "/api-keys/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/api-keys/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<ApiKeyResponseDTO>("api_keys", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetApiKeyBuilder<'a> {
    type Output = Result<ApiKeyResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`ApiKeys::get_api_keys`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetApiKeysBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetApiKeysBuilder`].
pub type GetApiKeysArgs<'a> = GetApiKeysBuilder<'a>;
impl GetApiKeysBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<ApiKeyResponseDTO>, crate::Error> {
        tracing::trace!(module = "ApiKeys", method = "GET", path = "/api-keys", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/api-keys")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<Vec<ApiKeyResponseDTO>>("api_keys", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetApiKeysBuilder<'a> {
    type Output = Result<Vec<ApiKeyResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`ApiKeys::get_my_api_key`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMyApiKeyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetMyApiKeyBuilder`].
pub type GetMyApiKeyArgs<'a> = GetMyApiKeyBuilder<'a>;
impl GetMyApiKeyBuilder<'_> {
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
    pub async fn send(self) -> Result<ApiKeyResponseDTO, crate::Error> {
        tracing::trace!(
            module = "ApiKeys",
            method = "GET",
            path = "/api-keys/me",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/api-keys/me")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<ApiKeyResponseDTO>("api_keys", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetMyApiKeyBuilder<'a> {
    type Output = Result<ApiKeyResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`ApiKeys::rotate_api_key`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RotateApiKeyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::ApiKeyId,
}

/// Type alias for [`RotateApiKeyBuilder`].
pub type RotateApiKeyArgs<'a> = RotateApiKeyBuilder<'a>;
impl RotateApiKeyBuilder<'_> {
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
    pub async fn send(self) -> Result<ApiKeyCreateResponseDTO, crate::Error> {
        tracing::trace!(
            module = "ApiKeys",
            method = "POST",
            path = "/api-keys/{id}/rotate",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, &format!("/api-keys/{}/rotate", self.id))?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<ApiKeyCreateResponseDTO>("api_keys", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for RotateApiKeyBuilder<'a> {
    type Output = Result<ApiKeyCreateResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`ApiKeys::update_api_key`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateApiKeyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::ApiKeyId,
    #[serde(skip)]
    pub body: &'a ApiKeyUpdateDTO,
}

/// Type alias for [`UpdateApiKeyBuilder`].
pub type UpdateApiKeyArgs<'a> = UpdateApiKeyBuilder<'a>;
impl UpdateApiKeyBuilder<'_> {
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
    pub async fn send(self) -> Result<ApiKeyResponseDTO, crate::Error> {
        tracing::trace!(
            module = "ApiKeys",
            method = "PUT",
            path = "/api-keys/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/api-keys/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<ApiKeyResponseDTO>("api_keys", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateApiKeyBuilder<'a> {
    type Output = Result<ApiKeyResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `ApiKeys` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct ApiKeys<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> ApiKeys<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create an API key
    /// Creates a new API key. It will be limited to the permissions specified.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `ApiKeyCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `ApiKeyCreateResponseDTO`
    pub fn create_api_key(&self, body: &'a ApiKeyCreateDTO) -> CreateApiKeyBuilder<'a> {
        CreateApiKeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete an API key
    /// Deletes an API key identified by its ID. The current user must own this API key.
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
    pub fn delete_api_key(&self, id: impl Into<immich_models::id::ApiKeyId>) -> DeleteApiKeyBuilder<'a> {
        DeleteApiKeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve an API key
    /// Retrieve an API key by its ID. The current user must own this API key.
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
    /// | `200` |  |
    ///
    /// **Returns:** `ApiKeyResponseDTO`
    pub fn get_api_key(&self, id: impl Into<immich_models::id::ApiKeyId>) -> GetApiKeyBuilder<'a> {
        GetApiKeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// List all API keys
    /// Retrieve all API keys of the current user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<ApiKeyResponseDTO>`
    pub fn get_api_keys(&self) -> GetApiKeysBuilder<'a> {
        GetApiKeysBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve the current API key
    /// Retrieve the API key that is used to access this endpoint.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `ApiKeyResponseDTO`
    pub fn get_my_api_key(&self) -> GetMyApiKeyBuilder<'a> {
        GetMyApiKeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Rotate an API key
    /// Generates a new secret for an API key, immediately invalidating the previous one. The current user must own this API key.
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
    /// | `201` |  |
    ///
    /// **Returns:** `ApiKeyCreateResponseDTO`
    pub fn rotate_api_key(&self, id: impl Into<immich_models::id::ApiKeyId>) -> RotateApiKeyBuilder<'a> {
        RotateApiKeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Update an API key
    /// Updates the name and permissions of an API key by its ID. The current user must own this API key.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `ApiKeyUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `ApiKeyResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_api_key(
        &self,
        id: impl Into<immich_models::id::ApiKeyId>,
        body: &'a ApiKeyUpdateDTO,
    ) -> UpdateApiKeyBuilder<'a> {
        UpdateApiKeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }
}
