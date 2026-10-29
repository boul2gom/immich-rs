// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `duplicates`.

use immich_models::albums::{BulkIdResponseDTO, BulkIdsDTO};
use immich_models::duplicates::{DuplicateResolveDTO, DuplicateResponseDTO};

/// Fluent request builder for [`Duplicates::delete_duplicate`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteDuplicateBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`DeleteDuplicateBuilder`].
pub type DeleteDuplicateArgs<'a> = DeleteDuplicateBuilder<'a>;
impl DeleteDuplicateBuilder<'_> {
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
            module = "Duplicates",
            method = "DELETE",
            path = "/duplicates/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/duplicates/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("duplicates", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteDuplicateBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Duplicates::delete_duplicates`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteDuplicatesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a BulkIdsDTO,
}

/// Type alias for [`DeleteDuplicatesBuilder`].
pub type DeleteDuplicatesArgs<'a> = DeleteDuplicatesBuilder<'a>;
impl DeleteDuplicatesBuilder<'_> {
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
            module = "Duplicates",
            method = "DELETE",
            path = "/duplicates",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/duplicates")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("duplicates", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteDuplicatesBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Duplicates::get_asset_duplicates`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetDuplicatesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetAssetDuplicatesBuilder`].
pub type GetAssetDuplicatesArgs<'a> = GetAssetDuplicatesBuilder<'a>;
impl GetAssetDuplicatesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<DuplicateResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Duplicates",
            method = "GET",
            path = "/duplicates",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/duplicates")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<Vec<DuplicateResponseDTO>>("duplicates", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetDuplicatesBuilder<'a> {
    type Output = Result<Vec<DuplicateResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Duplicates::resolve_duplicates`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ResolveDuplicatesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a DuplicateResolveDTO,
}

/// Type alias for [`ResolveDuplicatesBuilder`].
pub type ResolveDuplicatesArgs<'a> = ResolveDuplicatesBuilder<'a>;
impl ResolveDuplicatesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<BulkIdResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Duplicates",
            method = "POST",
            path = "/duplicates/resolve",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/duplicates/resolve")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<Vec<BulkIdResponseDTO>>("duplicates", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for ResolveDuplicatesBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Duplicates` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Duplicates<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Duplicates<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Dismiss a duplicate group
    /// Dismiss a duplicate group by its ID, unlinking all assets in the group without deleting them.
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
    pub fn delete_duplicate(&self, id: uuid::Uuid) -> DeleteDuplicateBuilder<'a> {
        DeleteDuplicateBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Delete duplicates
    /// Delete multiple duplicate assets specified by their IDs.
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
    pub fn delete_duplicates(&self, body: &'a BulkIdsDTO) -> DeleteDuplicatesBuilder<'a> {
        DeleteDuplicatesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Retrieve duplicates
    /// Retrieve a list of duplicate assets available to the authenticated user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<DuplicateResponseDTO>`
    pub fn get_asset_duplicates(&self) -> GetAssetDuplicatesBuilder<'a> {
        GetAssetDuplicatesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Resolve duplicate groups
    /// Resolve duplicate groups by synchronizing metadata across assets and deleting/trashing duplicates.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `DuplicateResolveDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<BulkIdResponseDTO>`
    pub fn resolve_duplicates(&self, body: &'a DuplicateResolveDTO) -> ResolveDuplicatesBuilder<'a> {
        ResolveDuplicatesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
