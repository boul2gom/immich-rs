// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `partners`.

use immich_models::partners::{PartnerCreateDTO, PartnerDirection, PartnerResponseDTO, PartnerUpdateDTO};

/// Fluent request builder for [`Partners::create_partner`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreatePartnerBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a PartnerCreateDTO,
}

/// Type alias for [`CreatePartnerBuilder`].
pub type CreatePartnerArgs<'a> = CreatePartnerBuilder<'a>;
impl CreatePartnerBuilder<'_> {
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
    pub async fn send(self) -> Result<PartnerResponseDTO, crate::Error> {
        tracing::trace!(module = "Partners", method = "POST", path = "/partners", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/partners")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<PartnerResponseDTO>("partners", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreatePartnerBuilder<'a> {
    type Output = Result<PartnerResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Partners::create_partner_deprecated`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreatePartnerDeprecatedBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PartnerId,
}

/// Type alias for [`CreatePartnerDeprecatedBuilder`].
pub type CreatePartnerDeprecatedArgs<'a> = CreatePartnerDeprecatedBuilder<'a>;
impl CreatePartnerDeprecatedBuilder<'_> {
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
    pub async fn send(self) -> Result<PartnerResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Partners",
            method = "POST",
            path = "/partners/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, &format!("/partners/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<PartnerResponseDTO>("partners", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for CreatePartnerDeprecatedBuilder<'a> {
    type Output = Result<PartnerResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Partners::get_partners`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPartnersBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "direction")]
    pub direction: PartnerDirection,
}

/// Type alias for [`GetPartnersBuilder`].
pub type GetPartnersArgs<'a> = GetPartnersBuilder<'a>;
impl GetPartnersBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<PartnerResponseDTO>, crate::Error> {
        tracing::trace!(module = "Partners", method = "GET", path = "/partners", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/partners")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<Vec<PartnerResponseDTO>>("partners", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetPartnersBuilder<'a> {
    type Output = Result<Vec<PartnerResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Partners::remove_partner`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct RemovePartnerBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PartnerId,
}

/// Type alias for [`RemovePartnerBuilder`].
pub type RemovePartnerArgs<'a> = RemovePartnerBuilder<'a>;
impl RemovePartnerBuilder<'_> {
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
            module = "Partners",
            method = "DELETE",
            path = "/partners/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/partners/{}", self.id))?;
        request = self.request_config.apply(request);
        self.client.execute_unit("partners", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for RemovePartnerBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Partners::update_partner`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdatePartnerBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: immich_models::id::PartnerId,
    #[serde(skip)]
    pub body: &'a PartnerUpdateDTO,
}

/// Type alias for [`UpdatePartnerBuilder`].
pub type UpdatePartnerArgs<'a> = UpdatePartnerBuilder<'a>;
impl UpdatePartnerBuilder<'_> {
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
    pub async fn send(self) -> Result<PartnerResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Partners",
            method = "PUT",
            path = "/partners/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/partners/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<PartnerResponseDTO>("partners", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdatePartnerBuilder<'a> {
    type Output = Result<PartnerResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Partners` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Partners<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Partners<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create a partner
    /// Create a new partner to share assets with.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `PartnerCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    ///
    /// **Returns:** `PartnerResponseDTO`
    pub fn create_partner(&self, body: &'a PartnerCreateDTO) -> CreatePartnerBuilder<'a> {
        CreatePartnerBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Create a partner
    /// Create a new partner to share assets with.
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
    /// | `201` |  |
    ///
    /// **Returns:** `PartnerResponseDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn create_partner_deprecated(
        &self,
        id: impl Into<immich_models::id::PartnerId>,
    ) -> CreatePartnerDeprecatedBuilder<'a> {
        CreatePartnerDeprecatedBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Retrieve partners
    /// Retrieve a list of partners with whom assets are shared.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `direction` | `PartnerDirection` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PartnerResponseDTO>`
    pub fn get_partners(&self, direction: PartnerDirection) -> GetPartnersBuilder<'a> {
        GetPartnersBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            direction,
        }
    }

    /// Remove a partner
    /// Stop sharing assets with a partner.
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
    pub fn remove_partner(&self, id: impl Into<immich_models::id::PartnerId>) -> RemovePartnerBuilder<'a> {
        RemovePartnerBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }

    /// Update a partner
    /// Specify whether a partner's assets should appear in the user's timeline.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `PartnerUpdateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `PartnerResponseDTO`
    pub fn update_partner(
        &self,
        id: impl Into<immich_models::id::PartnerId>,
        body: &'a PartnerUpdateDTO,
    ) -> UpdatePartnerBuilder<'a> {
        UpdatePartnerBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
            body,
        }
    }
}
