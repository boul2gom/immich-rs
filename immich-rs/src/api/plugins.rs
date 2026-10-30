// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `plugins`.

use immich_models::id::PluginId;
use immich_models::plugins::{
    PluginMethodResponseDTO, PluginResponseDTO, PluginTemplateResponseDTO, WorkflowTrigger, WorkflowType,
};

/// Fluent request builder for [`Plugins::get_plugin`].
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct GetPluginBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    id: PluginId,
}

impl GetPluginBuilder<'_> {
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
    pub async fn send(self) -> Result<PluginResponseDTO, crate::Error> {
        tracing::trace!(
            module = "plugins",
            method = "GET",
            path = "/plugins/{id}",
            "HTTP request"
        );
        let __seg_1 = self.id.to_string();
        let mut request = self.client.prepare_request_segments(
            reqwest::Method::GET,
            &[
                crate::core::PathSegment::Static("plugins"),
                crate::core::PathSegment::Param(__seg_1.as_str()),
            ],
        )?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchPluginMethodsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    description: Option<String>,
    enabled: Option<bool>,
    id: Option<uuid::Uuid>,
    name: Option<String>,
    plugin_name: Option<String>,
    plugin_version: Option<String>,
    title: Option<String>,
    trigger: Option<WorkflowTrigger>,
    type_: Option<WorkflowType>,
}

impl SearchPluginMethodsBuilder<'_> {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<PluginMethodResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
            description: Option<&'b String>,
            #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
            enabled: Option<&'b bool>,
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b uuid::Uuid>,
            #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
            name: Option<&'b String>,
            #[serde(rename = "pluginName", skip_serializing_if = "Option::is_none")]
            plugin_name: Option<&'b String>,
            #[serde(rename = "pluginVersion", skip_serializing_if = "Option::is_none")]
            plugin_version: Option<&'b String>,
            #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
            title: Option<&'b String>,
            #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
            trigger: Option<&'b WorkflowTrigger>,
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            type_: Option<&'b WorkflowType>,
        }
        tracing::trace!(
            module = "plugins",
            method = "GET",
            path = "/plugins/methods",
            "HTTP request"
        );
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/plugins/methods")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            description: self.description.as_ref(),
            enabled: self.enabled.as_ref(),
            id: self.id.as_ref(),
            name: self.name.as_ref(),
            plugin_name: self.plugin_name.as_ref(),
            plugin_version: self.plugin_version.as_ref(),
            title: self.title.as_ref(),
            trigger: self.trigger.as_ref(),
            type_: self.type_.as_ref(),
        };
        request = request.query(&query);
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchPluginTemplatesBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
}

impl SearchPluginTemplatesBuilder<'_> {
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
    pub async fn send(self) -> Result<Vec<PluginTemplateResponseDTO>, crate::Error> {
        tracing::trace!(
            module = "plugins",
            method = "GET",
            path = "/plugins/templates",
            "HTTP request"
        );
        let mut request = self
            .client
            .prepare_request(reqwest::Method::GET, "/plugins/templates")?;
        request = self.request_config.apply(request)?;
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
#[derive(Debug, Clone)]
#[must_use = "request builders are lazy and do nothing unless awaited or sent"]
pub struct SearchPluginsBuilder<'a> {
    client: &'a crate::ImmichClient,
    request_config: crate::core::RequestConfig,
    description: Option<String>,
    enabled: Option<bool>,
    id: Option<PluginId>,
    name: Option<String>,
    title: Option<String>,
    version: Option<String>,
}

impl SearchPluginsBuilder<'_> {
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
    pub fn id(mut self, value: impl Into<PluginId>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets or clears `id`.
    pub fn set_id(mut self, value: Option<PluginId>) -> Self {
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
    /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
    pub async fn send(self) -> Result<Vec<PluginResponseDTO>, crate::Error> {
        #[derive(serde::Serialize)]
        struct QueryParams<'b> {
            #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
            description: Option<&'b String>,
            #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
            enabled: Option<&'b bool>,
            #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
            id: Option<&'b PluginId>,
            #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
            name: Option<&'b String>,
            #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
            title: Option<&'b String>,
            #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
            version: Option<&'b String>,
        }
        tracing::trace!(module = "plugins", method = "GET", path = "/plugins", "HTTP request");
        let mut request = self.client.prepare_request(reqwest::Method::GET, "/plugins")?;
        request = self.request_config.apply(request)?;
        let query = QueryParams {
            description: self.description.as_ref(),
            enabled: self.enabled.as_ref(),
            id: self.id.as_ref(),
            name: self.name.as_ref(),
            title: self.title.as_ref(),
            version: self.version.as_ref(),
        };
        request = request.query(&query);
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

/// Client handle for `Plugins` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Plugins<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> Plugins<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Retrieve a plugin
    ///
    /// Retrieve information about a specific plugin by its ID.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `id` | `PluginId` | ✓ |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `PluginResponseDTO`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn get_plugin(&self, id: impl Into<PluginId>) -> GetPluginBuilder<'a> {
        GetPluginBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
            id: id.into(),
        }
    }
    /// Retrieve plugin methods
    ///
    /// Retrieve a list of plugin methods
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `description` | `String` |  |  |
    /// | `enabled` | `bool` |  | Whether the plugin method is enabled |
    /// | `id` | `uuid::Uuid` |  | Plugin method ID |
    /// | `name` | `String` |  |  |
    /// | `plugin_name` | `String` |  | Plugin name |
    /// | `plugin_version` | `String` |  | Plugin version |
    /// | `title` | `String` |  |  |
    /// | `trigger` | `WorkflowTrigger` |  | Workflow trigger |
    /// | `type_` | `WorkflowType` |  | Workflow types |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PluginMethodResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
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
    ///
    /// Retrieve workflow templates provided by installed plugins
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PluginTemplateResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
    pub fn search_plugin_templates(&self) -> SearchPluginTemplatesBuilder<'a> {
        SearchPluginTemplatesBuilder {
            client: self.client,
            request_config: crate::core::RequestConfig::default(),
        }
    }
    /// List all plugins
    ///
    /// Retrieve a list of plugins available to the authenticated user.
    ///
    /// ### Parameters
    ///
    /// | Name | Type | Required | Description |
    /// |------|------|:--------:|-------------|
    /// | `description` | `String` |  |  |
    /// | `enabled` | `bool` |  | Whether the plugin is enabled |
    /// | `id` | `PluginId` |  | Plugin ID |
    /// | `name` | `String` |  |  |
    /// | `title` | `String` |  |  |
    /// | `version` | `String` |  |  |
    ///
    /// ### Responses
    ///
    /// | Code | Description |
    /// |------|-------------|
    /// | `200` |  |
    ///
    /// **Returns:** `Vec<PluginResponseDTO>`
    ///
    /// # Errors
    /// Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.
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
