// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `workflows`.

pub use crate::id::WorkflowId;
pub use crate::plugins::{WorkflowTrigger, WorkflowType};

/// `WorkflowCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `description` | [`String`] |  | Workflow description |
/// | `enabled` | `bool` |  | Workflow enabled |
/// | `logging` | `bool` |  | Workflow logs run results |
/// | `name` | [`String`] |  | Workflow name |
/// | `steps` | [`Vec<WorkflowStepDTO>`](crate::workflows::WorkflowStepDTO) |  |  |
/// | `trigger` | [`WorkflowTrigger`] | ✓ | Workflow trigger type |
///
/// See: <https://api.immich.app/models/WorkflowCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct WorkflowCreateDTO {
    /// Workflow description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Workflow enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// Workflow logs run results
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: Option<bool>,
    /// Workflow name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub steps: Vec<WorkflowStepDTO>,
    /// Workflow trigger type (required)
    pub trigger: WorkflowTrigger,
}

impl WorkflowCreateDTO {
    /// Creates a new [`WorkflowCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(trigger: WorkflowTrigger) -> Self {
        Self {
            description: None,
            enabled: None,
            logging: None,
            name: None,
            steps: Vec::new(),
            trigger,
        }
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
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
    /// Sets `logging`.
    #[must_use]
    pub fn logging(mut self, value: bool) -> Self {
        self.logging = Some(value);
        self
    }
    /// Sets or clears `logging`.
    #[must_use]
    pub fn set_logging(mut self, value: Option<bool>) -> Self {
        self.logging = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: impl IntoIterator<Item = WorkflowStepDTO>) -> Self {
        self.steps = value.into_iter().collect();
        self
    }
    /// Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = value;
        self
    }
}

/// `WorkflowLogEntryDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `at` | [`DateTime`](::chrono::DateTime) | ✓ | Workflow run date/time (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Workflow log entry ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `last_step` | `object` |  | Last step ran, if the workflow ended early |
/// | `result` | [`WorkflowResult`] | ✓ |  |
/// | `trigger_data_id` | [`Uuid`](::uuid::Uuid) |  | Workflow trigger data ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\|00000000-0000-0000-0000-000000000000\|ffffffff-ffff-ffff-ffff-ffffffffffff)$`) |
///
/// See: <https://api.immich.app/models/WorkflowLogEntryDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct WorkflowLogEntryDTO {
    /// Workflow run date/time (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    pub at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Workflow log entry ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: ::uuid::Uuid,
    /// Last step ran, if the workflow ended early
    #[serde(rename = "lastStep", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub last_step: Option<WorkflowLogEntryLastStep>,
    /// (required)
    pub result: WorkflowResult,
    /// Workflow trigger data ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$`)
    #[serde(
        rename = "triggerDataId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trigger_data_id: Option<::uuid::Uuid>,
}

impl WorkflowLogEntryDTO {
    /// Creates a new [`WorkflowLogEntryDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(at: ::chrono::DateTime<::chrono::offset::Utc>, id: ::uuid::Uuid, result: WorkflowResult) -> Self {
        Self {
            at,
            id,
            last_step: None,
            result,
            trigger_data_id: None,
        }
    }
    /// Sets `at`.
    #[must_use]
    pub fn at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }
    /// Sets `last_step`.
    #[must_use]
    pub fn last_step(mut self, value: WorkflowLogEntryLastStep) -> Self {
        self.last_step = Some(value);
        self
    }
    /// Sets or clears `last_step`.
    #[must_use]
    pub fn set_last_step(mut self, value: Option<WorkflowLogEntryLastStep>) -> Self {
        self.last_step = value;
        self
    }
    /// Sets `result`.
    #[must_use]
    pub fn result(mut self, value: WorkflowResult) -> Self {
        self.result = value;
        self
    }
    /// Sets `trigger_data_id`.
    #[must_use]
    pub fn trigger_data_id(mut self, value: ::uuid::Uuid) -> Self {
        self.trigger_data_id = Some(value);
        self
    }
    /// Sets or clears `trigger_data_id`.
    #[must_use]
    pub fn set_trigger_data_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.trigger_data_id = value;
        self
    }
}

/// Last step ran, if the workflow ended early
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct WorkflowLogEntryLastStep {
    /// Index of the step in the workflow
    pub index: ::std::num::NonZeroU64,
    /// Method of the step
    pub method: String,
}

