// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `config`.

use immich_models::system::config::AdminConfigDTO;
use immich_models::system::config::notifications::ConfigTemplateStorageOptionDTO;

/// Fluent request builder for [`Config::get_admin_config`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAdminConfigBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetAdminConfigBuilder`].
pub type GetAdminConfigArgs<'a> = GetAdminConfigBuilder<'a>;
impl GetAdminConfigBuilder<'_> {
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
    pub async fn send(self) -> Result<AdminConfigDTO, crate::Error> {
        tracing::trace!(
            module = "Config",
            method = "GET",
            path = "/admin/config",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/admin/config")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<AdminConfigDTO>("config", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAdminConfigBuilder<'a> {
    type Output = Result<AdminConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Config::get_admin_config_defaults`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetAdminConfigDefaultsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetAdminConfigDefaultsBuilder`].
pub type GetAdminConfigDefaultsArgs<'a> = GetAdminConfigDefaultsBuilder<'a>;
impl GetAdminConfigDefaultsBuilder<'_> {
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
    pub async fn send(self) -> Result<AdminConfigDTO, crate::Error> {
        tracing::trace!(
            module = "Config",
            method = "GET",
            path = "/admin/config/defaults",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/admin/config/defaults")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<AdminConfigDTO>("config", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetAdminConfigDefaultsBuilder<'a> {
    type Output = Result<AdminConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Config::get_config`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetConfigBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetConfigBuilder`].
pub type GetConfigArgs<'a> = GetConfigBuilder<'a>;
impl GetConfigBuilder<'_> {
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
    pub async fn send(self) -> Result<AdminConfigDTO, crate::Error> {
        tracing::trace!(
            module = "Config",
            method = "GET",
            path = "/system-config",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/system-config")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<AdminConfigDTO>("config", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetConfigBuilder<'a> {
    type Output = Result<AdminConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Config::get_config_defaults`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetConfigDefaultsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetConfigDefaultsBuilder`].
pub type GetConfigDefaultsArgs<'a> = GetConfigDefaultsBuilder<'a>;
impl GetConfigDefaultsBuilder<'_> {
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
    pub async fn send(self) -> Result<AdminConfigDTO, crate::Error> {
        tracing::trace!(
            module = "Config",
            method = "GET",
            path = "/system-config/defaults",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/system-config/defaults")?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<AdminConfigDTO>("config", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetConfigDefaultsBuilder<'a> {
    type Output = Result<AdminConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Config::get_storage_template_options`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetStorageTemplateOptionsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`GetStorageTemplateOptionsBuilder`].
pub type GetStorageTemplateOptionsArgs<'a> = GetStorageTemplateOptionsBuilder<'a>;
impl GetStorageTemplateOptionsBuilder<'_> {
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
    pub async fn send(self) -> Result<ConfigTemplateStorageOptionDTO, crate::Error> {
        tracing::trace!(
            module = "Config",
            method = "GET",
            path = "/system-config/storage-template-options",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/system-config/storage-template-options")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<ConfigTemplateStorageOptionDTO>("config", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetStorageTemplateOptionsBuilder<'a> {
    type Output = Result<ConfigTemplateStorageOptionDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Config::update_admin_config`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateAdminConfigBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AdminConfigDTO,
}

/// Type alias for [`UpdateAdminConfigBuilder`].
pub type UpdateAdminConfigArgs<'a> = UpdateAdminConfigBuilder<'a>;
impl UpdateAdminConfigBuilder<'_> {
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
    pub async fn send(self) -> Result<AdminConfigDTO, crate::Error> {
        tracing::trace!(
            module = "Config",
            method = "PUT",
            path = "/admin/config",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/admin/config")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<AdminConfigDTO>("config", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateAdminConfigBuilder<'a> {
    type Output = Result<AdminConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Config::update_config`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct UpdateConfigBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub body: &'a AdminConfigDTO,
}

/// Type alias for [`UpdateConfigBuilder`].
pub type UpdateConfigArgs<'a> = UpdateConfigBuilder<'a>;
impl UpdateConfigBuilder<'_> {
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
    pub async fn send(self) -> Result<AdminConfigDTO, crate::Error> {
        tracing::trace!(
            module = "Config",
            method = "PUT",
            path = "/system-config",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::PUT, "/system-config")?;
        request = self.request_config.apply(request);
        request = request.json(self.body);

        let response = self.client.execute::<AdminConfigDTO>("config", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for UpdateConfigBuilder<'a> {
    type Output = Result<AdminConfigDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Config` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Config<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Config<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Get the admin configuration
    /// Retrieve admin configuration.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AdminConfigDTO`
    pub fn get_admin_config(&self) -> GetAdminConfigBuilder<'a> {
        GetAdminConfigBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get the system configuration defaults
    /// Retrieve the default value of every system configuration property.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AdminConfigDTO`
    pub fn get_admin_config_defaults(&self) -> GetAdminConfigDefaultsBuilder<'a> {
        GetAdminConfigDefaultsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get system configuration
    /// Retrieve the current system configuration.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AdminConfigDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn get_config(&self) -> GetConfigBuilder<'a> {
        GetConfigBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get system configuration defaults
    /// Retrieve the default values for the system configuration.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AdminConfigDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn get_config_defaults(&self) -> GetConfigDefaultsBuilder<'a> {
        GetConfigDefaultsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Get storage template options
    /// Retrieve exemplary storage template options.
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `ConfigTemplateStorageOptionDTO`
    pub fn get_storage_template_options(&self) -> GetStorageTemplateOptionsBuilder<'a> {
        GetStorageTemplateOptionsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// Update the system configuration
    /// Update the system configuration with a new system configuration.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AdminConfigDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AdminConfigDTO`
    pub fn update_admin_config(&self, body: &'a AdminConfigDTO) -> UpdateAdminConfigBuilder<'a> {
        UpdateAdminConfigBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }

    /// Update system configuration
    /// Update the system configuration with a new system configuration.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `body` | `AdminConfigDTO` | ✅ | Request body |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `AdminConfigDTO`
    #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")]
    pub fn update_config(&self, body: &'a AdminConfigDTO) -> UpdateConfigBuilder<'a> {
        UpdateConfigBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            body,
        }
    }
}
