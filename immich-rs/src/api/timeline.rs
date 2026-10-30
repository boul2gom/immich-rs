// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `timeline`.

use immich_models::albums::AssetOrder;
use immich_models::assets::AssetVisibility;
use immich_models::id::{AlbumId, PersonId, TagId, UserId};
use immich_models::timeline::{AssetOrderBy, TimeBucketAssetResponseDTO, TimeBucketsResponseDTO};

/// Fluent request builder for [`Timeline::get_time_bucket`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetTimeBucketBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    album_id: Option<AlbumId>,
    bbox: Option<String>,
    is_favorite: Option<bool>,
    is_trashed: Option<bool>,
    key: Option<String>,
    order: Option<AssetOrder>,
    order_by: Option<AssetOrderBy>,
    person_id: Option<PersonId>,
    slug: Option<String>,
    tag_id: Option<TagId>,
    time_bucket: String,
    user_id: Option<UserId>,
    visibility: Option<AssetVisibility>,
    with_coordinates: Option<bool>,
    with_partners: Option<bool>,
    with_stacked: Option<bool>,
}

impl GetTimeBucketBuilder<'_> {
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
    /// Filter assets belonging to a specific album
    pub fn album_id(mut self, value: impl Into<AlbumId>) -> Self {
        self.album_id = Some(value.into());
        self
    }
    /// Sets or clears `albumId`.
    pub fn set_album_id(mut self, value: Option<AlbumId>) -> Self {
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
    pub fn person_id(mut self, value: impl Into<PersonId>) -> Self {
        self.person_id = Some(value.into());
        self
    }
    /// Sets or clears `personId`.
    pub fn set_person_id(mut self, value: Option<PersonId>) -> Self {
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
    pub fn tag_id(mut self, value: impl Into<TagId>) -> Self {
        self.tag_id = Some(value.into());
        self
    }
    /// Sets or clears `tagId`.
    pub fn set_tag_id(mut self, value: Option<TagId>) -> Self {
        self.tag_id = value;
        self
    }
    /// Filter assets by specific user ID
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = Some(value.into());
        self
    }
    /// Sets or clears `userId`.
    pub fn set_user_id(mut self, value: Option<UserId>) -> Self {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<TimeBucketAssetResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "albumId", skip_serializing_if = "Option::is_none")]
            album_id: Option<&'b AlbumId>,
            #[serde(rename = "bbox", skip_serializing_if = "Option::is_none")]
            bbox: Option<&'b String>,
            #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
            is_favorite: Option<&'b bool>,
            #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
            is_trashed: Option<&'b bool>,
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
            order: Option<&'b AssetOrder>,
            #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
            order_by: Option<&'b AssetOrderBy>,
            #[serde(rename = "personId", skip_serializing_if = "Option::is_none")]
            person_id: Option<&'b PersonId>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
            #[serde(rename = "tagId", skip_serializing_if = "Option::is_none")]
            tag_id: Option<&'b TagId>,
            #[serde(rename = "timeBucket")]
            time_bucket: &'b String,
            #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
            user_id: Option<&'b UserId>,
            #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
            visibility: Option<&'b AssetVisibility>,
            #[serde(rename = "withCoordinates", skip_serializing_if = "Option::is_none")]
            with_coordinates: Option<&'b bool>,
            #[serde(rename = "withPartners", skip_serializing_if = "Option::is_none")]
            with_partners: Option<&'b bool>,
            #[serde(rename = "withStacked", skip_serializing_if = "Option::is_none")]
            with_stacked: Option<&'b bool>,
        }
        tracing::trace!(
            module = "timeline",
            method = "GET",
            path = "/timeline/bucket",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/timeline/bucket")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            album_id: self.album_id.as_ref(),
            bbox: self.bbox.as_ref(),
            is_favorite: self.is_favorite.as_ref(),
            is_trashed: self.is_trashed.as_ref(),
            key: self.key.as_ref(),
            order: self.order.as_ref(),
            order_by: self.order_by.as_ref(),
            person_id: self.person_id.as_ref(),
            slug: self.slug.as_ref(),
            tag_id: self.tag_id.as_ref(),
            time_bucket: &self.time_bucket,
            user_id: self.user_id.as_ref(),
            visibility: self.visibility.as_ref(),
            with_coordinates: self.with_coordinates.as_ref(),
            with_partners: self.with_partners.as_ref(),
            with_stacked: self.with_stacked.as_ref(),
        };
        request = request.query(&query);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetTimeBucketsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    album_id: Option<AlbumId>,
    bbox: Option<String>,
    is_favorite: Option<bool>,
    is_trashed: Option<bool>,
    key: Option<String>,
    order: Option<AssetOrder>,
    order_by: Option<AssetOrderBy>,
    person_id: Option<PersonId>,
    slug: Option<String>,
    tag_id: Option<TagId>,
    user_id: Option<UserId>,
    visibility: Option<AssetVisibility>,
    with_coordinates: Option<bool>,
    with_partners: Option<bool>,
    with_stacked: Option<bool>,
}

