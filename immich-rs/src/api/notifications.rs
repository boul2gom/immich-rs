// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `notifications`.

use immich_models::id::NotificationId;
use immich_models::notifications::{
    NotificationDTO, NotificationDeleteAllDTO, NotificationLevel, NotificationType, NotificationUpdateAllDTO,
    NotificationUpdateDTO,
};

/// Fluent request builder for [`Notifications::delete_notification`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteNotificationBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: NotificationId,
}

impl DeleteNotificationBuilder<'_> {
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
            module = "notifications",
            method = "DELETE",
            path = "/notifications/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("notifications"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteNotificationsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: NotificationDeleteAllDTO,
}

impl DeleteNotificationsBuilder<'_> {
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
            module = "notifications",
            method = "DELETE",
            path = "/notifications",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/notifications")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetNotificationBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: NotificationId,
}

impl GetNotificationBuilder<'_> {
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
            module = "notifications",
            method = "GET",
            path = "/notifications/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("notifications"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetNotificationsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: Option<NotificationId>,
    level: Option<NotificationLevel>,
    type_: Option<NotificationType>,
    unread: Option<bool>,
}

impl GetNotificationsBuilder<'_> {
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
    /// Filter by notification ID
    pub fn id(mut self, value: impl Into<NotificationId>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<NotificationId>) -> Self {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<NotificationDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b NotificationId>,
            #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
            level: Option<&'b NotificationLevel>,
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            type_: Option<&'b NotificationType>,
            #[serde(rename = "unread", skip_serializing_if = "Option::is_none")]
            unread: Option<&'b bool>,
        }
        tracing::trace!(
            module = "notifications",
            method = "GET",
            path = "/notifications",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/notifications")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            id: self.id.as_ref(),
            level: self.level.as_ref(),
            type_: self.type_.as_ref(),
            unread: self.unread.as_ref(),
        };
        request = request.query(&query);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateNotificationBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: NotificationId,
    body: NotificationUpdateDTO,
}

impl UpdateNotificationBuilder<'_> {
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
            module = "notifications",
            method = "PUT",
            path = "/notifications/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("notifications"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateNotificationsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: NotificationUpdateAllDTO,
}

impl UpdateNotificationsBuilder<'_> {
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
            module = "notifications",
            method = "PUT",
            path = "/notifications",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/notifications")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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
    /// Delete a notification
    ///
    /// Delete a specific notification.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `NotificationId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_notification(&self, id: impl Into<NotificationId>) -> DeleteNotificationBuilder<'a> {
        DeleteNotificationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Delete notifications
    ///
    /// Delete a list of notifications at once.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `NotificationDeleteAllDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_notifications(&self, body: impl Into<NotificationDeleteAllDTO>) -> DeleteNotificationsBuilder<'a> {
        DeleteNotificationsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Get a notification
    ///
    /// Retrieve a specific notification identified by id.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `NotificationId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `NotificationDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_notification(&self, id: impl Into<NotificationId>) -> GetNotificationBuilder<'a> {
        GetNotificationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve notifications
    ///
    /// Retrieve a list of notifications.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `NotificationId` |  | Filter by notification ID |
    /// | `level` | `NotificationLevel` |  |  |
    /// | `type_` | `NotificationType` |  |  |
    /// | `unread` | `bool` |  | Filter by unread status |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<NotificationDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
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
    ///
    /// Update a specific notification to set its read status.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `NotificationId` | ✓ |  |
    /// | `body` | `NotificationUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `NotificationDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_notification(
        &self,
        id: impl Into<NotificationId>,
        body: impl Into<NotificationUpdateDTO>,
    ) -> UpdateNotificationBuilder<'a> {
        UpdateNotificationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Update notifications
    ///
    /// Update a list of notifications. Allows to bulk-set the read status of notifications.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `NotificationUpdateAllDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_notifications(&self, body: impl Into<NotificationUpdateAllDTO>) -> UpdateNotificationsBuilder<'a> {
        UpdateNotificationsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
}
