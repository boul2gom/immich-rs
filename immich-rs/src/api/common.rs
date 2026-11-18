// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `common`.

use immich_models::authentication::LoginResponseDTO;
use immich_models::common::{
    PasskeyAuthenticationFinishDTO, PasskeyAuthenticationOptionsDTO, PasskeyRegistrationFinishDTO,
    PasskeyRegistrationOptionsDTO, PasskeyResponseDTO, PasskeyUpdateDTO,
};

/// Fluent request builder for [`Common::delete_passkey`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct DeletePasskeyBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: uuid::Uuid,
}

impl DeletePasskeyBuilder<'_> {
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
            module = "common",
            method = "DELETE",
            path = "/passkeys/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::DELETE,
            &[
                crate::core::PathSegment::Static("passkeys"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        self.client.execute_unit("common", request).await?;
        Ok(())
    }
}

impl<'a> std::future::IntoFuture for DeletePasskeyBuilder<'a> {
    type Output = Result<(), crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::finish_authentication`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct FinishAuthenticationBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: PasskeyAuthenticationFinishDTO,
}

impl FinishAuthenticationBuilder<'_> {
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
    pub async fn send(self) -> Result<LoginResponseDTO, crate::Error> {
        tracing::trace!(
            module = "common",
            method = "POST",
            path = "/passkeys/authentication/finish",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/passkeys/authentication/finish")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<LoginResponseDTO>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for FinishAuthenticationBuilder<'a> {
    type Output = Result<LoginResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::finish_registration`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct FinishRegistrationBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    body: PasskeyRegistrationFinishDTO,
}

impl FinishRegistrationBuilder<'_> {
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
    pub async fn send(self) -> Result<PasskeyResponseDTO, crate::Error> {
        tracing::trace!(
            module = "common",
            method = "POST",
            path = "/passkeys/registration/finish",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/passkeys/registration/finish")?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<PasskeyResponseDTO>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for FinishRegistrationBuilder<'a> {
    type Output = Result<PasskeyResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::get_passkey`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPasskeyBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: uuid::Uuid,
}

impl GetPasskeyBuilder<'_> {
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
    pub async fn send(self) -> Result<PasskeyResponseDTO, crate::Error> {
        tracing::trace!(
            module = "common",
            method = "GET",
            path = "/passkeys/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("passkeys"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        let response = self.client.execute::<PasskeyResponseDTO>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetPasskeyBuilder<'a> {
    type Output = Result<PasskeyResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::search_passkeys`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchPasskeysBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: Option<uuid::Uuid>,
}

impl SearchPasskeysBuilder<'_> {
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
    /// Filter by passkey ID
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<PasskeyResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b uuid::Uuid>,
        }
        tracing::trace!(module = "common", method = "GET", path = "/passkeys", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/passkeys")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams { id: self.id.as_ref() };
        request = request.query(&query);
        let response = self
            .client
            .execute::<Vec<PasskeyResponseDTO>>("common", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchPasskeysBuilder<'a> {
    type Output = Result<Vec<PasskeyResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::start_authentication`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct StartAuthenticationBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl StartAuthenticationBuilder<'_> {
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
    pub async fn send(self) -> Result<PasskeyAuthenticationOptionsDTO, crate::Error> {
        tracing::trace!(
            module = "common",
            method = "POST",
            path = "/passkeys/authentication/start",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/passkeys/authentication/start")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<PasskeyAuthenticationOptionsDTO>("common", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for StartAuthenticationBuilder<'a> {
    type Output = Result<PasskeyAuthenticationOptionsDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::start_registration`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct StartRegistrationBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl StartRegistrationBuilder<'_> {
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
    pub async fn send(self) -> Result<PasskeyRegistrationOptionsDTO, crate::Error> {
        tracing::trace!(
            module = "common",
            method = "POST",
            path = "/passkeys/registration/start",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::POST, "/passkeys/registration/start")?;
        request = self.request_config.apply(request)?;
        let response = self
            .client
            .execute::<PasskeyRegistrationOptionsDTO>("common", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for StartRegistrationBuilder<'a> {
    type Output = Result<PasskeyRegistrationOptionsDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Common::update_passkey`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdatePasskeyBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: uuid::Uuid,
    body: PasskeyUpdateDTO,
}

impl UpdatePasskeyBuilder<'_> {
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
    pub async fn send(self) -> Result<PasskeyResponseDTO, crate::Error> {
        tracing::trace!(
            module = "common",
            method = "PATCH",
            path = "/passkeys/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::PATCH,
            &[
                crate::core::PathSegment::Static("passkeys"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
        request = request.json(&self.body);
        let response = self.client.execute::<PasskeyResponseDTO>("common", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdatePasskeyBuilder<'a> {
    type Output = Result<PasskeyResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Client handle for `Common` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Common<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Common<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Delete a passkey
    ///
    /// Delete a passkey by its ID. The current user must own this passkey.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `204` |  |
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn delete_passkey(&self, id: uuid::Uuid) -> DeletePasskeyBuilder<'a> {
        DeletePasskeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }
    /// Finish passkey authentication
    ///
    /// Verify the response from the authenticator and receive a session token.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `PasskeyAuthenticationFinishDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `LoginResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn finish_authentication(
        &self,
        body: impl Into<PasskeyAuthenticationFinishDTO>,
    ) -> FinishAuthenticationBuilder<'a> {
        FinishAuthenticationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Finish passkey registration
    ///
    /// Verify the response from the authenticator and save the new passkey for the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `body` | `PasskeyRegistrationFinishDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `201` |  |
    ///
    /// **Returns:** `PasskeyResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn finish_registration(&self, body: impl Into<PasskeyRegistrationFinishDTO>) -> FinishRegistrationBuilder<'a> {
        FinishRegistrationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body: body.into(),
        }
    }
    /// Retrieve a passkey
    ///
    /// Retrieve a passkey by its ID. The current user must own this passkey.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PasskeyResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_passkey(&self, id: uuid::Uuid) -> GetPasskeyBuilder<'a> {
        GetPasskeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }
    /// Search passkeys
    ///
    /// Search the passkeys of the current user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `uuid::Uuid` |  | Filter by passkey ID |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PasskeyResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn search_passkeys(&self) -> SearchPasskeysBuilder<'a> {
        SearchPasskeysBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: None,
        }
    }
    /// Start passkey authentication
    ///
    /// Generate the options used by the browser to sign in with a passkey.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PasskeyAuthenticationOptionsDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn start_authentication(&self) -> StartAuthenticationBuilder<'a> {
        StartAuthenticationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Start passkey registration
    ///
    /// Generate the options used by the browser to create a new passkey for the current user.
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PasskeyRegistrationOptionsDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn start_registration(&self) -> StartRegistrationBuilder<'a> {
        StartRegistrationBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// Update a passkey
    ///
    /// Update the name of a passkey by its ID. The current user must own this passkey.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `uuid::Uuid` | ✓ |  |
    /// | `body` | `PasskeyUpdateDTO` | ✓ | JSON request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PasskeyResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn update_passkey(&self, id: uuid::Uuid, body: impl Into<PasskeyUpdateDTO>) -> UpdatePasskeyBuilder<'a> {
        UpdatePasskeyBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
            body: body.into(),
        }
    }
}
