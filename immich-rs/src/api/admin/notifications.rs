// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `notifications`.

use immich_models::admin::notifications::{
    NotificationCreateDTO, TemplateDTO, TemplateResponseDTO, TestEmailResponseDTO,
};
use immich_models::notifications::NotificationDTO;
use immich_models::system::config::AdminConfigSmtpDTO;

/// Fluent request builder for [`Notifications::create_notification`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateNotificationBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: NotificationCreateDTO,
}

impl CreateNotificationBuilder<'_> {
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
    pub async fn send(self) -> Result<NotificationDTO, crate::Error> {
        tracing::trace!(
            module = "admin/notifications",
            method = "POST",
            path = "/admin/notifications",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/notifications")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<NotificationDTO>("admin/notifications", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateNotificationBuilder<'a> {
    type Output = Result<NotificationDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Notifications::get_notification_template_admin`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetNotificationTemplateAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    name: String,
    body: TemplateDTO,
}

impl GetNotificationTemplateAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<TemplateResponseDTO, crate::Error> {
        tracing::trace!(
            module = "admin/notifications",
            method = "POST",
            path = "/admin/notifications/templates/{name}",
            "HTTP request"
        );
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::POST,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("notifications"),
                crate::core::PathSegment::Static("templates"),
                crate::core::PathSegment::Param(self.name.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<TemplateResponseDTO>("admin/notifications", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetNotificationTemplateAdminBuilder<'a> {
    type Output = Result<TemplateResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Notifications::send_test_email_admin`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SendTestEmailAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AdminConfigSmtpDTO,
}

impl SendTestEmailAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<TestEmailResponseDTO, crate::Error> {
        tracing::trace!(
            module = "admin/notifications",
            method = "POST",
            path = "/admin/notifications/test-email",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/notifications/test-email")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<TestEmailResponseDTO>("admin/notifications", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SendTestEmailAdminBuilder<'a> {
    type Output = Result<TestEmailResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Notifications` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Notifications<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Notifications<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Create a notification
    ///
    /// Create a new notification for a specific user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `NotificationCreateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `NotificationDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn create_notification(&self, body: impl Into<NotificationCreateDTO>) -> CreateNotificationBuilder<'a> {
        CreateNotificationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Render email template
    ///
    /// Retrieve a preview of the provided email template.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `name` | `String` | ✓ |  |
    /// | `body` | `TemplateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `TemplateResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_notification_template_admin(
        &self,
        name: impl Into<String>,
        body: impl Into<TemplateDTO>,
    ) -> GetNotificationTemplateAdminBuilder<'a> {
        GetNotificationTemplateAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name: name.into(),
            body: body.into(),
        }
    }
    /// Send test email
    ///
    /// Send a test email using the provided SMTP configuration.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AdminConfigSmtpDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `TestEmailResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn send_test_email_admin(&self, body: impl Into<AdminConfigSmtpDTO>) -> SendTestEmailAdminBuilder<'a> {
        SendTestEmailAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
}
