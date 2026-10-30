// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `people`.

use immich_models::albums::BulkIdResponseDTO;
use immich_models::assets::{PersonResponseDTO, PersonUserRole};
use immich_models::id::{AssetId, PersonId, UserId};
use immich_models::people::{
    AssetFaceUpdateDTO, MergePersonDTO, PeopleDeleteDTO, PeopleResponseDTO, PeopleUpdateDTO, PeopleUsersUpsertDTO,
    PersonCreateDTO, PersonDeleteDTO, PersonStatisticsResponseDTO, PersonUpdateDTO, PersonUsersDeleteDTO,
    PersonUsersResponseDTO, SharingDirection,
};

/// Fluent request builder for [`People::create_person`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreatePersonBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: PersonCreateDTO,
}

impl CreatePersonBuilder<'_> {
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
    pub async fn send(self) -> Result<PersonResponseDTO, crate::Error> {
        tracing::trace!(module = "people", method = "POST", path = "/people", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/people")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<PersonResponseDTO>("people", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreatePersonBuilder<'a> {
    type Output = Result<PersonResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::delete_people`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeletePeopleBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: PeopleDeleteDTO,
}

impl DeletePeopleBuilder<'_> {
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
        tracing::trace!(module = "people", method = "DELETE", path = "/people", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/people")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("people", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeletePeopleBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::delete_person`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeletePersonBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: PersonId,
    body: PersonDeleteDTO,
}

impl DeletePersonBuilder<'_> {
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
            module = "people",
            method = "DELETE",
            path = "/people/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("people"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("people", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeletePersonBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::get_all_people`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAllPeopleBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    closest_asset_id: Option<AssetId>,
    closest_person_id: Option<PersonId>,
    is_favorite: Option<bool>,
    is_hidden: Option<bool>,
    name: Option<String>,
    page: Option<i64>,
    shared_by_id: Option<UserId>,
    shared_with_id: Option<UserId>,
    size: Option<i64>,
    with_hidden: Option<bool>,
}

