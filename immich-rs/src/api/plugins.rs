// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! API endpoints for `plugins`.

use immich_models::plugins::{
    PluginMethodResponseDTO, PluginResponseDTO, PluginTemplateResponseDTO, WorkflowTrigger, WorkflowType,
};

/// Fluent request builder for [`Plugins::get_plugin`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPluginBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(skip)]
    pub id: uuid::Uuid,
}

/// Type alias for [`GetPluginBuilder`].
pub type GetPluginArgs<'a> = GetPluginBuilder<'a>;
impl GetPluginBuilder<'_> {
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
    pub async fn send(self) -> Result<PluginResponseDTO, crate::Error> {
        tracing::trace!(
            module = "Plugins",
            method = "GET",
            path = "/plugins/{id}",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, &format!("/plugins/{}", self.id))?;
        request = self.request_config.apply(request);

        let response = self.client.execute::<PluginResponseDTO>("plugins", request).await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for GetPluginBuilder<'a> {
    type Output = Result<PluginResponseDTO, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Plugins::search_plugin_methods`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchPluginMethodsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pluginName", skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
    #[serde(rename = "pluginVersion", skip_serializing_if = "Option::is_none")]
    pub plugin_version: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<WorkflowTrigger>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<WorkflowType>,
}

/// Type alias for [`SearchPluginMethodsBuilder`].
pub type SearchPluginMethodsArgs<'a> = SearchPluginMethodsBuilder<'a>;
impl SearchPluginMethodsBuilder<'_> {
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

    /// Sets `description`.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Sets or clears `description`.
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }

    /// Whether the plugin method is enabled
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Sets or clears `enabled`.
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }

    /// Plugin method ID
    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.id = value;
        self
    }

    /// Sets `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets or clears `name`.
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }

    /// Plugin name
    pub fn plugin_name(mut self, value: impl Into<String>) -> Self {
        self.plugin_name = Some(value.into());
        self
    }

    /// Sets or clears `pluginName`.
    pub fn set_plugin_name(mut self, value: Option<String>) -> Self {
        self.plugin_name = value;
        self
    }

    /// Plugin version
    pub fn plugin_version(mut self, value: impl Into<String>) -> Self {
        self.plugin_version = Some(value.into());
        self
    }

    /// Sets or clears `pluginVersion`.
    pub fn set_plugin_version(mut self, value: Option<String>) -> Self {
        self.plugin_version = value;
        self
    }

    /// Sets `title`.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Sets or clears `title`.
    pub fn set_title(mut self, value: Option<String>) -> Self {
        self.title = value;
        self
    }

    /// Workflow trigger
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = Some(value);
        self
    }

    /// Sets or clears `trigger`.
    pub fn set_trigger(mut self, value: Option<WorkflowTrigger>) -> Self {
        self.trigger = value;
        self
    }

    /// Workflow types
    pub fn type_(mut self, value: WorkflowType) -> Self {
        self.type_ = Some(value);
        self
    }

    /// Sets or clears `type`.
    pub fn set_type_(mut self, value: Option<WorkflowType>) -> Self {
        self.type_ = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<PluginMethodResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Plugins",
            method = "GET",
            path = "/plugins/methods",
            "HTTP request"
        );

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/plugins/methods")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<Vec<PluginMethodResponseDTO>>("plugins", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchPluginMethodsBuilder<'a> {
    type Output = Result<Vec<PluginMethodResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Plugins::search_plugin_templates`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchPluginTemplatesBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
}

/// Type alias for [`SearchPluginTemplatesBuilder`].
pub type SearchPluginTemplatesArgs<'a> = SearchPluginTemplatesBuilder<'a>;
impl SearchPluginTemplatesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<PluginTemplateResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "Plugins",
            method = "GET",
            path = "/plugins/templates",
            "HTTP request"
        );

        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/plugins/templates")?;
        request = self.request_config.apply(request);

        let response = self
            .client
            .execute::<Vec<PluginTemplateResponseDTO>>("plugins", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchPluginTemplatesBuilder<'a> {
    type Output = Result<Vec<PluginTemplateResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

/// Fluent request builder for [`Plugins::search_plugins`].
#[derive(serde::Serialize, Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchPluginsBuilder<'a> {
    #[serde(skip)]
    client: &'a crate::ImmichClient,
    #[serde(skip)]
    pub request_config: crate::core::RequestConfig,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Type alias for [`SearchPluginsBuilder`].
pub type SearchPluginsArgs<'a> = SearchPluginsBuilder<'a>;
impl SearchPluginsBuilder<'_> {
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

    /// Sets `description`.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Sets or clears `description`.
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }

    /// Whether the plugin is enabled
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Sets or clears `enabled`.
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }

    /// Plugin ID
    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.id = value;
        self
    }

    /// Sets `name`.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Sets or clears `name`.
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }

    /// Sets `title`.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Sets or clears `title`.
    pub fn set_title(mut self, value: Option<String>) -> Self {
        self.title = value;
        self
    }

    /// Sets `version`.
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Sets or clears `version`.
    pub fn set_version(mut self, value: Option<String>) -> Self {
        self.version = value;
        self
    }

    /// Sends the HTTP request and awaits the server response.
    ///
    /// # Errors
    ///
    /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
    pub async fn send(self) -> Result<Vec<PluginResponseDTO>, crate::Error> {
        tracing::trace!(module = "Plugins", method = "GET", path = "/plugins", "HTTP request");

        let mut request = self.client.prepare_request(reqwest::Method::GET, "/plugins")?;
        request = self.request_config.apply(request);
        request = request.query(&self);

        let response = self
            .client
            .execute::<Vec<PluginResponseDTO>>("plugins", request)
            .await?;
        Ok(response)
    }
}

impl<'a> std::future::IntoFuture for SearchPluginsBuilder<'a> {
    type Output = Result<Vec<PluginResponseDTO>, crate::Error>;
    type IntoFuture = crate::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

///Client handle for `Plugins` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Plugins<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Plugins<'a> {
    pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }

    /// Retrieve a plugin
    /// Retrieve information about a specific plugin by its ID.
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
    /// **Returns:** `PluginResponseDTO`
    pub fn get_plugin(&self, id: uuid::Uuid) -> GetPluginBuilder<'a> {
        GetPluginBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id,
        }
    }

    /// Retrieve plugin methods
    /// Retrieve a list of plugin methods
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `description` | `String` |  |  |
    /// | `enabled` | `bool` |  | Whether the plugin method is enabled |
    /// | `id` | `uuid::Uuid` |  | Plugin method ID |
    /// | `name` | `String` |  |  |
    /// | `pluginName` | `String` |  | Plugin name |
    /// | `pluginVersion` | `String` |  | Plugin version |
    /// | `title` | `String` |  |  |
    /// | `trigger` | `WorkflowTrigger` |  | Workflow trigger |
    /// | `type` | `WorkflowType` |  | Workflow types |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PluginMethodResponseDTO>`
    pub fn search_plugin_methods(&self) -> SearchPluginMethodsBuilder<'a> {
        SearchPluginMethodsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            description: None,
            enabled: None,
            id: None,
            name: None,
            plugin_name: None,
            plugin_version: None,
            title: None,
            trigger: None,
            type_: None,
        }
    }

    /// Retrieve workflow templates
    /// Retrieve workflow templates provided by installed plugins
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PluginTemplateResponseDTO>`
    pub fn search_plugin_templates(&self) -> SearchPluginTemplatesBuilder<'a> {
        SearchPluginTemplatesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }

    /// List all plugins
    /// Retrieve a list of plugins available to the authenticated user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |--------|--------|:--------:|--------|
    /// | `description` | `String` |  |  |
    /// | `enabled` | `bool` |  | Whether the plugin is enabled |
    /// | `id` | `uuid::Uuid` |  | Plugin ID |
    /// | `name` | `String` |  |  |
    /// | `title` | `String` |  |  |
    /// | `version` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |--------|--------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PluginResponseDTO>`
    pub fn search_plugins(&self) -> SearchPluginsBuilder<'a> {
        SearchPluginsBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            description: None,
            enabled: None,
            id: None,
            name: None,
            title: None,
            version: None,
        }
    }
}
