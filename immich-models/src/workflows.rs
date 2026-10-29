// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `workflows`.

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
pub struct WorkflowCreateDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub steps: Vec<WorkflowStepDTO>,
    pub trigger: WorkflowTrigger,
}

impl WorkflowCreateDTO {
    ///Creates a new [`WorkflowCreateDTO`].
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

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    ///Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
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

    ///Sets `logging`.
    #[must_use]
    pub fn logging(mut self, value: bool) -> Self {
        self.logging = Some(value);
        self
    }

    ///Sets or clears `logging`.
    #[must_use]
    pub fn set_logging(mut self, value: Option<bool>) -> Self {
        self.logging = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }

    ///Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: Vec<WorkflowStepDTO>) -> Self {
        self.steps = value;
        self
    }

    ///Sets `trigger`.
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
/// | `at` | [`DateTime`](::chrono::DateTime) | ✓ | Workflow run date/time |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Workflow log entry ID |
/// | `last_step` | `object` |  | Last step ran, if the workflow ended early |
/// | `result` | [`WorkflowResult`] | ✓ |  |
/// | `trigger_data_id` | [`Uuid`](::uuid::Uuid) |  | Workflow trigger data ID |
///
/// See: <https://api.immich.app/models/WorkflowLogEntryDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowLogEntryDTO {
    pub at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::uuid::Uuid,
    /// Last step ran, if the workflow ended early
    #[serde(rename = "lastStep", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub last_step: Option<WorkflowLogEntryLastStep>,
    pub result: WorkflowResult,
    /// Workflow trigger data ID
    #[serde(
        rename = "triggerDataId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trigger_data_id: Option<::uuid::Uuid>,
}

impl WorkflowLogEntryDTO {
    ///Creates a new [`WorkflowLogEntryDTO`].
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

    ///Sets `at`.
    #[must_use]
    pub fn at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `last_step`.
    #[must_use]
    pub fn last_step(mut self, value: WorkflowLogEntryLastStep) -> Self {
        self.last_step = Some(value);
        self
    }

    ///Sets or clears `last_step`.
    #[must_use]
    pub fn set_last_step(mut self, value: Option<WorkflowLogEntryLastStep>) -> Self {
        self.last_step = value;
        self
    }

    ///Sets `result`.
    #[must_use]
    pub fn result(mut self, value: WorkflowResult) -> Self {
        self.result = value;
        self
    }

    ///Sets `trigger_data_id`.
    #[must_use]
    pub fn trigger_data_id(mut self, value: ::uuid::Uuid) -> Self {
        self.trigger_data_id = Some(value);
        self
    }

    ///Sets or clears `trigger_data_id`.
    #[must_use]
    pub fn set_trigger_data_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.trigger_data_id = value;
        self
    }
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowLogEntryLastStep {
    ///Index of the step in the workflow
    pub index: ::std::num::NonZeroU64,
    ///Method of the step
    pub method: String,
}

impl WorkflowLogEntryLastStep {
    ///Creates a new [`WorkflowLogEntryLastStep`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(index: ::std::num::NonZeroU64, method: impl Into<String>) -> Self {
        Self {
            index,
            method: method.into(),
        }
    }

    ///Sets `index`.
    #[must_use]
    pub fn index(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.index = value;
        self
    }

    ///Sets `method`.
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
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Workflow ID |
/// | `logging` | `bool` | ✓ | Workflow logs run results |
/// | `name` | [`String`] | ✓ | Workflow name |
/// | `steps` | [`Vec<WorkflowStepDTO>`](crate::workflows::WorkflowStepDTO) | ✓ | Workflow steps |
/// | `trigger` | [`WorkflowTrigger`] | ✓ | Workflow trigger type |
/// | `updated_at` | [`String`] | ✓ | Update date |
///
/// See: <https://api.immich.app/models/WorkflowResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowResponseDTO {
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub enabled: bool,
    pub id: ::uuid::Uuid,
    pub logging: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub steps: Vec<WorkflowStepDTO>,
    pub trigger: WorkflowTrigger,
    /// Update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl WorkflowResponseDTO {
    ///Creates a new [`WorkflowResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: impl Into<String>,
        enabled: bool,
        id: ::uuid::Uuid,
        logging: bool,
        steps: Vec<WorkflowStepDTO>,
        trigger: WorkflowTrigger,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            created_at: created_at.into(),
            description: None,
            enabled,
            id,
            logging,
            name: None,
            steps,
            trigger,
            updated_at: updated_at.into(),
        }
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
        self.description = Some(value.into());
        self
    }

    ///Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `logging`.
    #[must_use]
    pub fn logging(mut self, value: bool) -> Self {
        self.logging = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }

    ///Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: Vec<WorkflowStepDTO>) -> Self {
        self.steps = value;
        self
    }

    ///Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = value.into();
        self
    }
}

