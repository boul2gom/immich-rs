// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `tags`.

use immich_models::albums::{BulkIdResponseDTO, BulkIdsDTO};
use immich_models::assets::TagResponseDTO;
use immich_models::tags::{TagBulkAssetsDTO, TagBulkAssetsResponseDTO, TagCreateDTO, TagUpdateDTO, TagUpsertDTO};

/// Fluent request builder for [`Tags::bulk_tag_assets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct BulkTagAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a TagBulkAssetsDTO,
}

/// Type alias for [`BulkTagAssetsBuilder`].
pub type BulkTagAssetsArgs<'a> = BulkTagAssetsBuilder<'a>;
impl BulkTagAssetsBuilder<'_> {
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
    pub async fn send(self) -> Result<TagBulkAssetsResponseDTO, crate::Error> {
        tracing::trace!(module = "Tags", method = "PUT", path = "/tags/assets", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/tags/assets")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<TagBulkAssetsResponseDTO>("tags", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for BulkTagAssetsBuilder<'a> {
    type Output = Result<TagBulkAssetsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Tags::create_tag`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateTagBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a TagCreateDTO,
}

/// Type alias for [`CreateTagBuilder`].
pub type CreateTagArgs<'a> = CreateTagBuilder<'a>;
impl CreateTagBuilder<'_> {
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
    pub async fn send(self) -> Result<TagResponseDTO, crate::Error> {
        tracing::trace!(module = "Tags", method = "POST", path = "/tags", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/tags")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<TagResponseDTO>("tags", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateTagBuilder<'a> {
    type Output = Result<TagResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Tags::delete_tag`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteTagBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::TagId,
}

/// Type alias for [`DeleteTagBuilder`].
pub type DeleteTagArgs<'a> = DeleteTagBuilder<'a>;
impl DeleteTagBuilder<'_> {
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
        tracing::trace!(module = "Tags", method = "DELETE", path = "/tags/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/tags/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("tags", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteTagBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Tags::get_all_tags`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAllTagsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetAllTagsBuilder`].
pub type GetAllTagsArgs<'a> = GetAllTagsBuilder<'a>;
impl GetAllTagsBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<TagResponseDTO>, crate::Error> {
        tracing::trace!(module = "Tags", method = "GET", path = "/tags", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/tags")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<Vec<TagResponseDTO>>("tags", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAllTagsBuilder<'a> {
    type Output = Result<Vec<TagResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Tags::get_tag_by_id`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetTagByIdBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::TagId,
}

/// Type alias for [`GetTagByIdBuilder`].
pub type GetTagByIdArgs<'a> = GetTagByIdBuilder<'a>;
impl GetTagByIdBuilder<'_> {
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
    pub async fn send(self) -> Result<TagResponseDTO, crate::Error> {
        tracing::trace!(module = "Tags", method = "GET", path = "/tags/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/tags/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<TagResponseDTO>("tags", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetTagByIdBuilder<'a> {
    type Output = Result<TagResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Tags::tag_assets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct TagAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::TagId,
    #[serde(skip)]
    pub body: &'a BulkIdsDTO,
}

/// Type alias for [`TagAssetsBuilder`].
pub type TagAssetsArgs<'a> = TagAssetsBuilder<'a>;
impl TagAssetsBuilder<'_> {
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
            module = "Tags",
            method = "PUT",
            path = "/tags/{id}/assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/tags/{}/assets", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<Vec<BulkIdResponseDTO>>("tags", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for TagAssetsBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Tags::untag_assets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UntagAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::TagId,
    #[serde(skip)]
    pub body: &'a BulkIdsDTO,
}

/// Type alias for [`UntagAssetsBuilder`].
pub type UntagAssetsArgs<'a> = UntagAssetsBuilder<'a>;
impl UntagAssetsBuilder<'_> {
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
            module = "Tags",
            method = "DELETE",
            path = "/tags/{id}/assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/tags/{}/assets", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<Vec<BulkIdResponseDTO>>("tags", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UntagAssetsBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Tags::update_tag`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateTagBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::TagId,
    #[serde(skip)]
    pub body: &'a TagUpdateDTO,
}

/// Type alias for [`UpdateTagBuilder`].
pub type UpdateTagArgs<'a> = UpdateTagBuilder<'a>;
impl UpdateTagBuilder<'_> {
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
    pub async fn send(self) -> Result<TagResponseDTO, crate::Error> {
        tracing::trace!(module = "Tags", method = "PUT", path = "/tags/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/tags/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<TagResponseDTO>("tags", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateTagBuilder<'a> {
    type Output = Result<TagResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Tags::upsert_tags`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpsertTagsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a TagUpsertDTO,
}

/// Type alias for [`UpsertTagsBuilder`].
pub type UpsertTagsArgs<'a> = UpsertTagsBuilder<'a>;
impl UpsertTagsBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<TagResponseDTO>, crate::Error> {
        tracing::trace!(module = "Tags", method = "PUT", path = "/tags", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/tags")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<Vec<TagResponseDTO>>("tags", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpsertTagsBuilder<'a> {
    type Output = Result<Vec<TagResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Tags` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Tags<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Tags<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Tag assets
    /// Add multiple tags to multiple assets in a single request.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `TagBulkAssetsDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `TagBulkAssetsResponseDTO`
    pub fn bulk_tag_assets(&self, body: &'a TagBulkAssetsDTO) -> BulkTagAssetsBuilder<'a> {
        BulkTagAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Create a tag
    /// Create a new tag by providing a name and optional color.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `TagCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `TagResponseDTO`
    pub fn create_tag(&self, body: &'a TagCreateDTO) -> CreateTagBuilder<'a> {
        CreateTagBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete a tag
    /// Delete a specific tag by its ID.
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
    pub fn delete_tag(&self, id: impl Into<immich_models::id::TagId>) -> DeleteTagBuilder<'a> {
        DeleteTagBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve tags
    /// Retrieve a list of all tags.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<TagResponseDTO>`
    pub fn get_all_tags(&self) -> GetAllTagsBuilder<'a> {
        GetAllTagsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Retrieve a tag
    /// Retrieve a specific tag by its ID.
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
    /// **Returns:** `TagResponseDTO`
    pub fn get_tag_by_id(&self, id: impl Into<immich_models::id::TagId>) -> GetTagByIdBuilder<'a> {
        GetTagByIdBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Tag assets
    /// Add a tag to all the specified assets.
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
    pub fn tag_assets(&self, id: impl Into<immich_models::id::TagId>, body: &'a BulkIdsDTO) -> TagAssetsBuilder<'a> {
        TagAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Untag assets
    /// Remove a tag from all the specified assets.
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
    pub fn untag_assets(
        &self,
        id: impl Into<immich_models::id::TagId>,
        body: &'a BulkIdsDTO,
    ) -> UntagAssetsBuilder<'a> {
        UntagAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Update a tag
    /// Update an existing tag identified by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `TagUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `TagResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_tag(&self, id: impl Into<immich_models::id::TagId>, body: &'a TagUpdateDTO) -> UpdateTagBuilder<'a> {
        UpdateTagBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Upsert tags
    /// Create or update multiple tags in a single request.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `TagUpsertDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<TagResponseDTO>`
    pub fn upsert_tags(&self, body: &'a TagUpsertDTO) -> UpsertTagsBuilder<'a> {
        UpsertTagsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
