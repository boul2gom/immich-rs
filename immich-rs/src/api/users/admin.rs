// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `admin`.

use immich_models::assets::{AssetStatsResponseDTO, AssetVisibility};
use immich_models::authentication::UserAdminResponseDTO;
use immich_models::id::UserId;
use immich_models::sessions::SessionResponseDTO;
use immich_models::users::admin::{UserAdminCreateDTO, UserAdminDeleteDTO, UserAdminUpdateDTO};
use immich_models::users::{
    CalendarHeatmapResponseDTO, CalendarHeatmapType, UserPreferencesResponseDTO, UserPreferencesUpdateDTO,
};

/// Fluent request builder for [`Admin::create_user_admin`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateUserAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: UserAdminCreateDTO,
}

impl CreateUserAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<UserAdminResponseDTO, crate::Error> {
        tracing::trace!(
            module = "users/admin",
            method = "POST",
            path = "/admin/users",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/admin/users")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<UserAdminResponseDTO>("users/admin", request)
            .await?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteUserAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: UserId,
    body: UserAdminDeleteDTO,
}

impl DeleteUserAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<UserAdminResponseDTO, crate::Error> {
        tracing::trace!(
            module = "users/admin",
            method = "DELETE",
            path = "/admin/users/{id}",
            "HTTP request"
        );
        let __seg_2 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("users"),
                crate::core::PathSegment::Param(__seg_2.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<UserAdminResponseDTO>("users/admin", request)
            .await?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: UserId,
}

impl GetUserAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<UserAdminResponseDTO, crate::Error> {
        tracing::trace!(
            module = "users/admin",
            method = "GET",
            path = "/admin/users/{id}",
            "HTTP request"
        );
        let __seg_2 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("users"),
                crate::core::PathSegment::Param(__seg_2.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<UserAdminResponseDTO>("users/admin", request)
            .await?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserCalendarHeatmapAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    from: Option<String>,
    id: UserId,
    to: Option<String>,
    type_: Option<CalendarHeatmapType>,
}

impl GetUserCalendarHeatmapAdminBuilder<'_> {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<CalendarHeatmapResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
            from: Option<&'b String>,
            #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
            to: Option<&'b String>,
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            type_: Option<&'b CalendarHeatmapType>,
        }
        tracing::trace!(
            module = "users/admin",
            method = "GET",
            path = "/admin/users/{id}/calendar-heatmap",
            "HTTP request"
        );
        let __seg_2 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("users"),
                crate::core::PathSegment::Param(__seg_2.as_str()),
                crate::core::PathSegment::Static("calendar-heatmap"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            from: self.from.as_ref(),
            to: self.to.as_ref(),
            type_: self.type_.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<CalendarHeatmapResponseDTO>("users/admin", request)
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserPreferencesAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: UserId,
}

impl GetUserPreferencesAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<UserPreferencesResponseDTO, crate::Error> {
        tracing::trace!(
            module = "users/admin",
            method = "GET",
            path = "/admin/users/{id}/preferences",
            "HTTP request"
        );
        let __seg_2 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("users"),
                crate::core::PathSegment::Param(__seg_2.as_str()),
                crate::core::PathSegment::Static("preferences"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<UserPreferencesResponseDTO>("users/admin", request)
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserSessionsAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: UserId,
}

impl GetUserSessionsAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<SessionResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "users/admin",
            method = "GET",
            path = "/admin/users/{id}/sessions",
            "HTTP request"
        );
        let __seg_2 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("users"),
                crate::core::PathSegment::Param(__seg_2.as_str()),
                crate::core::PathSegment::Static("sessions"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<Vec<SessionResponseDTO>>("users/admin", request)
            .await?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUserStatisticsAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: UserId,
    is_favorite: Option<bool>,
    is_trashed: Option<bool>,
    visibility: Option<AssetVisibility>,
}

impl GetUserStatisticsAdminBuilder<'_> {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AssetStatsResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
            is_favorite: Option<&'b bool>,
            #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
            is_trashed: Option<&'b bool>,
            #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
            visibility: Option<&'b AssetVisibility>,
        }
        tracing::trace!(
            module = "users/admin",
            method = "GET",
            path = "/admin/users/{id}/statistics",
            "HTTP request"
        );
        let __seg_2 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("users"),
                crate::core::PathSegment::Param(__seg_2.as_str()),
                crate::core::PathSegment::Static("statistics"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            is_favorite: self.is_favorite.as_ref(),
            is_trashed: self.is_trashed.as_ref(),
            visibility: self.visibility.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<AssetStatsResponseDTO>("users/admin", request)
            .await?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RestoreUserAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: UserId,
}

impl RestoreUserAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<UserAdminResponseDTO, crate::Error> {
        tracing::trace!(
            module = "users/admin",
            method = "POST",
            path = "/admin/users/{id}/restore",
            "HTTP request"
        );
        let __seg_2 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::POST,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("users"),
                crate::core::PathSegment::Param(__seg_2.as_str()),
                crate::core::PathSegment::Static("restore"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<UserAdminResponseDTO>("users/admin", request)
            .await?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchUsersAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: Option<UserId>,
    with_deleted: Option<bool>,
}

impl SearchUsersAdminBuilder<'_> {
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
    /// User ID filter
    pub fn id(mut self, value: impl Into<UserId>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<UserId>) -> Self {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<UserAdminResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b UserId>,
            #[serde(rename = "withDeleted", skip_serializing_if = "Option::is_none")]
            with_deleted: Option<&'b bool>,
        }
        tracing::trace!(
            module = "users/admin",
            method = "GET",
            path = "/admin/users",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/admin/users")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            id: self.id.as_ref(),
            with_deleted: self.with_deleted.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<Vec<UserAdminResponseDTO>>("users/admin", request)
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateUserAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: UserId,
    body: UserAdminUpdateDTO,
}

impl UpdateUserAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<UserAdminResponseDTO, crate::Error> {
        tracing::trace!(
            module = "users/admin",
            method = "PUT",
            path = "/admin/users/{id}",
            "HTTP request"
        );
        let __seg_2 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("users"),
                crate::core::PathSegment::Param(__seg_2.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<UserAdminResponseDTO>("users/admin", request)
            .await?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateUserPreferencesAdminBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: UserId,
    body: UserPreferencesUpdateDTO,
}

impl UpdateUserPreferencesAdminBuilder<'_> {
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
    pub async fn send(self) -> Result<UserPreferencesResponseDTO, crate::Error> {
        tracing::trace!(
            module = "users/admin",
            method = "PUT",
            path = "/admin/users/{id}/preferences",
            "HTTP request"
        );
        let __seg_2 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("admin"),
                crate::core::PathSegment::Static("users"),
                crate::core::PathSegment::Param(__seg_2.as_str()),
                crate::core::PathSegment::Static("preferences"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<UserPreferencesResponseDTO>("users/admin", request)
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

/// Client handle for `Admin` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Admin<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Admin<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Create a user
    ///
    /// Create a new user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `UserAdminCreateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn create_user_admin(&self, body: impl Into<UserAdminCreateDTO>) -> CreateUserAdminBuilder<'a> {
        CreateUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete a user
    ///
    /// Delete a user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `UserId` | ✓ |  |
    /// | `body` | `UserAdminDeleteDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_user_admin(
        &self,
        id: impl Into<UserId>,
        body: impl Into<UserAdminDeleteDTO>,
    ) -> DeleteUserAdminBuilder<'a> {
        DeleteUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Retrieve a user
    ///
    /// Retrieve  a specific user by their ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `UserId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_user_admin(&self, id: impl Into<UserId>) -> GetUserAdminBuilder<'a> {
        GetUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve calendar heatmap activity
    ///
    /// Retrieve activity counts for a specified period, in a calendar heatmap format.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `from` | `String` |  | Start date in UTC |
    /// | `id` | `UserId` | ✓ |  |
    /// | `to` | `String` |  | End date in UTC |
    /// | `type_` | `CalendarHeatmapType` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `CalendarHeatmapResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_user_calendar_heatmap_admin(&self, id: impl Into<UserId>) -> GetUserCalendarHeatmapAdminBuilder<'a> {
        GetUserCalendarHeatmapAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            from: None,
            id: id.into(),
            to: None,
            type_: None,
        }
    }
    /// Retrieve user preferences
    ///
    /// Retrieve the preferences of a specific user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `UserId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserPreferencesResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_user_preferences_admin(&self, id: impl Into<UserId>) -> GetUserPreferencesAdminBuilder<'a> {
        GetUserPreferencesAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve user sessions
    ///
    /// Retrieve all sessions for a specific user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `UserId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<SessionResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_user_sessions_admin(&self, id: impl Into<UserId>) -> GetUserSessionsAdminBuilder<'a> {
        GetUserSessionsAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve user statistics
    ///
    /// Retrieve asset statistics for a specific user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `UserId` | ✓ |  |
    /// | `is_favorite` | `bool` |  | Filter by favorite status |
    /// | `is_trashed` | `bool` |  | Filter by trash status |
    /// | `visibility` | `AssetVisibility` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AssetStatsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_user_statistics_admin(&self, id: impl Into<UserId>) -> GetUserStatisticsAdminBuilder<'a> {
        GetUserStatisticsAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            is_favorite: None,
            is_trashed: None,
            visibility: None,
        }
    }
    /// Restore a deleted user
    ///
    /// Restore a previously deleted user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `UserId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn restore_user_admin(&self, id: impl Into<UserId>) -> RestoreUserAdminBuilder<'a> {
        RestoreUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Search users
    ///
    /// Search for users.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `UserId` |  | User ID filter |
    /// | `with_deleted` | `bool` |  | Include deleted users |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<UserAdminResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn search_users_admin(&self) -> SearchUsersAdminBuilder<'a> {
        SearchUsersAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: None,
            with_deleted: None,
        }
    }
    /// Update a user
    ///
    /// Update an existing user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `UserId` | ✓ |  |
    /// | `body` | `UserAdminUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserAdminResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_user_admin(
        &self,
        id: impl Into<UserId>,
        body: impl Into<UserAdminUpdateDTO>,
    ) -> UpdateUserAdminBuilder<'a> {
        UpdateUserAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Update user preferences
    ///
    /// Update the preferences of a specific user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `UserId` | ✓ |  |
    /// | `body` | `UserPreferencesUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `UserPreferencesResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_user_preferences_admin(
        &self,
        id: impl Into<UserId>,
        body: impl Into<UserPreferencesUpdateDTO>,
    ) -> UpdateUserPreferencesAdminBuilder<'a> {
        UpdateUserPreferencesAdminBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
}
