// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `activities`.

use immich_models::activities::{
    ActivityCreateDTO, ActivityResponseDTO, ActivityStatisticsResponseDTO, ReactionLevel, ReactionType,
};
use immich_models::id::{ActivityId, AlbumId, AssetId, UserId};

/// Fluent request builder for [`Activities::create_activity`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateActivityBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: ActivityCreateDTO,
}

impl CreateActivityBuilder<'_> {
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
    pub async fn send(self) -> Result<ActivityResponseDTO, crate::Error> {
        tracing::trace!(
            module = "activities",
            method = "POST",
            path = "/activities",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/activities")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<ActivityResponseDTO>("activities", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateActivityBuilder<'a> {
    type Output = Result<ActivityResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Activities::delete_activity`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteActivityBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: ActivityId,
}

impl DeleteActivityBuilder<'_> {
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
            module = "activities",
            method = "DELETE",
            path = "/activities/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("activities"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("activities", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteActivityBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Activities::get_activities`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetActivitiesBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    album_id: AlbumId,
    asset_id: Option<AssetId>,
    level: Option<ReactionLevel>,
    type_: Option<ReactionType>,
    user_id: Option<UserId>,
}

impl GetActivitiesBuilder<'_> {
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
    /// Asset ID (if activity is for an asset)
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = Some(value.into());
        self
    }
    /// Sets or clears `assetId`.
    pub fn set_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.asset_id = value;
        self
    }
    /// Sets `level`.
    pub fn level(mut self, value: ReactionLevel) -> Self {
        self.level = Some(value);
        self
    }
    /// Sets or clears `level`.
    pub fn set_level(mut self, value: Option<ReactionLevel>) -> Self {
        self.level = value;
        self
    }
    /// Sets `type`.
    pub fn type_(mut self, value: ReactionType) -> Self {
        self.type_ = Some(value);
        self
    }
    /// Sets or clears `type`.
    pub fn set_type_(mut self, value: Option<ReactionType>) -> Self {
        self.type_ = value;
        self
    }
    /// Filter by user ID
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = Some(value.into());
        self
    }
    /// Sets or clears `userId`.
    pub fn set_user_id(mut self, value: Option<UserId>) -> Self {
        self.user_id = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<ActivityResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "albumId")]
            album_id: &'b AlbumId,
            #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
            asset_id: Option<&'b AssetId>,
            #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
            level: Option<&'b ReactionLevel>,
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            type_: Option<&'b ReactionType>,
            #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
            user_id: Option<&'b UserId>,
        }
        tracing::trace!(
            module = "activities",
            method = "GET",
            path = "/activities",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/activities")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            album_id: &self.album_id,
            asset_id: self.asset_id.as_ref(),
            level: self.level.as_ref(),
            type_: self.type_.as_ref(),
            user_id: self.user_id.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<Vec<ActivityResponseDTO>>("activities", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetActivitiesBuilder<'a> {
    type Output = Result<Vec<ActivityResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Activities::get_activity_statistics`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetActivityStatisticsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    album_id: AlbumId,
    asset_id: Option<AssetId>,
}

impl GetActivityStatisticsBuilder<'_> {
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
    /// Asset ID (if activity is for an asset)
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = Some(value.into());
        self
    }
    /// Sets or clears `assetId`.
    pub fn set_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.asset_id = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<ActivityStatisticsResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "albumId")]
            album_id: &'b AlbumId,
            #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
            asset_id: Option<&'b AssetId>,
        }
        tracing::trace!(
            module = "activities",
            method = "GET",
            path = "/activities/statistics",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/activities/statistics")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            album_id: &self.album_id,
            asset_id: self.asset_id.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<ActivityStatisticsResponseDTO>("activities", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetActivityStatisticsBuilder<'a> {
    type Output = Result<ActivityStatisticsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Activities` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Activities<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Activities<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Create an activity
    ///
    /// Create a like or a comment for an album, or an asset in an album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `ActivityCreateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `ActivityResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn create_activity(&self, body: impl Into<ActivityCreateDTO>) -> CreateActivityBuilder<'a> {
        CreateActivityBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete an activity
    ///
    /// Removes a like or comment from a given album or asset in an album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `ActivityId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_activity(&self, id: impl Into<ActivityId>) -> DeleteActivityBuilder<'a> {
        DeleteActivityBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// List all activities
    ///
    /// Returns a list of activities for the selected asset or album. The activities are returned in sorted order, with the oldest activities appearing first.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `album_id` | `AlbumId` | ✓ | Album ID |
    /// | `asset_id` | `AssetId` |  | Asset ID (if activity is for an asset) |
    /// | `level` | `ReactionLevel` |  |  |
    /// | `type_` | `ReactionType` |  |  |
    /// | `user_id` | `UserId` |  | Filter by user ID |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<ActivityResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_activities(&self, album_id: impl Into<AlbumId>) -> GetActivitiesBuilder<'a> {
        GetActivitiesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            album_id: album_id.into(),
            asset_id: None,
            level: None,
            type_: None,
            user_id: None,
        }
    }
    /// Retrieve activity statistics
    ///
    /// Returns the number of likes and comments for a given album or asset in an album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `album_id` | `AlbumId` | ✓ | Album ID |
    /// | `asset_id` | `AssetId` |  | Asset ID (if activity is for an asset) |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `ActivityStatisticsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_activity_statistics(&self, album_id: impl Into<AlbumId>) -> GetActivityStatisticsBuilder<'a> {
        GetActivityStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            album_id: album_id.into(),
            asset_id: None,
        }
    }
}
