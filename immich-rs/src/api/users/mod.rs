// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `users`.

pub mod admin;

use immich_models::activities::UserResponseDTO;
use immich_models::authentication::UserAdminResponseDTO;
use immich_models::server::{LicenseKeyDTO, LicenseResponseDTO};
use immich_models::users::{
    CalendarHeatmapResponseDTO, CalendarHeatmapType, CreateProfileImageResponseDTO, OnboardingDTO,
    OnboardingResponseDTO, UserPreferencesResponseDTO, UserPreferencesUpdateDTO, UserUpdateMeDTO,
};

/// Fluent request builder for [`Users::create_profile_image`].
#[derive(Debug)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateProfileImageBuilder<'a> {
    client: &'a crate::ImmichClient,
    pub request_config: crate::core::RequestConfig,
    pub form: reqwest::multipart::Form,
}

/// Type alias for [`CreateProfileImageBuilder`].
pub type CreateProfileImageArgs<'a> = CreateProfileImageBuilder<'a>;
impl CreateProfileImageBuilder<'_> {
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
    pub async fn send(self) -> Result<CreateProfileImageResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Users",
            method = "POST",
            path = "/users/profile-image",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/users/profile-image")?;
        request = self.request_config.apply(request);
        request = request.multipart(self.form);

        let response = self
            .client
            .execute::<CreateProfileImageResponseDTO>("users", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateProfileImageBuilder<'a> {
    type Output = Result<CreateProfileImageResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::delete_profile_image`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteProfileImageBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`DeleteProfileImageBuilder`].
pub type DeleteProfileImageArgs<'a> = DeleteProfileImageBuilder<'a>;
impl DeleteProfileImageBuilder<'_> {
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
            module = "Users",
            method = "DELETE",
            path = "/users/profile-image",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, "/users/profile-image")?;
        request = self.request_config.apply(request);
        self.client.execute_unit("users", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteProfileImageBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::delete_user_license`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteUserLicenseBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`DeleteUserLicenseBuilder`].
pub type DeleteUserLicenseArgs<'a> = DeleteUserLicenseBuilder<'a>;
impl DeleteUserLicenseBuilder<'_> {
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
            module = "Users",
            method = "DELETE",
            path = "/users/me/license",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, "/users/me/license")?;
        request = self.request_config.apply(request);
        self.client.execute_unit("users", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteUserLicenseBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::delete_user_onboarding`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteUserOnboardingBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`DeleteUserOnboardingBuilder`].
pub type DeleteUserOnboardingArgs<'a> = DeleteUserOnboardingBuilder<'a>;
impl DeleteUserOnboardingBuilder<'_> {
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
            module = "Users",
            method = "DELETE",
            path = "/users/me/onboarding",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, "/users/me/onboarding")?;
        request = self.request_config.apply(request);
        self.client.execute_unit("users", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteUserOnboardingBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::get_my_calendar_heatmap`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMyCalendarHeatmapBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<CalendarHeatmapType>,
}

/// Type alias for [`GetMyCalendarHeatmapBuilder`].
pub type GetMyCalendarHeatmapArgs<'a> = GetMyCalendarHeatmapBuilder<'a>;
impl GetMyCalendarHeatmapBuilder<'_> {
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

    /// Start date in UTC
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    /// Sets or clears `from`.
    pub fn set_from(mut self, value: Option<String>) -> Self {
        self.from = value;
        self
    }

    /// End date in UTC
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    /// Sets or clears `to`.
    pub fn set_to(mut self, value: Option<String>) -> Self {
        self.to = value;
        self
    }

    /// Sets `type`.
    pub fn type_(mut self, value: CalendarHeatmapType) -> Self {
        self.type_ = Some(value);
        self
    }

    /// Sets or clears `type`.
    pub fn set_type_(mut self, value: Option<CalendarHeatmapType>) -> Self {
        self.type_ = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<CalendarHeatmapResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Users",
            method = "GET",
            path = "/users/me/calendar-heatmap",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/users/me/calendar-heatmap")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<CalendarHeatmapResponseDTO>("users", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetMyCalendarHeatmapBuilder<'a> {
    type Output = Result<CalendarHeatmapResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::get_my_preferences`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMyPreferencesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetMyPreferencesBuilder`].
pub type GetMyPreferencesArgs<'a> = GetMyPreferencesBuilder<'a>;
impl GetMyPreferencesBuilder<'_> {
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
    pub async fn send(self) -> Result<UserPreferencesResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Users",
            method = "GET",
            path = "/users/me/preferences",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/users/me/preferences")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<UserPreferencesResponseDTO>("users", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetMyPreferencesBuilder<'a> {
    type Output = Result<UserPreferencesResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::get_my_user`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMyUserBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetMyUserBuilder`].
