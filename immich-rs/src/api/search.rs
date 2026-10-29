// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `search`.

use immich_models::assets::{AssetResponseDTO, AssetTypeEnum, AssetVisibility, PersonResponseDTO};
use immich_models::search::{
    MetadataSearchDTO, PlacesResponseDTO, RandomSearchDTO, SearchExploreResponseDTO, SearchResponseDTO,
    SearchStatisticsResponseDTO, SearchSuggestionType, SmartSearchDTO, StatisticsSearchDTO,
};

/// Fluent request builder for [`Search::get_assets_by_city`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAssetsByCityBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetAssetsByCityBuilder`].
pub type GetAssetsByCityArgs<'a> = GetAssetsByCityBuilder<'a>;
impl GetAssetsByCityBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<AssetResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "GET",
            path = "/search/cities",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/search/cities")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<Vec<AssetResponseDTO>>("search", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAssetsByCityBuilder<'a> {
    type Output = Result<Vec<AssetResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Search::get_explore_data`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetExploreDataBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetExploreDataBuilder`].
pub type GetExploreDataArgs<'a> = GetExploreDataBuilder<'a>;
impl GetExploreDataBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<SearchExploreResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "GET",
            path = "/search/explore",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/search/explore")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<Vec<SearchExploreResponseDTO>>("search", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetExploreDataBuilder<'a> {
    type Output = Result<Vec<SearchExploreResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Search::get_search_suggestions`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetSearchSuggestionsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "includeNull", skip_serializing_if = "Option::is_none")]
    pub include_null: Option<bool>,
    #[serde(rename = "lensModel", skip_serializing_if = "Option::is_none")]
    pub lens_model: Option<String>,
    #[serde(rename = "make", skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub type_: SearchSuggestionType,
}

/// Type alias for [`GetSearchSuggestionsBuilder`].
pub type GetSearchSuggestionsArgs<'a> = GetSearchSuggestionsBuilder<'a>;
impl GetSearchSuggestionsBuilder<'_> {
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

    /// Filter by country
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Sets or clears `country`.
    pub fn set_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }

    /// Include null values in suggestions
    pub fn include_null(mut self, value: bool) -> Self {
        self.include_null = Some(value);
        self
    }

    /// Sets or clears `includeNull`.
    pub fn set_include_null(mut self, value: Option<bool>) -> Self {
        self.include_null = value;
        self
    }

    /// Filter by lens model
    pub fn lens_model(mut self, value: impl Into<String>) -> Self {
        self.lens_model = Some(value.into());
        self
    }

    /// Sets or clears `lensModel`.
    pub fn set_lens_model(mut self, value: Option<String>) -> Self {
        self.lens_model = value;
        self
    }

    /// Filter by camera make
    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }

    /// Sets or clears `make`.
    pub fn set_make(mut self, value: Option<String>) -> Self {
        self.make = value;
        self
    }

    /// Filter by camera model
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Sets or clears `model`.
    pub fn set_model(mut self, value: Option<String>) -> Self {
        self.model = value;
        self
    }

    /// Filter by state/province
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Sets or clears `state`.
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<String>, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "GET",
            path = "/search/suggestions",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/search/suggestions")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self.client.execute::<Vec<String>>("search", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetSearchSuggestionsBuilder<'a> {
    type Output = Result<Vec<String>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Search::search_asset_statistics`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchAssetStatisticsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a StatisticsSearchDTO,
}

/// Type alias for [`SearchAssetStatisticsBuilder`].
pub type SearchAssetStatisticsArgs<'a> = SearchAssetStatisticsBuilder<'a>;
impl SearchAssetStatisticsBuilder<'_> {
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
    pub async fn send(self) -> Result<SearchStatisticsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "POST",
            path = "/search/statistics",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/search/statistics")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<SearchStatisticsResponseDTO>("search", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchAssetStatisticsBuilder<'a> {
    type Output = Result<SearchStatisticsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Search::search_assets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip)]
    pub body: &'a MetadataSearchDTO,
}

