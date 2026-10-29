// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `plugins`.

/// `PluginMethodResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `description` | [`String`] | ✓ | Description |
/// | `host_functions` | `bool` | ✓ |  |
/// | `key` | [`String`] | ✓ | Key |
/// | `name` | [`String`] | ✓ | Name |
/// | `schema` | `object` |  |  |
/// | `title` | [`String`] | ✓ | Title |
/// | `types` | [`Vec<WorkflowType>`](crate::plugins::WorkflowType) | ✓ | Workflow types |
/// | `ui_hints` | [`Vec<String>`](::std::vec::Vec) | ✓ | Ui hints |
///
/// See: <https://api.immich.app/models/PluginMethodResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PluginMethodResponseDTO {
    pub description: String,
    /// (required)
    #[serde(rename = "hostFunctions")]
    pub host_functions: bool,
    pub key: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub schema: ::serde_json::Map<String, ::serde_json::Value>,
    pub title: String,
    pub types: Vec<WorkflowType>,
    /// Ui hints (required)
    #[serde(rename = "uiHints")]
    pub ui_hints: Vec<String>,
}

impl PluginMethodResponseDTO {
    ///Creates a new [`PluginMethodResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        description: impl Into<String>,
        host_functions: bool,
        key: impl Into<String>,
        name: impl Into<String>,
        title: impl Into<String>,
        types: Vec<WorkflowType>,
        ui_hints: Vec<String>,
    ) -> Self {
        Self {
            description: description.into(),
            host_functions,
            key: key.into(),
            name: name.into(),
            schema: <::serde_json::Map<String, ::serde_json::Value>>::default(),
            title: title.into(),
            types,
            ui_hints,
        }
    }

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = value.into();
        self
    }

    ///Sets `host_functions`.
    #[must_use]
    pub fn host_functions(mut self, value: bool) -> Self {
        self.host_functions = value;
        self
    }

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `schema`.
    #[must_use]
    pub fn schema(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.schema = value;
        self
    }

    ///Sets `title`.
    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }

    ///Sets `types`.
    #[must_use]
    pub fn types(mut self, value: Vec<WorkflowType>) -> Self {
        self.types = value;
        self
    }

    ///Sets `ui_hints`.
    #[must_use]
    pub fn ui_hints(mut self, value: Vec<String>) -> Self {
        self.ui_hints = value;
        self
    }
}

/// `PluginResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `author` | [`String`] | ✓ | Plugin author |
/// | `created_at` | [`String`] | ✓ | Creation date |
/// | `description` | [`String`] | ✓ | Plugin description |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Plugin ID |
/// | `methods` | [`Vec<PluginMethodResponseDTO>`](crate::plugins::PluginMethodResponseDTO) | ✓ | Plugin methods |
/// | `name` | [`String`] | ✓ | Plugin name |
/// | `title` | [`String`] | ✓ | Plugin title |
/// | `updated_at` | [`String`] | ✓ | Last update date |
/// | `version` | [`String`] | ✓ | Plugin version |
///
/// See: <https://api.immich.app/models/PluginResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PluginResponseDTO {
    pub author: String,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub description: String,
    pub id: ::uuid::Uuid,
    pub methods: Vec<PluginMethodResponseDTO>,
    pub name: String,
    pub title: String,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub version: String,
}

impl PluginResponseDTO {
    ///Creates a new [`PluginResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        author: impl Into<String>,
        created_at: impl Into<String>,
        description: impl Into<String>,
        id: ::uuid::Uuid,
        methods: Vec<PluginMethodResponseDTO>,
        name: impl Into<String>,
        title: impl Into<String>,
        updated_at: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            author: author.into(),
            created_at: created_at.into(),
            description: description.into(),
            id,
            methods,
            name: name.into(),
            title: title.into(),
            updated_at: updated_at.into(),
            version: version.into(),
        }
    }

    ///Sets `author`.
    #[must_use]
    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = value.into();
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = value.into();
        self
    }

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = value.into();
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `methods`.
    #[must_use]
    pub fn methods(mut self, value: Vec<PluginMethodResponseDTO>) -> Self {
        self.methods = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `title`.
    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = value.into();
        self
    }

    ///Sets `version`.
    #[must_use]
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = value.into();
        self
    }
}

