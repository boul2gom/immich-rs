// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `admin`.

use immich_models::assets::{AssetStatsResponseDTO, AssetVisibility};
use immich_models::authentication::UserAdminResponseDTO;
use immich_models::sessions::SessionResponseDTO;
use immich_models::users::admin::{UserAdminCreateDTO, UserAdminDeleteDTO, UserAdminUpdateDTO};
use immich_models::users::{
    CalendarHeatmapResponseDTO, CalendarHeatmapType, UserPreferencesResponseDTO, UserPreferencesUpdateDTO,
};

/// Fluent request builder for [`Admin::create_user_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateUserAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a UserAdminCreateDTO,
}

/// Type alias for [`CreateUserAdminBuilder`].
pub type CreateUserAdminArgs<'a> = CreateUserAdminBuilder<'a>;
impl CreateUserAdminBuilder<'_> {
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
        tracing::trace!(module = "Admin", method = "POST", path = "/admin/users", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/admin/users")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<UserAdminResponseDTO>("admin", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateUserAdminBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::delete_user_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteUserAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a UserAdminDeleteDTO,
}

/// Type alias for [`DeleteUserAdminBuilder`].
pub type DeleteUserAdminArgs<'a> = DeleteUserAdminBuilder<'a>;
impl DeleteUserAdminBuilder<'_> {
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
            module = "Admin",
            method = "DELETE",
            path = "/admin/users/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/admin/users/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<UserAdminResponseDTO>("admin", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for DeleteUserAdminBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::get_user_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetUserAdminBuilder`].
pub type GetUserAdminArgs<'a> = GetUserAdminBuilder<'a>;
impl GetUserAdminBuilder<'_> {
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
            module = "Admin",
            method = "GET",
            path = "/admin/users/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/admin/users/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<UserAdminResponseDTO>("admin", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserAdminBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::get_user_calendar_heatmap_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserCalendarHeatmapAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<CalendarHeatmapType>,
}

/// Type alias for [`GetUserCalendarHeatmapAdminBuilder`].
pub type GetUserCalendarHeatmapAdminArgs<'a> = GetUserCalendarHeatmapAdminBuilder<'a>;
impl GetUserCalendarHeatmapAdminBuilder<'_> {
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
            module = "Admin",
            method = "GET",
            path = "/admin/users/{id}/calendar-heatmap",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::GET,
            &format!("/admin/users/{}/calendar-heatmap", self.id),
        )?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<CalendarHeatmapResponseDTO>("admin", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserCalendarHeatmapAdminBuilder<'a> {
    type Output = Result<CalendarHeatmapResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::get_user_preferences_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserPreferencesAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetUserPreferencesAdminBuilder`].
pub type GetUserPreferencesAdminArgs<'a> = GetUserPreferencesAdminBuilder<'a>;
impl GetUserPreferencesAdminBuilder<'_> {
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
            module = "Admin",
            method = "GET",
            path = "/admin/users/{id}/preferences",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/admin/users/{}/preferences", self.id))?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<UserPreferencesResponseDTO>("admin", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserPreferencesAdminBuilder<'a> {
    type Output = Result<UserPreferencesResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::get_user_sessions_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserSessionsAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetUserSessionsAdminBuilder`].
pub type GetUserSessionsAdminArgs<'a> = GetUserSessionsAdminBuilder<'a>;
impl GetUserSessionsAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<SessionResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "GET",
            path = "/admin/users/{id}/sessions",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/admin/users/{}/sessions", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<Vec<SessionResponseDTO>>("admin", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserSessionsAdminBuilder<'a> {
    type Output = Result<Vec<SessionResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::get_user_statistics_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserStatisticsAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
    pub is_trashed: Option<bool>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AssetVisibility>,
}

/// Type alias for [`GetUserStatisticsAdminBuilder`].
pub type GetUserStatisticsAdminArgs<'a> = GetUserStatisticsAdminBuilder<'a>;
impl GetUserStatisticsAdminBuilder<'_> {
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

    /// Filter by favorite status
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    /// Sets or clears `isFavorite`.
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }

    /// Filter by trash status
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = Some(value);
        self
    }

    /// Sets or clears `isTrashed`.
    pub fn set_is_trashed(mut self, value: Option<bool>) -> Self {
        self.is_trashed = value;
        self
    }

    /// Sets `visibility`.
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Sets or clears `visibility`.
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AssetStatsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Admin",
            method = "GET",
            path = "/admin/users/{id}/statistics",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/admin/users/{}/statistics", self.id))?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self.client.execute::<AssetStatsResponseDTO>("admin", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUserStatisticsAdminBuilder<'a> {
    type Output = Result<AssetStatsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::restore_user_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RestoreUserAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`RestoreUserAdminBuilder`].
pub type RestoreUserAdminArgs<'a> = RestoreUserAdminBuilder<'a>;
impl RestoreUserAdminBuilder<'_> {
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
            module = "Admin",
            method = "POST",
            path = "/admin/users/{id}/restore",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, &format!("/admin/users/{}/restore", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<UserAdminResponseDTO>("admin", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for RestoreUserAdminBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::search_users_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchUsersAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "withDeleted", skip_serializing_if = "Option::is_none")]
    pub with_deleted: Option<bool>,
}

