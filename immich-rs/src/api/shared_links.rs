// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `shared_links`.

use immich_models::shared_links::{
    AssetIdsDTO, AssetIdsResponseDTO, SharedLinkCreateDTO, SharedLinkEditDTO, SharedLinkLoginDTO, SharedLinkResponseDTO,
};

/// Fluent request builder for [`SharedLinks::add_shared_link_assets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddSharedLinkAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a AssetIdsDTO,
}

/// Type alias for [`AddSharedLinkAssetsBuilder`].
pub type AddSharedLinkAssetsArgs<'a> = AddSharedLinkAssetsBuilder<'a>;
impl AddSharedLinkAssetsBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<AssetIdsResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "SharedLinks",
            method = "PUT",
            path = "/shared-links/{id}/assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/shared-links/{}/assets", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateSharedLinkBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a SharedLinkCreateDTO,
}

/// Type alias for [`CreateSharedLinkBuilder`].
pub type CreateSharedLinkArgs<'a> = CreateSharedLinkBuilder<'a>;
impl CreateSharedLinkBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        tracing::trace!(
            module = "SharedLinks",
            method = "POST",
            path = "/shared-links",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/shared-links")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAllSharedLinksBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "albumId", skip_serializing_if = "Option::is_none")]
    pub album_id: Option<immich_models::id::AlbumId>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
}

/// Type alias for [`GetAllSharedLinksBuilder`].
pub type GetAllSharedLinksArgs<'a> = GetAllSharedLinksBuilder<'a>;
impl GetAllSharedLinksBuilder<'_> {
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

    /// Filter by album ID
    pub fn album_id(mut self, value: impl Into<immich_models::id::AlbumId>) -> Self {
        self.album_id = Some(value.into());
        self
    }

    /// Sets or clears `albumId`.
    pub fn set_album_id(mut self, value: Option<immich_models::id::AlbumId>) -> Self {
        self.album_id = value;
        self
    }

    /// Filter by shared link ID
    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.id = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<SharedLinkResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "SharedLinks",
            method = "GET",
            path = "/shared-links",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/shared-links")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMySharedLinkBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Type alias for [`GetMySharedLinkBuilder`].
pub type GetMySharedLinkArgs<'a> = GetMySharedLinkBuilder<'a>;
impl GetMySharedLinkBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        tracing::trace!(
            module = "SharedLinks",
            method = "GET",
            path = "/shared-links/me",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/shared-links/me")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetSharedLinkByIdBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetSharedLinkByIdBuilder`].
pub type GetSharedLinkByIdArgs<'a> = GetSharedLinkByIdBuilder<'a>;
impl GetSharedLinkByIdBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        tracing::trace!(
            module = "SharedLinks",
            method = "GET",
            path = "/shared-links/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/shared-links/{}", self.id))?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveSharedLinkBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`RemoveSharedLinkBuilder`].
pub type RemoveSharedLinkArgs<'a> = RemoveSharedLinkBuilder<'a>;
impl RemoveSharedLinkBuilder<'_> {
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
            module = "SharedLinks",
            method = "DELETE",
            path = "/shared-links/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/shared-links/{}", self.id))?;
        request = self.request_config.apply(request);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveSharedLinkAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a AssetIdsDTO,
}

/// Type alias for [`RemoveSharedLinkAssetsBuilder`].
pub type RemoveSharedLinkAssetsArgs<'a> = RemoveSharedLinkAssetsBuilder<'a>;
impl RemoveSharedLinkAssetsBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<AssetIdsResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "SharedLinks",
            method = "DELETE",
            path = "/shared-links/{id}/assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/shared-links/{}/assets", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SharedLinkLoginBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip)]
    pub body: &'a SharedLinkLoginDTO,
}

/// Type alias for [`SharedLinkLoginBuilder`].
pub type SharedLinkLoginArgs<'a> = SharedLinkLoginBuilder<'a>;
impl SharedLinkLoginBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        tracing::trace!(
            module = "SharedLinks",
            method = "POST",
            path = "/shared-links/login",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/shared-links/login")?;
        request = self.request_config.apply(request);
        request = request.query(&self);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateSharedLinkBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a SharedLinkEditDTO,
}

/// Type alias for [`UpdateSharedLinkBuilder`].
pub type UpdateSharedLinkArgs<'a> = UpdateSharedLinkBuilder<'a>;
impl UpdateSharedLinkBuilder<'_> {
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
    pub async fn send(self) -> Result<SharedLinkResponseDTO, crate::Error> {
        tracing::trace!(
            module = "SharedLinks",
            method = "PATCH",
            path = "/shared-links/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PATCH, &format!("/shared-links/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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

///Client handle for `SharedLinks` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct SharedLinks<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> SharedLinks<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Add assets to a shared link
    /// Add assets to a specific shared link by its ID. This endpoint is only relevant for shared link of type individual.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `AssetIdsDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetIdsResponseDTO>`
    pub fn add_shared_link_assets(&self, id: uuid::Uuid, body: &'a AssetIdsDTO) -> AddSharedLinkAssetsBuilder<'a> {
        AddSharedLinkAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }

    /// Create a shared link
    /// Create a new shared link.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `SharedLinkCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    pub fn create_shared_link(&self, body: &'a SharedLinkCreateDTO) -> CreateSharedLinkBuilder<'a> {
        CreateSharedLinkBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Retrieve all shared links
    /// Retrieve a list of all shared links.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `albumId` | `uuid::Uuid` |  | Filter by album ID |
    /// | `id` | `uuid::Uuid` |  | Filter by shared link ID |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<SharedLinkResponseDTO>`
    pub fn get_all_shared_links(&self) -> GetAllSharedLinksBuilder<'a> {
        GetAllSharedLinksBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            album_id: None,
            id: None,
        }
    }

    /// Retrieve current shared link
    /// Retrieve the current shared link associated with authentication method.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    pub fn get_my_shared_link(&self) -> GetMySharedLinkBuilder<'a> {
        GetMySharedLinkBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
        }
    }

    /// Retrieve a shared link
    /// Retrieve a specific shared link by its ID.
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
    /// | `200` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    pub fn get_shared_link_by_id(&self, id: uuid::Uuid) -> GetSharedLinkByIdBuilder<'a> {
        GetSharedLinkByIdBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Delete a shared link
    /// Delete a specific shared link by its ID.
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
    pub fn remove_shared_link(&self, id: uuid::Uuid) -> RemoveSharedLinkBuilder<'a> {
        RemoveSharedLinkBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Remove assets from a shared link
    /// Remove assets from a specific shared link by its ID. This endpoint is only relevant for shared link of type individual.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `AssetIdsDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetIdsResponseDTO>`
    pub fn remove_shared_link_assets(
        &self,
        id: uuid::Uuid,
        body: &'a AssetIdsDTO,
    ) -> RemoveSharedLinkAssetsBuilder<'a> {
        RemoveSharedLinkAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }

    /// Shared link login
    /// Login to a password protected shared link
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `key` | `String` |  |  |
    /// | `slug` | `String` |  |  |
    /// | `body` | `SharedLinkLoginDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    pub fn shared_link_login(&self, body: &'a SharedLinkLoginDTO) -> SharedLinkLoginBuilder<'a> {
        SharedLinkLoginBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            key: None,
            slug: None,
            body,
        }
    }

    /// Update a shared link
    /// Update an existing shared link by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `SharedLinkEditDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `SharedLinkResponseDTO`
    pub fn update_shared_link(&self, id: uuid::Uuid, body: &'a SharedLinkEditDTO) -> UpdateSharedLinkBuilder<'a> {
        UpdateSharedLinkBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }
}
