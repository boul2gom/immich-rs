// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `libraries`.

use immich_models::libraries::{
    CreateLibraryDTO, LibraryResponseDTO, LibraryStatsResponseDTO, UpdateLibraryDTO, ValidateLibraryDTO,
    ValidateLibraryResponseDTO,
};

/// Fluent request builder for [`Libraries::create_library`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateLibraryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a CreateLibraryDTO,
}

/// Type alias for [`CreateLibraryBuilder`].
pub type CreateLibraryArgs<'a> = CreateLibraryBuilder<'a>;
impl CreateLibraryBuilder<'_> {
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
    pub async fn send(self) -> Result<LibraryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Libraries",
            method = "POST",
            path = "/libraries",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/libraries")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<LibraryResponseDTO>("libraries", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateLibraryBuilder<'a> {
    type Output = Result<LibraryResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Libraries::delete_library`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteLibraryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::LibraryId,
}

/// Type alias for [`DeleteLibraryBuilder`].
pub type DeleteLibraryArgs<'a> = DeleteLibraryBuilder<'a>;
impl DeleteLibraryBuilder<'_> {
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
            module = "Libraries",
            method = "DELETE",
            path = "/libraries/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/libraries/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("libraries", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteLibraryBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Libraries::get_all_libraries`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAllLibrariesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetAllLibrariesBuilder`].
pub type GetAllLibrariesArgs<'a> = GetAllLibrariesBuilder<'a>;
impl GetAllLibrariesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<LibraryResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Libraries",
            method = "GET",
            path = "/libraries",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/libraries")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<Vec<LibraryResponseDTO>>("libraries", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAllLibrariesBuilder<'a> {
    type Output = Result<Vec<LibraryResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Libraries::get_library`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetLibraryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::LibraryId,
}

/// Type alias for [`GetLibraryBuilder`].
pub type GetLibraryArgs<'a> = GetLibraryBuilder<'a>;
impl GetLibraryBuilder<'_> {
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
    pub async fn send(self) -> Result<LibraryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Libraries",
            method = "GET",
            path = "/libraries/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/libraries/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<LibraryResponseDTO>("libraries", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetLibraryBuilder<'a> {
    type Output = Result<LibraryResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Libraries::get_library_statistics`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetLibraryStatisticsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::LibraryId,
}

/// Type alias for [`GetLibraryStatisticsBuilder`].
pub type GetLibraryStatisticsArgs<'a> = GetLibraryStatisticsBuilder<'a>;
impl GetLibraryStatisticsBuilder<'_> {
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
    pub async fn send(self) -> Result<LibraryStatsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Libraries",
            method = "GET",
            path = "/libraries/{id}/statistics",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/libraries/{}/statistics", self.id))?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<LibraryStatsResponseDTO>("libraries", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetLibraryStatisticsBuilder<'a> {
    type Output = Result<LibraryStatsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Libraries::scan_library`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ScanLibraryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::LibraryId,
}

/// Type alias for [`ScanLibraryBuilder`].
pub type ScanLibraryArgs<'a> = ScanLibraryBuilder<'a>;
impl ScanLibraryBuilder<'_> {
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
            module = "Libraries",
            method = "POST",
            path = "/libraries/{id}/scan",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, &format!("/libraries/{}/scan", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("libraries", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for ScanLibraryBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Libraries::update_library`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateLibraryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::LibraryId,
    #[serde(skip)]
    pub body: &'a UpdateLibraryDTO,
}

/// Type alias for [`UpdateLibraryBuilder`].
pub type UpdateLibraryArgs<'a> = UpdateLibraryBuilder<'a>;
impl UpdateLibraryBuilder<'_> {
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
    pub async fn send(self) -> Result<LibraryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Libraries",
            method = "PUT",
            path = "/libraries/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/libraries/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<LibraryResponseDTO>("libraries", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateLibraryBuilder<'a> {
    type Output = Result<LibraryResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Libraries::validate`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ValidateBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::LibraryId,
    #[serde(skip)]
    pub body: &'a ValidateLibraryDTO,
}

/// Type alias for [`ValidateBuilder`].
pub type ValidateArgs<'a> = ValidateBuilder<'a>;
impl ValidateBuilder<'_> {
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
    pub async fn send(self) -> Result<ValidateLibraryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Libraries",
            method = "POST",
            path = "/libraries/{id}/validate",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, &format!("/libraries/{}/validate", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<ValidateLibraryResponseDTO>("libraries", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for ValidateBuilder<'a> {
    type Output = Result<ValidateLibraryResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Libraries` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Libraries<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Libraries<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create a library
    /// Create a new external library.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `CreateLibraryDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `LibraryResponseDTO`
    pub fn create_library(&self, body: &'a CreateLibraryDTO) -> CreateLibraryBuilder<'a> {
        CreateLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete a library
    /// Delete an external library by its ID.
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
    pub fn delete_library(&self, id: impl Into<immich_models::id::LibraryId>) -> DeleteLibraryBuilder<'a> {
        DeleteLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve libraries
    /// Retrieve a list of external libraries.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<LibraryResponseDTO>`
    pub fn get_all_libraries(&self) -> GetAllLibrariesBuilder<'a> {
        GetAllLibrariesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve a library
    /// Retrieve an external library by its ID.
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
    /// **Returns:** `LibraryResponseDTO`
    pub fn get_library(&self, id: impl Into<immich_models::id::LibraryId>) -> GetLibraryBuilder<'a> {
        GetLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve library statistics
    /// Retrieve statistics for a specific external library, including number of videos, images, and storage usage.
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
    /// **Returns:** `LibraryStatsResponseDTO`
    pub fn get_library_statistics(
        &self,
        id: impl Into<immich_models::id::LibraryId>,
    ) -> GetLibraryStatisticsBuilder<'a> {
        GetLibraryStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Scan a library
    /// Queue a scan for the external library to find and import new assets.
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
    pub fn scan_library(&self, id: impl Into<immich_models::id::LibraryId>) -> ScanLibraryBuilder<'a> {
        ScanLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Update a library
    /// Update an existing external library.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `UpdateLibraryDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `LibraryResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_library(
        &self,
        id: impl Into<immich_models::id::LibraryId>,
        body: &'a UpdateLibraryDTO,
    ) -> UpdateLibraryBuilder<'a> {
        UpdateLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Validate library settings
    /// Validate the settings of an external library.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `ValidateLibraryDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `ValidateLibraryResponseDTO`
    pub fn validate(
        &self,
        id: impl Into<immich_models::id::LibraryId>,
        body: &'a ValidateLibraryDTO,
    ) -> ValidateBuilder<'a> {
        ValidateBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }
}