/// Type alias for [`SearchAssetsBuilder`].
pub type SearchAssetsArgs<'a> = SearchAssetsBuilder<'a>;
impl<'a> SearchAssetsBuilder<'a> {
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

    /// Creates a paginator for this request, providing streams over pages or individual items.
    #[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
    pub fn into_paginator(self) -> SearchAssetsPaginator<'a> {
        SearchAssetsPaginator { builder: self }
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<SearchResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "POST",
            path = "/search/metadata",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/search/metadata")?;
        request = self.request_config.apply(request);
        request = request.query(&self);
        request = request.json(self.body);

        let response = self.client.execute::<SearchResponseDTO>("search", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchAssetsBuilder<'a> {
    type Output = Result<SearchResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Paginator for [`SearchAssetsBuilder`].
#[derive(Debug, Clone)]
#[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
pub struct SearchAssetsPaginator<'a> {
    builder: SearchAssetsBuilder<'a>,
}

impl<'a> SearchAssetsPaginator<'a> {
    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding pages of items.
    #[must_use = "streams do nothing unless polled"]
    pub fn pages(self) -> impl futures_core::Stream<Item = Result<Vec<AssetResponseDTO>, crate::Error>> + 'a {
        let client = self.builder.client;
        let mut body = (*self.builder.body).clone();
        async_stream::try_stream! {
            loop { let res = client.search().search_assets(& body).send(). await ?; let
            assets = res.assets; let is_empty = assets.items.is_empty(); let next =
            assets.next_page.as_deref().and_then(| p | p.parse:: < u64 > ().ok())
            .and_then(std::num::NonZeroU64::new); yield assets.items; match next {
            Some(page) if ! is_empty => body.page = Some(page), _ => break, } }
        }
    }

    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding individual items, flattened across pages.
    #[must_use = "streams do nothing unless polled"]
    pub fn items(self) -> impl futures_core::Stream<Item = Result<AssetResponseDTO, crate::Error>> + 'a {
        use futures_util::StreamExt;
        self.pages().flat_map(|page_res| match page_res {
            Ok(items) => futures_util::stream::iter(items.into_iter().map(Ok)).left_stream(),
            Err(err) => futures_util::stream::once(async move { Err(err) }).right_stream(),
        })
    }
}

/// Fluent request builder for [`Search::search_large_assets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchLargeAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub album_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "createdAfter", skip_serializing_if = "Option::is_none")]
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "createdBefore", skip_serializing_if = "Option::is_none")]
    pub created_before: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isEncoded", skip_serializing_if = "Option::is_none")]
    pub is_encoded: Option<bool>,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "isMotion", skip_serializing_if = "Option::is_none")]
    pub is_motion: Option<bool>,
    #[serde(rename = "isNotInAlbum", skip_serializing_if = "Option::is_none")]
    pub is_not_in_album: Option<bool>,
    #[serde(rename = "isOffline", skip_serializing_if = "Option::is_none")]
    pub is_offline: Option<bool>,
    #[serde(rename = "lensModel", skip_serializing_if = "Option::is_none")]
    pub lens_model: Option<String>,
    #[serde(rename = "libraryId", skip_serializing_if = "Option::is_none")]
    pub library_id: Option<immich_models::id::LibraryId>,
    #[serde(rename = "make", skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(rename = "minFileSize", skip_serializing_if = "Option::is_none")]
    pub min_file_size: Option<i64>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "ocr", skip_serializing_if = "Option::is_none")]
    pub ocr: Option<String>,
    #[serde(skip)]
    pub person_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "rating", skip_serializing_if = "Option::is_none")]
    pub rating: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip)]
    pub tag_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "takenAfter", skip_serializing_if = "Option::is_none")]
    pub taken_after: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "takenBefore", skip_serializing_if = "Option::is_none")]
    pub taken_before: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "trashedAfter", skip_serializing_if = "Option::is_none")]
    pub trashed_after: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "trashedBefore", skip_serializing_if = "Option::is_none")]
    pub trashed_before: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<AssetTypeEnum>,
    #[serde(rename = "updatedAfter", skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedBefore", skip_serializing_if = "Option::is_none")]
    pub updated_before: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AssetVisibility>,
    #[serde(rename = "withDeleted", skip_serializing_if = "Option::is_none")]
    pub with_deleted: Option<bool>,
    #[serde(rename = "withExif", skip_serializing_if = "Option::is_none")]
    pub with_exif: Option<bool>,
}