/// Workflow run result
///
/// See: <https://api.immich.app/models/WorkflowResult>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum WorkflowResult {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "halted")]
    Halted,
    #[serde(rename = "error")]
    Error,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for WorkflowResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
            Self::Halted => f.write_str("halted"),
            Self::Error => f.write_str("error"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for WorkflowResult {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "halted" => Ok(Self::Halted),
            "error" => Ok(Self::Error),
            _ => Err(format!("invalid value for {}: {}", "WorkflowResult", value).into()),
        }
    }
}

impl TryFrom<&str> for WorkflowResult {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for WorkflowResult {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for WorkflowResult {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
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
pub struct WorkflowShareResponseDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub steps: Vec<WorkflowShareStepDTO>,
    pub trigger: WorkflowTrigger,
}

impl WorkflowShareResponseDTO {
    ///Creates a new [`WorkflowShareResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(steps: Vec<WorkflowShareStepDTO>, trigger: WorkflowTrigger) -> Self {
        Self {
            description: None,
            name: None,
            steps,
            trigger,
        }
    }

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    ///Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }

    ///Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: Vec<WorkflowShareStepDTO>) -> Self {
        self.steps = value;
        self
    }

    ///Sets `trigger`.
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
pub struct WorkflowShareStepDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    pub method: String,
}

impl WorkflowShareStepDTO {
    ///Creates a new [`WorkflowShareStepDTO`].
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
pub struct WorkflowStepDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    pub method: String,
}

impl WorkflowStepDTO {
    ///Creates a new [`WorkflowStepDTO`].
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

/// `WorkflowTriggerResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `trigger` | [`WorkflowTrigger`] | ✓ | Trigger type |
/// | `types` | [`Vec<WorkflowType>`](crate::plugins::WorkflowType) | ✓ | Workflow types |
///
/// See: <https://api.immich.app/models/WorkflowTriggerResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowTriggerResponseDTO {
    pub trigger: WorkflowTrigger,
    pub types: Vec<WorkflowType>,
}

impl WorkflowTriggerResponseDTO {
    ///Creates a new [`WorkflowTriggerResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(trigger: WorkflowTrigger, types: Vec<WorkflowType>) -> Self {
        Self { trigger, types }
    }

    ///Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = value;
        self
    }

    ///Sets `types`.
    #[must_use]
    pub fn types(mut self, value: Vec<WorkflowType>) -> Self {
        self.types = value;
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
pub struct WorkflowUpdateDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logging: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub steps: Vec<WorkflowStepDTO>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger: Option<WorkflowTrigger>,
}

impl WorkflowUpdateDTO {
    ///Creates a new [`WorkflowUpdateDTO`].
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

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    ///Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
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

    ///Sets `logging`.
    #[must_use]
    pub fn logging(mut self, value: bool) -> Self {
        self.logging = Some(value);
        self
    }

    ///Sets or clears `logging`.
    #[must_use]
    pub fn set_logging(mut self, value: Option<bool>) -> Self {
        self.logging = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }

    ///Sets `steps`.
    #[must_use]
    pub fn steps(mut self, value: Vec<WorkflowStepDTO>) -> Self {
        self.steps = value;
        self
    }

    ///Sets `trigger`.
    #[must_use]
    pub fn trigger(mut self, value: WorkflowTrigger) -> Self {
        self.trigger = Some(value);
        self
    }

    ///Sets or clears `trigger`.
    #[must_use]
    pub fn set_trigger(mut self, value: Option<WorkflowTrigger>) -> Self {
        self.trigger = value;
        self
    }
}
