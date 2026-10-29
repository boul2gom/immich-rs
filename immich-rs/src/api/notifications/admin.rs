// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `admin`.

use immich_models::notifications::NotificationDTO;
use immich_models::notifications::admin::{
    NotificationCreateDTO, TemplateDTO, TemplateResponseDTO, TestEmailResponseDTO,
};
use immich_models::system::config::AdminConfigSmtpDTO;

/// Fluent request builder for [`Admin::create_notification`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateNotificationBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a NotificationCreateDTO,
}

/// Type alias for [`CreateNotificationBuilder`].
pub type CreateNotificationArgs<'a> = CreateNotificationBuilder<'a>;
impl CreateNotificationBuilder<'_> {
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
    pub async fn send(self) -> Result<NotificationDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "POST",
            path = "/admin/notifications",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/notifications")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<NotificationDTO>("admin", request).await?;
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

/// Fluent request builder for [`Admin::get_notification_template_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetNotificationTemplateAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub name: String,
    #[serde(skip)]
    pub body: &'a TemplateDTO,
}

/// Type alias for [`GetNotificationTemplateAdminBuilder`].
pub type GetNotificationTemplateAdminArgs<'a> = GetNotificationTemplateAdminBuilder<'a>;
impl GetNotificationTemplateAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<TemplateResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "POST",
            path = "/admin/notifications/templates/{name}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::POST,
            &format!("/admin/notifications/templates/{}", ::urlencoding::encode(&self.name)),
        )?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<TemplateResponseDTO>("admin", request).await?;
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

/// Fluent request builder for [`Admin::send_test_email_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SendTestEmailAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AdminConfigSmtpDTO,
}

/// Type alias for [`SendTestEmailAdminBuilder`].
pub type SendTestEmailAdminArgs<'a> = SendTestEmailAdminBuilder<'a>;
impl SendTestEmailAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<TestEmailResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "POST",
            path = "/admin/notifications/test-email",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/admin/notifications/test-email")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<TestEmailResponseDTO>("admin", request).await?;
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

///Client handle for `Admin` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Admin<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Admin<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create a notification
    /// Create a new notification for a specific user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `NotificationCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `NotificationDTO`
    pub fn create_notification(&self, body: &'a NotificationCreateDTO) -> CreateNotificationBuilder<'a> {
        CreateNotificationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Render email template
    /// Retrieve a preview of the provided email template.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `name` | `String` | ✓ |  |
    /// | `body` | `TemplateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `TemplateResponseDTO`
    pub fn get_notification_template_admin(
        &self,
        name: impl Into<String>,
        body: &'a TemplateDTO,
    ) -> GetNotificationTemplateAdminBuilder<'a> {
        GetNotificationTemplateAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name: name.into(),
            body,
        }
    }

    /// Send test email
    /// Send a test email using the provided SMTP configuration.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AdminConfigSmtpDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `TestEmailResponseDTO`
    pub fn send_test_email_admin(&self, body: &'a AdminConfigSmtpDTO) -> SendTestEmailAdminBuilder<'a> {
        SendTestEmailAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