impl GetTimeBucketsBuilder<'_> {
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
    /// Filter assets belonging to a specific album
    pub fn album_id(mut self, value: impl Into<AlbumId>) -> Self {
        self.album_id = Some(value.into());
        self
    }
    /// Sets or clears `albumId`.
    pub fn set_album_id(mut self, value: Option<AlbumId>) -> Self {
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
    pub fn person_id(mut self, value: impl Into<PersonId>) -> Self {
        self.person_id = Some(value.into());
        self
    }
    /// Sets or clears `personId`.
    pub fn set_person_id(mut self, value: Option<PersonId>) -> Self {
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
    pub fn tag_id(mut self, value: impl Into<TagId>) -> Self {
        self.tag_id = Some(value.into());
        self
    }
    /// Sets or clears `tagId`.
    pub fn set_tag_id(mut self, value: Option<TagId>) -> Self {
        self.tag_id = value;
        self
    }
    /// Filter assets by specific user ID
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = Some(value.into());
        self
    }
    /// Sets or clears `userId`.
    pub fn set_user_id(mut self, value: Option<UserId>) -> Self {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<TimeBucketsResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "albumId", skip_serializing_if = "Option::is_none")]
            album_id: Option<&'b AlbumId>,
            #[serde(rename = "bbox", skip_serializing_if = "Option::is_none")]
            bbox: Option<&'b String>,
            #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
            is_favorite: Option<&'b bool>,
            #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
            is_trashed: Option<&'b bool>,
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
            order: Option<&'b AssetOrder>,
            #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
            order_by: Option<&'b AssetOrderBy>,
            #[serde(rename = "personId", skip_serializing_if = "Option::is_none")]
            person_id: Option<&'b PersonId>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
            #[serde(rename = "tagId", skip_serializing_if = "Option::is_none")]
            tag_id: Option<&'b TagId>,
            #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
            user_id: Option<&'b UserId>,
            #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
            visibility: Option<&'b AssetVisibility>,
            #[serde(rename = "withCoordinates", skip_serializing_if = "Option::is_none")]
            with_coordinates: Option<&'b bool>,
            #[serde(rename = "withPartners", skip_serializing_if = "Option::is_none")]
            with_partners: Option<&'b bool>,
            #[serde(rename = "withStacked", skip_serializing_if = "Option::is_none")]
            with_stacked: Option<&'b bool>,
        }
        tracing::trace!(
            module = "timeline",
            method = "GET",
            path = "/timeline/buckets",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/timeline/buckets")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            album_id: self.album_id.as_ref(),
            bbox: self.bbox.as_ref(),
            is_favorite: self.is_favorite.as_ref(),
            is_trashed: self.is_trashed.as_ref(),
            key: self.key.as_ref(),
            order: self.order.as_ref(),
            order_by: self.order_by.as_ref(),
            person_id: self.person_id.as_ref(),
            slug: self.slug.as_ref(),
            tag_id: self.tag_id.as_ref(),
            user_id: self.user_id.as_ref(),
            visibility: self.visibility.as_ref(),
            with_coordinates: self.with_coordinates.as_ref(),
            with_partners: self.with_partners.as_ref(),
            with_stacked: self.with_stacked.as_ref(),
        };
        request = request.query(&query);
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

