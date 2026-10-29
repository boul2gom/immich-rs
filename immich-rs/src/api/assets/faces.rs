// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `faces`.

use immich_models::assets::PersonResponseDTO;
use immich_models::assets::faces::{AssetFaceCreateDTO, AssetFaceDeleteDTO, AssetFaceResponseDTO, FaceDTO};

/// Fluent request builder for [`Faces::create_face`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct CreateFaceBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AssetFaceCreateDTO,
}

/// Type alias for [`CreateFaceBuilder`].
pub type CreateFaceArgs<'a> = CreateFaceBuilder<'a>;
impl CreateFaceBuilder<'_> {
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
        tracing::trace!(module = "Faces", method = "POST", path = "/faces", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::POST, "/faces")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("faces", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for CreateFaceBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Faces::delete_face`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeleteFaceBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a AssetFaceDeleteDTO,
}

/// Type alias for [`DeleteFaceBuilder`].
pub type DeleteFaceArgs<'a> = DeleteFaceBuilder<'a>;
impl DeleteFaceBuilder<'_> {
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
            module = "Faces",
            method = "DELETE",
            path = "/faces/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::DELETE, &format!("/faces/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);
        self.client.execute_unit("faces", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeleteFaceBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Faces::get_faces`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetFacesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetFacesBuilder`].
pub type GetFacesArgs<'a> = GetFacesBuilder<'a>;
impl GetFacesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<AssetFaceResponseDTO>, crate::Error> {
        tracing::trace!(module = "Faces", method = "GET", path = "/faces", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/faces")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<Vec<AssetFaceResponseDTO>>("faces", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetFacesBuilder<'a> {
    type Output = Result<Vec<AssetFaceResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Faces::reassign_faces_by_id`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct ReassignFacesByIdBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
    #[serde(skip)]
    pub body: &'a FaceDTO,
}

/// Type alias for [`ReassignFacesByIdBuilder`].
pub type ReassignFacesByIdArgs<'a> = ReassignFacesByIdBuilder<'a>;
impl ReassignFacesByIdBuilder<'_> {
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
        tracing::trace!(module = "Faces", method = "PUT", path = "/faces/{id}", "HTTP request");

        let mut request = self
            .client
            .prepare_request(reqwest::Method::PUT, &format!("/faces/{}", self.id))?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<PersonResponseDTO>("faces", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for ReassignFacesByIdBuilder<'a> {
    type Output = Result<PersonResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Faces` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Faces<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Faces<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Create a face
    /// Create a new face that has not been discovered by facial recognition. The content of the bounding box is considered a face.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AssetFaceCreateDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `201` |  |
    pub fn create_face(&self, body: &'a AssetFaceCreateDTO) -> CreateFaceBuilder<'a> {
        CreateFaceBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Delete a face
    /// Delete a face identified by the id. Optionally can be force deleted.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `AssetFaceDeleteDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `204` |  |
    pub fn delete_face(&self, id: uuid::Uuid, body: &'a AssetFaceDeleteDTO) -> DeleteFaceBuilder<'a> {
        DeleteFaceBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }

    /// Retrieve faces for asset
    /// Retrieve all faces belonging to an asset.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ | Face ID |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<AssetFaceResponseDTO>`
    pub fn get_faces(&self, id: uuid::Uuid) -> GetFacesBuilder<'a> {
        GetFacesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Re-assign a face to another person
    /// Re-assign the face provided in the body to the person identified by the id in the path parameter.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `FaceDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `PersonResponseDTO`
    pub fn reassign_faces_by_id(&self, id: uuid::Uuid, body: &'a FaceDTO) -> ReassignFacesByIdBuilder<'a> {
        ReassignFacesByIdBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body,
        }
    }
}
