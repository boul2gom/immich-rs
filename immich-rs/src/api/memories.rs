// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `memories`.

use immich_models::albums::{BulkIdResponseDTO, BulkIdsDTO};
use immich_models::id::MemoryId;
use immich_models::memories::{
    MemoryCreateDTO, MemoryResponseDTO, MemorySearchOrder, MemoryStatisticsResponseDTO, MemoryType, MemoryUpdateDTO,
};

/// Fluent request builder for [`Memories::add_memory_assets`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct AddMemoryAssetsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: MemoryId,
    body: BulkIdsDTO,
}

impl AddMemoryAssetsBuilder<'_> {
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
            module = "memories",
            method = "PUT",
            path = "/memories/{id}/assets",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("memories"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("assets"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateMemoryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: MemoryCreateDTO,
}

impl CreateMemoryBuilder<'_> {
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
    pub async fn send(self) -> Result<MemoryResponseDTO, crate::Error> {
        tracing::trace!(module = "memories", method = "POST", path = "/memories", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/memories")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteMemoryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: MemoryId,
}

impl DeleteMemoryBuilder<'_> {
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
            module = "memories",
            method = "DELETE",
            path = "/memories/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("memories"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetMemoryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: MemoryId,
}

impl GetMemoryBuilder<'_> {
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
    pub async fn send(self) -> Result<MemoryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "memories",
            method = "GET",
            path = "/memories/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("memories"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct MemoriesStatisticsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    for_: Option<String>,
    id: Option<MemoryId>,
    is_saved: Option<bool>,
    is_trashed: Option<bool>,
    is_upcoming: Option<bool>,
    order: Option<MemorySearchOrder>,
    page: Option<i64>,
    size: Option<i64>,
    type_: Option<MemoryType>,
}

impl MemoriesStatisticsBuilder<'_> {
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
    pub fn id(mut self, value: impl Into<MemoryId>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<MemoryId>) -> Self {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<MemoryStatisticsResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "for", skip_serializing_if = "Option::is_none")]
            for_: Option<&'b String>,
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b MemoryId>,
            #[serde(rename = "isSaved", skip_serializing_if = "Option::is_none")]
            is_saved: Option<&'b bool>,
            #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
            is_trashed: Option<&'b bool>,
            #[serde(rename = "isUpcoming", skip_serializing_if = "Option::is_none")]
            is_upcoming: Option<&'b bool>,
            #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
            order: Option<&'b MemorySearchOrder>,
            #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
            page: Option<&'b i64>,
            #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
            size: Option<&'b i64>,
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            type_: Option<&'b MemoryType>,
        }
        tracing::trace!(
            module = "memories",
            method = "GET",
            path = "/memories/statistics",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/memories/statistics")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            for_: self.for_.as_ref(),
            id: self.id.as_ref(),
            is_saved: self.is_saved.as_ref(),
            is_trashed: self.is_trashed.as_ref(),
            is_upcoming: self.is_upcoming.as_ref(),
            order: self.order.as_ref(),
            page: self.page.as_ref(),
            size: self.size.as_ref(),
            type_: self.type_.as_ref(),
        };
        request = request.query(&query);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveMemoryAssetsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: MemoryId,
    body: BulkIdsDTO,
}

impl RemoveMemoryAssetsBuilder<'_> {
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
            module = "memories",
            method = "DELETE",
            path = "/memories/{id}/assets",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("memories"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("assets"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchMemoriesBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    for_: Option<String>,
    id: Option<MemoryId>,
    is_saved: Option<bool>,
    is_trashed: Option<bool>,
    is_upcoming: Option<bool>,
    order: Option<MemorySearchOrder>,
    page: Option<i64>,
    size: Option<i64>,
    type_: Option<MemoryType>,
}

impl<'a> SearchMemoriesBuilder<'a> {
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
    pub fn id(mut self, value: impl Into<MemoryId>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<MemoryId>) -> Self {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<MemoryResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "for", skip_serializing_if = "Option::is_none")]
            for_: Option<&'b String>,
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b MemoryId>,
            #[serde(rename = "isSaved", skip_serializing_if = "Option::is_none")]
            is_saved: Option<&'b bool>,
            #[serde(rename = "isTrashed", skip_serializing_if = "Option::is_none")]
            is_trashed: Option<&'b bool>,
            #[serde(rename = "isUpcoming", skip_serializing_if = "Option::is_none")]
            is_upcoming: Option<&'b bool>,
            #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
            order: Option<&'b MemorySearchOrder>,
            #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
            page: Option<&'b i64>,
            #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
            size: Option<&'b i64>,
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            type_: Option<&'b MemoryType>,
        }
        tracing::trace!(module = "memories", method = "GET", path = "/memories", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/memories")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            for_: self.for_.as_ref(),
            id: self.id.as_ref(),
            is_saved: self.is_saved.as_ref(),
            is_trashed: self.is_trashed.as_ref(),
            is_upcoming: self.is_upcoming.as_ref(),
            order: self.order.as_ref(),
            page: self.page.as_ref(),
            size: self.size.as_ref(),
            type_: self.type_.as_ref(),
        };
        request = request.query(&query);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateMemoryBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: MemoryId,
    body: MemoryUpdateDTO,
}

