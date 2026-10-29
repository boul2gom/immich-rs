// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `timeline`.

use immich_models::albums::AssetOrder;
use immich_models::assets::AssetVisibility;
use immich_models::timeline::{AssetOrderBy, TimeBucketAssetResponseDTO, TimeBucketsResponseDTO};

/// Fluent request builder for [`Timeline::get_time_bucket`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetTimeBucketBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "albumId", skip_serializing_if = "Option::is_none")]
    pub album_id: Option<immich_models::id::AlbumId>,
    #[serde(rename = "bbox", skip_serializing_if = "Option::is_none")]
    pub bbox: Option<String>,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
    pub is_trashed: Option<bool>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<AssetOrder>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<AssetOrderBy>,
    #[serde(rename = "personId", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<immich_models::id::PersonId>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "tagId", skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<immich_models::id::TagId>,
    #[serde(rename = "timeBucket")]
    pub time_bucket: String,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<immich_models::id::UserId>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AssetVisibility>,
    #[serde(rename = "withCoordinates", skip_serializing_if = "Option::is_none")]
    pub with_coordinates: Option<bool>,
    #[serde(rename = "withPartners", skip_serializing_if = "Option::is_none")]
    pub with_partners: Option<bool>,
    #[serde(rename = "withStacked", skip_serializing_if = "Option::is_none")]
    pub with_stacked: Option<bool>,
}

