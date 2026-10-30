// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `libraries`.

use immich_models::id::LibraryId;
use immich_models::libraries::{
    CreateLibraryDTO, LibraryResponseDTO, LibraryStatsResponseDTO, UpdateLibraryDTO, ValidateLibraryDTO,
    ValidateLibraryResponseDTO,
};

/// Fluent request builder for [`Libraries::create_library`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateLibraryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: CreateLibraryDTO,
}

impl CreateLibraryBuilder<'_> {
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
    pub async fn send(self) -> Result<LibraryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "libraries",
            method = "POST",
            path = "/libraries",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/libraries")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteLibraryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: LibraryId,
}

impl DeleteLibraryBuilder<'_> {
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
            module = "libraries",
            method = "DELETE",
            path = "/libraries/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("libraries"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAllLibrariesBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetAllLibrariesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<LibraryResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "libraries",
            method = "GET",
            path = "/libraries",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/libraries")?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetLibraryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: LibraryId,
}

impl GetLibraryBuilder<'_> {
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
    pub async fn send(self) -> Result<LibraryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "libraries",
            method = "GET",
            path = "/libraries/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("libraries"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetLibraryStatisticsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: LibraryId,
}

impl GetLibraryStatisticsBuilder<'_> {
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
    pub async fn send(self) -> Result<LibraryStatsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "libraries",
            method = "GET",
            path = "/libraries/{id}/statistics",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("libraries"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("statistics"),
            ],
        )?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ScanLibraryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: LibraryId,
}

impl ScanLibraryBuilder<'_> {
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
            module = "libraries",
            method = "POST",
            path = "/libraries/{id}/scan",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::POST,
            &[
                crate::core::PathSegment::Static("libraries"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("scan"),
            ],
        )?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateLibraryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: LibraryId,
    body: UpdateLibraryDTO,
}

impl UpdateLibraryBuilder<'_> {
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
    pub async fn send(self) -> Result<LibraryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "libraries",
            method = "PUT",
            path = "/libraries/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("libraries"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ValidateBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: LibraryId,
    body: ValidateLibraryDTO,
}

impl ValidateBuilder<'_> {
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
    pub async fn send(self) -> Result<ValidateLibraryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "libraries",
            method = "POST",
            path = "/libraries/{id}/validate",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::POST,
            &[
                crate::core::PathSegment::Static("libraries"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("validate"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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

/// Client handle for `Libraries` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Libraries<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Libraries<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Create a library
    ///
    /// Create a new external library.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `CreateLibraryDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `LibraryResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn create_library(&self, body: impl Into<CreateLibraryDTO>) -> CreateLibraryBuilder<'a> {
        CreateLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete a library
    ///
    /// Delete an external library by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `LibraryId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_library(&self, id: impl Into<LibraryId>) -> DeleteLibraryBuilder<'a> {
        DeleteLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve libraries
    ///
    /// Retrieve a list of external libraries.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<LibraryResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_all_libraries(&self) -> GetAllLibrariesBuilder<'a> {
        GetAllLibrariesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Retrieve a library
    ///
    /// Retrieve an external library by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `LibraryId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `LibraryResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_library(&self, id: impl Into<LibraryId>) -> GetLibraryBuilder<'a> {
        GetLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve library statistics
    ///
    /// Retrieve statistics for a specific external library, including number of videos, images, and storage usage.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `LibraryId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `LibraryStatsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_library_statistics(&self, id: impl Into<LibraryId>) -> GetLibraryStatisticsBuilder<'a> {
        GetLibraryStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Scan a library
    ///
    /// Queue a scan for the external library to find and import new assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `LibraryId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn scan_library(&self, id: impl Into<LibraryId>) -> ScanLibraryBuilder<'a> {
        ScanLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Update a library
    ///
    /// Update an existing external library.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `LibraryId` | ✓ |  |
    /// | `body` | `UpdateLibraryDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `LibraryResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_library(
        &self,
        id: impl Into<LibraryId>,
        body: impl Into<UpdateLibraryDTO>,
    ) -> UpdateLibraryBuilder<'a> {
        UpdateLibraryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Validate library settings
    ///
    /// Validate the settings of an external library.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `LibraryId` | ✓ |  |
    /// | `body` | `ValidateLibraryDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `ValidateLibraryResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn validate(&self, id: impl Into<LibraryId>, body: impl Into<ValidateLibraryDTO>) -> ValidateBuilder<'a> {
        ValidateBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
}
