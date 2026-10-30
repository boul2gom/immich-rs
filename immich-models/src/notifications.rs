// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `notifications`.

pub use crate::id::NotificationId;

/// `NotificationDeleteAllDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ids` | [`Vec<NotificationId>`](crate::id::NotificationId) | ✓ | Notification IDs to delete |
///
/// See: <https://api.immich.app/models/NotificationDeleteAllDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct NotificationDeleteAllDTO {
    /// Notification IDs to delete (required)
    pub ids: Vec<NotificationId>,
}

impl NotificationDeleteAllDTO {
    /// Creates a new [`NotificationDeleteAllDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: impl IntoIterator<Item = impl Into<NotificationId>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: impl IntoIterator<Item = impl Into<NotificationId>>) -> Self {
        self.ids = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `NotificationDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `data` | `object` |  | Additional notification data |
/// | `description` | [`String`] |  | Notification description |
/// | `id` | [`NotificationId`](crate::id::NotificationId) | ✓ | Notification ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `level` | [`NotificationLevel`] | ✓ |  |
/// | `read_at` | [`DateTime`](::chrono::DateTime) |  | Date when notification was read (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `title` | [`String`] | ✓ | Notification title |
/// | `type_` | [`NotificationType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/NotificationDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct NotificationDTO {
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Additional notification data
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    /// Notification description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Notification ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: NotificationId,
    /// (required)
    pub level: NotificationLevel,
    /// Date when notification was read (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "readAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Notification title (required)
    pub title: String,
    /// (required)
    #[serde(rename = "type")]
    pub type_: NotificationType,
}

impl NotificationDTO {
    /// Creates a new [`NotificationDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<NotificationId>,
        level: NotificationLevel,
        title: impl Into<String>,
        type_: NotificationType,
    ) -> Self {
        Self {
            created_at,
            data: <::serde_json::Map<String, ::serde_json::Value>>::default(),
            description: None,
            id: id.into(),
            level,
            read_at: None,
            title: title.into(),
            type_,
        }
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `data`.
    #[must_use]
    pub fn data(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.data = value;
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
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<NotificationId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `level`.
    #[must_use]
    pub fn level(mut self, value: NotificationLevel) -> Self {
        self.level = value;
        self
    }
    /// Sets `read_at`.
    #[must_use]
    pub fn read_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.read_at = Some(value);
        self
    }
    /// Sets or clears `read_at`.
    #[must_use]
    pub fn set_read_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.read_at = value;
        self
    }
    /// Sets `title`.
    #[must_use]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: NotificationType) -> Self {
        self.type_ = value;
        self
    }
}

/// Notification level
///
/// See: <https://api.immich.app/models/NotificationLevel>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum NotificationLevel {
    Success,
    Error,
    Warning,
    Info,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl NotificationLevel {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Success => "success",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for NotificationLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for NotificationLevel {
    fn from(value: &str) -> Self {
        match value {
            "success" => Self::Success,
            "error" => Self::Error,
            "warning" => Self::Warning,
            "info" => Self::Info,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for NotificationLevel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "success" => Self::Success,
            "error" => Self::Error,
            "warning" => Self::Warning,
            "info" => Self::Info,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for NotificationLevel {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for NotificationLevel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for NotificationLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for NotificationLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = NotificationLevel;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(NotificationLevel::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(NotificationLevel::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Notification type
///
/// See: <https://api.immich.app/models/NotificationType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum NotificationType {
    JobFailed,
    BackupFailed,
    SystemMessage,
    AlbumInvite,
    AlbumUpdate,
    ClusterGroupRequest,
    Custom,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl NotificationType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::JobFailed => "JobFailed",
            Self::BackupFailed => "BackupFailed",
            Self::SystemMessage => "SystemMessage",
            Self::AlbumInvite => "AlbumInvite",
            Self::AlbumUpdate => "AlbumUpdate",
            Self::ClusterGroupRequest => "ClusterGroupRequest",
            Self::Custom => "Custom",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for NotificationType {
    fn from(value: &str) -> Self {
        match value {
            "JobFailed" => Self::JobFailed,
            "BackupFailed" => Self::BackupFailed,
            "SystemMessage" => Self::SystemMessage,
            "AlbumInvite" => Self::AlbumInvite,
            "AlbumUpdate" => Self::AlbumUpdate,
            "ClusterGroupRequest" => Self::ClusterGroupRequest,
            "Custom" => Self::Custom,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for NotificationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "JobFailed" => Self::JobFailed,
            "BackupFailed" => Self::BackupFailed,
            "SystemMessage" => Self::SystemMessage,
            "AlbumInvite" => Self::AlbumInvite,
            "AlbumUpdate" => Self::AlbumUpdate,
            "ClusterGroupRequest" => Self::ClusterGroupRequest,
            "Custom" => Self::Custom,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for NotificationType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for NotificationType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for NotificationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for NotificationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = NotificationType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(NotificationType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(NotificationType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `NotificationUpdateAllDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ids` | [`Vec<NotificationId>`](crate::id::NotificationId) | ✓ | Notification IDs to update |
/// | `read_at` | [`DateTime`](::chrono::DateTime) |  | Date when notifications were read (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/NotificationUpdateAllDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct NotificationUpdateAllDTO {
    /// Notification IDs to update (required)
    pub ids: Vec<NotificationId>,
    /// Date when notifications were read (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "readAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl NotificationUpdateAllDTO {
    /// Creates a new [`NotificationUpdateAllDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: impl IntoIterator<Item = impl Into<NotificationId>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
            read_at: None,
        }
    }
    /// Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: impl IntoIterator<Item = impl Into<NotificationId>>) -> Self {
        self.ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `read_at`.
    #[must_use]
    pub fn read_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.read_at = Some(value);
        self
    }
    /// Sets or clears `read_at`.
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
/// | `read_at` | [`DateTime`](::chrono::DateTime) |  | Date when notification was read (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/NotificationUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct NotificationUpdateDTO {
    /// Date when notification was read (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "readAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl NotificationUpdateDTO {
    /// Creates a new [`NotificationUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { read_at: None }
    }
    /// Sets `read_at`.
    #[must_use]
    pub fn read_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.read_at = Some(value);
        self
    }
    /// Sets or clears `read_at`.
    #[must_use]
    pub fn set_read_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.read_at = value;
        self
    }
}
