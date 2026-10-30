// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `admin`.

pub use crate::activities::UserAvatarColor;

/// `UserAdminCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] |  |  |
/// | `email` | [`String`] | ✓ | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `is_admin` | `bool` |  | Grant admin privileges |
/// | `name` | [`String`] | ✓ | User name |
/// | `notify` | `bool` |  | Send notification email |
/// | `password` | [`String`] | ✓ | User password |
/// | `pin_code` | [`String`] |  | PIN code (pattern: `^\d{6}$`) |
/// | `quota_size_in_bytes` | `i64` |  | Storage quota in bytes |
/// | `should_change_password` | `bool` |  | Require password change on next login |
/// | `storage_label` | [`String`] |  | Storage label |
///
/// See: <https://api.immich.app/models/UserAdminCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserAdminCreateDTO {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) (required)
    pub email: String,
    /// Grant admin privileges
    #[serde(rename = "isAdmin", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_admin: Option<bool>,
    /// User name (required)
    pub name: String,
    /// Send notification email
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notify: Option<bool>,
    /// User password (required)
    pub password: String,
    /// PIN code (pattern: `^\d{6}$`)
    #[serde(rename = "pinCode", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pin_code: Option<String>,
    /// Storage quota in bytes
    #[serde(
        rename = "quotaSizeInBytes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub quota_size_in_bytes: Option<i64>,
    /// Require password change on next login
    #[serde(
        rename = "shouldChangePassword",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub should_change_password: Option<bool>,
    /// Storage label
    #[serde(
        rename = "storageLabel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub storage_label: Option<String>,
}

impl UserAdminCreateDTO {
    /// Creates a new [`UserAdminCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(email: impl Into<String>, name: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            avatar_color: None,
            email: email.into(),
            is_admin: None,
            name: name.into(),
            notify: None,
            password: password.into(),
            pin_code: None,
            quota_size_in_bytes: None,
            should_change_password: None,
            storage_label: None,
        }
    }
    /// Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }
    /// Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }
    /// Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = Some(value);
        self
    }
    /// Sets or clears `is_admin`.
    #[must_use]
    pub fn set_is_admin(mut self, value: Option<bool>) -> Self {
        self.is_admin = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `notify`.
    #[must_use]
    pub fn notify(mut self, value: bool) -> Self {
        self.notify = Some(value);
        self
    }
    /// Sets or clears `notify`.
    #[must_use]
    pub fn set_notify(mut self, value: Option<bool>) -> Self {
        self.notify = value;
        self
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = value.into();
        self
    }
    /// Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = Some(value.into());
        self
    }
    /// Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<String>) -> Self {
        self.pin_code = value;
        self
    }
    /// Sets `quota_size_in_bytes`.
    #[must_use]
    pub fn quota_size_in_bytes(mut self, value: i64) -> Self {
        self.quota_size_in_bytes = Some(value);
        self
    }
    /// Sets or clears `quota_size_in_bytes`.
    #[must_use]
    pub fn set_quota_size_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_size_in_bytes = value;
        self
    }
    /// Sets `should_change_password`.
    #[must_use]
    pub fn should_change_password(mut self, value: bool) -> Self {
        self.should_change_password = Some(value);
        self
    }
    /// Sets or clears `should_change_password`.
    #[must_use]
    pub fn set_should_change_password(mut self, value: Option<bool>) -> Self {
        self.should_change_password = value;
        self
    }
    /// Sets `storage_label`.
    #[must_use]
    pub fn storage_label(mut self, value: impl Into<String>) -> Self {
        self.storage_label = Some(value.into());
        self
    }
    /// Sets or clears `storage_label`.
    #[must_use]
    pub fn set_storage_label(mut self, value: Option<String>) -> Self {
        self.storage_label = value;
        self
    }
}

/// `UserAdminDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `force` | `bool` |  | Force delete even if user has assets |
///
/// See: <https://api.immich.app/models/UserAdminDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct UserAdminDeleteDTO {
    /// Force delete even if user has assets
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub force: Option<bool>,
}

impl UserAdminDeleteDTO {
    /// Creates a new [`UserAdminDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { force: None }
    }
    /// Sets `force`.
    #[must_use]
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }
    /// Sets or clears `force`.
    #[must_use]
    pub fn set_force(mut self, value: Option<bool>) -> Self {
        self.force = value;
        self
    }
}

/// `UserAdminUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] |  |  |
/// | `email` | [`String`] |  | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `is_admin` | `bool` |  | Grant admin privileges |
/// | `name` | [`String`] |  | User name |
/// | `password` | [`String`] |  | User password |
/// | `pin_code` | [`String`] |  | PIN code (pattern: `^\d{6}$`) |
/// | `quota_size_in_bytes` | `i64` |  | Storage quota in bytes |
/// | `should_change_password` | `bool` |  | Require password change on next login |
/// | `storage_label` | [`String`] |  | Storage label |
///
/// See: <https://api.immich.app/models/UserAdminUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct UserAdminUpdateDTO {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: Option<String>,
    /// Grant admin privileges
    #[serde(rename = "isAdmin", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_admin: Option<bool>,
    /// User name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// User password
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// PIN code (pattern: `^\d{6}$`)
    #[serde(rename = "pinCode", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pin_code: Option<String>,
    /// Storage quota in bytes
    #[serde(
        rename = "quotaSizeInBytes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub quota_size_in_bytes: Option<i64>,
    /// Require password change on next login
    #[serde(
        rename = "shouldChangePassword",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub should_change_password: Option<bool>,
    /// Storage label
    #[serde(
        rename = "storageLabel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub storage_label: Option<String>,
}

impl UserAdminUpdateDTO {
    /// Creates a new [`UserAdminUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            avatar_color: None,
            email: None,
            is_admin: None,
            name: None,
            password: None,
            pin_code: None,
            quota_size_in_bytes: None,
            should_change_password: None,
            storage_label: None,
        }
    }
    /// Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }
    /// Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }
    /// Sets or clears `email`.
    #[must_use]
    pub fn set_email(mut self, value: Option<String>) -> Self {
        self.email = value;
        self
    }
    /// Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = Some(value);
        self
    }
    /// Sets or clears `is_admin`.
    #[must_use]
    pub fn set_is_admin(mut self, value: Option<bool>) -> Self {
        self.is_admin = value;
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
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }
    /// Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }
    /// Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = Some(value.into());
        self
    }
    /// Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<String>) -> Self {
        self.pin_code = value;
        self
    }
    /// Sets `quota_size_in_bytes`.
    #[must_use]
    pub fn quota_size_in_bytes(mut self, value: i64) -> Self {
        self.quota_size_in_bytes = Some(value);
        self
    }
    /// Sets or clears `quota_size_in_bytes`.
    #[must_use]
    pub fn set_quota_size_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_size_in_bytes = value;
        self
    }
    /// Sets `should_change_password`.
    #[must_use]
    pub fn should_change_password(mut self, value: bool) -> Self {
        self.should_change_password = Some(value);
        self
    }
    /// Sets or clears `should_change_password`.
    #[must_use]
    pub fn set_should_change_password(mut self, value: Option<bool>) -> Self {
        self.should_change_password = value;
        self
    }
    /// Sets `storage_label`.
    #[must_use]
    pub fn storage_label(mut self, value: impl Into<String>) -> Self {
        self.storage_label = Some(value.into());
        self
    }
    /// Sets or clears `storage_label`.
    #[must_use]
    pub fn set_storage_label(mut self, value: Option<String>) -> Self {
        self.storage_label = value;
        self
    }
}
