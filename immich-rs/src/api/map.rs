// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `map`.

use immich_models::albums::MapMarkerResponseDTO;
use immich_models::map::MapReverseGeocodeResponseDTO;

/// Fluent request builder for [`Map::get_map_markers`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMapMarkersBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    file_created_after: Option<chrono::DateTime<chrono::Utc>>,
    file_created_before: Option<chrono::DateTime<chrono::Utc>>,
    is_archived: Option<bool>,
    is_favorite: Option<bool>,
    with_partners: Option<bool>,
    with_shared_albums: Option<bool>,
}

impl GetMapMarkersBuilder<'_> {
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
    /// Filter assets created after this date
    pub fn file_created_after(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.file_created_after = Some(value);
        self
    }
    /// Sets or clears `fileCreatedAfter`.
    pub fn set_file_created_after(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.file_created_after = value;
        self
    }
    /// Filter assets created before this date
    pub fn file_created_before(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.file_created_before = Some(value);
        self
    }
    /// Sets or clears `fileCreatedBefore`.
    pub fn set_file_created_before(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.file_created_before = value;
        self
    }
    /// Filter by archived status
    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }
    /// Sets or clears `isArchived`.
    pub fn set_is_archived(mut self, value: Option<bool>) -> Self {
        self.is_archived = value;
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
    /// Include partner assets
    pub fn with_partners(mut self, value: bool) -> Self {
        self.with_partners = Some(value);
        self
    }
    /// Sets or clears `withPartners`.
    pub fn set_with_partners(mut self, value: Option<bool>) -> Self {
        self.with_partners = value;
        self
    }
    /// Include shared album assets
    pub fn with_shared_albums(mut self, value: bool) -> Self {
        self.with_shared_albums = Some(value);
        self
    }
    /// Sets or clears `withSharedAlbums`.
    pub fn set_with_shared_albums(mut self, value: Option<bool>) -> Self {
        self.with_shared_albums = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<MapMarkerResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "fileCreatedAfter", skip_serializing_if = "Option::is_none")]
            file_created_after: Option<&'b chrono::DateTime<chrono::Utc>>,
            #[serde(rename = "fileCreatedBefore", skip_serializing_if = "Option::is_none")]
            file_created_before: Option<&'b chrono::DateTime<chrono::Utc>>,
            #[serde(rename = "isArchived", skip_serializing_if = "Option::is_none")]
            is_archived: Option<&'b bool>,
            #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
            is_favorite: Option<&'b bool>,
            #[serde(rename = "withPartners", skip_serializing_if = "Option::is_none")]
            with_partners: Option<&'b bool>,
            #[serde(rename = "withSharedAlbums", skip_serializing_if = "Option::is_none")]
            with_shared_albums: Option<&'b bool>,
        }
        tracing::trace!(module = "map", method = "GET", path = "/map/markers", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/map/markers")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            file_created_after: self.file_created_after.as_ref(),
            file_created_before: self.file_created_before.as_ref(),
            is_archived: self.is_archived.as_ref(),
            is_favorite: self.is_favorite.as_ref(),
            with_partners: self.with_partners.as_ref(),
            with_shared_albums: self.with_shared_albums.as_ref(),
        };
        request = request.query(&query);
        let response = self.client.execute::<Vec<MapMarkerResponseDTO>>("map", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetMapMarkersBuilder<'a> {
    type Output = Result<Vec<MapMarkerResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Map::reverse_geocode`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ReverseGeocodeBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    lat: f64,
    lon: f64,
}

impl ReverseGeocodeBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<MapReverseGeocodeResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "lat")]
            lat: &'b f64,
            #[serde(rename = "lon")]
            lon: &'b f64,
        }
        tracing::trace!(
            module = "map",
            method = "GET",
            path = "/map/reverse-geocode",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/map/reverse-geocode")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            lat: &self.lat,
            lon: &self.lon,
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<Vec<MapReverseGeocodeResponseDTO>>("map", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for ReverseGeocodeBuilder<'a> {
    type Output = Result<Vec<MapReverseGeocodeResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Map` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Map<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Map<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Retrieve map markers
    ///
    /// Retrieve a list of latitude and longitude coordinates for every asset with location data.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `file_created_after` | `chrono::DateTime<chrono::Utc>` |  | Filter assets created after this date |
    /// | `file_created_before` | `chrono::DateTime<chrono::Utc>` |  | Filter assets created before this date |
    /// | `is_archived` | `bool` |  | Filter by archived status |
    /// | `is_favorite` | `bool` |  | Filter by favorite status |
    /// | `with_partners` | `bool` |  | Include partner assets |
    /// | `with_shared_albums` | `bool` |  | Include shared album assets |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<MapMarkerResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_map_markers(&self) -> GetMapMarkersBuilder<'a> {
        GetMapMarkersBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            file_created_after: None,
            file_created_before: None,
            is_archived: None,
            is_favorite: None,
            with_partners: None,
            with_shared_albums: None,
        }
    }
    /// Reverse geocode coordinates
    ///
    /// Retrieve location information (e.g., city, country) for given latitude and longitude coordinates.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `lat` | `f64` | ✓ | Latitude (-90 to 90) |
    /// | `lon` | `f64` | ✓ | Longitude (-180 to 180) |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<MapReverseGeocodeResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn reverse_geocode(&self, lat: f64, lon: f64) -> ReverseGeocodeBuilder<'a> {
        ReverseGeocodeBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            lat,
            lon,
        }
    }
}
