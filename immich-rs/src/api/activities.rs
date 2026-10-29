// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `activities`.

use immich_models::activities::{
    ActivityCreateDTO, ActivityResponseDTO, ActivityStatisticsResponseDTO, ReactionLevel, ReactionType,
};

/// Fluent request builder for [`Activities::create_activity`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateActivityBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a ActivityCreateDTO,
}

/// Type alias for [`CreateActivityBuilder`].
pub type CreateActivityArgs<'a> = CreateActivityBuilder<'a>;
impl CreateActivityBuilder<'_> {
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
    pub async fn send(self) -> Result<ActivityResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Activities",
            method = "POST",
            path = "/activities",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/activities")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteActivityBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::ActivityId,
}

/// Type alias for [`DeleteActivityBuilder`].
pub type DeleteActivityArgs<'a> = DeleteActivityBuilder<'a>;
impl DeleteActivityBuilder<'_> {
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
            module = "Activities",
            method = "DELETE",
            path = "/activities/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/activities/{}", self.id))?;
        request = self.request_config.apply(request);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetActivitiesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "albumId")]
    pub album_id: immich_models::id::AlbumId,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<immich_models::id::AssetId>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<ReactionLevel>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<ReactionType>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<immich_models::id::UserId>,
}

/// Type alias for [`GetActivitiesBuilder`].
pub type GetActivitiesArgs<'a> = GetActivitiesBuilder<'a>;
impl GetActivitiesBuilder<'_> {
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

    /// Asset ID (if activity is for an asset)
    pub fn asset_id(mut self, value: impl Into<immich_models::id::AssetId>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Sets or clears `assetId`.
    pub fn set_asset_id(mut self, value: Option<immich_models::id::AssetId>) -> Self {
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
    pub fn user_id(mut self, value: impl Into<immich_models::id::UserId>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Sets or clears `userId`.
    pub fn set_user_id(mut self, value: Option<immich_models::id::UserId>) -> Self {
        self.user_id = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<ActivityResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Activities",
            method = "GET",
            path = "/activities",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/activities")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetActivityStatisticsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "albumId")]
    pub album_id: immich_models::id::AlbumId,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<immich_models::id::AssetId>,
}

/// Type alias for [`GetActivityStatisticsBuilder`].
pub type GetActivityStatisticsArgs<'a> = GetActivityStatisticsBuilder<'a>;
impl GetActivityStatisticsBuilder<'_> {
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

    /// Asset ID (if activity is for an asset)
    pub fn asset_id(mut self, value: impl Into<immich_models::id::AssetId>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Sets or clears `assetId`.
    pub fn set_asset_id(mut self, value: Option<immich_models::id::AssetId>) -> Self {
        self.asset_id = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<ActivityStatisticsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Activities",
            method = "GET",
            path = "/activities/statistics",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/activities/statistics")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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

///Client handle for `Activities` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Activities<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Activities<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create an activity
    /// Create a like or a comment for an album, or an asset in an album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `ActivityCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `ActivityResponseDTO`
    pub fn create_activity(&self, body: &'a ActivityCreateDTO) -> CreateActivityBuilder<'a> {
        CreateActivityBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete an activity
    /// Removes a like or comment from a given album or asset in an album.
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
    pub fn delete_activity(&self, id: impl Into<immich_models::id::ActivityId>) -> DeleteActivityBuilder<'a> {
        DeleteActivityBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// List all activities
    /// Returns a list of activities for the selected asset or album. The activities are returned in sorted order, with the oldest activities appearing first.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `albumId` | `uuid::Uuid` | ✓ | Album ID |
    /// | `assetId` | `uuid::Uuid` |  | Asset ID (if activity is for an asset) |
    /// | `level` | `ReactionLevel` |  |  |
    /// | `type` | `ReactionType` |  |  |
    /// | `userId` | `uuid::Uuid` |  | Filter by user ID |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<ActivityResponseDTO>`
    pub fn get_activities(&self, album_id: impl Into<immich_models::id::AlbumId>) -> GetActivitiesBuilder<'a> {
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
    /// Returns the number of likes and comments for a given album or asset in an album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `albumId` | `uuid::Uuid` | ✓ | Album ID |
    /// | `assetId` | `uuid::Uuid` |  | Asset ID (if activity is for an asset) |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `ActivityStatisticsResponseDTO`
    pub fn get_activity_statistics(
        &self,
        album_id: impl Into<immich_models::id::AlbumId>,
    ) -> GetActivityStatisticsBuilder<'a> {
        GetActivityStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            album_id: album_id.into(),
            asset_id: None,
        }
    }
}
