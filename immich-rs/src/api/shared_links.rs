// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `shared_links`.

use immich_models::id::{AlbumId, SharedLinkId};
use immich_models::shared_links::{
    AssetIdsDTO, AssetIdsResponseDTO, SharedLinkCreateDTO, SharedLinkEditDTO, SharedLinkLoginDTO, SharedLinkResponseDTO,
};

/// Fluent request builder for [`SharedLinks::add_shared_link_assets`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddSharedLinkAssetsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: SharedLinkId,
    body: AssetIdsDTO,
}

impl AddSharedLinkAssetsBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<AssetIdsResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "shared_links",
            method = "PUT",
            path = "/shared-links/{id}/assets",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("shared-links"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("assets"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<Vec<AssetIdsResponseDTO>>("shared_links", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for AddSharedLinkAssetsBuilder<'a> {
    type Output = Result<Vec<AssetIdsResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`SharedLinks::create_shared_link`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateSharedLinkBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: SharedLinkCreateDTO,
}

impl CreateSharedLinkBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        tracing::trace!(
            module = "shared_links",
            method = "POST",
            path = "/shared-links",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/shared-links")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<SharedLinkResponseDTO>("shared_links", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateSharedLinkBuilder<'a> {
    type Output = Result<SharedLinkResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`SharedLinks::get_all_shared_links`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAllSharedLinksBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    album_id: Option<AlbumId>,
    id: Option<SharedLinkId>,
}

impl GetAllSharedLinksBuilder<'_> {
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
    /// Filter by album ID
    pub fn album_id(mut self, value: impl Into<AlbumId>) -> Self {
        self.album_id = Some(value.into());
        self
    }
    /// Sets or clears `albumId`.
    pub fn set_album_id(mut self, value: Option<AlbumId>) -> Self {
        self.album_id = value;
        self
    }
    /// Filter by shared link ID
    pub fn id(mut self, value: impl Into<SharedLinkId>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<SharedLinkId>) -> Self {
        self.id = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<SharedLinkResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "albumId", skip_serializing_if = "Option::is_none")]
            album_id: Option<&'b AlbumId>,
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b SharedLinkId>,
        }
        tracing::trace!(
            module = "shared_links",
            method = "GET",
            path = "/shared-links",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/shared-links")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            album_id: self.album_id.as_ref(),
            id: self.id.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<Vec<SharedLinkResponseDTO>>("shared_links", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAllSharedLinksBuilder<'a> {
    type Output = Result<Vec<SharedLinkResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`SharedLinks::get_my_shared_link`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMySharedLinkBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    key: Option<String>,
    slug: Option<String>,
}

impl GetMySharedLinkBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "shared_links",
            method = "GET",
            path = "/shared-links/me",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/shared-links/me")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        let response = self
            .client
            .execute::<SharedLinkResponseDTO>("shared_links", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetMySharedLinkBuilder<'a> {
    type Output = Result<SharedLinkResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`SharedLinks::get_shared_link_by_id`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetSharedLinkByIdBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: SharedLinkId,
}

impl GetSharedLinkByIdBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        tracing::trace!(
            module = "shared_links",
            method = "GET",
            path = "/shared-links/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("shared-links"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<SharedLinkResponseDTO>("shared_links", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetSharedLinkByIdBuilder<'a> {
    type Output = Result<SharedLinkResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`SharedLinks::remove_shared_link`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveSharedLinkBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: SharedLinkId,
}

impl RemoveSharedLinkBuilder<'_> {
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
            module = "shared_links",
            method = "DELETE",
            path = "/shared-links/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("shared-links"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("shared_links", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for RemoveSharedLinkBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`SharedLinks::remove_shared_link_assets`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveSharedLinkAssetsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: SharedLinkId,
    body: AssetIdsDTO,
}

impl RemoveSharedLinkAssetsBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<AssetIdsResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "shared_links",
            method = "DELETE",
            path = "/shared-links/{id}/assets",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("shared-links"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("assets"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<Vec<AssetIdsResponseDTO>>("shared_links", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for RemoveSharedLinkAssetsBuilder<'a> {
    type Output = Result<Vec<AssetIdsResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`SharedLinks::shared_link_login`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SharedLinkLoginBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    key: Option<String>,
    slug: Option<String>,
    body: SharedLinkLoginDTO,
}

impl SharedLinkLoginBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
            key: Option<&'b String>,
            #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
            slug: Option<&'b String>,
        }
        tracing::trace!(
            module = "shared_links",
            method = "POST",
            path = "/shared-links/login",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/shared-links/login")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            key: self.key.as_ref(),
            slug: self.slug.as_ref(),
        };
        request = request.query(&query);
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<SharedLinkResponseDTO>("shared_links", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SharedLinkLoginBuilder<'a> {
    type Output = Result<SharedLinkResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`SharedLinks::update_shared_link`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateSharedLinkBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: SharedLinkId,
    body: SharedLinkEditDTO,
}

impl UpdateSharedLinkBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        tracing::trace!(
            module = "shared_links",
            method = "PATCH",
            path = "/shared-links/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PATCH,
            &[
                crate::core::PathSegment::Static("shared-links"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self
            .client
            .execute::<SharedLinkResponseDTO>("shared_links", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateSharedLinkBuilder<'a> {
    type Output = Result<SharedLinkResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `SharedLinks` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct SharedLinks<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> SharedLinks<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Add assets to a shared link
    ///
    /// Add assets to a specific shared link by its ID. This endpoint is only relevant for shared link of type individual.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `SharedLinkId` | ✓ |  |
    /// | `body` | `AssetIdsDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetIdsResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn add_shared_link_assets(
        &self,
        id: impl Into<SharedLinkId>,
        body: impl Into<AssetIdsDTO>,
    ) -> AddSharedLinkAssetsBuilder<'a> {
        AddSharedLinkAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Create a shared link
    ///
    /// Create a new shared link.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `SharedLinkCreateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn create_shared_link(&self, body: impl Into<SharedLinkCreateDTO>) -> CreateSharedLinkBuilder<'a> {
        CreateSharedLinkBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Retrieve all shared links
    ///
    /// Retrieve a list of all shared links.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `album_id` | `AlbumId` |  | Filter by album ID |
    /// | `id` | `SharedLinkId` |  | Filter by shared link ID |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<SharedLinkResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_all_shared_links(&self) -> GetAllSharedLinksBuilder<'a> {
        GetAllSharedLinksBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            album_id: None,
            id: None,
        }
    }
    /// Retrieve current shared link
    ///
    /// Retrieve the current shared link associated with authentication method.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_my_shared_link(&self) -> GetMySharedLinkBuilder<'a> {
        GetMySharedLinkBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
        }
    }
    /// Retrieve a shared link
    ///
    /// Retrieve a specific shared link by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `SharedLinkId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_shared_link_by_id(&self, id: impl Into<SharedLinkId>) -> GetSharedLinkByIdBuilder<'a> {
        GetSharedLinkByIdBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Delete a shared link
    ///
    /// Delete a specific shared link by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `SharedLinkId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn remove_shared_link(&self, id: impl Into<SharedLinkId>) -> RemoveSharedLinkBuilder<'a> {
        RemoveSharedLinkBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Remove assets from a shared link
    ///
    /// Remove assets from a specific shared link by its ID. This endpoint is only relevant for shared link of type individual.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `SharedLinkId` | ✓ |  |
    /// | `body` | `AssetIdsDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetIdsResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn remove_shared_link_assets(
        &self,
        id: impl Into<SharedLinkId>,
        body: impl Into<AssetIdsDTO>,
    ) -> RemoveSharedLinkAssetsBuilder<'a> {
        RemoveSharedLinkAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Shared link login
    ///
    /// Login to a password protected shared link
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    /// | `body` | `SharedLinkLoginDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn shared_link_login(&self, body: impl Into<SharedLinkLoginDTO>) -> SharedLinkLoginBuilder<'a> {
        SharedLinkLoginBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
            body: body.into(),
        }
    }
    /// Update a shared link
    ///
    /// Update an existing shared link by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `SharedLinkId` | ✓ |  |
    /// | `body` | `SharedLinkEditDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_shared_link(
        &self,
        id: impl Into<SharedLinkId>,
        body: impl Into<SharedLinkEditDTO>,
    ) -> UpdateSharedLinkBuilder<'a> {
        UpdateSharedLinkBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
}
