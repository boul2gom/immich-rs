// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `sessions`.

/// `SessionCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `device_o_s` | [`String`] |  | Device OS |
/// | `device_type` | [`String`] |  | Device type |
/// | `duration` | `i64` |  | Session duration in seconds |
///
/// See: <https://api.immich.app/models/SessionCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct SessionCreateDTO {
    /// Device OS
    #[serde(rename = "deviceOS", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub device_os: Option<String>,
    /// Device type
    #[serde(
        rename = "deviceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub device_type: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: Option<::std::num::NonZeroU64>,
}

impl SessionCreateDTO {
    ///Creates a new [`SessionCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            device_os: None,
            device_type: None,
            duration: None,
        }
    }

    ///Sets `device_os`.
    #[must_use]
    pub fn device_os(mut self, value: impl Into<String>) -> Self {
        self.device_os = Some(value.into());
        self
    }

    ///Sets or clears `device_os`.
    #[must_use]
    pub fn set_device_os(mut self, value: Option<String>) -> Self {
        self.device_os = value;
        self
    }

    ///Sets `device_type`.
    #[must_use]
    pub fn device_type(mut self, value: impl Into<String>) -> Self {
        self.device_type = Some(value.into());
        self
    }

    ///Sets or clears `device_type`.
    #[must_use]
    pub fn set_device_type(mut self, value: Option<String>) -> Self {
        self.device_type = value;
        self
    }

    ///Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.duration = Some(value);
        self
    }

    ///Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.duration = value;
        self
    }
}