/// Client handle for `Timeline` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Timeline<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Timeline<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Get time bucket
    ///
    /// Retrieve a string of all asset ids in a given time bucket.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `album_id` | `AlbumId` |  | Filter assets belonging to a specific album |
    /// | `bbox` | `String` |  | Bounding box coordinates as west,south,east,north (WGS84) |
    /// | `is_favorite` | `bool` |  | Filter by favorite status (true for favorites only, false for non-favorites only) |
    /// | `is_trashed` | `bool` |  | Filter by trash status (true for trashed assets only, false for non-trashed only) |
    /// | `key` | `String` |  |  |
    /// | `order` | `AssetOrder` |  | Sort order for assets within time buckets (ASC for oldest first, DESC for newest first) |
    /// | `order_by` | `AssetOrderBy` |  | Date to group and order assets by (takenAt for date taken, createdAt for date added to Immich) |
    /// | `person_id` | `PersonId` |  | Filter assets containing a specific person (face recognition) |
    /// | `slug` | `String` |  |  |
    /// | `tag_id` | `TagId` |  | Filter assets with a specific tag |
    /// | `time_bucket` | `String` | ✓ | Time bucket identifier in YYYY-MM-DDT00:00:00.000Z format |
    /// | `user_id` | `UserId` |  | Filter assets by specific user ID |
    /// | `visibility` | `AssetVisibility` |  | Filter by asset visibility status (ARCHIVE, TIMELINE, HIDDEN, LOCKED) |
    /// | `with_coordinates` | `bool` |  | Include location data in the response |
    /// | `with_partners` | `bool` |  | Include assets shared by partners |
    /// | `with_stacked` | `bool` |  | Include stacked assets in the response. When true, only primary assets from stacks are returned. |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `TimeBucketAssetResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
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
    ///
    /// Retrieve a list of all minimal time buckets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `album_id` | `AlbumId` |  | Filter assets belonging to a specific album |
    /// | `bbox` | `String` |  | Bounding box coordinates as west,south,east,north (WGS84) |
    /// | `is_favorite` | `bool` |  | Filter by favorite status (true for favorites only, false for non-favorites only) |
    /// | `is_trashed` | `bool` |  | Filter by trash status (true for trashed assets only, false for non-trashed only) |
    /// | `key` | `String` |  |  |
    /// | `order` | `AssetOrder` |  | Sort order for assets within time buckets (ASC for oldest first, DESC for newest first) |
    /// | `order_by` | `AssetOrderBy` |  | Date to group and order assets by (takenAt for date taken, createdAt for date added to Immich) |
    /// | `person_id` | `PersonId` |  | Filter assets containing a specific person (face recognition) |
    /// | `slug` | `String` |  |  |
    /// | `tag_id` | `TagId` |  | Filter assets with a specific tag |
    /// | `user_id` | `UserId` |  | Filter assets by specific user ID |
    /// | `visibility` | `AssetVisibility` |  | Filter by asset visibility status (ARCHIVE, TIMELINE, HIDDEN, LOCKED) |
    /// | `with_coordinates` | `bool` |  | Include location data in the response |
    /// | `with_partners` | `bool` |  | Include assets shared by partners |
    /// | `with_stacked` | `bool` |  | Include stacked assets in the response. When true, only primary assets from stacks are returned. |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<TimeBucketsResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
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