pub type GetMyUserArgs<'a> = GetMyUserBuilder<'a>;
impl GetMyUserBuilder<'_> {
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
    pub async fn send(self) -> Result<UserAdminResponseDTO, crate::Error> {
        tracing::trace!(module = "Users", method = "GET", path = "/users/me", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/users/me")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<UserAdminResponseDTO>("users", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetMyUserBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::get_profile_image`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetProfileImageBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::UserId,
}

/// Type alias for [`GetProfileImageBuilder`].
pub type GetProfileImageArgs<'a> = GetProfileImageBuilder<'a>;
impl GetProfileImageBuilder<'_> {
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
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "Users",
            method = "GET",
            path = "/users/{id}/profile-image",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/users/{}/profile-image", self.id))?;
        request = self.request_config.apply(request);
        let raw_response = self.client.execute_stream("users", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for GetProfileImageBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::get_user`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::UserId,
}

/// Type alias for [`GetUserBuilder`].
pub type GetUserArgs<'a> = GetUserBuilder<'a>;
impl GetUserBuilder<'_> {
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
    pub async fn send(self) -> Result<UserResponseDTO, crate::Error> {
        tracing::trace!(module = "Users", method = "GET", path = "/users/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/users/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<UserResponseDTO>("users", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserBuilder<'a> {
    type Output = Result<UserResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::get_user_license`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserLicenseBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetUserLicenseBuilder`].
pub type GetUserLicenseArgs<'a> = GetUserLicenseBuilder<'a>;
impl GetUserLicenseBuilder<'_> {
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
    pub async fn send(self) -> Result<LicenseResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Users",
            method = "GET",
            path = "/users/me/license",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/users/me/license")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<LicenseResponseDTO>("users", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserLicenseBuilder<'a> {
    type Output = Result<LicenseResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::get_user_onboarding`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserOnboardingBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetUserOnboardingBuilder`].
pub type GetUserOnboardingArgs<'a> = GetUserOnboardingBuilder<'a>;
impl GetUserOnboardingBuilder<'_> {
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
    pub async fn send(self) -> Result<OnboardingResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Users",
            method = "GET",
            path = "/users/me/onboarding",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/users/me/onboarding")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<OnboardingResponseDTO>("users", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserOnboardingBuilder<'a> {
    type Output = Result<OnboardingResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::search_users`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchUsersBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`SearchUsersBuilder`].
pub type SearchUsersArgs<'a> = SearchUsersBuilder<'a>;
impl SearchUsersBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<UserResponseDTO>, crate::Error> {
        tracing::trace!(module = "Users", method = "GET", path = "/users", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/users")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<Vec<UserResponseDTO>>("users", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchUsersBuilder<'a> {
    type Output = Result<Vec<UserResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::set_user_license`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SetUserLicenseBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a LicenseKeyDTO,
}

/// Type alias for [`SetUserLicenseBuilder`].
pub type SetUserLicenseArgs<'a> = SetUserLicenseBuilder<'a>;
impl SetUserLicenseBuilder<'_> {
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
    pub async fn send(self) -> Result<LicenseResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Users",
            method = "PUT",
            path = "/users/me/license",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/users/me/license")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<LicenseResponseDTO>("users", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SetUserLicenseBuilder<'a> {
    type Output = Result<LicenseResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::set_user_onboarding`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SetUserOnboardingBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a OnboardingDTO,
}

/// Type alias for [`SetUserOnboardingBuilder`].
pub type SetUserOnboardingArgs<'a> = SetUserOnboardingBuilder<'a>;
impl SetUserOnboardingBuilder<'_> {
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
    pub async fn send(self) -> Result<OnboardingResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Users",
            method = "PUT",
            path = "/users/me/onboarding",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, "/users/me/onboarding")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<OnboardingResponseDTO>("users", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SetUserOnboardingBuilder<'a> {
    type Output = Result<OnboardingResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::update_my_preferences`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateMyPreferencesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a UserPreferencesUpdateDTO,
}

/// Type alias for [`UpdateMyPreferencesBuilder`].
pub type UpdateMyPreferencesArgs<'a> = UpdateMyPreferencesBuilder<'a>;
impl UpdateMyPreferencesBuilder<'_> {
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
    pub async fn send(self) -> Result<UserPreferencesResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Users",
            method = "PUT",
            path = "/users/me/preferences",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, "/users/me/preferences")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<UserPreferencesResponseDTO>("users", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateMyPreferencesBuilder<'a> {
    type Output = Result<UserPreferencesResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Users::update_my_user`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateMyUserBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a UserUpdateMeDTO,
}

/// Type alias for [`UpdateMyUserBuilder`].
pub type UpdateMyUserArgs<'a> = UpdateMyUserBuilder<'a>;
impl UpdateMyUserBuilder<'_> {
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
    pub async fn send(self) -> Result<UserAdminResponseDTO, crate::Error> {
        tracing::trace!(module = "Users", method = "PUT", path = "/users/me", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/users/me")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<UserAdminResponseDTO>("users", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateMyUserBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Users` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Users<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Users<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create user profile image
    /// Upload and set a new profile image for the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `CreateProfileImageDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `CreateProfileImageResponseDTO`
    pub fn create_profile_image(&self, form: reqwest::multipart::Form) -> CreateProfileImageBuilder<'a> {
        CreateProfileImageBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            form,
        }
    }

    /// Delete user profile image
    /// Delete the profile image of the current user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_profile_image(&self) -> DeleteProfileImageBuilder<'a> {
        DeleteProfileImageBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Delete user product key
    /// Delete the registered product key for the current user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_user_license(&self) -> DeleteUserLicenseBuilder<'a> {
        DeleteUserLicenseBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Delete user onboarding
    /// Delete the onboarding status of the current user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_user_onboarding(&self) -> DeleteUserOnboardingBuilder<'a> {
        DeleteUserOnboardingBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve calendar heatmap activity
    /// Retrieve activity counts for a specified period, in a calendar heatmap format.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `from` | `String` |  | Start date in UTC |
    /// | `to` | `String` |  | End date in UTC |
    /// | `type` | `CalendarHeatmapType` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `CalendarHeatmapResponseDTO`
    pub fn get_my_calendar_heatmap(&self) -> GetMyCalendarHeatmapBuilder<'a> {
        GetMyCalendarHeatmapBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            from: None,
            to: None,
            type_: None,
        }
    }

    /// Get my preferences
    /// Retrieve the preferences for the current user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserPreferencesResponseDTO`
    pub fn get_my_preferences(&self) -> GetMyPreferencesBuilder<'a> {
        GetMyPreferencesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get current user
    /// Retrieve information about the user making the API request.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    pub fn get_my_user(&self) -> GetMyUserBuilder<'a> {
        GetMyUserBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve user profile image
    /// Retrieve the profile image file for a user.
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
    /// **Returns:** `reqwest::Response`
    pub fn get_profile_image(&self, id: impl Into<immich_models::id::UserId>) -> GetProfileImageBuilder<'a> {
        GetProfileImageBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve a user
    /// Retrieve a specific user by their ID.
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
    /// **Returns:** `UserResponseDTO`
    pub fn get_user(&self, id: impl Into<immich_models::id::UserId>) -> GetUserBuilder<'a> {
        GetUserBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve user product key
    /// Retrieve information about whether the current user has a registered product key.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `LicenseResponseDTO`
    pub fn get_user_license(&self) -> GetUserLicenseBuilder<'a> {
        GetUserLicenseBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve user onboarding
    /// Retrieve the onboarding status of the current user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `OnboardingResponseDTO`
    pub fn get_user_onboarding(&self) -> GetUserOnboardingBuilder<'a> {
        GetUserOnboardingBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get all users
    /// Retrieve a list of all users on the server.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<UserResponseDTO>`
    pub fn search_users(&self) -> SearchUsersBuilder<'a> {
        SearchUsersBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Set user product key
    /// Register a product key for the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `LicenseKeyDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `LicenseResponseDTO`
    pub fn set_user_license(&self, body: &'a LicenseKeyDTO) -> SetUserLicenseBuilder<'a> {
        SetUserLicenseBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Update user onboarding
    /// Update the onboarding status of the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `OnboardingDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `OnboardingResponseDTO`
    pub fn set_user_onboarding(&self, body: &'a OnboardingDTO) -> SetUserOnboardingBuilder<'a> {
        SetUserOnboardingBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Update my preferences
    /// Update the preferences of the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `UserPreferencesUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserPreferencesResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_my_preferences(&self, body: &'a UserPreferencesUpdateDTO) -> UpdateMyPreferencesBuilder<'a> {
        UpdateMyPreferencesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Update current user
    /// Update the current user making the API request.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `UserUpdateMeDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_my_user(&self, body: &'a UserUpdateMeDTO) -> UpdateMyUserBuilder<'a> {
        UpdateMyUserBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
