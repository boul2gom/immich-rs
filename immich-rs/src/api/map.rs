// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `map`.

use immich_models::albums::MapMarkerResponseDTO;
use immich_models::map::MapReverseGeocodeResponseDTO;

/// Fluent request builder for [`Map::get_map_markers`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMapMarkersBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "fileCreatedAfter", skip_serializing_if = "Option::is_none")]
    pub file_created_after: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "fileCreatedBefore", skip_serializing_if = "Option::is_none")]
    pub file_created_before: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "isArchived", skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "withPartners", skip_serializing_if = "Option::is_none")]
    pub with_partners: Option<bool>,
    #[serde(rename = "withSharedAlbums", skip_serializing_if = "Option::is_none")]
    pub with_shared_albums: Option<bool>,
}

/// Type alias for [`GetMapMarkersBuilder`].
pub type GetMapMarkersArgs<'a> = GetMapMarkersBuilder<'a>;
impl GetMapMarkersBuilder<'_> {
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<MapMarkerResponseDTO>, crate::Error> {
        tracing::trace!(module = "Map", method = "GET", path = "/map/markers", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/map/markers")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ReverseGeocodeBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "lat")]
    pub lat: f64,
    #[serde(rename = "lon")]
    pub lon: f64,
}

/// Type alias for [`ReverseGeocodeBuilder`].
pub type ReverseGeocodeArgs<'a> = ReverseGeocodeBuilder<'a>;
impl ReverseGeocodeBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<MapReverseGeocodeResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Map",
            method = "GET",
            path = "/map/reverse-geocode",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/map/reverse-geocode")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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

///Client handle for `Map` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Map<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Map<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Retrieve map markers
    /// Retrieve a list of latitude and longitude coordinates for every asset with location data.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `fileCreatedAfter` | `chrono::DateTime<chrono::Utc>` |  | Filter assets created after this date |
    /// | `fileCreatedBefore` | `chrono::DateTime<chrono::Utc>` |  | Filter assets created before this date |
    /// | `isArchived` | `bool` |  | Filter by archived status |
    /// | `isFavorite` | `bool` |  | Filter by favorite status |
    /// | `withPartners` | `bool` |  | Include partner assets |
    /// | `withSharedAlbums` | `bool` |  | Include shared album assets |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<MapMarkerResponseDTO>`
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
    /// Retrieve location information (e.g., city, country) for given latitude and longitude coordinates.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `lat` | `f64` | ✓ | Latitude (-90 to 90) |
    /// | `lon` | `f64` | ✓ | Longitude (-180 to 180) |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<MapReverseGeocodeResponseDTO>`
    pub fn reverse_geocode(&self, lat: f64, lon: f64) -> ReverseGeocodeBuilder<'a> {
        ReverseGeocodeBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            lat,
            lon,
        }
    }
}
