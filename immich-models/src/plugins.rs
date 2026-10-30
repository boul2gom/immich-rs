// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `plugins`.

pub use crate::id::PluginId;

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
#[non_exhaustive]
pub struct PluginMethodResponseDTO {
    /// Description (required)
    pub description: String,
    /// (required)
    #[serde(rename = "hostFunctions")]
    pub host_functions: bool,
    /// Key (required)
    pub key: String,
    /// Name (required)
    pub name: String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub schema: ::serde_json::Map<String, ::serde_json::Value>,
    /// Title (required)
    pub title: String,
    /// Workflow types (required)
    pub types: Vec<WorkflowType>,
    /// Ui hints (required)
    #[serde(rename = "uiHints")]
    pub ui_hints: Vec<String>,
}

impl PluginMethodResponseDTO {
    /// Creates a new [`PluginMethodResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        description: impl Into<String>,
        host_functions: bool,
        key: impl Into<String>,
        name: impl Into<String>,
        title: impl Into<String>,
        types: impl IntoIterator<Item = WorkflowType>,
        ui_hints: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            description: description.into(),
            host_functions,
            key: key.into(),
            name: name.into(),
            schema: <::serde_json::Map<String, ::serde_json::Value>>::default(),
            title: title.into(),
            types: types.into_iter().collect(),
            ui_hints: ui_hints.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = value.into();
        self
    }
    /// Sets `host_functions`.
    #[must_use]
    pub fn host_functions(mut self, value: bool) -> Self {
        self.host_functions = value;
        self
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `schema`.
    #[must_use]
    pub fn schema(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.schema = value;
        self
    }
    /// Sets `title`.
    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }
    /// Sets `types`.
    #[must_use]
    pub fn types(mut self, value: impl IntoIterator<Item = WorkflowType>) -> Self {
        self.types = value.into_iter().collect();
        self
    }
    /// Sets `ui_hints`.
    #[must_use]
    pub fn ui_hints(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.ui_hints = value.into_iter().map(Into::into).collect();
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
/// | `id` | [`PluginId`](crate::id::PluginId) | ✓ | Plugin ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `methods` | [`Vec<PluginMethodResponseDTO>`](crate::plugins::PluginMethodResponseDTO) | ✓ | Plugin methods |
/// | `name` | [`String`] | ✓ | Plugin name |
/// | `title` | [`String`] | ✓ | Plugin title |
/// | `updated_at` | [`String`] | ✓ | Last update date |
/// | `version` | [`String`] | ✓ | Plugin version |
///
/// See: <https://api.immich.app/models/PluginResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PluginResponseDTO {
    /// Plugin author (required)
    pub author: String,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Plugin description (required)
    pub description: String,
    /// Plugin ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: PluginId,
    /// Plugin methods (required)
    pub methods: Vec<PluginMethodResponseDTO>,
    /// Plugin name (required)
    pub name: String,
    /// Plugin title (required)
    pub title: String,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// Plugin version (required)
    pub version: String,
}

impl PluginResponseDTO {
    /// Creates a new [`PluginResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        author: impl Into<String>,
        created_at: impl Into<String>,
        description: impl Into<String>,
        id: impl Into<PluginId>,
        methods: impl IntoIterator<Item = PluginMethodResponseDTO>,
        name: impl Into<String>,
        title: impl Into<String>,
        updated_at: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            author: author.into(),
            created_at: created_at.into(),
            description: description.into(),
            id: id.into(),
            methods: methods.into_iter().collect(),
            name: name.into(),
            title: title.into(),
            updated_at: updated_at.into(),
            version: version.into(),
        }
    }
    /// Sets `author`.
    #[must_use]
    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = value.into();
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = value.into();
        self
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = value.into();
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<PluginId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `methods`.
    #[must_use]
    pub fn methods(mut self, value: impl IntoIterator<Item = PluginMethodResponseDTO>) -> Self {
        self.methods = value.into_iter().collect();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `title`.
    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = value.into();
        self
    }
    /// Sets `version`.
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
#[non_exhaustive]
pub struct PluginTemplateResponseDTO {
    /// Template description (required)
    pub description: String,
    /// Template key (unique across all templates) (required)
    pub key: String,
    /// Workflow steps (required)
    pub steps: Vec<PluginTemplateStepResponseDTO>,
    /// Template title (required)
    pub title: String,
    /// Workflow trigger (required)
    pub trigger: WorkflowTrigger,
    /// Ui hints, for example "smart-album" (required)
    #[serde(rename = "uiHints")]
    pub ui_hints: Vec<String>,
}