/// Type alias for [`SearchUsersAdminBuilder`].
pub type SearchUsersAdminArgs<'a> = SearchUsersAdminBuilder<'a>;
impl SearchUsersAdminBuilder<'_> {
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

    /// User ID filter
    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.id = value;
        self
    }

    /// Include deleted users
    pub fn with_deleted(mut self, value: bool) -> Self {
        self.with_deleted = Some(value);
        self
    }

    /// Sets or clears `withDeleted`.
    pub fn set_with_deleted(mut self, value: Option<bool>) -> Self {
        self.with_deleted = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<UserAdminResponseDTO>, crate::Error> {
        tracing::trace!(module = "Admin", method = "GET", path = "/admin/users", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/admin/users")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<Vec<UserAdminResponseDTO>>("admin", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchUsersAdminBuilder<'a> {
    type Output = Result<Vec<UserAdminResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::update_user_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateUserAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a UserAdminUpdateDTO,
}

/// Type alias for [`UpdateUserAdminBuilder`].
pub type UpdateUserAdminArgs<'a> = UpdateUserAdminBuilder<'a>;
impl UpdateUserAdminBuilder<'_> {
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
            module = "Admin",
            method = "PUT",
            path = "/admin/users/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/admin/users/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<UserAdminResponseDTO>("admin", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateUserAdminBuilder<'a> {
    type Output = Result<UserAdminResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Admin::update_user_preferences_admin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateUserPreferencesAdminBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a UserPreferencesUpdateDTO,
}

/// Type alias for [`UpdateUserPreferencesAdminBuilder`].
pub type UpdateUserPreferencesAdminArgs<'a> = UpdateUserPreferencesAdminBuilder<'a>;
impl UpdateUserPreferencesAdminBuilder<'_> {
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
            module = "Admin",
            method = "PUT",
            path = "/admin/users/{id}/preferences",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/admin/users/{}/preferences", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<UserPreferencesResponseDTO>("admin", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateUserPreferencesAdminBuilder<'a> {
    type Output = Result<UserPreferencesResponseDTO, crate::Error>;
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

    /// Create a user
    /// Create a new user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `UserAdminCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    pub fn create_user_admin(&self, body: &'a UserAdminCreateDTO) -> CreateUserAdminBuilder<'a> {
        CreateUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete a user
    /// Delete a user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `UserAdminDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    pub fn delete_user_admin(&self, id: uuid::Uuid, body: &'a UserAdminDeleteDTO) -> DeleteUserAdminBuilder<'a> {
        DeleteUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }

    /// Retrieve a user
    /// Retrieve  a specific user by their ID.
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
    /// **Returns:** `UserAdminResponseDTO`
    pub fn get_user_admin(&self, id: uuid::Uuid) -> GetUserAdminBuilder<'a> {
        GetUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
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
    /// | `id` | `uuid::Uuid` | ✓ |  |
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
    pub fn get_user_calendar_heatmap_admin(&self, id: uuid::Uuid) -> GetUserCalendarHeatmapAdminBuilder<'a> {
        GetUserCalendarHeatmapAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            from: None,
            id,
            to: None,
            type_: None,
        }
    }

    /// Retrieve user preferences
    /// Retrieve the preferences of a specific user.
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
    /// **Returns:** `UserPreferencesResponseDTO`
    pub fn get_user_preferences_admin(&self, id: uuid::Uuid) -> GetUserPreferencesAdminBuilder<'a> {
        GetUserPreferencesAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Retrieve user sessions
    /// Retrieve all sessions for a specific user.
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
    /// **Returns:** `Vec<SessionResponseDTO>`
    pub fn get_user_sessions_admin(&self, id: uuid::Uuid) -> GetUserSessionsAdminBuilder<'a> {
        GetUserSessionsAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Retrieve user statistics
    /// Retrieve asset statistics for a specific user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `isFavorite` | `bool` |  | Filter by favorite status |
    /// | `isTrashed` | `bool` |  | Filter by trash status |
    /// | `visibility` | `AssetVisibility` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetStatsResponseDTO`
    pub fn get_user_statistics_admin(&self, id: uuid::Uuid) -> GetUserStatisticsAdminBuilder<'a> {
        GetUserStatisticsAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            is_favorite: None,
            is_trashed: None,
            visibility: None,
        }
    }

    /// Restore a deleted user
    /// Restore a previously deleted user.
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
    /// **Returns:** `UserAdminResponseDTO`
    pub fn restore_user_admin(&self, id: uuid::Uuid) -> RestoreUserAdminBuilder<'a> {
        RestoreUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Search users
    /// Search for users.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` |  | User ID filter |
    /// | `withDeleted` | `bool` |  | Include deleted users |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<UserAdminResponseDTO>`
    pub fn search_users_admin(&self) -> SearchUsersAdminBuilder<'a> {
        SearchUsersAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: None,
            with_deleted: None,
        }
    }

    /// Update a user
    /// Update an existing user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `UserAdminUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_user_admin(&self, id: uuid::Uuid, body: &'a UserAdminUpdateDTO) -> UpdateUserAdminBuilder<'a> {
        UpdateUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }

    /// Update user preferences
    /// Update the preferences of a specific user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
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
    pub fn update_user_preferences_admin(
        &self,
        id: uuid::Uuid,
        body: &'a UserPreferencesUpdateDTO,
    ) -> UpdateUserPreferencesAdminBuilder<'a> {
        UpdateUserPreferencesAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }
}
