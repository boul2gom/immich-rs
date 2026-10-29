// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `albums`.

use immich_models::albums::{
    AddUsersDTO, AlbumResponseDTO, AlbumStatisticsResponseDTO, AlbumsAddAssetsDTO, AlbumsAddAssetsResponseDTO,
    BulkIdResponseDTO, BulkIdsDTO, CreateAlbumDTO, MapMarkerResponseDTO, UpdateAlbumDTO, UpdateAlbumUserDTO,
};

/// Fluent request builder for [`Albums::add_assets_to_album`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddAssetsToAlbumBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AlbumId,
    #[serde(skip)]
    pub body: &'a BulkIdsDTO,
}

/// Type alias for [`AddAssetsToAlbumBuilder`].
pub type AddAssetsToAlbumArgs<'a> = AddAssetsToAlbumBuilder<'a>;
impl AddAssetsToAlbumBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<BulkIdResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Albums",
            method = "PUT",
            path = "/albums/{id}/assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/albums/{}/assets", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddAssetsToAlbumsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AlbumsAddAssetsDTO,
}

/// Type alias for [`AddAssetsToAlbumsBuilder`].
pub type AddAssetsToAlbumsArgs<'a> = AddAssetsToAlbumsBuilder<'a>;
impl AddAssetsToAlbumsBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumsAddAssetsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Albums",
            method = "PUT",
            path = "/albums/assets",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/albums/assets")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddUsersToAlbumBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AlbumId,
    #[serde(skip)]
    pub body: &'a AddUsersDTO,
}

/// Type alias for [`AddUsersToAlbumBuilder`].
pub type AddUsersToAlbumArgs<'a> = AddUsersToAlbumBuilder<'a>;
impl AddUsersToAlbumBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Albums",
            method = "PUT",
            path = "/albums/{id}/users",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/albums/{}/users", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateAlbumBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a CreateAlbumDTO,
}

/// Type alias for [`CreateAlbumBuilder`].
pub type CreateAlbumArgs<'a> = CreateAlbumBuilder<'a>;
impl CreateAlbumBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumResponseDTO, crate::Error> {
        tracing::trace!(module = "Albums", method = "POST", path = "/albums", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/albums")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteAlbumBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AlbumId,
}

/// Type alias for [`DeleteAlbumBuilder`].
pub type DeleteAlbumArgs<'a> = DeleteAlbumBuilder<'a>;
impl DeleteAlbumBuilder<'_> {
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
            module = "Albums",
            method = "DELETE",
            path = "/albums/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/albums/{}", self.id))?;
        request = self.request_config.apply(request);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAlbumInfoBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AlbumId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Type alias for [`GetAlbumInfoBuilder`].
