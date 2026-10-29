// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `authentication`.

use immich_models::authentication::{
    AuthStatusResponseDTO, ChangePasswordDTO, LoginCredentialDTO, LoginResponseDTO, LogoutResponseDTO,
    OAuthAuthorizeResponseDTO, OAuthCallbackDTO, OAuthConfigDTO, PinCodeChangeDTO, PinCodeResetDTO, PinCodeSetupDTO,
    SessionUnlockDTO, SignUpDTO, UserAdminResponseDTO, ValidateAccessTokenResponseDTO,
};

/// Fluent request builder for [`Authentication::change_password`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ChangePasswordBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a ChangePasswordDTO,
}

/// Type alias for [`ChangePasswordBuilder`].
pub type ChangePasswordArgs<'a> = ChangePasswordBuilder<'a>;
impl ChangePasswordBuilder<'_> {
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
        tracing::trace!(
            module = "Authentication",
            method = "POST",
            path = "/auth/change-password",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/auth/change-password")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<UserAdminResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for ChangePasswordBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::change_pin_code`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ChangePinCodeBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a PinCodeChangeDTO,
}

/// Type alias for [`ChangePinCodeBuilder`].
pub type ChangePinCodeArgs<'a> = ChangePinCodeBuilder<'a>;
impl ChangePinCodeBuilder<'_> {
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
            module = "Authentication",
            method = "PUT",
            path = "/auth/pin-code",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/auth/pin-code")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("authentication", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for ChangePinCodeBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::finish_o_auth`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct FinishOAuthBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a OAuthCallbackDTO,
}

/// Type alias for [`FinishOAuthBuilder`].
pub type FinishOAuthArgs<'a> = FinishOAuthBuilder<'a>;
impl FinishOAuthBuilder<'_> {
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
    pub async fn send(self) -> Result<LoginResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Authentication",
            method = "POST",
            path = "/oauth/callback",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/oauth/callback")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<LoginResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for FinishOAuthBuilder<'a> {
    type Output = Result<LoginResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::get_auth_status`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAuthStatusBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetAuthStatusBuilder`].
pub type GetAuthStatusArgs<'a> = GetAuthStatusBuilder<'a>;
impl GetAuthStatusBuilder<'_> {
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
    pub async fn send(self) -> Result<AuthStatusResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Authentication",
            method = "GET",
            path = "/auth/status",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/auth/status")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<AuthStatusResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAuthStatusBuilder<'a> {
    type Output = Result<AuthStatusResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::link_o_auth_account`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct LinkOAuthAccountBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a OAuthCallbackDTO,
}

/// Type alias for [`LinkOAuthAccountBuilder`].
pub type LinkOAuthAccountArgs<'a> = LinkOAuthAccountBuilder<'a>;
impl LinkOAuthAccountBuilder<'_> {
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
        tracing::trace!(
            module = "Authentication",
            method = "POST",
            path = "/oauth/link",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/oauth/link")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<UserAdminResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for LinkOAuthAccountBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::lock_auth_session`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct LockAuthSessionBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`LockAuthSessionBuilder`].
pub type LockAuthSessionArgs<'a> = LockAuthSessionBuilder<'a>;
impl LockAuthSessionBuilder<'_> {
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
            module = "Authentication",
            method = "POST",
            path = "/auth/session/lock",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/auth/session/lock")?;
        request = self.request_config.apply(request);
        self.client.execute_unit("authentication", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for LockAuthSessionBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::login`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct LoginBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a LoginCredentialDTO,
}

/// Type alias for [`LoginBuilder`].
pub type LoginArgs<'a> = LoginBuilder<'a>;
impl LoginBuilder<'_> {
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
    pub async fn send(self) -> Result<LoginResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Authentication",
            method = "POST",
            path = "/auth/login",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/auth/login")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<LoginResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for LoginBuilder<'a> {
    type Output = Result<LoginResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::logout`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct LogoutBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`LogoutBuilder`].
pub type LogoutArgs<'a> = LogoutBuilder<'a>;
impl LogoutBuilder<'_> {
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
    pub async fn send(self) -> Result<LogoutResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Authentication",
            method = "POST",
            path = "/auth/logout",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/auth/logout")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<LogoutResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for LogoutBuilder<'a> {
    type Output = Result<LogoutResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::logout_o_auth`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct LogoutOAuthBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`LogoutOAuthBuilder`].
pub type LogoutOAuthArgs<'a> = LogoutOAuthBuilder<'a>;
impl LogoutOAuthBuilder<'_> {
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
            module = "Authentication",
            method = "POST",
            path = "/oauth/backchannel-logout",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/oauth/backchannel-logout")?;
        request = self.request_config.apply(request);
        self.client.execute_unit("authentication", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for LogoutOAuthBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::redirect_o_auth_to_mobile`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RedirectOAuthToMobileBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`RedirectOAuthToMobileBuilder`].
