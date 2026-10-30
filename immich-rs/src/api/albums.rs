// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `albums`.

use immich_models::albums::{
    AddUsersDTO, AlbumResponseDTO, AlbumStatisticsResponseDTO, AlbumsAddAssetsDTO, AlbumsAddAssetsResponseDTO,
    BulkIdResponseDTO, BulkIdsDTO, CreateAlbumDTO, MapMarkerResponseDTO, UpdateAlbumDTO, UpdateAlbumUserDTO,
};
use immich_models::id::{AlbumId, AssetId};

/// Fluent request builder for [`Albums::add_assets_to_album`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddAssetsToAlbumBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AlbumId,
    body: BulkIdsDTO,
}

impl AddAssetsToAlbumBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<BulkIdResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "albums",
            method = "PUT",
            path = "/albums/{id}/assets",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("albums"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("assets"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<Vec<BulkIdResponseDTO>>("albums", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for AddAssetsToAlbumBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::add_assets_to_albums`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddAssetsToAlbumsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: AlbumsAddAssetsDTO,
}

impl AddAssetsToAlbumsBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumsAddAssetsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "albums",
            method = "PUT",
            path = "/albums/assets",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/albums/assets")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<AlbumsAddAssetsResponseDTO>("albums", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for AddAssetsToAlbumsBuilder<'a> {
    type Output = Result<AlbumsAddAssetsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::add_users_to_album`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddUsersToAlbumBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AlbumId,
    body: AddUsersDTO,
}

impl AddUsersToAlbumBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumResponseDTO, crate::Error> {
        tracing::trace!(
            module = "albums",
            method = "PUT",
            path = "/albums/{id}/users",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("albums"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("users"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<AlbumResponseDTO>("albums", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for AddUsersToAlbumBuilder<'a> {
    type Output = Result<AlbumResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::create_album`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateAlbumBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: CreateAlbumDTO,
}

impl CreateAlbumBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumResponseDTO, crate::Error> {
        tracing::trace!(module = "albums", method = "POST", path = "/albums", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/albums")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<AlbumResponseDTO>("albums", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateAlbumBuilder<'a> {
    type Output = Result<AlbumResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::delete_album`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteAlbumBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AlbumId,
}

impl DeleteAlbumBuilder<'_> {
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
            module = "albums",
            method = "DELETE",
            path = "/albums/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("albums"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("albums", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteAlbumBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::get_album_info`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAlbumInfoBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AlbumId,
    key: Option<String>,
    slug: Option<String>,
}

impl GetAlbumInfoBuilder<'_> {
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
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<AlbumResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(module = "albums", method = "GET", path = "/albums/{id}", "HTTP request");
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("albums"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let response = self.client.execute::<AlbumResponseDTO>("albums", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAlbumInfoBuilder<'a> {
    type Output = Result<AlbumResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::get_album_map_markers`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAlbumMapMarkersBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AlbumId,
    key: Option<String>,
    slug: Option<String>,
}

impl GetAlbumMapMarkersBuilder<'_> {
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
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<MapMarkerResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "albums",
            method = "GET",
            path = "/albums/{id}/map-markers",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("albums"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("map-markers"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<Vec<MapMarkerResponseDTO>>("albums", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAlbumMapMarkersBuilder<'a> {
    type Output = Result<Vec<MapMarkerResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::get_album_statistics`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAlbumStatisticsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl GetAlbumStatisticsBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumStatisticsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "albums",
            method = "GET",
            path = "/albums/statistics",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/albums/statistics")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<AlbumStatisticsResponseDTO>("albums", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAlbumStatisticsBuilder<'a> {
    type Output = Result<AlbumStatisticsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::get_all_albums`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAllAlbumsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    asset_id: Option<AssetId>,
    id: Option<AlbumId>,
    is_owned: Option<bool>,
    is_shared: Option<bool>,
    name: Option<String>,
}

impl GetAllAlbumsBuilder<'_> {
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
    /// Filter albums containing this asset ID (ignores other parameters)
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = Some(value.into());
        self
    }
    /// Sets or clears `assetId`.
    pub fn set_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.asset_id = value;
        self
    }
    /// Album ID
    pub fn id(mut self, value: impl Into<AlbumId>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<AlbumId>) -> Self {
        self.id = value;
        self
    }
    /// Filter by ownership: true = only owned, false = only shared-with-me, undefined = no filter
    pub fn is_owned(mut self, value: bool) -> Self {
        self.is_owned = Some(value);
        self
    }
    /// Sets or clears `isOwned`.
    pub fn set_is_owned(mut self, value: Option<bool>) -> Self {
        self.is_owned = value;
        self
    }
    /// Filter by shared status: true = only shared, false = not shared, undefined = no filter
    pub fn is_shared(mut self, value: bool) -> Self {
        self.is_shared = Some(value);
        self
    }
    /// Sets or clears `isShared`.
    pub fn set_is_shared(mut self, value: Option<bool>) -> Self {
        self.is_shared = value;
        self
    }
    /// Album name (exact match)
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<AlbumResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
            asset_id: Option<&'b AssetId>,
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b AlbumId>,
            #[serde(rename = "isOwned", skip_serializing_if = "Option::is_none")]
            is_owned: Option<&'b bool>,
            #[serde(rename = "isShared", skip_serializing_if = "Option::is_none")]
            is_shared: Option<&'b bool>,
            #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
            name: Option<&'b String>,
        }
        tracing::trace!(module = "albums", method = "GET", path = "/albums", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/albums")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            asset_id: self.asset_id.as_ref(),
            id: self.id.as_ref(),
            is_owned: self.is_owned.as_ref(),
            is_shared: self.is_shared.as_ref(),
            name: self.name.as_ref(),
        };
        request = request.query(&query);
        let response = self.client.execute::<Vec<AlbumResponseDTO>>("albums", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAllAlbumsBuilder<'a> {
    type Output = Result<Vec<AlbumResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::remove_asset_from_album`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveAssetFromAlbumBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AlbumId,
    body: BulkIdsDTO,
}

impl RemoveAssetFromAlbumBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<BulkIdResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "albums",
            method = "DELETE",
            path = "/albums/{id}/assets",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("albums"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("assets"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<Vec<BulkIdResponseDTO>>("albums", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for RemoveAssetFromAlbumBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::remove_user_from_album`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveUserFromAlbumBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AlbumId,
    user_id: String,
}

impl RemoveUserFromAlbumBuilder<'_> {
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
            module = "albums",
            method = "DELETE",
            path = "/albums/{id}/user/{userId}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("albums"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("user"),
                crate::core::PathSegment::Param(self.user_id.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("albums", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for RemoveUserFromAlbumBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::update_album_info`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAlbumInfoBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AlbumId,
    body: UpdateAlbumDTO,
}

impl UpdateAlbumInfoBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumResponseDTO, crate::Error> {
        tracing::trace!(
            module = "albums",
            method = "PATCH",
            path = "/albums/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PATCH,
            &[
                crate::core::PathSegment::Static("albums"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<AlbumResponseDTO>("albums", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateAlbumInfoBuilder<'a> {
    type Output = Result<AlbumResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Albums::update_album_user`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAlbumUserBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: AlbumId,
    user_id: String,
    body: UpdateAlbumUserDTO,
}

impl UpdateAlbumUserBuilder<'_> {
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
            module = "albums",
            method = "PUT",
            path = "/albums/{id}/user/{userId}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("albums"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("user"),
                crate::core::PathSegment::Param(self.user_id.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("albums", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for UpdateAlbumUserBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Albums` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Albums<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Albums<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Add assets to an album
    ///
    /// Add multiple assets to a specific album by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AlbumId` | ✓ |  |
    /// | `body` | `BulkIdsDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<BulkIdResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn add_assets_to_album(
        &self,
        id: impl Into<AlbumId>,
        body: impl Into<BulkIdsDTO>,
    ) -> AddAssetsToAlbumBuilder<'a> {
        AddAssetsToAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Add assets to albums
    ///
    /// Send a list of asset IDs and album IDs to add each asset to each album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `AlbumsAddAssetsDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumsAddAssetsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn add_assets_to_albums(&self, body: impl Into<AlbumsAddAssetsDTO>) -> AddAssetsToAlbumsBuilder<'a> {
        AddAssetsToAlbumsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Share album with users
    ///
    /// Share an album with multiple users. Each user can be given a specific role in the album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AlbumId` | ✓ |  |
    /// | `body` | `AddUsersDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn add_users_to_album(
        &self,
        id: impl Into<AlbumId>,
        body: impl Into<AddUsersDTO>,
    ) -> AddUsersToAlbumBuilder<'a> {
        AddUsersToAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Create an album
    ///
    /// Create a new album. The album can also be created with initial users and assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `CreateAlbumDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `AlbumResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn create_album(&self, body: impl Into<CreateAlbumDTO>) -> CreateAlbumBuilder<'a> {
        CreateAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete an album
    ///
    /// Delete a specific album by its ID. Note the album is initially trashed and then immediately scheduled for deletion, but relies on a background job to complete the process.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AlbumId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_album(&self, id: impl Into<AlbumId>) -> DeleteAlbumBuilder<'a> {
        DeleteAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve an album
    ///
    /// Retrieve information about a specific album by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AlbumId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_album_info(&self, id: impl Into<AlbumId>) -> GetAlbumInfoBuilder<'a> {
        GetAlbumInfoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }
    /// Retrieve album map markers
    ///
    /// Retrieve map marker information for a specific album by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AlbumId` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
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
    pub fn get_album_map_markers(&self, id: impl Into<AlbumId>) -> GetAlbumMapMarkersBuilder<'a> {
        GetAlbumMapMarkersBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }
    /// Retrieve album statistics
    ///
    /// Returns statistics about the albums available to the authenticated user.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumStatisticsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_album_statistics(&self) -> GetAlbumStatisticsBuilder<'a> {
        GetAlbumStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// List all albums
    ///
    /// Retrieve a list of albums available to the authenticated user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `asset_id` | `AssetId` |  | Filter albums containing this asset ID (ignores other parameters) |
    /// | `id` | `AlbumId` |  | Album ID |
    /// | `is_owned` | `bool` |  | Filter by ownership: true = only owned, false = only shared-with-me, undefined = no filter |
    /// | `is_shared` | `bool` |  | Filter by shared status: true = only shared, false = not shared, undefined = no filter |
    /// | `name` | `String` |  | Album name (exact match) |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AlbumResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_all_albums(&self) -> GetAllAlbumsBuilder<'a> {
        GetAllAlbumsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            asset_id: None,
            id: None,
            is_owned: None,
            is_shared: None,
            name: None,
        }
    }
    /// Remove assets from an album
    ///
    /// Remove multiple assets from a specific album by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AlbumId` | ✓ |  |
    /// | `body` | `BulkIdsDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<BulkIdResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn remove_asset_from_album(
        &self,
        id: impl Into<AlbumId>,
        body: impl Into<BulkIdsDTO>,
    ) -> RemoveAssetFromAlbumBuilder<'a> {
        RemoveAssetFromAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Remove user from album
    ///
    /// Remove a user from an album. Use an ID of "me" to leave a shared album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AlbumId` | ✓ | Album ID |
    /// | `user_id` | `String` | ✓ | Album user ID, or "me" to reference the current user. |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn remove_user_from_album(
        &self,
        id: impl Into<AlbumId>,
        user_id: impl Into<String>,
    ) -> RemoveUserFromAlbumBuilder<'a> {
        RemoveUserFromAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            user_id: user_id.into(),
        }
    }
    /// Update an album
    ///
    /// Update the information of a specific album by its ID. This endpoint can be used to update the album name, description, sort order, etc. However, it is not used to add or remove assets or users from the album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AlbumId` | ✓ |  |
    /// | `body` | `UpdateAlbumDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_album_info(
        &self,
        id: impl Into<AlbumId>,
        body: impl Into<UpdateAlbumDTO>,
    ) -> UpdateAlbumInfoBuilder<'a> {
        UpdateAlbumInfoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Update user role
    ///
    /// Change the role for a specific user in a specific album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `AlbumId` | ✓ | Album ID |
    /// | `user_id` | `String` | ✓ | Album user ID, or "me" to reference the current user. |
    /// | `body` | `UpdateAlbumUserDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_album_user(
        &self,
        id: impl Into<AlbumId>,
        user_id: impl Into<String>,
        body: impl Into<UpdateAlbumUserDTO>,
    ) -> UpdateAlbumUserBuilder<'a> {
        UpdateAlbumUserBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            user_id: user_id.into(),
            body: body.into(),
        }
    }
}