/// Type alias for [`SearchLargeAssetsBuilder`].
pub type SearchLargeAssetsArgs<'a> = SearchLargeAssetsBuilder<'a>;
impl SearchLargeAssetsBuilder<'_> {
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

    /// Filter by album IDs
    pub fn album_ids(mut self, value: Vec<uuid::Uuid>) -> Self {
        self.album_ids = Some(value);
        self
    }

    /// Sets or clears `albumIds`.
    pub fn set_album_ids(mut self, value: Option<Vec<uuid::Uuid>>) -> Self {
        self.album_ids = value;
        self
    }

    /// Filter by city name
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    /// Sets or clears `city`.
    pub fn set_city(mut self, value: Option<String>) -> Self {
        self.city = value;
        self
    }

    /// Filter by country name
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Sets or clears `country`.
    pub fn set_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }

    /// Filter by creation date (after)
    pub fn created_after(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_after = Some(value);
        self
    }

    /// Sets or clears `createdAfter`.
    pub fn set_created_after(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.created_after = value;
        self
    }

    /// Filter by creation date (before)
    pub fn created_before(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_before = Some(value);
        self
    }

    /// Sets or clears `createdBefore`.
    pub fn set_created_before(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.created_before = value;
        self
    }

    /// Filter by encoded status
    pub fn is_encoded(mut self, value: bool) -> Self {
        self.is_encoded = Some(value);
        self
    }

    /// Sets or clears `isEncoded`.
    pub fn set_is_encoded(mut self, value: Option<bool>) -> Self {
        self.is_encoded = value;
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

    /// Filter by motion photo status
    pub fn is_motion(mut self, value: bool) -> Self {
        self.is_motion = Some(value);
        self
    }

    /// Sets or clears `isMotion`.
    pub fn set_is_motion(mut self, value: Option<bool>) -> Self {
        self.is_motion = value;
        self
    }

    /// Filter assets not in any album
    pub fn is_not_in_album(mut self, value: bool) -> Self {
        self.is_not_in_album = Some(value);
        self
    }

    /// Sets or clears `isNotInAlbum`.
    pub fn set_is_not_in_album(mut self, value: Option<bool>) -> Self {
        self.is_not_in_album = value;
        self
    }

    /// Filter by offline status
    pub fn is_offline(mut self, value: bool) -> Self {
        self.is_offline = Some(value);
        self
    }

    /// Sets or clears `isOffline`.
    pub fn set_is_offline(mut self, value: Option<bool>) -> Self {
        self.is_offline = value;
        self
    }

    /// Filter by lens model
    pub fn lens_model(mut self, value: impl Into<String>) -> Self {
        self.lens_model = Some(value.into());
        self
    }

    /// Sets or clears `lensModel`.
    pub fn set_lens_model(mut self, value: Option<String>) -> Self {
        self.lens_model = value;
        self
    }

    /// Library ID to filter by
    pub fn library_id(mut self, value: impl Into<immich_models::id::LibraryId>) -> Self {
        self.library_id = Some(value.into());
        self
    }

    /// Sets or clears `libraryId`.
    pub fn set_library_id(mut self, value: Option<immich_models::id::LibraryId>) -> Self {
        self.library_id = value;
        self
    }

    /// Filter by camera make
    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }

    /// Sets or clears `make`.
    pub fn set_make(mut self, value: Option<String>) -> Self {
        self.make = value;
        self
    }

    /// Minimum file size in bytes
    pub fn min_file_size(mut self, value: i64) -> Self {
        self.min_file_size = Some(value);
        self
    }

    /// Sets or clears `minFileSize`.
    pub fn set_min_file_size(mut self, value: Option<i64>) -> Self {
        self.min_file_size = value;
        self
    }

    /// Filter by camera model
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Sets or clears `model`.
    pub fn set_model(mut self, value: Option<String>) -> Self {
        self.model = value;
        self
    }

    /// Filter by OCR text content
    pub fn ocr(mut self, value: impl Into<String>) -> Self {
        self.ocr = Some(value.into());
        self
    }

    /// Sets or clears `ocr`.
    pub fn set_ocr(mut self, value: Option<String>) -> Self {
        self.ocr = value;
        self
    }

    /// Filter by person IDs
    pub fn person_ids(mut self, value: Vec<uuid::Uuid>) -> Self {
        self.person_ids = Some(value);
        self
    }

    /// Sets or clears `personIds`.
    pub fn set_person_ids(mut self, value: Option<Vec<uuid::Uuid>>) -> Self {
        self.person_ids = value;
        self
    }

    /// Filter by rating [1-5], or null for unrated
    pub fn rating(mut self, value: i64) -> Self {
        self.rating = Some(value);
        self
    }

    /// Sets or clears `rating`.
    pub fn set_rating(mut self, value: Option<i64>) -> Self {
        self.rating = value;
        self
    }

    /// Number of results to return
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    /// Sets or clears `size`.
    pub fn set_size(mut self, value: Option<i64>) -> Self {
        self.size = value;
        self
    }

    /// Filter by state/province name
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Sets or clears `state`.
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }

    /// Filter by tag IDs
    pub fn tag_ids(mut self, value: Vec<uuid::Uuid>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    /// Sets or clears `tagIds`.
    pub fn set_tag_ids(mut self, value: Option<Vec<uuid::Uuid>>) -> Self {
        self.tag_ids = value;
        self
    }

    /// Filter by taken date (after)
    pub fn taken_after(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.taken_after = Some(value);
        self
    }

    /// Sets or clears `takenAfter`.
    pub fn set_taken_after(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.taken_after = value;
        self
    }

    /// Filter by taken date (before)
    pub fn taken_before(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.taken_before = Some(value);
        self
    }

    /// Sets or clears `takenBefore`.
    pub fn set_taken_before(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.taken_before = value;
        self
    }

    /// Filter by trash date (after)
    pub fn trashed_after(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.trashed_after = Some(value);
        self
    }

    /// Sets or clears `trashedAfter`.
    pub fn set_trashed_after(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.trashed_after = value;
        self
    }

    /// Filter by trash date (before)
    pub fn trashed_before(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.trashed_before = Some(value);
        self
    }

    /// Sets or clears `trashedBefore`.
    pub fn set_trashed_before(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.trashed_before = value;
        self
    }

    /// Sets `type`.
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = Some(value);
        self
    }

    /// Sets or clears `type`.
    pub fn set_type_(mut self, value: Option<AssetTypeEnum>) -> Self {
        self.type_ = value;
        self
    }

    /// Filter by update date (after)
    pub fn updated_after(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_after = Some(value);
        self
    }

    /// Sets or clears `updatedAfter`.
    pub fn set_updated_after(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.updated_after = value;
        self
    }

    /// Filter by update date (before)
    pub fn updated_before(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_before = Some(value);
        self
    }

    /// Sets or clears `updatedBefore`.
    pub fn set_updated_before(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.updated_before = value;
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

    /// Include deleted assets
    pub fn with_deleted(mut self, value: bool) -> Self {
        self.with_deleted = Some(value);
        self
    }

    /// Sets or clears `withDeleted`.
    pub fn set_with_deleted(mut self, value: Option<bool>) -> Self {
        self.with_deleted = value;
        self
    }

    /// Include EXIF data in response
    pub fn with_exif(mut self, value: bool) -> Self {
        self.with_exif = Some(value);
        self
    }

    /// Sets or clears `withExif`.
    pub fn set_with_exif(mut self, value: Option<bool>) -> Self {
        self.with_exif = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<AssetResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "POST",
            path = "/search/large-assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/search/large-assets")?;
        request = self.request_config.apply(request);
        request = request.query(&self);
        if let Some(items) = &self.album_ids {
            for item in items {
                request = request.query(&[("albumIds", item)]);
            }
        }
        if let Some(items) = &self.person_ids {
            for item in items {
                request = request.query(&[("personIds", item)]);
            }
        }
        if let Some(items) = &self.tag_ids {
            for item in items {
                request = request.query(&[("tagIds", item)]);
            }
        }

        let response = self.client.execute::<Vec<AssetResponseDTO>>("search", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchLargeAssetsBuilder<'a> {
    type Output = Result<Vec<AssetResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Search::search_person`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchPersonBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "withHidden", skip_serializing_if = "Option::is_none")]
    pub with_hidden: Option<bool>,
}

/// Type alias for [`SearchPersonBuilder`].
pub type SearchPersonArgs<'a> = SearchPersonBuilder<'a>;
impl SearchPersonBuilder<'_> {
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

    /// Include hidden people
    pub fn with_hidden(mut self, value: bool) -> Self {
        self.with_hidden = Some(value);
        self
    }

    /// Sets or clears `withHidden`.
    pub fn set_with_hidden(mut self, value: Option<bool>) -> Self {
        self.with_hidden = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<PersonResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "GET",
            path = "/search/person",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/search/person")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self.client.execute::<Vec<PersonResponseDTO>>("search", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchPersonBuilder<'a> {
    type Output = Result<Vec<PersonResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Search::search_places`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchPlacesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "name")]
    pub name: String,
}