pub type GetAlbumInfoArgs<'a> = GetAlbumInfoBuilder<'a>;
impl GetAlbumInfoBuilder<'_> {
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

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<AlbumResponseDTO, crate::Error> {
        tracing::trace!(module = "Albums", method = "GET", path = "/albums/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/albums/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAlbumMapMarkersBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AlbumId,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Type alias for [`GetAlbumMapMarkersBuilder`].
pub type GetAlbumMapMarkersArgs<'a> = GetAlbumMapMarkersBuilder<'a>;
impl GetAlbumMapMarkersBuilder<'_> {
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

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<MapMarkerResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Albums",
            method = "GET",
            path = "/albums/{id}/map-markers",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/albums/{}/map-markers", self.id))?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAlbumStatisticsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetAlbumStatisticsBuilder`].
pub type GetAlbumStatisticsArgs<'a> = GetAlbumStatisticsBuilder<'a>;
impl GetAlbumStatisticsBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumStatisticsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Albums",
            method = "GET",
            path = "/albums/statistics",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/albums/statistics")?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAllAlbumsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<immich_models::id::AssetId>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<immich_models::id::AlbumId>,
    #[serde(rename = "isOwned", skip_serializing_if = "Option::is_none")]
    pub is_owned: Option<bool>,
    #[serde(rename = "isShared", skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Type alias for [`GetAllAlbumsBuilder`].
pub type GetAllAlbumsArgs<'a> = GetAllAlbumsBuilder<'a>;
impl GetAllAlbumsBuilder<'_> {
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

    /// Filter albums containing this asset ID (ignores other parameters)
    pub fn asset_id(mut self, value: impl Into<immich_models::id::AssetId>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Sets or clears `assetId`.
    pub fn set_asset_id(mut self, value: Option<immich_models::id::AssetId>) -> Self {
        self.asset_id = value;
        self
    }

    /// Album ID
    pub fn id(mut self, value: impl Into<immich_models::id::AlbumId>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<immich_models::id::AlbumId>) -> Self {
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<AlbumResponseDTO>, crate::Error> {
        tracing::trace!(module = "Albums", method = "GET", path = "/albums", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/albums")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveAssetFromAlbumBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AlbumId,
    #[serde(skip)]
    pub body: &'a BulkIdsDTO,
}

/// Type alias for [`RemoveAssetFromAlbumBuilder`].
pub type RemoveAssetFromAlbumArgs<'a> = RemoveAssetFromAlbumBuilder<'a>;
impl RemoveAssetFromAlbumBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<BulkIdResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Albums",
            method = "DELETE",
            path = "/albums/{id}/assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/albums/{}/assets", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveUserFromAlbumBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AlbumId,
    #[serde(skip)]
    pub user_id: String,
}

/// Type alias for [`RemoveUserFromAlbumBuilder`].
pub type RemoveUserFromAlbumArgs<'a> = RemoveUserFromAlbumBuilder<'a>;
impl RemoveUserFromAlbumBuilder<'_> {
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
            module = "Albums",
            method = "DELETE",
            path = "/albums/{id}/user/{userId}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::DELETE,
            &format!("/albums/{}/user/{}", self.id, ::urlencoding::encode(&self.user_id)),
        )?;
        request = self.request_config.apply(request);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAlbumInfoBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AlbumId,
    #[serde(skip)]
    pub body: &'a UpdateAlbumDTO,
}

/// Type alias for [`UpdateAlbumInfoBuilder`].
pub type UpdateAlbumInfoArgs<'a> = UpdateAlbumInfoBuilder<'a>;
impl UpdateAlbumInfoBuilder<'_> {
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
    pub async fn send(self) -> Result<AlbumResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Albums",
            method = "PATCH",
            path = "/albums/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PATCH, &format!("/albums/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAlbumUserBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::AlbumId,
    #[serde(skip)]
    pub user_id: String,
    #[serde(skip)]
    pub body: &'a UpdateAlbumUserDTO,
}

/// Type alias for [`UpdateAlbumUserBuilder`].
pub type UpdateAlbumUserArgs<'a> = UpdateAlbumUserBuilder<'a>;
impl UpdateAlbumUserBuilder<'_> {
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
            module = "Albums",
            method = "PUT",
            path = "/albums/{id}/user/{userId}",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(
            reqwest::Method::PUT,
            &format!("/albums/{}/user/{}", self.id, ::urlencoding::encode(&self.user_id)),
        )?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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

///Client handle for `Albums` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Albums<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Albums<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Add assets to an album
    /// Add multiple assets to a specific album by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `BulkIdsDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<BulkIdResponseDTO>`
    pub fn add_assets_to_album(
        &self,
        id: impl Into<immich_models::id::AlbumId>,
        body: &'a BulkIdsDTO,
    ) -> AddAssetsToAlbumBuilder<'a> {
        AddAssetsToAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Add assets to albums
    /// Send a list of asset IDs and album IDs to add each asset to each album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AlbumsAddAssetsDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumsAddAssetsResponseDTO`
    pub fn add_assets_to_albums(&self, body: &'a AlbumsAddAssetsDTO) -> AddAssetsToAlbumsBuilder<'a> {
        AddAssetsToAlbumsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Share album with users
    /// Share an album with multiple users. Each user can be given a specific role in the album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `AddUsersDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumResponseDTO`
    pub fn add_users_to_album(
        &self,
        id: impl Into<immich_models::id::AlbumId>,
        body: &'a AddUsersDTO,
    ) -> AddUsersToAlbumBuilder<'a> {
        AddUsersToAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Create an album
    /// Create a new album. The album can also be created with initial users and assets.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `CreateAlbumDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `AlbumResponseDTO`
    pub fn create_album(&self, body: &'a CreateAlbumDTO) -> CreateAlbumBuilder<'a> {
        CreateAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete an album
    /// Delete a specific album by its ID. Note the album is initially trashed and then immediately scheduled for deletion, but relies on a background job to complete the process.
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
    pub fn delete_album(&self, id: impl Into<immich_models::id::AlbumId>) -> DeleteAlbumBuilder<'a> {
        DeleteAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve an album
    /// Retrieve information about a specific album by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumResponseDTO`
    pub fn get_album_info(&self, id: impl Into<immich_models::id::AlbumId>) -> GetAlbumInfoBuilder<'a> {
        GetAlbumInfoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }

    /// Retrieve album map markers
    /// Retrieve map marker information for a specific album by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<MapMarkerResponseDTO>`
    pub fn get_album_map_markers(&self, id: impl Into<immich_models::id::AlbumId>) -> GetAlbumMapMarkersBuilder<'a> {
        GetAlbumMapMarkersBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            key: None,
            slug: None,
        }
    }

    /// Retrieve album statistics
    /// Returns statistics about the albums available to the authenticated user.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumStatisticsResponseDTO`
    pub fn get_album_statistics(&self) -> GetAlbumStatisticsBuilder<'a> {
        GetAlbumStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// List all albums
    /// Retrieve a list of albums available to the authenticated user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `assetId` | `uuid::Uuid` |  | Filter albums containing this asset ID (ignores other parameters) |
    /// | `id` | `uuid::Uuid` |  | Album ID |
    /// | `isOwned` | `bool` |  | Filter by ownership: true = only owned, false = only shared-with-me, undefined = no filter |
    /// | `isShared` | `bool` |  | Filter by shared status: true = only shared, false = not shared, undefined = no filter |
    /// | `name` | `String` |  | Album name (exact match) |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AlbumResponseDTO>`
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
    /// Remove multiple assets from a specific album by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `BulkIdsDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<BulkIdResponseDTO>`
    pub fn remove_asset_from_album(
        &self,
        id: impl Into<immich_models::id::AlbumId>,
        body: &'a BulkIdsDTO,
    ) -> RemoveAssetFromAlbumBuilder<'a> {
        RemoveAssetFromAlbumBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Remove user from album
    /// Remove a user from an album. Use an ID of "me" to leave a shared album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ | Album ID |
    /// | `userId` | `String` | ✓ | Album user ID, or "me" to reference the current user. |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn remove_user_from_album(
        &self,
        id: impl Into<immich_models::id::AlbumId>,
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
    /// Update the information of a specific album by its ID. This endpoint can be used to update the album name, description, sort order, etc. However, it is not used to add or remove assets or users from the album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `UpdateAlbumDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AlbumResponseDTO`
    pub fn update_album_info(
        &self,
        id: impl Into<immich_models::id::AlbumId>,
        body: &'a UpdateAlbumDTO,
    ) -> UpdateAlbumInfoBuilder<'a> {
        UpdateAlbumInfoBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Update user role
    /// Change the role for a specific user in a specific album.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ | Album ID |
    /// | `userId` | `String` | ✓ | Album user ID, or "me" to reference the current user. |
    /// | `body` | `UpdateAlbumUserDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn update_album_user(
        &self,
        id: impl Into<immich_models::id::AlbumId>,
        user_id: impl Into<String>,
        body: &'a UpdateAlbumUserDTO,
    ) -> UpdateAlbumUserBuilder<'a> {
        UpdateAlbumUserBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            user_id: user_id.into(),
            body,
        }
    }
}
