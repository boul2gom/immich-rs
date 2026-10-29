// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `memories`.

use immich_models::albums::{BulkIdResponseDTO, BulkIdsDTO};
use immich_models::memories::{
    MemoryCreateDTO, MemoryResponseDTO, MemorySearchOrder, MemoryStatisticsResponseDTO, MemoryType, MemoryUpdateDTO,
};

/// Fluent request builder for [`Memories::add_memory_assets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddMemoryAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::MemoryId,
    #[serde(skip)]
    pub body: &'a BulkIdsDTO,
}

/// Type alias for [`AddMemoryAssetsBuilder`].
pub type AddMemoryAssetsArgs<'a> = AddMemoryAssetsBuilder<'a>;
impl AddMemoryAssetsBuilder<'_> {
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
            module = "Memories",
            method = "PUT",
            path = "/memories/{id}/assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/memories/{}/assets", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<Vec<BulkIdResponseDTO>>("memories", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for AddMemoryAssetsBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Memories::create_memory`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateMemoryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a MemoryCreateDTO,
}

/// Type alias for [`CreateMemoryBuilder`].
pub type CreateMemoryArgs<'a> = CreateMemoryBuilder<'a>;
impl CreateMemoryBuilder<'_> {
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
    pub async fn send(self) -> Result<MemoryResponseDTO, crate::Error> {
        tracing::trace!(module = "Memories", method = "POST", path = "/memories", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/memories")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<MemoryResponseDTO>("memories", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreateMemoryBuilder<'a> {
    type Output = Result<MemoryResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Memories::delete_memory`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteMemoryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::MemoryId,
}

/// Type alias for [`DeleteMemoryBuilder`].
pub type DeleteMemoryArgs<'a> = DeleteMemoryBuilder<'a>;
impl DeleteMemoryBuilder<'_> {
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
            module = "Memories",
            method = "DELETE",
            path = "/memories/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/memories/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("memories", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteMemoryBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Memories::get_memory`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMemoryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::MemoryId,
}

/// Type alias for [`GetMemoryBuilder`].
pub type GetMemoryArgs<'a> = GetMemoryBuilder<'a>;
impl GetMemoryBuilder<'_> {
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
    pub async fn send(self) -> Result<MemoryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Memories",
            method = "GET",
            path = "/memories/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/memories/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<MemoryResponseDTO>("memories", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetMemoryBuilder<'a> {
    type Output = Result<MemoryResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Memories::memories_statistics`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct MemoriesStatisticsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "for", skip_serializing_if = "Option::is_none")]
    pub for_: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<immich_models::id::MemoryId>,
    #[serde(rename = "isSaved", skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
    pub is_trashed: Option<bool>,
    #[serde(rename = "isUpcoming", skip_serializing_if = "Option::is_none")]
    pub is_upcoming: Option<bool>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<MemorySearchOrder>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<MemoryType>,
}

/// Type alias for [`MemoriesStatisticsBuilder`].
pub type MemoriesStatisticsArgs<'a> = MemoriesStatisticsBuilder<'a>;
impl MemoriesStatisticsBuilder<'_> {
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

    /// Filter by date
    pub fn for_(mut self, value: impl Into<String>) -> Self {
        self.for_ = Some(value.into());
        self
    }

    /// Sets or clears `for`.
    pub fn set_for_(mut self, value: Option<String>) -> Self {
        self.for_ = value;
        self
    }

    /// Memory ID
    pub fn id(mut self, value: impl Into<immich_models::id::MemoryId>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<immich_models::id::MemoryId>) -> Self {
        self.id = value;
        self
    }

    /// Filter by saved status
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = Some(value);
        self
    }

    /// Sets or clears `isSaved`.
    pub fn set_is_saved(mut self, value: Option<bool>) -> Self {
        self.is_saved = value;
        self
    }

    /// Include trashed memories
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = Some(value);
        self
    }

    /// Sets or clears `isTrashed`.
    pub fn set_is_trashed(mut self, value: Option<bool>) -> Self {
        self.is_trashed = value;
        self
    }

    /// Filter by memories that have not been shown yet
    pub fn is_upcoming(mut self, value: bool) -> Self {
        self.is_upcoming = Some(value);
        self
    }

    /// Sets or clears `isUpcoming`.
    pub fn set_is_upcoming(mut self, value: Option<bool>) -> Self {
        self.is_upcoming = value;
        self
    }

    /// Sets `order`.
    pub fn order(mut self, value: MemorySearchOrder) -> Self {
        self.order = Some(value);
        self
    }

    /// Sets or clears `order`.
    pub fn set_order(mut self, value: Option<MemorySearchOrder>) -> Self {
        self.order = value;
        self
    }

    /// Page number
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Sets or clears `page`.
    pub fn set_page(mut self, value: Option<i64>) -> Self {
        self.page = value;
        self
    }

    /// Number of memories to return
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    /// Sets or clears `size`.
    pub fn set_size(mut self, value: Option<i64>) -> Self {
        self.size = value;
        self
    }

    /// Sets `type`.
    pub fn type_(mut self, value: MemoryType) -> Self {
        self.type_ = Some(value);
        self
    }

    /// Sets or clears `type`.
    pub fn set_type_(mut self, value: Option<MemoryType>) -> Self {
        self.type_ = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<MemoryStatisticsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Memories",
            method = "GET",
            path = "/memories/statistics",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/memories/statistics")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<MemoryStatisticsResponseDTO>("memories", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for MemoriesStatisticsBuilder<'a> {
    type Output = Result<MemoryStatisticsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Memories::remove_memory_assets`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveMemoryAssetsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::MemoryId,
    #[serde(skip)]
    pub body: &'a BulkIdsDTO,
}

