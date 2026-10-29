// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `metadata`.

use immich_models::server::VersionCheckStateResponseDTO;
use immich_models::system::metadata::{AdminOnboardingUpdateDTO, ReverseGeocodingStateResponseDTO};

/// Fluent request builder for [`Metadata::get_admin_onboarding`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAdminOnboardingBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetAdminOnboardingBuilder`].
pub type GetAdminOnboardingArgs<'a> = GetAdminOnboardingBuilder<'a>;
impl GetAdminOnboardingBuilder<'_> {
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
    pub async fn send(self) -> Result<AdminOnboardingUpdateDTO, crate::Error> {
        tracing::trace!(
            module = "Metadata",
            method = "GET",
            path = "/system-metadata/admin-onboarding",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/system-metadata/admin-onboarding")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<AdminOnboardingUpdateDTO>("metadata", request)
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetReverseGeocodingStateBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetReverseGeocodingStateBuilder`].
pub type GetReverseGeocodingStateArgs<'a> = GetReverseGeocodingStateBuilder<'a>;
impl GetReverseGeocodingStateBuilder<'_> {
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
    pub async fn send(self) -> Result<ReverseGeocodingStateResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Metadata",
            method = "GET",
            path = "/system-metadata/reverse-geocoding-state",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/system-metadata/reverse-geocoding-state")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<ReverseGeocodingStateResponseDTO>("metadata", request)
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetVersionCheckStateBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetVersionCheckStateBuilder`].
pub type GetVersionCheckStateArgs<'a> = GetVersionCheckStateBuilder<'a>;
impl GetVersionCheckStateBuilder<'_> {
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
    pub async fn send(self) -> Result<VersionCheckStateResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Metadata",
            method = "GET",
            path = "/system-metadata/version-check-state",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/system-metadata/version-check-state")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<VersionCheckStateResponseDTO>("metadata", request)
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAdminOnboardingBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AdminOnboardingUpdateDTO,
}

/// Type alias for [`UpdateAdminOnboardingBuilder`].
pub type UpdateAdminOnboardingArgs<'a> = UpdateAdminOnboardingBuilder<'a>;
impl UpdateAdminOnboardingBuilder<'_> {
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
            module = "Metadata",
            method = "POST",
            path = "/system-metadata/admin-onboarding",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/system-metadata/admin-onboarding")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("metadata", request).await?;
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

///Client handle for `Metadata` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Metadata<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Metadata<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Retrieve admin onboarding
    /// Retrieve the current admin onboarding status.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AdminOnboardingUpdateDTO`
    pub fn get_admin_onboarding(&self) -> GetAdminOnboardingBuilder<'a> {
        GetAdminOnboardingBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve reverse geocoding state
    /// Retrieve the current state of the reverse geocoding import.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `ReverseGeocodingStateResponseDTO`
    pub fn get_reverse_geocoding_state(&self) -> GetReverseGeocodingStateBuilder<'a> {
        GetReverseGeocodingStateBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve version check state
    /// Retrieve the current state of the version check process.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `VersionCheckStateResponseDTO`
    pub fn get_version_check_state(&self) -> GetVersionCheckStateBuilder<'a> {
        GetVersionCheckStateBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Update admin onboarding
    /// Update the admin onboarding status.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AdminOnboardingUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn update_admin_onboarding(&self, body: &'a AdminOnboardingUpdateDTO) -> UpdateAdminOnboardingBuilder<'a> {
        UpdateAdminOnboardingBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
