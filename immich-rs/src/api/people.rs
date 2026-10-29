// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `people`.

use immich_models::albums::BulkIdResponseDTO;
use immich_models::assets::{PersonResponseDTO, PersonUserRole};
use immich_models::people::{
    AssetFaceUpdateDTO, MergePersonDTO, PeopleDeleteDTO, PeopleResponseDTO, PeopleUpdateDTO, PeopleUsersUpsertDTO,
    PersonCreateDTO, PersonDeleteDTO, PersonStatisticsResponseDTO, PersonUpdateDTO, PersonUsersDeleteDTO,
    PersonUsersResponseDTO, SharingDirection,
};

/// Fluent request builder for [`People::create_person`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreatePersonBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a PersonCreateDTO,
}

/// Type alias for [`CreatePersonBuilder`].
pub type CreatePersonArgs<'a> = CreatePersonBuilder<'a>;
impl CreatePersonBuilder<'_> {
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
    pub async fn send(self) -> Result<PersonResponseDTO, crate::Error> {
        tracing::trace!(module = "People", method = "POST", path = "/people", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/people")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeletePeopleBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a PeopleDeleteDTO,
}

/// Type alias for [`DeletePeopleBuilder`].
pub type DeletePeopleArgs<'a> = DeletePeopleBuilder<'a>;
impl DeletePeopleBuilder<'_> {
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
        tracing::trace!(module = "People", method = "DELETE", path = "/people", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/people")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeletePersonBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PersonId,
    #[serde(skip)]
    pub body: &'a PersonDeleteDTO,
}

/// Type alias for [`DeletePersonBuilder`].
pub type DeletePersonArgs<'a> = DeletePersonBuilder<'a>;
impl DeletePersonBuilder<'_> {
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
            module = "People",
            method = "DELETE",
            path = "/people/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/people/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAllPeopleBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "closestAssetId", skip_serializing_if = "Option::is_none")]
    pub closest_asset_id: Option<uuid::Uuid>,
    #[serde(rename = "closestPersonId", skip_serializing_if = "Option::is_none")]
    pub closest_person_id: Option<uuid::Uuid>,
    #[serde(rename = "isFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "sharedById", skip_serializing_if = "Option::is_none")]
    pub shared_by_id: Option<uuid::Uuid>,
    #[serde(rename = "sharedWithId", skip_serializing_if = "Option::is_none")]
    pub shared_with_id: Option<uuid::Uuid>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "withHidden", skip_serializing_if = "Option::is_none")]
    pub with_hidden: Option<bool>,
}

/// Type alias for [`GetAllPeopleBuilder`].
pub type GetAllPeopleArgs<'a> = GetAllPeopleBuilder<'a>;
impl<'a> GetAllPeopleBuilder<'a> {
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

    /// Closest asset ID for similarity search
    pub fn closest_asset_id(mut self, value: uuid::Uuid) -> Self {
        self.closest_asset_id = Some(value);
        self
    }