pub type RedirectOAuthToMobileArgs<'a> = RedirectOAuthToMobileBuilder<'a>;
impl RedirectOAuthToMobileBuilder<'_> {
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
            module = "Authentication",
            method = "GET",
            path = "/oauth/mobile-redirect",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/oauth/mobile-redirect")?;
        request = self.request_config.apply(request);
        self.client.execute_unit("authentication", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for RedirectOAuthToMobileBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::reset_pin_code`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ResetPinCodeBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a PinCodeResetDTO,
}

/// Type alias for [`ResetPinCodeBuilder`].
pub type ResetPinCodeArgs<'a> = ResetPinCodeBuilder<'a>;
impl ResetPinCodeBuilder<'_> {
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
            module = "Authentication",
            method = "DELETE",
            path = "/auth/pin-code",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/auth/pin-code")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("authentication", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for ResetPinCodeBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::setup_pin_code`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SetupPinCodeBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a PinCodeSetupDTO,
}

/// Type alias for [`SetupPinCodeBuilder`].
pub type SetupPinCodeArgs<'a> = SetupPinCodeBuilder<'a>;
impl SetupPinCodeBuilder<'_> {
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
            module = "Authentication",
            method = "POST",
            path = "/auth/pin-code",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/auth/pin-code")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("authentication", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for SetupPinCodeBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::sign_up_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SignUpAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a SignUpDTO,
}

/// Type alias for [`SignUpAdminBuilder`].
pub type SignUpAdminArgs<'a> = SignUpAdminBuilder<'a>;
impl SignUpAdminBuilder<'_> {
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
        tracing::trace!(
            module = "Authentication",
            method = "POST",
            path = "/auth/admin-sign-up",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/auth/admin-sign-up")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<UserAdminResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SignUpAdminBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::start_o_auth`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct StartOAuthBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a OAuthConfigDTO,
}

/// Type alias for [`StartOAuthBuilder`].
pub type StartOAuthArgs<'a> = StartOAuthBuilder<'a>;
impl StartOAuthBuilder<'_> {
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
    pub async fn send(self) -> Result<OAuthAuthorizeResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Authentication",
            method = "POST",
            path = "/oauth/authorize",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/oauth/authorize")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<OAuthAuthorizeResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for StartOAuthBuilder<'a> {
    type Output = Result<OAuthAuthorizeResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::unlink_all_o_auth_accounts_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UnlinkAllOAuthAccountsAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`UnlinkAllOAuthAccountsAdminBuilder`].
pub type UnlinkAllOAuthAccountsAdminArgs<'a> = UnlinkAllOAuthAccountsAdminBuilder<'a>;
impl UnlinkAllOAuthAccountsAdminBuilder<'_> {
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
            module = "Authentication",
            method = "POST",
            path = "/admin/auth/unlink-all",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/auth/unlink-all")?;
        request = self.request_config.apply(request);
        self.client.execute_unit("authentication", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for UnlinkAllOAuthAccountsAdminBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::unlink_o_auth_account`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UnlinkOAuthAccountBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`UnlinkOAuthAccountBuilder`].
pub type UnlinkOAuthAccountArgs<'a> = UnlinkOAuthAccountBuilder<'a>;
impl UnlinkOAuthAccountBuilder<'_> {
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
        tracing::trace!(
            module = "Authentication",
            method = "POST",
            path = "/oauth/unlink",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/oauth/unlink")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<UserAdminResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UnlinkOAuthAccountBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::unlock_auth_session`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UnlockAuthSessionBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a SessionUnlockDTO,
}

/// Type alias for [`UnlockAuthSessionBuilder`].
pub type UnlockAuthSessionArgs<'a> = UnlockAuthSessionBuilder<'a>;
impl UnlockAuthSessionBuilder<'_> {
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
            module = "Authentication",
            method = "POST",
            path = "/auth/session/unlock",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/auth/session/unlock")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("authentication", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for UnlockAuthSessionBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Authentication::validate_access_token`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ValidateAccessTokenBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`ValidateAccessTokenBuilder`].