impl PluginTemplateResponseDTO {
    /// Creates a new [`PluginTemplateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        description: impl Into<String>,
        key: impl Into<String>,
        steps: impl IntoIterator<Item = PluginTemplateStepResponseDTO>,
        title: impl Into<String>,
        trigger: WorkflowTrigger,
        ui_hints: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            description: description.into(),
            key: key.into(),
            steps: steps.into_iter().collect(),
            title: title.into(),
            trigger,
            ui_hints: ui_hints.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = value.into();
        self
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }
    /// Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: impl IntoIterator<Item = PluginTemplateStepResponseDTO>) -> Self {
        self.steps = value.into_iter().collect();
        self
    }
    /// Sets `title`.
    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }
    /// Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = value;
        self
    }
    /// Sets `ui_hints`.
    #[must_use]
    pub fn ui_hints(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.ui_hints = value.into_iter().map(Into::into).collect();
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
#[non_exhaustive]
pub struct PluginTemplateStepResponseDTO {
    /// Step configuration (required)
    pub config: Option<::serde_json::Map<String, ::serde_json::Value>>,
    /// Whether the step is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// Step plugin method (required)
    pub method: String,
}

impl PluginTemplateStepResponseDTO {
    /// Creates a new [`PluginTemplateStepResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            config: None,
            enabled: None,
            method: method.into(),
        }
    }
    /// Sets `config`.
    #[must_use]
    pub fn config(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.config = Some(value);
        self
    }
    /// Sets or clears `config`.
    #[must_use]
    pub fn set_config(mut self, value: Option<::serde_json::Map<String, ::serde_json::Value>>) -> Self {
        self.config = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `method`.
    #[must_use]
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = value.into();
        self
    }
}

/// Plugin trigger type
///
/// See: <https://api.immich.app/models/WorkflowTrigger>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum WorkflowTrigger {
    AssetCreate,
    AssetMetadataExtraction,
    AssetTagged,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl WorkflowTrigger {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AssetCreate => "AssetCreate",
            Self::AssetMetadataExtraction => "AssetMetadataExtraction",
            Self::AssetTagged => "AssetTagged",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for WorkflowTrigger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for WorkflowTrigger {
    fn from(value: &str) -> Self {
        match value {
            "AssetCreate" => Self::AssetCreate,
            "AssetMetadataExtraction" => Self::AssetMetadataExtraction,
            "AssetTagged" => Self::AssetTagged,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for WorkflowTrigger {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AssetCreate" => Self::AssetCreate,
            "AssetMetadataExtraction" => Self::AssetMetadataExtraction,
            "AssetTagged" => Self::AssetTagged,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for WorkflowTrigger {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for WorkflowTrigger {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for WorkflowTrigger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkflowTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = WorkflowTrigger;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(WorkflowTrigger::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(WorkflowTrigger::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Workflow type
///
/// See: <https://api.immich.app/models/WorkflowType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum WorkflowType {
    AssetV1,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl WorkflowType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AssetV1 => "AssetV1",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for WorkflowType {
    fn from(value: &str) -> Self {
        match value {
            "AssetV1" => Self::AssetV1,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for WorkflowType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AssetV1" => Self::AssetV1,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for WorkflowType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for WorkflowType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for WorkflowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkflowType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = WorkflowType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(WorkflowType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(WorkflowType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}