/// Type alias for [`SearchPlacesBuilder`].
pub type SearchPlacesArgs<'a> = SearchPlacesBuilder<'a>;
impl SearchPlacesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<PlacesResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "GET",
            path = "/search/places",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/search/places")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self.client.execute::<Vec<PlacesResponseDTO>>("search", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchPlacesBuilder<'a> {
    type Output = Result<Vec<PlacesResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Search::search_random`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchRandomBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a RandomSearchDTO,
}

/// Type alias for [`SearchRandomBuilder`].
pub type SearchRandomArgs<'a> = SearchRandomBuilder<'a>;
impl SearchRandomBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<AssetResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "POST",
            path = "/search/random",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/search/random")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<Vec<AssetResponseDTO>>("search", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchRandomBuilder<'a> {
    type Output = Result<Vec<AssetResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Search::search_smart`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchSmartBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a SmartSearchDTO,
}

/// Type alias for [`SearchSmartBuilder`].
pub type SearchSmartArgs<'a> = SearchSmartBuilder<'a>;
impl<'a> SearchSmartBuilder<'a> {
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

    /// Creates a paginator for this request, providing streams over pages or individual items.
    #[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
    pub fn into_paginator(self) -> SearchSmartPaginator<'a> {
        SearchSmartPaginator { builder: self }
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<SearchResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Search",
            method = "POST",
            path = "/search/smart",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/search/smart")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<SearchResponseDTO>("search", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchSmartBuilder<'a> {
    type Output = Result<SearchResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Paginator for [`SearchSmartBuilder`].
#[derive(Debug, Clone)]
#[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
pub struct SearchSmartPaginator<'a> {
    builder: SearchSmartBuilder<'a>,
}

