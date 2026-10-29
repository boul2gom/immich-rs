// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `admin`.

pub use crate::notifications::{NotificationLevel, NotificationType};

/// `NotificationCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `data` | `object` |  | Additional notification data |
/// | `description` | [`String`] |  | Notification description |
/// | `level` | [`NotificationLevel`] |  |  |
/// | `read_at` | [`DateTime`](::chrono::DateTime) |  | Date when notification was read |
/// | `title` | [`String`] | ✓ | Notification title |
/// | `type` | [`NotificationType`] |  |  |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID to send notification to |
///
/// See: <https://api.immich.app/models/NotificationCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct NotificationCreateDTO {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub level: Option<NotificationLevel>,
    /// Date when notification was read
    #[serde(rename = "readAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub title: String,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<NotificationType>,
    /// User ID to send notification to (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl NotificationCreateDTO {
    ///Creates a new [`NotificationCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(title: impl Into<String>, user_id: ::uuid::Uuid) -> Self {
        Self {
            data: <::serde_json::Map<String, ::serde_json::Value>>::default(),
            description: None,
            level: None,
            read_at: None,
            title: title.into(),
            type_: None,
            user_id,
        }
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

    ///Sets `level`.
    #[must_use]
    pub fn level(mut self, value: NotificationLevel) -> Self {
        self.level = Some(value);
        self
    }

    ///Sets or clears `level`.
    #[must_use]
    pub fn set_level(mut self, value: Option<NotificationLevel>) -> Self {
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
        self.type_ = Some(value);
        self
    }

    ///Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<NotificationType>) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TemplateDTO {
    pub template: String,
}

impl TemplateDTO {
    ///Creates a new [`TemplateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
        }
    }

    ///Sets `template`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TemplateResponseDTO {
    pub html: String,
    pub name: String,
}

impl TemplateResponseDTO {
    ///Creates a new [`TemplateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(html: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            html: html.into(),
            name: name.into(),
        }
    }

    ///Sets `html`.
    #[must_use]
    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = value.into();
        self
    }

    ///Sets `name`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TestEmailResponseDTO {
    /// Email message ID (required)
    #[serde(rename = "messageId")]
    pub message_id: String,
}

impl TestEmailResponseDTO {
    ///Creates a new [`TestEmailResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(message_id: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
        }
    }

    ///Sets `message_id`.
    #[must_use]
    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = value.into();
        self
    }
}