/// Type alias for [`GetTimeBucketBuilder`].
pub type GetTimeBucketArgs<'a> = GetTimeBucketBuilder<'a>;
impl GetTimeBucketBuilder<'_> {
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

    /// Filter assets belonging to a specific album
    pub fn album_id(mut self, value: impl Into<immich_models::id::AlbumId>) -> Self {
        self.album_id = Some(value.into());
        self
    }

    /// Sets or clears `albumId`.
    pub fn set_album_id(mut self, value: Option<immich_models::id::AlbumId>) -> Self {
        self.album_id = value;
        self
    }

    /// Bounding box coordinates as west,south,east,north (WGS84)
    pub fn bbox(mut self, value: impl Into<String>) -> Self {
        self.bbox = Some(value.into());
        self
    }

    /// Sets or clears `bbox`.
    pub fn set_bbox(mut self, value: Option<String>) -> Self {
        self.bbox = value;
        self
    }

    /// Filter by favorite status (true for favorites only, false for non-favorites only)
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    /// Sets or clears `isFavorite`.
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }

    /// Filter by trash status (true for trashed assets only, false for non-trashed only)
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = Some(value);
        self
    }

    /// Sets or clears `isTrashed`.
    pub fn set_is_trashed(mut self, value: Option<bool>) -> Self {
        self.is_trashed = value;
        self
    }

    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }

    /// Sort order for assets within time buckets (ASC for oldest first, DESC for newest first)
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = Some(value);
        self
    }

    /// Sets or clears `order`.
    pub fn set_order(mut self, value: Option<AssetOrder>) -> Self {
        self.order = value;
        self
    }

    /// Date to group and order assets by (takenAt for date taken, createdAt for date added to Immich)
    pub fn order_by(mut self, value: AssetOrderBy) -> Self {
        self.order_by = Some(value);
        self
    }

    /// Sets or clears `orderBy`.
    pub fn set_order_by(mut self, value: Option<AssetOrderBy>) -> Self {
        self.order_by = value;
        self
    }

    /// Filter assets containing a specific person (face recognition)
    pub fn person_id(mut self, value: impl Into<immich_models::id::PersonId>) -> Self {
        self.person_id = Some(value.into());
        self
    }

    /// Sets or clears `personId`.
    pub fn set_person_id(mut self, value: Option<immich_models::id::PersonId>) -> Self {
        self.person_id = value;
        self
    }

    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }

    /// Filter assets with a specific tag
    pub fn tag_id(mut self, value: impl Into<immich_models::id::TagId>) -> Self {
        self.tag_id = Some(value.into());
        self
    }

    /// Sets or clears `tagId`.
    pub fn set_tag_id(mut self, value: Option<immich_models::id::TagId>) -> Self {
        self.tag_id = value;
        self
    }

    /// Filter assets by specific user ID
    pub fn user_id(mut self, value: impl Into<immich_models::id::UserId>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Sets or clears `userId`.
    pub fn set_user_id(mut self, value: Option<immich_models::id::UserId>) -> Self {
        self.user_id = value;
        self
    }

    /// Filter by asset visibility status (ARCHIVE, TIMELINE, HIDDEN, LOCKED)
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Sets or clears `visibility`.
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }

    /// Include location data in the response
    pub fn with_coordinates(mut self, value: bool) -> Self {
        self.with_coordinates = Some(value);
        self
    }

    /// Sets or clears `withCoordinates`.
    pub fn set_with_coordinates(mut self, value: Option<bool>) -> Self {
        self.with_coordinates = value;
        self
    }

    /// Include assets shared by partners
    pub fn with_partners(mut self, value: bool) -> Self {
        self.with_partners = Some(value);
        self
    }

    /// Sets or clears `withPartners`.
    pub fn set_with_partners(mut self, value: Option<bool>) -> Self {
        self.with_partners = value;
        self
    }

    /// Include stacked assets in the response. When true, only primary assets from stacks are returned.
    pub fn with_stacked(mut self, value: bool) -> Self {
        self.with_stacked = Some(value);
        self
    }

    /// Sets or clears `withStacked`.
    pub fn set_with_stacked(mut self, value: Option<bool>) -> Self {
        self.with_stacked = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<TimeBucketAssetResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Timeline",
            method = "GET",
            path = "/timeline/bucket",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/timeline/bucket")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<TimeBucketAssetResponseDTO>("timeline", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetTimeBucketBuilder<'a> {
    type Output = Result<TimeBucketAssetResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Timeline::get_time_buckets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetTimeBucketsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "albumId", skip_serializing_if = "Option::is_none")]
    pub album_id: Option<immich_models::id::AlbumId>,
    #[serde(rename = "bbox", skip_serializing_if = "Option::is_none")]
    pub bbox: Option<String>,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
    pub is_trashed: Option<bool>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<AssetOrder>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<AssetOrderBy>,
    #[serde(rename = "personId", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<immich_models::id::PersonId>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "tagId", skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<immich_models::id::TagId>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<immich_models::id::UserId>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AssetVisibility>,
    #[serde(rename = "withCoordinates", skip_serializing_if = "Option::is_none")]
    pub with_coordinates: Option<bool>,
    #[serde(rename = "withPartners", skip_serializing_if = "Option::is_none")]
    pub with_partners: Option<bool>,
    #[serde(rename = "withStacked", skip_serializing_if = "Option::is_none")]
    pub with_stacked: Option<bool>,
}