impl WorkflowLogEntryLastStep {
    /// Creates a new [`WorkflowLogEntryLastStep`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(index: ::std::num::NonZeroU64, method: impl Into<String>) -> Self {
        Self {
            index,
            method: method.into(),
        }
    }
    /// Sets `index`.
    #[must_use]
    pub fn index(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.index = value;
        self
    }
    /// Sets `method`.
    #[must_use]
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = value.into();
        self
    }
}

/// `WorkflowResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`String`] | ✓ | Creation date |
/// | `description` | [`String`] | ✓ | Workflow description |
/// | `enabled` | `bool` | ✓ | Workflow enabled |
/// | `id` | [`WorkflowId`](crate::id::WorkflowId) | ✓ | Workflow ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `logging` | `bool` | ✓ | Workflow logs run results |
/// | `name` | [`String`] | ✓ | Workflow name |
/// | `steps` | [`Vec<WorkflowStepDTO>`](crate::workflows::WorkflowStepDTO) | ✓ | Workflow steps |
/// | `trigger` | [`WorkflowTrigger`] | ✓ | Workflow trigger type |
/// | `updated_at` | [`String`] | ✓ | Update date |
///
/// See: <https://api.immich.app/models/WorkflowResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct WorkflowResponseDTO {
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Workflow description (required)
    pub description: Option<String>,
    /// Workflow enabled (required)
    pub enabled: bool,
    /// Workflow ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: WorkflowId,
    /// Workflow logs run results (required)
    pub logging: bool,
    /// Workflow name (required)
    pub name: Option<String>,
    /// Workflow steps (required)
    pub steps: Vec<WorkflowStepDTO>,
    /// Workflow trigger type (required)
    pub trigger: WorkflowTrigger,
    /// Update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl WorkflowResponseDTO {
    /// Creates a new [`WorkflowResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: impl Into<String>,
        enabled: bool,
        id: impl Into<WorkflowId>,
        logging: bool,
        steps: impl IntoIterator<Item = WorkflowStepDTO>,
        trigger: WorkflowTrigger,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            created_at: created_at.into(),
            description: None,
            enabled,
            id: id.into(),
            logging,
            name: None,
            steps: steps.into_iter().collect(),
            trigger,
            updated_at: updated_at.into(),
        }
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
        self.description = Some(value.into());
        self
    }
    /// Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<WorkflowId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `logging`.
    #[must_use]
    pub fn logging(mut self, value: bool) -> Self {
        self.logging = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: impl IntoIterator<Item = WorkflowStepDTO>) -> Self {
        self.steps = value.into_iter().collect();
        self
    }
    /// Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = value;
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = value.into();
        self
    }
}