impl<'a> SearchSmartPaginator<'a> {
    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding pages of items.
    #[must_use = "streams do nothing unless polled"]
    pub fn pages(self) -> impl futures_core::Stream<Item = Result<Vec<AssetResponseDTO>, crate::Error>> + 'a {
        let client = self.builder.client;
        let mut body = (*self.builder.body).clone();
        async_stream::try_stream! {
            loop { let res = client.search().search_smart(& body).send(). await ?; let
            assets = res.assets; let is_empty = assets.items.is_empty(); let next =
            assets.next_page.as_deref().and_then(| p | p.parse:: < u64 > ().ok())
            .and_then(std::num::NonZeroU64::new); yield assets.items; match next {
            Some(page) if ! is_empty => body.page = Some(page), _ => break, } }
        }
    }

    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding individual items, flattened across pages.
    #[must_use = "streams do nothing unless polled"]
    pub fn items(self) -> impl futures_core::Stream<Item = Result<AssetResponseDTO, crate::Error>> + 'a {
        use futures_util::StreamExt;
        self.pages().flat_map(|page_res| match page_res {
            Ok(items) => futures_util::stream::iter(items.into_iter().map(Ok)).left_stream(),
            Err(err) => futures_util::stream::once(async move { Err(err) }).right_stream(),
        })
    }
}