/// Type alias for [`RemoveMemoryAssetsBuilder`].
pub type RemoveMemoryAssetsArgs<'a> = RemoveMemoryAssetsBuilder<'a>;
impl RemoveMemoryAssetsBuilder<'_> {
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
            module = "Memories",
            method = "DELETE",
            path = "/memories/{id}/assets",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/memories/{}/assets", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self
            .client
            .execute::<Vec<BulkIdResponseDTO>>("memories", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for RemoveMemoryAssetsBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Memories::search_memories`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchMemoriesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "for", skip_serializing_if = "Option::is_none")]
    pub for_: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<immich_models::id::MemoryId>,
    #[serde(rename = "isSaved", skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
    pub is_trashed: Option<bool>,
    #[serde(rename = "isUpcoming", skip_serializing_if = "Option::is_none")]
    pub is_upcoming: Option<bool>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<MemorySearchOrder>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<MemoryType>,
}

/// Type alias for [`SearchMemoriesBuilder`].
pub type SearchMemoriesArgs<'a> = SearchMemoriesBuilder<'a>;
impl<'a> SearchMemoriesBuilder<'a> {
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

    /// Filter by date
    pub fn for_(mut self, value: impl Into<String>) -> Self {
        self.for_ = Some(value.into());
        self
    }

    /// Sets or clears `for`.
    pub fn set_for_(mut self, value: Option<String>) -> Self {
        self.for_ = value;
        self
    }

    /// Memory ID
    pub fn id(mut self, value: impl Into<immich_models::id::MemoryId>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<immich_models::id::MemoryId>) -> Self {
        self.id = value;
        self
    }

    /// Filter by saved status
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = Some(value);
        self
    }

    /// Sets or clears `isSaved`.
    pub fn set_is_saved(mut self, value: Option<bool>) -> Self {
        self.is_saved = value;
        self
    }

    /// Include trashed memories
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = Some(value);
        self
    }

    /// Sets or clears `isTrashed`.
    pub fn set_is_trashed(mut self, value: Option<bool>) -> Self {
        self.is_trashed = value;
        self
    }

    /// Filter by memories that have not been shown yet
    pub fn is_upcoming(mut self, value: bool) -> Self {
        self.is_upcoming = Some(value);
        self
    }

    /// Sets or clears `isUpcoming`.
    pub fn set_is_upcoming(mut self, value: Option<bool>) -> Self {
        self.is_upcoming = value;
        self
    }

    /// Sets `order`.
    pub fn order(mut self, value: MemorySearchOrder) -> Self {
        self.order = Some(value);
        self
    }

    /// Sets or clears `order`.
    pub fn set_order(mut self, value: Option<MemorySearchOrder>) -> Self {
        self.order = value;
        self
    }

    /// Page number
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Sets or clears `page`.
    pub fn set_page(mut self, value: Option<i64>) -> Self {
        self.page = value;
        self
    }

    /// Number of memories to return
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    /// Sets or clears `size`.
    pub fn set_size(mut self, value: Option<i64>) -> Self {
        self.size = value;
        self
    }

    /// Sets `type`.
    pub fn type_(mut self, value: MemoryType) -> Self {
        self.type_ = Some(value);
        self
    }

    /// Sets or clears `type`.
    pub fn set_type_(mut self, value: Option<MemoryType>) -> Self {
        self.type_ = value;
        self
    }

    /// Creates a paginator for this request, providing streams over pages or individual items.
    #[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
    pub fn into_paginator(self) -> SearchMemoriesPaginator<'a> {
        SearchMemoriesPaginator { builder: self }
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<MemoryResponseDTO>, crate::Error> {
        tracing::trace!(module = "Memories", method = "GET", path = "/memories", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/memories")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<Vec<MemoryResponseDTO>>("memories", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchMemoriesBuilder<'a> {
    type Output = Result<Vec<MemoryResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Paginator for [`SearchMemoriesBuilder`].
#[derive(Debug, Clone)]
#[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
pub struct SearchMemoriesPaginator<'a> {
    builder: SearchMemoriesBuilder<'a>,
}

impl<'a> SearchMemoriesPaginator<'a> {
    /// Sets the page size for subsequent page requests.
    pub fn page_size(mut self, size: i64) -> Self {
        self.builder = self.builder.size(size);
        self
    }

    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding pages of items.
    #[must_use = "streams do nothing unless polled"]
    pub fn pages(self) -> impl futures_core::Stream<Item = Result<Vec<MemoryResponseDTO>, crate::Error>> + 'a {
        let builder = self.builder;
        let mut current_page = builder.page.unwrap_or(1);
        async_stream::try_stream! {
            loop { let items = builder.clone().page(current_page).send(). await ?; let
            count = items.len(); yield items; if count == 0 { break; } else if let
            Some(expected) = builder.size { let count_i64 = i64::try_from(count)
            .unwrap_or(i64::MAX); if count_i64 < expected { break; } } current_page += 1;
            }
        }
    }

    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding individual items, flattened across pages.
    #[must_use = "streams do nothing unless polled"]
    pub fn items(self) -> impl futures_core::Stream<Item = Result<MemoryResponseDTO, crate::Error>> + 'a {
        use futures_util::StreamExt;
        self.pages().flat_map(|page_res| match page_res {
            Ok(items) => futures_util::stream::iter(items.into_iter().map(Ok)).left_stream(),
            Err(err) => futures_util::stream::once(async move { Err(err) }).right_stream(),
        })
    }
}

/// Fluent request builder for [`Memories::update_memory`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateMemoryBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::MemoryId,
    #[serde(skip)]
    pub body: &'a MemoryUpdateDTO,
}

/// Type alias for [`UpdateMemoryBuilder`].
pub type UpdateMemoryArgs<'a> = UpdateMemoryBuilder<'a>;
impl UpdateMemoryBuilder<'_> {
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
    pub async fn send(self) -> Result<MemoryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Memories",
            method = "PUT",
            path = "/memories/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/memories/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<MemoryResponseDTO>("memories", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateMemoryBuilder<'a> {
    type Output = Result<MemoryResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Memories` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Memories<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Memories<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Add assets to a memory
    /// Add a list of asset IDs to a specific memory.
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
    pub fn add_memory_assets(
        &self,
        id: impl Into<immich_models::id::MemoryId>,
        body: &'a BulkIdsDTO,
    ) -> AddMemoryAssetsBuilder<'a> {
        AddMemoryAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Create a memory
    /// Create a new memory by providing a name, description, and a list of asset IDs to include in the memory.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `MemoryCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `MemoryResponseDTO`
    pub fn create_memory(&self, body: &'a MemoryCreateDTO) -> CreateMemoryBuilder<'a> {
        CreateMemoryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete a memory
    /// Delete a specific memory by its ID.
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
    pub fn delete_memory(&self, id: impl Into<immich_models::id::MemoryId>) -> DeleteMemoryBuilder<'a> {
        DeleteMemoryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve a memory
    /// Retrieve a specific memory by its ID.
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
    /// **Returns:** `MemoryResponseDTO`
    pub fn get_memory(&self, id: impl Into<immich_models::id::MemoryId>) -> GetMemoryBuilder<'a> {
        GetMemoryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve memories statistics
    /// Retrieve statistics about memories, such as total count and other relevant metrics.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `for` | `String` |  | Filter by date |
    /// | `id` | `uuid::Uuid` |  | Memory ID |
    /// | `isSaved` | `bool` |  | Filter by saved status |
    /// | `isTrashed` | `bool` |  | Include trashed memories |
    /// | `isUpcoming` | `bool` |  | Filter by memories that have not been shown yet |
    /// | `order` | `MemorySearchOrder` |  |  |
    /// | `page` | `i64` |  | Page number |
    /// | `size` | `i64` |  | Number of memories to return |
    /// | `type` | `MemoryType` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `MemoryStatisticsResponseDTO`
    pub fn memories_statistics(&self) -> MemoriesStatisticsBuilder<'a> {
        MemoriesStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            for_: None,
            id: None,
            is_saved: None,
            is_trashed: None,
            is_upcoming: None,
            order: None,
            page: None,
            size: None,
            type_: None,
        }
    }

    /// Remove assets from a memory
    /// Remove a list of asset IDs from a specific memory.
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
    pub fn remove_memory_assets(
        &self,
        id: impl Into<immich_models::id::MemoryId>,
        body: &'a BulkIdsDTO,
    ) -> RemoveMemoryAssetsBuilder<'a> {
        RemoveMemoryAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Retrieve memories
    /// Retrieve a list of memories. Memories are sorted descending by creation date by default, although they can also be sorted in ascending order, or randomly.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `for` | `String` |  | Filter by date |
    /// | `id` | `uuid::Uuid` |  | Memory ID |
    /// | `isSaved` | `bool` |  | Filter by saved status |
    /// | `isTrashed` | `bool` |  | Include trashed memories |
    /// | `isUpcoming` | `bool` |  | Filter by memories that have not been shown yet |
    /// | `order` | `MemorySearchOrder` |  |  |
    /// | `page` | `i64` |  | Page number |
    /// | `size` | `i64` |  | Number of memories to return |
    /// | `type` | `MemoryType` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<MemoryResponseDTO>`
    pub fn search_memories(&self) -> SearchMemoriesBuilder<'a> {
        SearchMemoriesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            for_: None,
            id: None,
            is_saved: None,
            is_trashed: None,
            is_upcoming: None,
            order: None,
            page: None,
            size: None,
            type_: None,
        }
    }

    /// Update a memory
    /// Update an existing memory by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `MemoryUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `MemoryResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_memory(
        &self,
        id: impl Into<immich_models::id::MemoryId>,
        body: &'a MemoryUpdateDTO,
    ) -> UpdateMemoryBuilder<'a> {
        UpdateMemoryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }
}