/// Workflow run result
///
/// See: <https://api.immich.app/models/WorkflowResult>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum WorkflowResult {
    Completed,
    Halted,
    Error,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl WorkflowResult {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Completed => "completed",
            Self::Halted => "halted",
            Self::Error => "error",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for WorkflowResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for WorkflowResult {
    fn from(value: &str) -> Self {
        match value {
            "completed" => Self::Completed,
            "halted" => Self::Halted,
            "error" => Self::Error,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for WorkflowResult {
    fn from(value: String) -> Self {
        match value.as_str() {
            "completed" => Self::Completed,
            "halted" => Self::Halted,
            "error" => Self::Error,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for WorkflowResult {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for WorkflowResult {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for WorkflowResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkflowResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = WorkflowResult;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(WorkflowResult::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(WorkflowResult::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `WorkflowShareResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `description` | [`String`] | ✓ | Workflow description |
/// | `name` | [`String`] | ✓ | Workflow name |
/// | `steps` | [`Vec<WorkflowShareStepDTO>`](crate::workflows::WorkflowShareStepDTO) | ✓ | Workflow steps |
/// | `trigger` | [`WorkflowTrigger`] | ✓ | Workflow trigger type |
///
/// See: <https://api.immich.app/models/WorkflowShareResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct WorkflowShareResponseDTO {
    /// Workflow description (required)
    pub description: Option<String>,
    /// Workflow name (required)
    pub name: Option<String>,
    /// Workflow steps (required)
    pub steps: Vec<WorkflowShareStepDTO>,
    /// Workflow trigger type (required)
    pub trigger: WorkflowTrigger,
}

impl WorkflowShareResponseDTO {
    /// Creates a new [`WorkflowShareResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(steps: impl IntoIterator<Item = WorkflowShareStepDTO>, trigger: WorkflowTrigger) -> Self {
        Self {
            description: None,
            name: None,
            steps: steps.into_iter().collect(),
            trigger,
        }
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: impl IntoIterator<Item = WorkflowShareStepDTO>) -> Self {
        self.steps = value.into_iter().collect();
        self
    }
    /// Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = value;
        self
    }
}

/// `WorkflowShareStepDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `config` | `object` | ✓ | Step configuration |
/// | `enabled` | `bool` |  | Step is enabled |
/// | `method` | [`String`] | ✓ | Step plugin method |
///
/// See: <https://api.immich.app/models/WorkflowShareStepDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct WorkflowShareStepDTO {
    /// Step configuration (required)
    pub config: Option<::serde_json::Map<String, ::serde_json::Value>>,
    /// Step is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// Step plugin method (required)
    pub method: String,
}

impl WorkflowShareStepDTO {
    /// Creates a new [`WorkflowShareStepDTO`].
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

/// `WorkflowStepDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `config` | `object` | ✓ | Step configuration |
/// | `enabled` | `bool` |  | Step is enabled |
/// | `method` | [`String`] | ✓ | Step plugin method |
///
/// See: <https://api.immich.app/models/WorkflowStepDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct WorkflowStepDTO {
    /// Step configuration (required)
    pub config: Option<::serde_json::Map<String, ::serde_json::Value>>,
    /// Step is enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// Step plugin method (required)
    pub method: String,
}

impl WorkflowStepDTO {
    /// Creates a new [`WorkflowStepDTO`].
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

/// `WorkflowTriggerResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `trigger` | [`WorkflowTrigger`] | ✓ | Trigger type |
/// | `types` | [`Vec<WorkflowType>`](crate::plugins::WorkflowType) | ✓ | Workflow types |
///
/// See: <https://api.immich.app/models/WorkflowTriggerResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct WorkflowTriggerResponseDTO {
    /// Trigger type (required)
    pub trigger: WorkflowTrigger,
    /// Workflow types (required)
    pub types: Vec<WorkflowType>,
}

impl WorkflowTriggerResponseDTO {
    /// Creates a new [`WorkflowTriggerResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(trigger: WorkflowTrigger, types: impl IntoIterator<Item = WorkflowType>) -> Self {
        Self {
            trigger,
            types: types.into_iter().collect(),
        }
    }
    /// Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = value;
        self
    }
    /// Sets `types`.
    #[must_use]
    pub fn types(mut self, value: impl IntoIterator<Item = WorkflowType>) -> Self {
        self.types = value.into_iter().collect();
        self
    }
}

/// `WorkflowUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `description` | [`String`] |  | Workflow description |
/// | `enabled` | `bool` |  | Workflow enabled |
/// | `logging` | `bool` |  | Workflow logs run results |
/// | `name` | [`String`] |  | Workflow name |
/// | `steps` | [`Vec<WorkflowStepDTO>`](crate::workflows::WorkflowStepDTO) |  |  |
/// | `trigger` | [`WorkflowTrigger`] |  | Workflow trigger type |
///
/// See: <https://api.immich.app/models/WorkflowUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub struct WorkflowUpdateDTO {
    /// Workflow description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Workflow enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// Workflow logs run results
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: Option<bool>,
    /// Workflow name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub steps: Vec<WorkflowStepDTO>,
    /// Workflow trigger type
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger: Option<WorkflowTrigger>,
}

impl WorkflowUpdateDTO {
    /// Creates a new [`WorkflowUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            description: None,
            enabled: None,
            logging: None,
            name: None,
            steps: Vec::new(),
            trigger: None,
        }
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
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
    /// Sets `logging`.
    #[must_use]
    pub fn logging(mut self, value: bool) -> Self {
        self.logging = Some(value);
        self
    }
    /// Sets or clears `logging`.
    #[must_use]
    pub fn set_logging(mut self, value: Option<bool>) -> Self {
        self.logging = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: impl IntoIterator<Item = WorkflowStepDTO>) -> Self {
        self.steps = value.into_iter().collect();
        self
    }
    /// Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = Some(value);
        self
    }
    /// Sets or clears `trigger`.
    #[must_use]
    pub fn set_trigger(mut self, value: Option<WorkflowTrigger>) -> Self {
        self.trigger = value;
        self
    }
}