    /// Sets or clears `closestAssetId`.
    pub fn set_closest_asset_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.closest_asset_id = value;
        self
    }

    /// Closest person ID for similarity search
    pub fn closest_person_id(mut self, value: uuid::Uuid) -> Self {
        self.closest_person_id = Some(value);
        self
    }

    /// Sets or clears `closestPersonId`.
    pub fn set_closest_person_id(mut self, value: Option<uuid::Uuid>) -> Self {
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
    pub fn shared_by_id(mut self, value: uuid::Uuid) -> Self {
        self.shared_by_id = Some(value);
        self
    }

    /// Sets or clears `sharedById`.
    pub fn set_shared_by_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.shared_by_id = value;
        self
    }

    /// Only include people to which the user was given access
    pub fn shared_with_id(mut self, value: uuid::Uuid) -> Self {
        self.shared_with_id = Some(value);
        self
    }

    /// Sets or clears `sharedWithId`.
    pub fn set_shared_with_id(mut self, value: Option<uuid::Uuid>) -> Self {
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
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<PeopleResponseDTO, crate::Error> {
        tracing::trace!(module = "People", method = "GET", path = "/people", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/people")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPersonBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PersonId,
}

/// Type alias for [`GetPersonBuilder`].
pub type GetPersonArgs<'a> = GetPersonBuilder<'a>;
impl GetPersonBuilder<'_> {
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
    pub async fn send(self) -> Result<PersonResponseDTO, crate::Error> {
        tracing::trace!(module = "People", method = "GET", path = "/people/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/people/{}", self.id))?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPersonStatisticsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PersonId,
}

/// Type alias for [`GetPersonStatisticsBuilder`].
pub type GetPersonStatisticsArgs<'a> = GetPersonStatisticsBuilder<'a>;
impl GetPersonStatisticsBuilder<'_> {
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
    pub async fn send(self) -> Result<PersonStatisticsResponseDTO, crate::Error> {
        tracing::trace!(
            module = "People",
            method = "GET",
            path = "/people/{id}/statistics",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/people/{}/statistics", self.id))?;
        request = self.request_config.apply(request);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPersonThumbnailBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PersonId,
}

/// Type alias for [`GetPersonThumbnailBuilder`].
pub type GetPersonThumbnailArgs<'a> = GetPersonThumbnailBuilder<'a>;
impl GetPersonThumbnailBuilder<'_> {
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
    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        tracing::trace!(
            module = "People",
            method = "GET",
            path = "/people/{id}/thumbnail",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/people/{}/thumbnail", self.id))?;
        request = self.request_config.apply(request);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetUsersForPeopleBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<SharingDirection>,
    #[serde(rename = "personId", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<immich_models::id::PersonId>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<PersonUserRole>,
    #[serde(rename = "sharedById", skip_serializing_if = "Option::is_none")]
    pub shared_by_id: Option<uuid::Uuid>,
    #[serde(rename = "sharedWithId", skip_serializing_if = "Option::is_none")]
    pub shared_with_id: Option<uuid::Uuid>,
}

/// Type alias for [`GetUsersForPeopleBuilder`].
pub type GetUsersForPeopleArgs<'a> = GetUsersForPeopleBuilder<'a>;
impl GetUsersForPeopleBuilder<'_> {
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
    pub fn person_id(mut self, value: impl Into<immich_models::id::PersonId>) -> Self {
        self.person_id = Some(value.into());
        self
    }

    /// Sets or clears `personId`.
    pub fn set_person_id(mut self, value: Option<immich_models::id::PersonId>) -> Self {
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
    pub fn shared_by_id(mut self, value: uuid::Uuid) -> Self {
        self.shared_by_id = Some(value);
        self
    }

    /// Sets or clears `sharedById`.
    pub fn set_shared_by_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.shared_by_id = value;
        self
    }

    /// User ID of the user that was given access
    pub fn shared_with_id(mut self, value: uuid::Uuid) -> Self {
        self.shared_with_id = Some(value);
        self
    }

    /// Sets or clears `sharedWithId`.
    pub fn set_shared_with_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.shared_with_id = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<PersonUsersResponseDTO, crate::Error> {
        tracing::trace!(
            module = "People",
            method = "GET",
            path = "/people/users",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/people/users")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct MergePeopleBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a MergePersonDTO,
}

/// Type alias for [`MergePeopleBuilder`].
pub type MergePeopleArgs<'a> = MergePeopleBuilder<'a>;
impl MergePeopleBuilder<'_> {
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
            module = "People",
            method = "POST",
            path = "/people/merge",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/people/merge")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct MergePersonLegacyBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PersonId,
    #[serde(skip)]
    pub body: &'a MergePersonDTO,
}

/// Type alias for [`MergePersonLegacyBuilder`].
pub type MergePersonLegacyArgs<'a> = MergePersonLegacyBuilder<'a>;
impl MergePersonLegacyBuilder<'_> {
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
            module = "People",
            method = "POST",
            path = "/people/{id}/merge",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, &format!("/people/{}/merge", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ReassignFacesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PersonId,
    #[serde(skip)]
    pub body: &'a AssetFaceUpdateDTO,
}

/// Type alias for [`ReassignFacesBuilder`].
pub type ReassignFacesArgs<'a> = ReassignFacesBuilder<'a>;
impl ReassignFacesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<PersonResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "People",
            method = "PUT",
            path = "/people/{id}/reassign",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/people/{}/reassign", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemoveUsersFromPeopleBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a PersonUsersDeleteDTO,
}

/// Type alias for [`RemoveUsersFromPeopleBuilder`].
pub type RemoveUsersFromPeopleArgs<'a> = RemoveUsersFromPeopleBuilder<'a>;
impl RemoveUsersFromPeopleBuilder<'_> {
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
            module = "People",
            method = "DELETE",
            path = "/people/users",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::DELETE, "/people/users")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdatePeopleBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a PeopleUpdateDTO,
}

/// Type alias for [`UpdatePeopleBuilder`].
pub type UpdatePeopleArgs<'a> = UpdatePeopleBuilder<'a>;
impl UpdatePeopleBuilder<'_> {
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
        tracing::trace!(module = "People", method = "PUT", path = "/people", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/people")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdatePersonBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PersonId,
    #[serde(skip)]
    pub body: &'a PersonUpdateDTO,
}

