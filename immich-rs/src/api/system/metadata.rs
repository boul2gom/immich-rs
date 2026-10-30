// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `metadata`.

use immich_models::server::VersionCheckStateResponseDTO;
use immich_models::system::metadata::{AdminOnboardingUpdateDTO, ReverseGeocodingStateResponseDTO};

/// Fluent request builder for [`Metadata::get_admin_onboarding`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAdminOnboardingBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetAdminOnboardingBuilder<'_> {
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
    pub async fn send(self) -> Result<AdminOnboardingUpdateDTO, crate::Error> {
        tracing::trace!(
            module = "system/metadata",
            method = "GET",
            path = "/system-metadata/admin-onboarding",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/system-metadata/admin-onboarding")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<AdminOnboardingUpdateDTO>("system/metadata", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAdminOnboardingBuilder<'a> {
    type Output = Result<AdminOnboardingUpdateDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Metadata::get_reverse_geocoding_state`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetReverseGeocodingStateBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetReverseGeocodingStateBuilder<'_> {
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
    pub async fn send(self) -> Result<ReverseGeocodingStateResponseDTO, crate::Error> {
        tracing::trace!(
            module = "system/metadata",
            method = "GET",
            path = "/system-metadata/reverse-geocoding-state",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/system-metadata/reverse-geocoding-state")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<ReverseGeocodingStateResponseDTO>("system/metadata", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetReverseGeocodingStateBuilder<'a> {
    type Output = Result<ReverseGeocodingStateResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Metadata::get_version_check_state`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetVersionCheckStateBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetVersionCheckStateBuilder<'_> {
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
    pub async fn send(self) -> Result<VersionCheckStateResponseDTO, crate::Error> {
        tracing::trace!(
            module = "system/metadata",
            method = "GET",
            path = "/system-metadata/version-check-state",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/system-metadata/version-check-state")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<VersionCheckStateResponseDTO>("system/metadata", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetVersionCheckStateBuilder<'a> {
    type Output = Result<VersionCheckStateResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Metadata::update_admin_onboarding`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAdminOnboardingBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AdminOnboardingUpdateDTO,
}

impl UpdateAdminOnboardingBuilder<'_> {
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
            module = "system/metadata",
            method = "POST",
            path = "/system-metadata/admin-onboarding",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/system-metadata/admin-onboarding")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("system/metadata", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for UpdateAdminOnboardingBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Metadata` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Metadata<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Metadata<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Retrieve admin onboarding
    ///
    /// Retrieve the current admin onboarding status.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AdminOnboardingUpdateDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_admin_onboarding(&self) -> GetAdminOnboardingBuilder<'a> {
        GetAdminOnboardingBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Retrieve reverse geocoding state
    ///
    /// Retrieve the current state of the reverse geocoding import.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `ReverseGeocodingStateResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_reverse_geocoding_state(&self) -> GetReverseGeocodingStateBuilder<'a> {
        GetReverseGeocodingStateBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Retrieve version check state
    ///
    /// Retrieve the current state of the version check process.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `VersionCheckStateResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_version_check_state(&self) -> GetVersionCheckStateBuilder<'a> {
        GetVersionCheckStateBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Update admin onboarding
    ///
    /// Update the admin onboarding status.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AdminOnboardingUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_admin_onboarding(
        &self,
        body: impl Into<AdminOnboardingUpdateDTO>,
    ) -> UpdateAdminOnboardingBuilder<'a> {
        UpdateAdminOnboardingBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
}
