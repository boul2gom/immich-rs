// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `notifications`.

pub use crate::id::UserId;
pub use crate::notifications::{NotificationLevel, NotificationType};

/// `NotificationCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `data` | `object` |  | Additional notification data |
/// | `description` | [`String`] |  | Notification description |
/// | `level` | [`NotificationLevel`] |  |  |
/// | `read_at` | [`DateTime`](::chrono::DateTime) |  | Date when notification was read (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `title` | [`String`] | ✓ | Notification title |
/// | `type_` | [`NotificationType`] |  |  |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID to send notification to (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/NotificationCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct NotificationCreateDTO {
    /// Additional notification data
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    /// Notification description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub level: Option<NotificationLevel>,
    /// Date when notification was read (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "readAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Notification title (required)
    pub title: String,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<NotificationType>,
    /// User ID to send notification to (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl NotificationCreateDTO {
    /// Creates a new [`NotificationCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(title: impl Into<String>, user_id: impl Into<UserId>) -> Self {
        Self {
            data: <::serde_json::Map<String, ::serde_json::Value>>::default(),
            description: None,
            level: None,
            read_at: None,
            title: title.into(),
            type_: None,
            user_id: user_id.into(),
        }
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
    /// Sets `level`.
    #[must_use]
    pub fn level(mut self, value: NotificationLevel) -> Self {
        self.level = Some(value);
        self
    }
    /// Sets or clears `level`.
    #[must_use]
    pub fn set_level(mut self, value: Option<NotificationLevel>) -> Self {
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
        self.type_ = Some(value);
        self
    }
    /// Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<NotificationType>) -> Self {
        self.type_ = value;
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
    }
}

/// `TemplateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `template` | [`String`] | ✓ | Template name |
///
/// See: <https://api.immich.app/models/TemplateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TemplateDTO {
    /// Template name (required)
    pub template: String,
}

impl TemplateDTO {
    /// Creates a new [`TemplateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
        }
    }
    /// Sets `template`.
    #[must_use]
    pub fn template(mut self, value: impl Into<String>) -> Self {
        self.template = value.into();
        self
    }
}

/// `TemplateResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `html` | [`String`] | ✓ | Template HTML content |
/// | `name` | [`String`] | ✓ | Template name |
///
/// See: <https://api.immich.app/models/TemplateResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TemplateResponseDTO {
    /// Template HTML content (required)
    pub html: String,
    /// Template name (required)
    pub name: String,
}

impl TemplateResponseDTO {
    /// Creates a new [`TemplateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(html: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            html: html.into(),
            name: name.into(),
        }
    }
    /// Sets `html`.
    #[must_use]
    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = value.into();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
}

/// `TestEmailResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `message_id` | [`String`] | ✓ | Email message ID |
///
/// See: <https://api.immich.app/models/TestEmailResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TestEmailResponseDTO {
    /// Email message ID (required)
    #[serde(rename = "messageId")]
    pub message_id: String,
}

impl TestEmailResponseDTO {
    /// Creates a new [`TestEmailResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(message_id: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
        }
    }
    /// Sets `message_id`.
    #[must_use]
    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = value.into();
        self
    }
}