/// `PluginTemplateResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `description` | [`String`] | ✓ | Template description |
/// | `key` | [`String`] | ✓ | Template key (unique across all templates) |
/// | `steps` | [`Vec<PluginTemplateStepResponseDTO>`](crate::plugins::PluginTemplateStepResponseDTO) | ✓ | Workflow steps |
/// | `title` | [`String`] | ✓ | Template title |
/// | `trigger` | [`WorkflowTrigger`] | ✓ | Workflow trigger |
/// | `ui_hints` | [`Vec<String>`](::std::vec::Vec) | ✓ | Ui hints, for example "smart-album" |
///
/// See: <https://api.immich.app/models/PluginTemplateResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PluginTemplateResponseDTO {
    pub description: String,
    pub key: String,
    pub steps: Vec<PluginTemplateStepResponseDTO>,
    pub title: String,
    pub trigger: WorkflowTrigger,
    /// Ui hints, for example "smart-album" (required)
    #[serde(rename = "uiHints")]
    pub ui_hints: Vec<String>,
}

impl PluginTemplateResponseDTO {
    ///Creates a new [`PluginTemplateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        description: impl Into<String>,
        key: impl Into<String>,
        steps: Vec<PluginTemplateStepResponseDTO>,
        title: impl Into<String>,
        trigger: WorkflowTrigger,
        ui_hints: Vec<String>,
    ) -> Self {
        Self {
            description: description.into(),
            key: key.into(),
            steps,
            title: title.into(),
            trigger,
            ui_hints,
        }
    }

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = value.into();
        self
    }

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }

    ///Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: Vec<PluginTemplateStepResponseDTO>) -> Self {
        self.steps = value;
        self
    }

    ///Sets `title`.
    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }

    ///Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = value;
        self
    }

    ///Sets `ui_hints`.
    #[must_use]
    pub fn ui_hints(mut self, value: Vec<String>) -> Self {
        self.ui_hints = value;
        self
    }
}

/// `PluginTemplateStepResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `config` | `object` | ✓ | Step configuration |
/// | `enabled` | `bool` |  | Whether the step is enabled |
/// | `method` | [`String`] | ✓ | Step plugin method |
///
/// See: <https://api.immich.app/models/PluginTemplateStepResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PluginTemplateStepResponseDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    pub method: String,
}

impl PluginTemplateStepResponseDTO {
    ///Creates a new [`PluginTemplateStepResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            config: None,
            enabled: None,
            method: method.into(),
        }
    }

    ///Sets `config`.
    #[must_use]
    pub fn config(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.config = Some(value);
        self
    }

    ///Sets or clears `config`.
    #[must_use]
    pub fn set_config(mut self, value: Option<::serde_json::Map<String, ::serde_json::Value>>) -> Self {
        self.config = value;
        self
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    ///Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `method`.
    #[must_use]
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = value.into();
        self
    }
}

/// Plugin trigger type
///
/// See: <https://api.immich.app/models/WorkflowTrigger>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum WorkflowTrigger {
    AssetCreate,
    AssetMetadataExtraction,
    AssetTagged,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for WorkflowTrigger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AssetCreate => f.write_str("AssetCreate"),
            Self::AssetMetadataExtraction => f.write_str("AssetMetadataExtraction"),
            Self::AssetTagged => f.write_str("AssetTagged"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for WorkflowTrigger {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "AssetCreate" => Ok(Self::AssetCreate),
            "AssetMetadataExtraction" => Ok(Self::AssetMetadataExtraction),
            "AssetTagged" => Ok(Self::AssetTagged),
            _ => Err(format!("invalid value for {}: {}", "WorkflowTrigger", value).into()),
        }
    }
}

impl TryFrom<&str> for WorkflowTrigger {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for WorkflowTrigger {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for WorkflowTrigger {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Workflow type
///
/// See: <https://api.immich.app/models/WorkflowType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum WorkflowType {
    AssetV1,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AssetV1 => f.write_str("AssetV1"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for WorkflowType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "AssetV1" => Ok(Self::AssetV1),
            _ => Err(format!("invalid value for {}: {}", "WorkflowType", value).into()),
        }
    }
}

impl TryFrom<&str> for WorkflowType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for WorkflowType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for WorkflowType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}