impl<'a> GetAllPeopleBuilder<'a> {
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
    /// Closest asset ID for similarity search
    pub fn closest_asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.closest_asset_id = Some(value.into());
        self
    }
    /// Sets or clears `closestAssetId`.
    pub fn set_closest_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.closest_asset_id = value;
        self
    }
    /// Closest person ID for similarity search
    pub fn closest_person_id(mut self, value: impl Into<PersonId>) -> Self {
        self.closest_person_id = Some(value.into());
        self
    }
    /// Sets or clears `closestPersonId`.
    pub fn set_closest_person_id(mut self, value: Option<PersonId>) -> Self {
        self.closest_person_id = value;
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
    /// Filter by hidden status
    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = Some(value);
        self
    }
    /// Sets or clears `isHidden`.
    pub fn set_is_hidden(mut self, value: Option<bool>) -> Self {
        self.is_hidden = value;
        self
    }
    /// Filter by person name
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Page number for pagination
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }
    /// Sets or clears `page`.
    pub fn set_page(mut self, value: Option<i64>) -> Self {
        self.page = value;
        self
    }
    /// Only include people to which the user gave access
    pub fn shared_by_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_by_id = Some(value.into());
        self
    }
    /// Sets or clears `sharedById`.
    pub fn set_shared_by_id(mut self, value: Option<UserId>) -> Self {
        self.shared_by_id = value;
        self
    }
    /// Only include people to which the user was given access
    pub fn shared_with_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_with_id = Some(value.into());
        self
    }
    /// Sets or clears `sharedWithId`.
    pub fn set_shared_with_id(mut self, value: Option<UserId>) -> Self {
        self.shared_with_id = value;
        self
    }
    /// Number of items per page
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }
    /// Sets or clears `size`.
    pub fn set_size(mut self, value: Option<i64>) -> Self {
        self.size = value;
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
    /// Creates a paginator for this request, providing streams over pages or individual items.
    #[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
    pub fn into_paginator(self) -> GetAllPeoplePaginator<'a> {
        GetAllPeoplePaginator { builder: self }
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<PeopleResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "closestAssetId", skip_serializing_if = "Option::is_none")]
            closest_asset_id: Option<&'b AssetId>,
            #[serde(rename = "closestPersonId", skip_serializing_if = "Option::is_none")]
            closest_person_id: Option<&'b PersonId>,
            #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
            is_favorite: Option<&'b bool>,
            #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
            is_hidden: Option<&'b bool>,
            #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
            name: Option<&'b String>,
            #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
            page: Option<&'b i64>,
            #[serde(rename = "sharedById", skip_serializing_if = "Option::is_none")]
            shared_by_id: Option<&'b UserId>,
            #[serde(rename = "sharedWithId", skip_serializing_if = "Option::is_none")]
            shared_with_id: Option<&'b UserId>,
            #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
            size: Option<&'b i64>,
            #[serde(rename = "withHidden", skip_serializing_if = "Option::is_none")]
            with_hidden: Option<&'b bool>,
        }
        tracing::trace!(module = "people", method = "GET", path = "/people", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/people")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            closest_asset_id: self.closest_asset_id.as_ref(),
            closest_person_id: self.closest_person_id.as_ref(),
            is_favorite: self.is_favorite.as_ref(),
            is_hidden: self.is_hidden.as_ref(),
            name: self.name.as_ref(),
            page: self.page.as_ref(),
            shared_by_id: self.shared_by_id.as_ref(),
            shared_with_id: self.shared_with_id.as_ref(),
            size: self.size.as_ref(),
            with_hidden: self.with_hidden.as_ref(),
        };
        request = request.query(&query);
        let response = self.client.execute::<PeopleResponseDTO>("people", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAllPeopleBuilder<'a> {
    type Output = Result<PeopleResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Paginator for [`GetAllPeopleBuilder`].
#[derive(Debug, Clone)]
#[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
pub struct GetAllPeoplePaginator<'a> {
    builder: GetAllPeopleBuilder<'a>,
}

impl<'a> GetAllPeoplePaginator<'a> {
    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding pages of items.
    #[must_use = "streams do nothing unless polled"]
    pub fn pages(self) -> impl futures_core::Stream<Item = Result<Vec<PersonResponseDTO>, crate::Error>> + 'a {
        let builder = self.builder;
        let mut current_page = builder.page.unwrap_or(1);
        async_stream::try_stream! {
            use crate ::utils::IsTrue; loop { let res = builder.clone()
            .page(current_page).send(). await ?; let items = res.people; let is_empty =
            items.is_empty(); let has_next = res.has_next_page; yield items; if is_empty
            || ! has_next.is_true() { break; } current_page += 1; }
        }
    }
    /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding individual items, flattened across pages.
    #[must_use = "streams do nothing unless polled"]
    pub fn items(self) -> impl futures_core::Stream<Item = Result<PersonResponseDTO, crate::Error>> + 'a {
        use futures_util::StreamExt;
        self.pages().flat_map(|page_res| match page_res {
            Ok(items) => futures_util::stream::iter(items.into_iter().map(Ok)).left_stream(),
            Err(err) => futures_util::stream::once(async move { Err(err) }).right_stream(),
        })
    }
}

/// Fluent request builder for [`People::get_person`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPersonBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: PersonId,
}