/// `SessionCreateResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `app_version` | [`String`] | ✓ | App version |
/// | `created_at` | [`String`] | ✓ | Creation date |
/// | `current` | `bool` | ✓ | Is current session |
/// | `device_o_s` | [`String`] | ✓ | Device OS |
/// | `device_type` | [`String`] | ✓ | Device type |
/// | `expires_at` | [`String`] |  | Expiration date |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Session ID |
/// | `is_pending_sync_reset` | `bool` | ✓ | Is pending sync reset |
/// | `token` | [`String`] | ✓ | Session token |
/// | `updated_at` | [`String`] | ✓ | Last update date |
///
/// See: <https://api.immich.app/models/SessionCreateResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SessionCreateResponseDTO {
    /// App version (required)
    #[serde(rename = "appVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub current: bool,
    /// Device OS (required)
    #[serde(rename = "deviceOS")]
    pub device_os: String,
    /// Device type (required)
    #[serde(rename = "deviceType")]
    pub device_type: String,
    /// Expiration date
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: Option<String>,
    pub id: ::uuid::Uuid,
    /// Is pending sync reset (required)
    #[serde(rename = "isPendingSyncReset")]
    pub is_pending_sync_reset: bool,
    pub token: String,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl SessionCreateResponseDTO {
    ///Creates a new [`SessionCreateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: impl Into<String>,
        current: bool,
        device_os: impl Into<String>,
        device_type: impl Into<String>,
        id: ::uuid::Uuid,
        is_pending_sync_reset: bool,
        token: impl Into<String>,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            app_version: None,
            created_at: created_at.into(),
            current,
            device_os: device_os.into(),
            device_type: device_type.into(),
            expires_at: None,
            id,
            is_pending_sync_reset,
            token: token.into(),
            updated_at: updated_at.into(),
        }
    }

    ///Sets `app_version`.
    #[must_use]
    pub fn app_version(mut self, value: impl Into<String>) -> Self {
        self.app_version = Some(value.into());
        self
    }

    ///Sets or clears `app_version`.
    #[must_use]
    pub fn set_app_version(mut self, value: Option<String>) -> Self {
        self.app_version = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = value.into();
        self
    }

    ///Sets `current`.
    #[must_use]
    pub fn current(mut self, value: bool) -> Self {
        self.current = value;
        self
    }

    ///Sets `device_os`.
    #[must_use]
    pub fn device_os(mut self, value: impl Into<String>) -> Self {
        self.device_os = value.into();
        self
    }

    ///Sets `device_type`.
    #[must_use]
    pub fn device_type(mut self, value: impl Into<String>) -> Self {
        self.device_type = value.into();
        self
    }

    ///Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    ///Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<String>) -> Self {
        self.expires_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_pending_sync_reset`.
    #[must_use]
    pub fn is_pending_sync_reset(mut self, value: bool) -> Self {
        self.is_pending_sync_reset = value;
        self
    }

    ///Sets `token`.
    #[must_use]
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = value.into();
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = value.into();
        self
    }
}

/// `SessionResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `app_version` | [`String`] | ✓ | App version |
/// | `created_at` | [`String`] | ✓ | Creation date |
/// | `current` | `bool` | ✓ | Is current session |
/// | `device_o_s` | [`String`] | ✓ | Device OS |
/// | `device_type` | [`String`] | ✓ | Device type |
/// | `expires_at` | [`String`] |  | Expiration date |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Session ID |
/// | `is_pending_sync_reset` | `bool` | ✓ | Is pending sync reset |
/// | `updated_at` | [`String`] | ✓ | Last update date |
///
/// See: <https://api.immich.app/models/SessionResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SessionResponseDTO {
    /// App version (required)
    #[serde(rename = "appVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub current: bool,
    /// Device OS (required)
    #[serde(rename = "deviceOS")]
    pub device_os: String,
    /// Device type (required)
    #[serde(rename = "deviceType")]
    pub device_type: String,
    /// Expiration date
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: Option<String>,
    pub id: ::uuid::Uuid,
    /// Is pending sync reset (required)
    #[serde(rename = "isPendingSyncReset")]
    pub is_pending_sync_reset: bool,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl SessionResponseDTO {
    ///Creates a new [`SessionResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: impl Into<String>,
        current: bool,
        device_os: impl Into<String>,
        device_type: impl Into<String>,
        id: ::uuid::Uuid,
        is_pending_sync_reset: bool,
        updated_at: impl Into<String>,
    ) -> Self {
        Self {
            app_version: None,
            created_at: created_at.into(),
            current,
            device_os: device_os.into(),
            device_type: device_type.into(),
            expires_at: None,
            id,
            is_pending_sync_reset,
            updated_at: updated_at.into(),
        }
    }

    ///Sets `app_version`.
    #[must_use]
    pub fn app_version(mut self, value: impl Into<String>) -> Self {
        self.app_version = Some(value.into());
        self
    }

    ///Sets or clears `app_version`.
    #[must_use]
    pub fn set_app_version(mut self, value: Option<String>) -> Self {
        self.app_version = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = value.into();
        self
    }

    ///Sets `current`.
    #[must_use]
    pub fn current(mut self, value: bool) -> Self {
        self.current = value;
        self
    }

    ///Sets `device_os`.
    #[must_use]
    pub fn device_os(mut self, value: impl Into<String>) -> Self {
        self.device_os = value.into();
        self
    }

    ///Sets `device_type`.
    #[must_use]
    pub fn device_type(mut self, value: impl Into<String>) -> Self {
        self.device_type = value.into();
        self
    }

    ///Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    ///Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<String>) -> Self {
        self.expires_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_pending_sync_reset`.
    #[must_use]
    pub fn is_pending_sync_reset(mut self, value: bool) -> Self {
        self.is_pending_sync_reset = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = value.into();
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SessionUnlockPinCode(pub String);

impl ::std::ops::Deref for SessionUnlockPinCode {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for SessionUnlockPinCode {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> =
            ::std::sync::LazyLock::new(|| ::regex::Regex::new("^\\d{6}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d{6}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for SessionUnlockPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SessionUnlockPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SessionUnlockPinCode {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for SessionUnlockPinCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `SessionUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `is_pending_sync_reset` | `bool` |  | Reset pending sync state |
///
/// See: <https://api.immich.app/models/SessionUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct SessionUpdateDTO {
    /// Reset pending sync state
    #[serde(
        rename = "isPendingSyncReset",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_pending_sync_reset: Option<bool>,
}

impl SessionUpdateDTO {
    ///Creates a new [`SessionUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            is_pending_sync_reset: None,
        }
    }

    ///Sets `is_pending_sync_reset`.
    #[must_use]
    pub fn is_pending_sync_reset(mut self, value: bool) -> Self {
        self.is_pending_sync_reset = Some(value);
        self
    }

    ///Sets or clears `is_pending_sync_reset`.
    #[must_use]
    pub fn set_is_pending_sync_reset(mut self, value: Option<bool>) -> Self {
        self.is_pending_sync_reset = value;
        self
    }
}