pub type ValidateAccessTokenArgs<'a> = ValidateAccessTokenBuilder<'a>;
impl ValidateAccessTokenBuilder<'_> {
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
    pub async fn send(self) -> Result<ValidateAccessTokenResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Authentication",
            method = "POST",
            path = "/auth/validateToken",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/auth/validateToken")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<ValidateAccessTokenResponseDTO>("authentication", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for ValidateAccessTokenBuilder<'a> {
    type Output = Result<ValidateAccessTokenResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Authentication` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Authentication<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Authentication<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Change password
    /// Change the password of the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `ChangePasswordDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    pub fn change_password(&self, body: &'a ChangePasswordDTO) -> ChangePasswordBuilder<'a> {
        ChangePasswordBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Change pin code
    /// Change the pin code for the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `PinCodeChangeDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn change_pin_code(&self, body: &'a PinCodeChangeDTO) -> ChangePinCodeBuilder<'a> {
        ChangePinCodeBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Finish OAuth
    /// Complete the OAuth authorization process by exchanging the authorization code for a session token.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `OAuthCallbackDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `LoginResponseDTO`
    pub fn finish_o_auth(&self, body: &'a OAuthCallbackDTO) -> FinishOAuthBuilder<'a> {
        FinishOAuthBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Retrieve auth status
    /// Get information about the current session, including whether the user has a password, and if the session can access locked assets.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AuthStatusResponseDTO`
    pub fn get_auth_status(&self) -> GetAuthStatusBuilder<'a> {
        GetAuthStatusBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Link OAuth account
    /// Link an OAuth account to the authenticated user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `OAuthCallbackDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    pub fn link_o_auth_account(&self, body: &'a OAuthCallbackDTO) -> LinkOAuthAccountBuilder<'a> {
        LinkOAuthAccountBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Lock auth session
    /// Remove elevated access to locked assets from the current session.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn lock_auth_session(&self) -> LockAuthSessionBuilder<'a> {
        LockAuthSessionBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Login
    /// Login with username and password and receive a session token.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `LoginCredentialDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `LoginResponseDTO`
    pub fn login(&self, body: &'a LoginCredentialDTO) -> LoginBuilder<'a> {
        LoginBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Logout
    /// Logout the current user and invalidate the session token.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `LogoutResponseDTO`
    pub fn logout(&self) -> LogoutBuilder<'a> {
        LogoutBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Backchannel OAuth logout
    /// Logout the OAuth account and invalidate the session specified by the sid claim or all sessions if the sid claim is not present.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    pub fn logout_o_auth(&self) -> LogoutOAuthBuilder<'a> {
        LogoutOAuthBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Redirect OAuth to mobile
    /// Requests to this URL are automatically forwarded to the mobile app, and is used in some cases for OAuth redirecting.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    pub fn redirect_o_auth_to_mobile(&self) -> RedirectOAuthToMobileBuilder<'a> {
        RedirectOAuthToMobileBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Reset pin code
    /// Reset the pin code for the current user by providing the account password
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `PinCodeResetDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn reset_pin_code(&self, body: &'a PinCodeResetDTO) -> ResetPinCodeBuilder<'a> {
        ResetPinCodeBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Setup pin code
    /// Setup a new pin code for the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `PinCodeSetupDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn setup_pin_code(&self, body: &'a PinCodeSetupDTO) -> SetupPinCodeBuilder<'a> {
        SetupPinCodeBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Register admin
    /// Create the first admin user in the system.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `SignUpDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    pub fn sign_up_admin(&self, body: &'a SignUpDTO) -> SignUpAdminBuilder<'a> {
        SignUpAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Start OAuth
    /// Initiate the OAuth authorization process.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `OAuthConfigDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `OAuthAuthorizeResponseDTO`
    pub fn start_o_auth(&self, body: &'a OAuthConfigDTO) -> StartOAuthBuilder<'a> {
        StartOAuthBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Unlink all OAuth accounts
    /// Unlinks all OAuth accounts associated with user accounts in the system.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn unlink_all_o_auth_accounts_admin(&self) -> UnlinkAllOAuthAccountsAdminBuilder<'a> {
        UnlinkAllOAuthAccountsAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Unlink OAuth account
    /// Unlink the OAuth account from the authenticated user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    pub fn unlink_o_auth_account(&self) -> UnlinkOAuthAccountBuilder<'a> {
        UnlinkOAuthAccountBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Unlock auth session
    /// Temporarily grant the session elevated access to locked assets by providing the correct PIN code.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `SessionUnlockDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn unlock_auth_session(&self, body: &'a SessionUnlockDTO) -> UnlockAuthSessionBuilder<'a> {
        UnlockAuthSessionBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Validate access token
    /// Validate the current authorization method is still valid.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `ValidateAccessTokenResponseDTO`
    pub fn validate_access_token(&self) -> ValidateAccessTokenBuilder<'a> {
        ValidateAccessTokenBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
}
