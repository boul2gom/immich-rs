// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `notifications`.

pub mod admin;

use immich_models::notifications::{
    NotificationDTO, NotificationDeleteAllDTO, NotificationLevel, NotificationType, NotificationUpdateAllDTO,
    NotificationUpdateDTO,
};

/// Fluent request builder for [`Notifications::delete_notification`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteNotificationBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`DeleteNotificationBuilder`].
pub type DeleteNotificationArgs<'a> = DeleteNotificationBuilder<'a>;
impl DeleteNotificationBuilder<'_> {
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
            module = "Notifications",
            method = "DELETE",
            path = "/notifications/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/notifications/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("notifications", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteNotificationBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Notifications::delete_notifications`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteNotificationsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a NotificationDeleteAllDTO,
}

/// Type alias for [`DeleteNotificationsBuilder`].
pub type DeleteNotificationsArgs<'a> = DeleteNotificationsBuilder<'a>;
impl DeleteNotificationsBuilder<'_> {
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
            module = "Notifications",
            method = "DELETE",
            path = "/notifications",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/notifications")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("notifications", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteNotificationsBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Notifications::get_notification`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetNotificationBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetNotificationBuilder`].
pub type GetNotificationArgs<'a> = GetNotificationBuilder<'a>;
impl GetNotificationBuilder<'_> {
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
            module = "Notifications",
            method = "GET",
            path = "/notifications/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/notifications/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<NotificationDTO>("notifications", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetNotificationBuilder<'a> {
    type Output = Result<NotificationDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Notifications::get_notifications`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetNotificationsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<NotificationLevel>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<NotificationType>,
    #[serde(rename = "unread", skip_serializing_if = "Option::is_none")]
    pub unread: Option<bool>,
}

/// Type alias for [`GetNotificationsBuilder`].
pub type GetNotificationsArgs<'a> = GetNotificationsBuilder<'a>;
impl GetNotificationsBuilder<'_> {
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

    /// Filter by notification ID
    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.id = value;
        self
    }

    /// Sets `level`.
    pub fn level(mut self, value: NotificationLevel) -> Self {
        self.level = Some(value);
        self
    }

    /// Sets or clears `level`.
    pub fn set_level(mut self, value: Option<NotificationLevel>) -> Self {
        self.level = value;
        self
    }

    /// Sets `type`.
    pub fn type_(mut self, value: NotificationType) -> Self {
        self.type_ = Some(value);
        self
    }

    /// Sets or clears `type`.
    pub fn set_type_(mut self, value: Option<NotificationType>) -> Self {
        self.type_ = value;
        self
    }

    /// Filter by unread status
    pub fn unread(mut self, value: bool) -> Self {
        self.unread = Some(value);
        self
    }

    /// Sets or clears `unread`.
    pub fn set_unread(mut self, value: Option<bool>) -> Self {
        self.unread = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<NotificationDTO>, crate::Error> {
        tracing::trace!(
            module = "Notifications",
            method = "GET",
            path = "/notifications",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/notifications")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<Vec<NotificationDTO>>("notifications", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetNotificationsBuilder<'a> {
    type Output = Result<Vec<NotificationDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Notifications::update_notification`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateNotificationBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a NotificationUpdateDTO,
}

/// Type alias for [`UpdateNotificationBuilder`].
pub type UpdateNotificationArgs<'a> = UpdateNotificationBuilder<'a>;
impl UpdateNotificationBuilder<'_> {
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
            module = "Notifications",
            method = "PUT",
            path = "/notifications/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/notifications/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<NotificationDTO>("notifications", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateNotificationBuilder<'a> {
    type Output = Result<NotificationDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Notifications::update_notifications`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateNotificationsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a NotificationUpdateAllDTO,
}

/// Type alias for [`UpdateNotificationsBuilder`].
pub type UpdateNotificationsArgs<'a> = UpdateNotificationsBuilder<'a>;
impl UpdateNotificationsBuilder<'_> {
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
            module = "Notifications",
            method = "PUT",
            path = "/notifications",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/notifications")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("notifications", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for UpdateNotificationsBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Notifications` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Notifications<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Notifications<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Delete a notification
    /// Delete a specific notification.
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
    pub fn delete_notification(&self, id: uuid::Uuid) -> DeleteNotificationBuilder<'a> {
        DeleteNotificationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Delete notifications
    /// Delete a list of notifications at once.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `NotificationDeleteAllDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_notifications(&self, body: &'a NotificationDeleteAllDTO) -> DeleteNotificationsBuilder<'a> {
        DeleteNotificationsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Get a notification
    /// Retrieve a specific notification identified by id.
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
    /// **Returns:** `NotificationDTO`
    pub fn get_notification(&self, id: uuid::Uuid) -> GetNotificationBuilder<'a> {
        GetNotificationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Retrieve notifications
    /// Retrieve a list of notifications.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` |  | Filter by notification ID |
    /// | `level` | `NotificationLevel` |  |  |
    /// | `type` | `NotificationType` |  |  |
    /// | `unread` | `bool` |  | Filter by unread status |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<NotificationDTO>`
    pub fn get_notifications(&self) -> GetNotificationsBuilder<'a> {
        GetNotificationsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: None,
            level: None,
            type_: None,
            unread: None,
        }
    }

    /// Update a notification
    /// Update a specific notification to set its read status.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `NotificationUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `NotificationDTO`
    pub fn update_notification(
        &self,
        id: uuid::Uuid,
        body: &'a NotificationUpdateDTO,
    ) -> UpdateNotificationBuilder<'a> {
        UpdateNotificationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }

    /// Update notifications
    /// Update a list of notifications. Allows to bulk-set the read status of notifications.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `NotificationUpdateAllDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn update_notifications(&self, body: &'a NotificationUpdateAllDTO) -> UpdateNotificationsBuilder<'a> {
        UpdateNotificationsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
