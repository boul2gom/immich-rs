// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `notifications`.

pub mod admin;

/// `NotificationDeleteAllDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Notification IDs to delete |
///
/// See: <https://api.immich.app/models/NotificationDeleteAllDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NotificationDeleteAllDTO {
    pub ids: Vec<::uuid::Uuid>,
}

impl NotificationDeleteAllDTO {
    ///Creates a new [`NotificationDeleteAllDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: Vec<::uuid::Uuid>) -> Self {
        Self { ids }
    }

    ///Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.ids = value;
        self
    }
}

/// `NotificationDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `data` | `object` |  | Additional notification data |
/// | `description` | [`String`] |  | Notification description |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Notification ID |
/// | `level` | [`NotificationLevel`] | ✓ |  |
/// | `read_at` | [`DateTime`](::chrono::DateTime) |  | Date when notification was read |
/// | `title` | [`String`] | ✓ | Notification title |
/// | `type` | [`NotificationType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/NotificationDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NotificationDTO {
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    pub id: ::uuid::Uuid,
    pub level: NotificationLevel,
    /// Date when notification was read
    #[serde(rename = "readAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub title: String,
    /// (required)
    #[serde(rename = "type")]
    pub type_: NotificationType,
}

impl NotificationDTO {
    ///Creates a new [`NotificationDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        level: NotificationLevel,
        title: impl Into<String>,
        type_: NotificationType,
    ) -> Self {
        Self {
            created_at,
            data: <::serde_json::Map<String, ::serde_json::Value>>::default(),
            description: None,
            id,
            level,
            read_at: None,
            title: title.into(),
            type_,
        }
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `data`.
    #[must_use]
    pub fn data(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.data = value;
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

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `level`.
    #[must_use]
    pub fn level(mut self, value: NotificationLevel) -> Self {
        self.level = value;
        self
    }

    ///Sets `read_at`.
    #[must_use]
    pub fn read_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.read_at = Some(value);
        self
    }

    ///Sets or clears `read_at`.
    #[must_use]
    pub fn set_read_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.read_at = value;
        self
    }

    ///Sets `title`.
    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: NotificationType) -> Self {
        self.type_ = value;
        self
    }
}

/// Notification level
///
/// See: <https://api.immich.app/models/NotificationLevel>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum NotificationLevel {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for NotificationLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Success => f.write_str("success"),
            Self::Error => f.write_str("error"),
            Self::Warning => f.write_str("warning"),
            Self::Info => f.write_str("info"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for NotificationLevel {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "success" => Ok(Self::Success),
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            "info" => Ok(Self::Info),
            _ => Err(format!("invalid value for {}: {}", "NotificationLevel", value).into()),
        }
    }
}

impl TryFrom<&str> for NotificationLevel {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for NotificationLevel {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for NotificationLevel {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Notification type
///
/// See: <https://api.immich.app/models/NotificationType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum NotificationType {
    JobFailed,
    BackupFailed,
    SystemMessage,
    AlbumInvite,
    AlbumUpdate,
    ClusterGroupRequest,
    Custom,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JobFailed => f.write_str("JobFailed"),
            Self::BackupFailed => f.write_str("BackupFailed"),
            Self::SystemMessage => f.write_str("SystemMessage"),
            Self::AlbumInvite => f.write_str("AlbumInvite"),
            Self::AlbumUpdate => f.write_str("AlbumUpdate"),
            Self::ClusterGroupRequest => f.write_str("ClusterGroupRequest"),
            Self::Custom => f.write_str("Custom"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for NotificationType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "JobFailed" => Ok(Self::JobFailed),
            "BackupFailed" => Ok(Self::BackupFailed),
            "SystemMessage" => Ok(Self::SystemMessage),
            "AlbumInvite" => Ok(Self::AlbumInvite),
            "AlbumUpdate" => Ok(Self::AlbumUpdate),
            "ClusterGroupRequest" => Ok(Self::ClusterGroupRequest),
            "Custom" => Ok(Self::Custom),
            _ => Err(format!("invalid value for {}: {}", "NotificationType", value).into()),
        }
    }
}

impl TryFrom<&str> for NotificationType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for NotificationType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for NotificationType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `NotificationUpdateAllDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Notification IDs to update |
/// | `read_at` | [`DateTime`](::chrono::DateTime) |  | Date when notifications were read |
///
/// See: <https://api.immich.app/models/NotificationUpdateAllDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NotificationUpdateAllDTO {
    pub ids: Vec<::uuid::Uuid>,
    /// Date when notifications were read
    #[serde(rename = "readAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl NotificationUpdateAllDTO {
    ///Creates a new [`NotificationUpdateAllDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: Vec<::uuid::Uuid>) -> Self {
        Self { ids, read_at: None }
    }

    ///Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.ids = value;
        self
    }

    ///Sets `read_at`.
    #[must_use]
    pub fn read_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.read_at = Some(value);
        self
    }

    ///Sets or clears `read_at`.
    #[must_use]
    pub fn set_read_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.read_at = value;
        self
    }
}

/// `NotificationUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `read_at` | [`DateTime`](::chrono::DateTime) |  | Date when notification was read |
///
/// See: <https://api.immich.app/models/NotificationUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct NotificationUpdateDTO {
    /// Date when notification was read
    #[serde(rename = "readAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl NotificationUpdateDTO {
    ///Creates a new [`NotificationUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { read_at: None }
    }

    ///Sets `read_at`.
    #[must_use]
    pub fn read_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.read_at = Some(value);
        self
    }

    ///Sets or clears `read_at`.
    #[must_use]
    pub fn set_read_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.read_at = value;
        self
    }
}