///Client handle for `Search` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Search<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Search<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Retrieve assets by city
    /// Retrieve a list of assets with each asset belonging to a different city. This endpoint is used on the places pages to show a single thumbnail for each city the user has assets in.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetResponseDTO>`
    pub fn get_assets_by_city(&self) -> GetAssetsByCityBuilder<'a> {
        GetAssetsByCityBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve explore data
    /// Retrieve data for the explore section, such as popular people and places.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<SearchExploreResponseDTO>`
    pub fn get_explore_data(&self) -> GetExploreDataBuilder<'a> {
        GetExploreDataBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve search suggestions
    /// Retrieve search suggestions based on partial input. This endpoint is used for typeahead search features.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `country` | `String` |  | Filter by country |
    /// | `includeNull` | `bool` |  | Include null values in suggestions |
    /// | `lensModel` | `String` |  | Filter by lens model |
    /// | `make` | `String` |  | Filter by camera make |
    /// | `model` | `String` |  | Filter by camera model |
    /// | `state` | `String` |  | Filter by state/province |
    /// | `type` | `SearchSuggestionType` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<String>`
    pub fn get_search_suggestions(&self, type_: SearchSuggestionType) -> GetSearchSuggestionsBuilder<'a> {
        GetSearchSuggestionsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            country: None,
            include_null: None,
            lens_model: None,
            make: None,
            model: None,
            state: None,
            type_,
        }
    }

    /// Search asset statistics
    /// Retrieve statistical data about assets based on search criteria, such as the total matching count.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `StatisticsSearchDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `SearchStatisticsResponseDTO`
    pub fn search_asset_statistics(&self, body: &'a StatisticsSearchDTO) -> SearchAssetStatisticsBuilder<'a> {
        SearchAssetStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Search assets by metadata
    /// Search for assets based on various metadata criteria.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    /// | `body` | `MetadataSearchDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `SearchResponseDTO`
    pub fn search_assets(&self, body: &'a MetadataSearchDTO) -> SearchAssetsBuilder<'a> {
        SearchAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
            body,
        }
    }

    /// Search large assets
    /// Search for assets that are considered large based on specified criteria.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `albumIds` | `Vec<uuid::Uuid>` |  | Filter by album IDs |
    /// | `city` | `String` |  | Filter by city name |
    /// | `country` | `String` |  | Filter by country name |
    /// | `createdAfter` | `chrono::DateTime<chrono::Utc>` |  | Filter by creation date (after) |
    /// | `createdBefore` | `chrono::DateTime<chrono::Utc>` |  | Filter by creation date (before) |
    /// | `isEncoded` | `bool` |  | Filter by encoded status |
    /// | `isFavorite` | `bool` |  | Filter by favorite status |
    /// | `isMotion` | `bool` |  | Filter by motion photo status |
    /// | `isNotInAlbum` | `bool` |  | Filter assets not in any album |
    /// | `isOffline` | `bool` |  | Filter by offline status |
    /// | `lensModel` | `String` |  | Filter by lens model |
    /// | `libraryId` | `uuid::Uuid` |  | Library ID to filter by |
    /// | `make` | `String` |  | Filter by camera make |
    /// | `minFileSize` | `i64` |  | Minimum file size in bytes |
    /// | `model` | `String` |  | Filter by camera model |
    /// | `ocr` | `String` |  | Filter by OCR text content |
    /// | `personIds` | `Vec<uuid::Uuid>` |  | Filter by person IDs |
    /// | `rating` | `i64` |  | Filter by rating [1-5], or null for unrated |
    /// | `size` | `i64` |  | Number of results to return |
    /// | `state` | `String` |  | Filter by state/province name |
    /// | `tagIds` | `Vec<uuid::Uuid>` |  | Filter by tag IDs |
    /// | `takenAfter` | `chrono::DateTime<chrono::Utc>` |  | Filter by taken date (after) |
    /// | `takenBefore` | `chrono::DateTime<chrono::Utc>` |  | Filter by taken date (before) |
    /// | `trashedAfter` | `chrono::DateTime<chrono::Utc>` |  | Filter by trash date (after) |
    /// | `trashedBefore` | `chrono::DateTime<chrono::Utc>` |  | Filter by trash date (before) |
    /// | `type` | `AssetTypeEnum` |  |  |
    /// | `updatedAfter` | `chrono::DateTime<chrono::Utc>` |  | Filter by update date (after) |
    /// | `updatedBefore` | `chrono::DateTime<chrono::Utc>` |  | Filter by update date (before) |
    /// | `visibility` | `AssetVisibility` |  |  |
    /// | `withDeleted` | `bool` |  | Include deleted assets |
    /// | `withExif` | `bool` |  | Include EXIF data in response |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetResponseDTO>`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn search_large_assets(&self) -> SearchLargeAssetsBuilder<'a> {
        SearchLargeAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            album_ids: None,
            city: None,
            country: None,
            created_after: None,
            created_before: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_not_in_album: None,
            is_offline: None,
            lens_model: None,
            library_id: None,
            make: None,
            min_file_size: None,
            model: None,
            ocr: None,
            person_ids: None,
            rating: None,
            size: None,
            state: None,
            tag_ids: None,
            taken_after: None,
            taken_before: None,
            trashed_after: None,
            trashed_before: None,
            type_: None,
            updated_after: None,
            updated_before: None,
            visibility: None,
            with_deleted: None,
            with_exif: None,
        }
    }

    /// Search people
    /// Search for people by name.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `name` | `String` | ✓ | Person name to search for |
    /// | `withHidden` | `bool` |  | Include hidden people |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PersonResponseDTO>`
    pub fn search_person(&self, name: impl Into<String>) -> SearchPersonBuilder<'a> {
        SearchPersonBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name: name.into(),
            with_hidden: None,
        }
    }

    /// Search places
    /// Search for places by name.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `name` | `String` | ✓ | Place name to search for |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PlacesResponseDTO>`
    pub fn search_places(&self, name: impl Into<String>) -> SearchPlacesBuilder<'a> {
        SearchPlacesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            name: name.into(),
        }
    }

    /// Search random assets
    /// Retrieve a random selection of assets based on the provided criteria.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `RandomSearchDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetResponseDTO>`
    pub fn search_random(&self, body: &'a RandomSearchDTO) -> SearchRandomBuilder<'a> {
        SearchRandomBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Smart asset search
    /// Perform a smart search for assets by using machine learning vectors to determine relevance.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `SmartSearchDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `SearchResponseDTO`
    pub fn search_smart(&self, body: &'a SmartSearchDTO) -> SearchSmartBuilder<'a> {
        SearchSmartBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
