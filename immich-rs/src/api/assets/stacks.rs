// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `stacks`.

use immich_models::albums::BulkIdsDTO;
use immich_models::assets::stacks::{StackCreateDTO, StackResponseDTO, StackUpdateDTO};

/// Fluent request builder for [`Stacks::create_stack`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateStackBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a StackCreateDTO,
}

/// Type alias for [`CreateStackBuilder`].
pub type CreateStackArgs<'a> = CreateStackBuilder<'a>;
impl CreateStackBuilder<'_> {
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
    pub async fn send(self) -> Result<StackResponseDTO, crate::Error> {
        tracing::trace!(module = "Stacks", method = "POST", path = "/stacks", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/stacks")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<StackResponseDTO>("stacks", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateStackBuilder<'a> {
    type Output = Result<StackResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Stacks::delete_stack`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteStackBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::StackId,
}

/// Type alias for [`DeleteStackBuilder`].
pub type DeleteStackArgs<'a> = DeleteStackBuilder<'a>;
impl DeleteStackBuilder<'_> {
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
            module = "Stacks",
            method = "DELETE",
            path = "/stacks/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/stacks/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("stacks", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteStackBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Stacks::delete_stacks`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteStacksBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a BulkIdsDTO,
}

/// Type alias for [`DeleteStacksBuilder`].
pub type DeleteStacksArgs<'a> = DeleteStacksBuilder<'a>;
impl DeleteStacksBuilder<'_> {
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
        tracing::trace!(module = "Stacks", method = "DELETE", path = "/stacks", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/stacks")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("stacks", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteStacksBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Stacks::get_stack`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetStackBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::StackId,
}

/// Type alias for [`GetStackBuilder`].
pub type GetStackArgs<'a> = GetStackBuilder<'a>;
impl GetStackBuilder<'_> {
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
    pub async fn send(self) -> Result<StackResponseDTO, crate::Error> {
        tracing::trace!(module = "Stacks", method = "GET", path = "/stacks/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/stacks/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<StackResponseDTO>("stacks", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetStackBuilder<'a> {
    type Output = Result<StackResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Stacks::remove_asset_from_stack`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveAssetFromStackBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub asset_id: immich_models::id::AssetId,
    #[serde(skip)]
    pub id: immich_models::id::StackId,
}

/// Type alias for [`RemoveAssetFromStackBuilder`].
pub type RemoveAssetFromStackArgs<'a> = RemoveAssetFromStackBuilder<'a>;
impl RemoveAssetFromStackBuilder<'_> {
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
            module = "Stacks",
            method = "DELETE",
            path = "/stacks/{id}/assets/{assetId}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::DELETE,
            &format!("/stacks/{}/assets/{}", self.id, self.asset_id),
        )?;
        request = self.request_config.apply(request);
        self.client.execute_unit("stacks", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for RemoveAssetFromStackBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Stacks::search_stacks`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchStacksBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "primaryAssetId", skip_serializing_if = "Option::is_none")]
    pub primary_asset_id: Option<immich_models::id::AssetId>,
}

/// Type alias for [`SearchStacksBuilder`].
pub type SearchStacksArgs<'a> = SearchStacksBuilder<'a>;
impl SearchStacksBuilder<'_> {
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

    /// Filter by primary asset ID
    pub fn primary_asset_id(mut self, value: impl Into<immich_models::id::AssetId>) -> Self {
        self.primary_asset_id = Some(value.into());
        self
    }

    /// Sets or clears `primaryAssetId`.
    pub fn set_primary_asset_id(mut self, value: Option<immich_models::id::AssetId>) -> Self {
        self.primary_asset_id = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<StackResponseDTO>, crate::Error> {
        tracing::trace!(module = "Stacks", method = "GET", path = "/stacks", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/stacks")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self.client.execute::<Vec<StackResponseDTO>>("stacks", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchStacksBuilder<'a> {
    type Output = Result<Vec<StackResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Stacks::update_stack`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateStackBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::StackId,
    #[serde(skip)]
    pub body: &'a StackUpdateDTO,
}

/// Type alias for [`UpdateStackBuilder`].
pub type UpdateStackArgs<'a> = UpdateStackBuilder<'a>;
impl UpdateStackBuilder<'_> {
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
    pub async fn send(self) -> Result<StackResponseDTO, crate::Error> {
        tracing::trace!(module = "Stacks", method = "PUT", path = "/stacks/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/stacks/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<StackResponseDTO>("stacks", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateStackBuilder<'a> {
    type Output = Result<StackResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Stacks` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Stacks<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Stacks<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create a stack
    /// Create a new stack by providing a name and a list of asset IDs to include in the stack. If any of the provided asset IDs are primary assets of an existing stack, the existing stack will be merged into the newly created stack.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `StackCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `StackResponseDTO`
    pub fn create_stack(&self, body: &'a StackCreateDTO) -> CreateStackBuilder<'a> {
        CreateStackBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete a stack
    /// Delete a specific stack by its ID.
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
    pub fn delete_stack(&self, id: impl Into<immich_models::id::StackId>) -> DeleteStackBuilder<'a> {
        DeleteStackBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Delete stacks
    /// Delete multiple stacks by providing a list of stack IDs.
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
    /// | `204` |  |
    pub fn delete_stacks(&self, body: &'a BulkIdsDTO) -> DeleteStacksBuilder<'a> {
        DeleteStacksBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Retrieve a stack
    /// Retrieve a specific stack by its ID.
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
    /// **Returns:** `StackResponseDTO`
    pub fn get_stack(&self, id: impl Into<immich_models::id::StackId>) -> GetStackBuilder<'a> {
        GetStackBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Remove an asset from a stack
    /// Remove a specific asset from a stack by providing the stack ID and asset ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `assetId` | `uuid::Uuid` | ✓ |  |
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn remove_asset_from_stack(
        &self,
        asset_id: impl Into<immich_models::id::AssetId>,
        id: impl Into<immich_models::id::StackId>,
    ) -> RemoveAssetFromStackBuilder<'a> {
        RemoveAssetFromStackBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            asset_id: asset_id.into(),
            id: id.into(),
        }
    }

    /// Retrieve stacks
    /// Retrieve a list of stacks.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `primaryAssetId` | `uuid::Uuid` |  | Filter by primary asset ID |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<StackResponseDTO>`
    pub fn search_stacks(&self) -> SearchStacksBuilder<'a> {
        SearchStacksBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            primary_asset_id: None,
        }
    }

    /// Update a stack
    /// Update an existing stack by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `StackUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `StackResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_stack(
        &self,
        id: impl Into<immich_models::id::StackId>,
        body: &'a StackUpdateDTO,
    ) -> UpdateStackBuilder<'a> {
        UpdateStackBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }
}