impl GetPersonBuilder<'_> {
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
    pub async fn send(self) -> Result<PersonResponseDTO, crate::Error> {
        tracing::trace!(module = "people", method = "GET", path = "/people/{id}", "HTTP request");
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("people"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self.client.execute::<PersonResponseDTO>("people", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetPersonBuilder<'a> {
    type Output = Result<PersonResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::get_person_statistics`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPersonStatisticsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: PersonId,
}

impl GetPersonStatisticsBuilder<'_> {
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
    pub async fn send(self) -> Result<PersonStatisticsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "people",
            method = "GET",
            path = "/people/{id}/statistics",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("people"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("statistics"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<PersonStatisticsResponseDTO>("people", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetPersonStatisticsBuilder<'a> {
    type Output = Result<PersonStatisticsResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::get_person_thumbnail`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPersonThumbnailBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: PersonId,
}

impl GetPersonThumbnailBuilder<'_> {
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
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "people",
            method = "GET",
            path = "/people/{id}/thumbnail",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("people"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("thumbnail"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let raw_response = self.client.execute_stream("people", request).await?;
        Ok(raw_response)
    }
}

impl<'a> std::future::IntoFuture for GetPersonThumbnailBuilder<'a> {
    type Output = Result<reqwest::Response, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::get_users_for_people`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUsersForPeopleBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    direction: Option<SharingDirection>,
    person_id: Option<PersonId>,
    role: Option<PersonUserRole>,
    shared_by_id: Option<UserId>,
    shared_with_id: Option<UserId>,
}

impl GetUsersForPeopleBuilder<'_> {
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
    /// Sets `direction`.
    pub fn direction(mut self, value: SharingDirection) -> Self {
        self.direction = Some(value);
        self
    }
    /// Sets or clears `direction`.
    pub fn set_direction(mut self, value: Option<SharingDirection>) -> Self {
        self.direction = value;
        self
    }
    /// Person ID
    pub fn person_id(mut self, value: impl Into<PersonId>) -> Self {
        self.person_id = Some(value.into());
        self
    }
    /// Sets or clears `personId`.
    pub fn set_person_id(mut self, value: Option<PersonId>) -> Self {
        self.person_id = value;
        self
    }
    /// Role of user
    pub fn role(mut self, value: PersonUserRole) -> Self {
        self.role = Some(value);
        self
    }
    /// Sets or clears `role`.
    pub fn set_role(mut self, value: Option<PersonUserRole>) -> Self {
        self.role = value;
        self
    }
    /// User ID of the user that gave access
    pub fn shared_by_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_by_id = Some(value.into());
        self
    }
    /// Sets or clears `sharedById`.
    pub fn set_shared_by_id(mut self, value: Option<UserId>) -> Self {
        self.shared_by_id = value;
        self
    }
    /// User ID of the user that was given access
    pub fn shared_with_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_with_id = Some(value.into());
        self
    }
    /// Sets or clears `sharedWithId`.
    pub fn set_shared_with_id(mut self, value: Option<UserId>) -> Self {
        self.shared_with_id = value;
        self
    }
    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<PersonUsersResponseDTO, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
            direction: Option<&'b SharingDirection>,
            #[serde(rename = "personId", skip_serializing_if = "Option::is_none")]
            person_id: Option<&'b PersonId>,
            #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
            role: Option<&'b PersonUserRole>,
            #[serde(rename = "sharedById", skip_serializing_if = "Option::is_none")]
            shared_by_id: Option<&'b UserId>,
            #[serde(rename = "sharedWithId", skip_serializing_if = "Option::is_none")]
            shared_with_id: Option<&'b UserId>,
        }
        tracing::trace!(
            module = "people",
            method = "GET",
            path = "/people/users",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/people/users")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            direction: self.direction.as_ref(),
            person_id: self.person_id.as_ref(),
            role: self.role.as_ref(),
            shared_by_id: self.shared_by_id.as_ref(),
            shared_with_id: self.shared_with_id.as_ref(),
        };
        request = request.query(&query);
        let response = self.client.execute::<PersonUsersResponseDTO>("people", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetUsersForPeopleBuilder<'a> {
    type Output = Result<PersonUsersResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::merge_people`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct MergePeopleBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: MergePersonDTO,
}

impl MergePeopleBuilder<'_> {
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
            module = "people",
            method = "POST",
            path = "/people/merge",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::POST, "/people/merge")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<Vec<BulkIdResponseDTO>>("people", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for MergePeopleBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::merge_person_legacy`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct MergePersonLegacyBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: PersonId,
    body: MergePersonDTO,
}

impl MergePersonLegacyBuilder<'_> {
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
            module = "people",
            method = "POST",
            path = "/people/{id}/merge",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::POST,
            &[
                crate::core::PathSegment::Static("people"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("merge"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<Vec<BulkIdResponseDTO>>("people", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for MergePersonLegacyBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::reassign_faces`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ReassignFacesBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: PersonId,
    body: AssetFaceUpdateDTO,
}

impl ReassignFacesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<PersonResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "people",
            method = "PUT",
            path = "/people/{id}/reassign",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("people"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
                crate::core::PathSegment::Static("reassign"),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<Vec<PersonResponseDTO>>("people", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for ReassignFacesBuilder<'a> {
    type Output = Result<Vec<PersonResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::remove_users_from_people`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveUsersFromPeopleBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: PersonUsersDeleteDTO,
}

impl RemoveUsersFromPeopleBuilder<'_> {
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
            module = "people",
            method = "DELETE",
            path = "/people/users",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/people/users")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("people", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for RemoveUsersFromPeopleBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::update_people`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdatePeopleBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: PeopleUpdateDTO,
}

impl UpdatePeopleBuilder<'_> {
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
        tracing::trace!(module = "people", method = "PUT", path = "/people", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/people")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<Vec<BulkIdResponseDTO>>("people", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdatePeopleBuilder<'a> {
    type Output = Result<Vec<BulkIdResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::update_person`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdatePersonBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: PersonId,
    body: PersonUpdateDTO,
}

impl UpdatePersonBuilder<'_> {
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
    pub async fn send(self) -> Result<PersonResponseDTO, crate::Error> {
        tracing::trace!(module = "people", method = "PUT", path = "/people/{id}", "HTTP request");
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PUT,
            &[
                crate::core::PathSegment::Static("people"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<PersonResponseDTO>("people", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdatePersonBuilder<'a> {
    type Output = Result<PersonResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`People::upsert_people_users`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpsertPeopleUsersBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: PeopleUsersUpsertDTO,
}

impl UpsertPeopleUsersBuilder<'_> {
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
            module = "people",
            method = "PUT",
            path = "/people/users",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/people/users")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        self.client.execute_unit("people", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for UpsertPeopleUsersBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `People` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct People<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> People<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Create a person
    ///
    /// Create a new person that can have multiple faces assigned to them.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `PersonCreateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `PersonResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn create_person(&self, body: impl Into<PersonCreateDTO>) -> CreatePersonBuilder<'a> {
        CreatePersonBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete people
    ///
    /// Bulk delete a list of people at once.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `PeopleDeleteDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_people(&self, body: impl Into<PeopleDeleteDTO>) -> DeletePeopleBuilder<'a> {
        DeletePeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Delete person
    ///
    /// Delete an individual person.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `PersonId` | ✓ |  |
    /// | `body` | `PersonDeleteDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_person(&self, id: impl Into<PersonId>, body: impl Into<PersonDeleteDTO>) -> DeletePersonBuilder<'a> {
        DeletePersonBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Get all people
    ///
    /// Retrieve a list of all people.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `closest_asset_id` | `AssetId` |  | Closest asset ID for similarity search |
    /// | `closest_person_id` | `PersonId` |  | Closest person ID for similarity search |
    /// | `is_favorite` | `bool` |  | Filter by favorite status |
    /// | `is_hidden` | `bool` |  | Filter by hidden status |
    /// | `name` | `String` |  | Filter by person name |
    /// | `page` | `i64` |  | Page number for pagination |
    /// | `shared_by_id` | `UserId` |  | Only include people to which the user gave access |
    /// | `shared_with_id` | `UserId` |  | Only include people to which the user was given access |
    /// | `size` | `i64` |  | Number of items per page |
    /// | `with_hidden` | `bool` |  | Include hidden people |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PeopleResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_all_people(&self) -> GetAllPeopleBuilder<'a> {
        GetAllPeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            closest_asset_id: None,
            closest_person_id: None,
            is_favorite: None,
            is_hidden: None,
            name: None,
            page: None,
            shared_by_id: None,
            shared_with_id: None,
            size: None,
            with_hidden: None,
        }
    }
    /// Get a person
    ///
    /// Retrieve a person by id.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `PersonId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PersonResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_person(&self, id: impl Into<PersonId>) -> GetPersonBuilder<'a> {
        GetPersonBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Get person statistics
    ///
    /// Retrieve statistics about a specific person.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `PersonId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PersonStatisticsResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_person_statistics(&self, id: impl Into<PersonId>) -> GetPersonStatisticsBuilder<'a> {
        GetPersonStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Get person thumbnail
    ///
    /// Retrieve the thumbnail file for a person.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `PersonId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `reqwest::Response`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_person_thumbnail(&self, id: impl Into<PersonId>) -> GetPersonThumbnailBuilder<'a> {
        GetPersonThumbnailBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Get people access
    ///
    /// Retrieve a list of users and the people to which they have been given access
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `direction` | `SharingDirection` |  |  |
    /// | `person_id` | `PersonId` |  | Person ID |
    /// | `role` | `PersonUserRole` |  | Role of user |
    /// | `shared_by_id` | `UserId` |  | User ID of the user that gave access |
    /// | `shared_with_id` | `UserId` |  | User ID of the user that was given access |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PersonUsersResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_users_for_people(&self) -> GetUsersForPeopleBuilder<'a> {
        GetUsersForPeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            direction: None,
            person_id: None,
            role: None,
            shared_by_id: None,
            shared_with_id: None,
        }
    }
    /// Merge people
    ///
    /// Merge an ordered list of people together into a single person. The final name and birth date are always the first defined value, following the order. Also automatically merges people for other users in the cluster group, skipping people that would result in overriding a previously set name or birth date.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `MergePersonDTO` | ✓ | JSON request body |
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
    pub fn merge_people(&self, body: impl Into<MergePersonDTO>) -> MergePeopleBuilder<'a> {
        MergePeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Merge people
    ///
    /// Merge a list of people into the person specified in the path parameter.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `PersonId` | ✓ |  |
    /// | `body` | `MergePersonDTO` | ✓ | JSON request body |
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
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn merge_person_legacy(
        &self,
        id: impl Into<PersonId>,
        body: impl Into<MergePersonDTO>,
    ) -> MergePersonLegacyBuilder<'a> {
        MergePersonLegacyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Reassign faces
    ///
    /// Bulk reassign a list of faces to a different person.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `PersonId` | ✓ |  |
    /// | `body` | `AssetFaceUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PersonResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn reassign_faces(
        &self,
        id: impl Into<PersonId>,
        body: impl Into<AssetFaceUpdateDTO>,
    ) -> ReassignFacesBuilder<'a> {
        ReassignFacesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Remove users from people
    ///
    /// Remove user access to people
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `PersonUsersDeleteDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn remove_users_from_people(&self, body: impl Into<PersonUsersDeleteDTO>) -> RemoveUsersFromPeopleBuilder<'a> {
        RemoveUsersFromPeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Update people
    ///
    /// Bulk update multiple people at once.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `PeopleUpdateDTO` | ✓ | JSON request body |
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
    pub fn update_people(&self, body: impl Into<PeopleUpdateDTO>) -> UpdatePeopleBuilder<'a> {
        UpdatePeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Update person
    ///
    /// Update an individual person.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `PersonId` | ✓ |  |
    /// | `body` | `PersonUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PersonResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_person(&self, id: impl Into<PersonId>, body: impl Into<PersonUpdateDTO>) -> UpdatePersonBuilder<'a> {
        UpdatePersonBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body: body.into(),
        }
    }
    /// Upsert user access
    ///
    /// Give users access to people
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `PeopleUsersUpsertDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn upsert_people_users(&self, body: impl Into<PeopleUsersUpsertDTO>) -> UpsertPeopleUsersBuilder<'a> {
        UpsertPeopleUsersBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
}