/// Type alias for [`GetTimeBucketsBuilder`].
pub type GetTimeBucketsArgs<'a> = GetTimeBucketsBuilder<'a>;
impl GetTimeBucketsBuilder<'_> {
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

    /// Filter assets belonging to a specific album
    pub fn album_id(mut self, value: impl Into<immich_models::id::AlbumId>) -> Self {
        self.album_id = Some(value.into());
        self
    }

    /// Sets or clears `albumId`.
    pub fn set_album_id(mut self, value: Option<immich_models::id::AlbumId>) -> Self {
        self.album_id = value;
        self
    }

    /// Bounding box coordinates as west,south,east,north (WGS84)
    pub fn bbox(mut self, value: impl Into<String>) -> Self {
        self.bbox = Some(value.into());
        self
    }

    /// Sets or clears `bbox`.
    pub fn set_bbox(mut self, value: Option<String>) -> Self {
        self.bbox = value;
        self
    }

    /// Filter by favorite status (true for favorites only, false for non-favorites only)
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    /// Sets or clears `isFavorite`.
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }

    /// Filter by trash status (true for trashed assets only, false for non-trashed only)
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = Some(value);
        self
    }

    /// Sets or clears `isTrashed`.
    pub fn set_is_trashed(mut self, value: Option<bool>) -> Self {
        self.is_trashed = value;
        self
    }

    /// Sets `key`.
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Sets or clears `key`.
    pub fn set_key(mut self, value: Option<String>) -> Self {
        self.key = value;
        self
    }

    /// Sort order for assets within time buckets (ASC for oldest first, DESC for newest first)
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = Some(value);
        self
    }

    /// Sets or clears `order`.
    pub fn set_order(mut self, value: Option<AssetOrder>) -> Self {
        self.order = value;
        self
    }

    /// Date to group and order assets by (takenAt for date taken, createdAt for date added to Immich)
    pub fn order_by(mut self, value: AssetOrderBy) -> Self {
        self.order_by = Some(value);
        self
    }

    /// Sets or clears `orderBy`.
    pub fn set_order_by(mut self, value: Option<AssetOrderBy>) -> Self {
        self.order_by = value;
        self
    }

    /// Filter assets containing a specific person (face recognition)
    pub fn person_id(mut self, value: impl Into<immich_models::id::PersonId>) -> Self {
        self.person_id = Some(value.into());
        self
    }

    /// Sets or clears `personId`.
    pub fn set_person_id(mut self, value: Option<immich_models::id::PersonId>) -> Self {
        self.person_id = value;
        self
    }

    /// Sets `slug`.
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Sets or clears `slug`.
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }

    /// Filter assets with a specific tag
    pub fn tag_id(mut self, value: impl Into<immich_models::id::TagId>) -> Self {
        self.tag_id = Some(value.into());
        self
    }

    /// Sets or clears `tagId`.
    pub fn set_tag_id(mut self, value: Option<immich_models::id::TagId>) -> Self {
        self.tag_id = value;
        self
    }

    /// Filter assets by specific user ID
    pub fn user_id(mut self, value: impl Into<immich_models::id::UserId>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Sets or clears `userId`.
    pub fn set_user_id(mut self, value: Option<immich_models::id::UserId>) -> Self {
        self.user_id = value;
        self
    }

    /// Filter by asset visibility status (ARCHIVE, TIMELINE, HIDDEN, LOCKED)
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Sets or clears `visibility`.
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }

    /// Include location data in the response
    pub fn with_coordinates(mut self, value: bool) -> Self {
        self.with_coordinates = Some(value);
        self
    }

    /// Sets or clears `withCoordinates`.
    pub fn set_with_coordinates(mut self, value: Option<bool>) -> Self {
        self.with_coordinates = value;
        self
    }

    /// Include assets shared by partners
    pub fn with_partners(mut self, value: bool) -> Self {
        self.with_partners = Some(value);
        self
    }

    /// Sets or clears `withPartners`.
    pub fn set_with_partners(mut self, value: Option<bool>) -> Self {
        self.with_partners = value;
        self
    }

    /// Include stacked assets in the response. When true, only primary assets from stacks are returned.
    pub fn with_stacked(mut self, value: bool) -> Self {
        self.with_stacked = Some(value);
        self
    }

    /// Sets or clears `withStacked`.
    pub fn set_with_stacked(mut self, value: Option<bool>) -> Self {
        self.with_stacked = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<TimeBucketsResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Timeline",
            method = "GET",
            path = "/timeline/buckets",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/timeline/buckets")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<Vec<TimeBucketsResponseDTO>>("timeline", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetTimeBucketsBuilder<'a> {
    type Output = Result<Vec<TimeBucketsResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Timeline` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Timeline<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Timeline<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Get time bucket
    /// Retrieve a string of all asset ids in a given time bucket.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `albumId` | `uuid::Uuid` |  | Filter assets belonging to a specific album |
    /// | `bbox` | `String` |  | Bounding box coordinates as west,south,east,north (WGS84) |
    /// | `isFavorite` | `bool` |  | Filter by favorite status (true for favorites only, false for non-favorites only) |
    /// | `isTrashed` | `bool` |  | Filter by trash status (true for trashed assets only, false for non-trashed only) |
    /// | `key` | `String` |  |  |
    /// | `order` | `AssetOrder` |  | Sort order for assets within time buckets (ASC for oldest first, DESC for newest first) |
    /// | `orderBy` | `AssetOrderBy` |  | Date to group and order assets by (takenAt for date taken, createdAt for date added to Immich) |
    /// | `personId` | `uuid::Uuid` |  | Filter assets containing a specific person (face recognition) |
    /// | `slug` | `String` |  |  |
    /// | `tagId` | `uuid::Uuid` |  | Filter assets with a specific tag |
    /// | `timeBucket` | `String` | ✓ | Time bucket identifier in YYYY-MM-DDT00:00:00.000Z format |
    /// | `userId` | `uuid::Uuid` |  | Filter assets by specific user ID |
    /// | `visibility` | `AssetVisibility` |  | Filter by asset visibility status (ARCHIVE, TIMELINE, HIDDEN, LOCKED) |
    /// | `withCoordinates` | `bool` |  | Include location data in the response |
    /// | `withPartners` | `bool` |  | Include assets shared by partners |
    /// | `withStacked` | `bool` |  | Include stacked assets in the response. When true, only primary assets from stacks are returned. |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `TimeBucketAssetResponseDTO`
    pub fn get_time_bucket(&self, time_bucket: impl Into<String>) -> GetTimeBucketBuilder<'a> {
        GetTimeBucketBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            album_id: None,
            bbox: None,
            is_favorite: None,
            is_trashed: None,
            key: None,
            order: None,
            order_by: None,
            person_id: None,
            slug: None,
            tag_id: None,
            time_bucket: time_bucket.into(),
            user_id: None,
            visibility: None,
            with_coordinates: None,
            with_partners: None,
            with_stacked: None,
        }
    }

    /// Get time buckets
    /// Retrieve a list of all minimal time buckets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `albumId` | `uuid::Uuid` |  | Filter assets belonging to a specific album |
    /// | `bbox` | `String` |  | Bounding box coordinates as west,south,east,north (WGS84) |
    /// | `isFavorite` | `bool` |  | Filter by favorite status (true for favorites only, false for non-favorites only) |
    /// | `isTrashed` | `bool` |  | Filter by trash status (true for trashed assets only, false for non-trashed only) |
    /// | `key` | `String` |  |  |
    /// | `order` | `AssetOrder` |  | Sort order for assets within time buckets (ASC for oldest first, DESC for newest first) |
    /// | `orderBy` | `AssetOrderBy` |  | Date to group and order assets by (takenAt for date taken, createdAt for date added to Immich) |
    /// | `personId` | `uuid::Uuid` |  | Filter assets containing a specific person (face recognition) |
    /// | `slug` | `String` |  |  |
    /// | `tagId` | `uuid::Uuid` |  | Filter assets with a specific tag |
    /// | `userId` | `uuid::Uuid` |  | Filter assets by specific user ID |
    /// | `visibility` | `AssetVisibility` |  | Filter by asset visibility status (ARCHIVE, TIMELINE, HIDDEN, LOCKED) |
    /// | `withCoordinates` | `bool` |  | Include location data in the response |
    /// | `withPartners` | `bool` |  | Include assets shared by partners |
    /// | `withStacked` | `bool` |  | Include stacked assets in the response. When true, only primary assets from stacks are returned. |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<TimeBucketsResponseDTO>`
    pub fn get_time_buckets(&self) -> GetTimeBucketsBuilder<'a> {
        GetTimeBucketsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            album_id: None,
            bbox: None,
            is_favorite: None,
            is_trashed: None,
            key: None,
            order: None,
            order_by: None,
            person_id: None,
            slug: None,
            tag_id: None,
            user_id: None,
            visibility: None,
            with_coordinates: None,
            with_partners: None,
            with_stacked: None,
        }
    }
}