impl UpdateMemoryBuilder<'_> {
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
    pub async fn send(self) -> Result<MemoryResponseDTO, crate::Error> {
        tracing::trace!(
            module = "memories",
            method = "PUT",
            path = "/memories/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("memories"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
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

/// Client handle for `Memories` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Memories<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Memories<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Add assets to a memory
    ///
    /// Add a list of asset IDs to a specific memory.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `MemoryId` | ✓ |  |
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
    pub fn add_memory_assets(
        &self,
        id: impl Into<MemoryId>,
        body: impl Into<BulkIdsDTO>,
    ) -> AddMemoryAssetsBuilder<'a> {
        AddMemoryAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Create a memory
    ///
    /// Create a new memory by providing a name, description, and a list of asset IDs to include in the memory.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `MemoryCreateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `MemoryResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn create_memory(&self, body: impl Into<MemoryCreateDTO>) -> CreateMemoryBuilder<'a> {
        CreateMemoryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete a memory
    ///
    /// Delete a specific memory by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `MemoryId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_memory(&self, id: impl Into<MemoryId>) -> DeleteMemoryBuilder<'a> {
        DeleteMemoryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve a memory
    ///
    /// Retrieve a specific memory by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `MemoryId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `MemoryResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_memory(&self, id: impl Into<MemoryId>) -> GetMemoryBuilder<'a> {
        GetMemoryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve memories statistics
    ///
    /// Retrieve statistics about memories, such as total count and other relevant metrics.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `for_` | `String` |  | Filter by date |
    /// | `id` | `MemoryId` |  | Memory ID |
    /// | `is_saved` | `bool` |  | Filter by saved status |
    /// | `is_trashed` | `bool` |  | Include trashed memories |
    /// | `is_upcoming` | `bool` |  | Filter by memories that have not been shown yet |
    /// | `order` | `MemorySearchOrder` |  |  |
    /// | `page` | `i64` |  | Page number |
    /// | `size` | `i64` |  | Number of memories to return |
    /// | `type_` | `MemoryType` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `MemoryStatisticsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
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
    ///
    /// Remove a list of asset IDs from a specific memory.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `MemoryId` | ✓ |  |
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
    pub fn remove_memory_assets(
        &self,
        id: impl Into<MemoryId>,
        body: impl Into<BulkIdsDTO>,
    ) -> RemoveMemoryAssetsBuilder<'a> {
        RemoveMemoryAssetsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Retrieve memories
    ///
    /// Retrieve a list of memories. Memories are sorted descending by creation date by default, although they can also be sorted in ascending order, or randomly.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `for_` | `String` |  | Filter by date |
    /// | `id` | `MemoryId` |  | Memory ID |
    /// | `is_saved` | `bool` |  | Filter by saved status |
    /// | `is_trashed` | `bool` |  | Include trashed memories |
    /// | `is_upcoming` | `bool` |  | Filter by memories that have not been shown yet |
    /// | `order` | `MemorySearchOrder` |  |  |
    /// | `page` | `i64` |  | Page number |
    /// | `size` | `i64` |  | Number of memories to return |
    /// | `type_` | `MemoryType` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<MemoryResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
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
    ///
    /// Update an existing memory by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `MemoryId` | ✓ |  |
    /// | `body` | `MemoryUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `MemoryResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_memory(&self, id: impl Into<MemoryId>, body: impl Into<MemoryUpdateDTO>) -> UpdateMemoryBuilder<'a> {
        UpdateMemoryBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
}