/// Type alias for [`UpdatePersonBuilder`].
pub type UpdatePersonArgs<'a> = UpdatePersonBuilder<'a>;
impl UpdatePersonBuilder<'_> {
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
    pub async fn send(self) -> Result<PersonResponseDTO, crate::Error> {
        tracing::trace!(module = "People", method = "PUT", path = "/people/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/people/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

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
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpsertPeopleUsersBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a PeopleUsersUpsertDTO,
}

/// Type alias for [`UpsertPeopleUsersBuilder`].
pub type UpsertPeopleUsersArgs<'a> = UpsertPeopleUsersBuilder<'a>;
impl UpsertPeopleUsersBuilder<'_> {
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
            module = "People",
            method = "PUT",
            path = "/people/users",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/people/users")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
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

///Client handle for `People` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct People<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> People<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create a person
    /// Create a new person that can have multiple faces assigned to them.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `PersonCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `PersonResponseDTO`
    pub fn create_person(&self, body: &'a PersonCreateDTO) -> CreatePersonBuilder<'a> {
        CreatePersonBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete people
    /// Bulk delete a list of people at once.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `PeopleDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_people(&self, body: &'a PeopleDeleteDTO) -> DeletePeopleBuilder<'a> {
        DeletePeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete person
    /// Delete an individual person.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `PersonDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_person(
        &self,
        id: impl Into<immich_models::id::PersonId>,
        body: &'a PersonDeleteDTO,
    ) -> DeletePersonBuilder<'a> {
        DeletePersonBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Get all people
    /// Retrieve a list of all people.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `closestAssetId` | `uuid::Uuid` |  | Closest asset ID for similarity search |
    /// | `closestPersonId` | `uuid::Uuid` |  | Closest person ID for similarity search |
    /// | `isFavorite` | `bool` |  | Filter by favorite status |
    /// | `isHidden` | `bool` |  | Filter by hidden status |
    /// | `name` | `String` |  | Filter by person name |
    /// | `page` | `i64` |  | Page number for pagination |
    /// | `sharedById` | `uuid::Uuid` |  | Only include people to which the user gave access |
    /// | `sharedWithId` | `uuid::Uuid` |  | Only include people to which the user was given access |
    /// | `size` | `i64` |  | Number of items per page |
    /// | `withHidden` | `bool` |  | Include hidden people |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `PeopleResponseDTO`
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
    /// Retrieve a person by id.
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
    /// **Returns:** `PersonResponseDTO`
    pub fn get_person(&self, id: impl Into<immich_models::id::PersonId>) -> GetPersonBuilder<'a> {
        GetPersonBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Get person statistics
    /// Retrieve statistics about a specific person.
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
    /// **Returns:** `PersonStatisticsResponseDTO`
    pub fn get_person_statistics(&self, id: impl Into<immich_models::id::PersonId>) -> GetPersonStatisticsBuilder<'a> {
        GetPersonStatisticsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Get person thumbnail
    /// Retrieve the thumbnail file for a person.
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
    /// **Returns:** `reqwest::Response`
    pub fn get_person_thumbnail(&self, id: impl Into<immich_models::id::PersonId>) -> GetPersonThumbnailBuilder<'a> {
        GetPersonThumbnailBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Get people access
    /// Retrieve a list of users and the people to which they have been given access
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `direction` | `SharingDirection` |  |  |
    /// | `personId` | `uuid::Uuid` |  | Person ID |
    /// | `role` | `PersonUserRole` |  | Role of user |
    /// | `sharedById` | `uuid::Uuid` |  | User ID of the user that gave access |
    /// | `sharedWithId` | `uuid::Uuid` |  | User ID of the user that was given access |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `PersonUsersResponseDTO`
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
    /// Merge an ordered list of people together into a single person. The final name and birth date are always the first defined value, following the order. Also automatically merges people for other users in the cluster group, skipping people that would result in overriding a previously set name or birth date.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `MergePersonDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<BulkIdResponseDTO>`
    pub fn merge_people(&self, body: &'a MergePersonDTO) -> MergePeopleBuilder<'a> {
        MergePeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Merge people
    /// Merge a list of people into the person specified in the path parameter.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `MergePersonDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<BulkIdResponseDTO>`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn merge_person_legacy(
        &self,
        id: impl Into<immich_models::id::PersonId>,
        body: &'a MergePersonDTO,
    ) -> MergePersonLegacyBuilder<'a> {
        MergePersonLegacyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Reassign faces
    /// Bulk reassign a list of faces to a different person.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `AssetFaceUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PersonResponseDTO>`
    pub fn reassign_faces(
        &self,
        id: impl Into<immich_models::id::PersonId>,
        body: &'a AssetFaceUpdateDTO,
    ) -> ReassignFacesBuilder<'a> {
        ReassignFacesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Remove users from people
    /// Remove user access to people
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `PersonUsersDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    pub fn remove_users_from_people(&self, body: &'a PersonUsersDeleteDTO) -> RemoveUsersFromPeopleBuilder<'a> {
        RemoveUsersFromPeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Update people
    /// Bulk update multiple people at once.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `PeopleUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<BulkIdResponseDTO>`
    pub fn update_people(&self, body: &'a PeopleUpdateDTO) -> UpdatePeopleBuilder<'a> {
        UpdatePeopleBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Update person
    /// Update an individual person.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `PersonUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `PersonResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_person(
        &self,
        id: impl Into<immich_models::id::PersonId>,
        body: &'a PersonUpdateDTO,
    ) -> UpdatePersonBuilder<'a> {
        UpdatePersonBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }

    /// Upsert user access
    /// Give users access to people
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `PeopleUsersUpsertDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    pub fn upsert_people_users(&self, body: &'a PeopleUsersUpsertDTO) -> UpsertPeopleUsersBuilder<'a> {
        UpsertPeopleUsersBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
