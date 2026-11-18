// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `common`.

pub use crate::albums::{AlbumUserRole, AssetOrder};
pub use crate::people::{PersonUsersDeleteDTO, PersonUsersDeleteItem, PersonUsersResponseDTO, PersonUsersResponseItem};
pub use crate::search::SearchOrderField;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// CQ mode
#[non_exhaustive]
pub enum CqMode {
    Auto,
    Cqp,
    Icq,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl CqMode {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Cqp => "cqp",
            Self::Icq => "icq",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for CqMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for CqMode {
    fn from(value: &str) -> Self {
        match value {
            "auto" => Self::Auto,
            "cqp" => Self::Cqp,
            "icq" => Self::Icq,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for CqMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "auto" => Self::Auto,
            "cqp" => Self::Cqp,
            "icq" => Self::Icq,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for CqMode {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for CqMode {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for CqMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for CqMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = CqMode;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(CqMode::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(CqMode::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `PasskeyAuthenticationFinishDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `response` | `object` | ✓ | Response returned by @simplewebauthn/browser's `startAuthentication()` |
///
/// See: <https://api.immich.app/models/PasskeyAuthenticationFinishDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PasskeyAuthenticationFinishDTO {
    /// Response returned by @simplewebauthn/browser's `startAuthentication()` (required)
    pub response: PasskeyAuthenticationFinishResponse,
}

impl PasskeyAuthenticationFinishDTO {
    /// Creates a new [`PasskeyAuthenticationFinishDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(response: PasskeyAuthenticationFinishResponse) -> Self {
        Self { response }
    }
    /// Sets `response`.
    #[must_use]
    pub fn response(mut self, value: PasskeyAuthenticationFinishResponse) -> Self {
        self.response = value;
        self
    }
}

/// Response returned by @simplewebauthn/browser's `startAuthentication()`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PasskeyAuthenticationFinishResponse {
    /// Credential ID
    pub id: String,
    pub response: PasskeyAuthenticationFinishResponseResponse,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<String, ::serde_json::Value>,
}

impl PasskeyAuthenticationFinishResponse {
    /// Creates a new [`PasskeyAuthenticationFinishResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        id: impl Into<String>,
        response: PasskeyAuthenticationFinishResponseResponse,
        extra: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self {
            id: id.into(),
            response,
            extra,
        }
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `response`.
    #[must_use]
    pub fn response(mut self, value: PasskeyAuthenticationFinishResponseResponse) -> Self {
        self.response = value;
        self
    }
    /// Sets `extra`.
    #[must_use]
    pub fn extra(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.extra = value;
        self
    }
}

/// `PasskeyAuthenticationFinishDtoResponseResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PasskeyAuthenticationFinishResponseResponse {
    /// User handle
    #[serde(rename = "userHandle", skip_serializing_if = "::std::option::Option::is_none")]
    pub user_handle: Option<String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<String, ::serde_json::Value>,
}

impl PasskeyAuthenticationFinishResponseResponse {
    /// Creates a new [`PasskeyAuthenticationFinishResponseResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(extra: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self {
            user_handle: None,
            extra,
        }
    }
    /// Sets `user_handle`.
    #[must_use]
    pub fn user_handle(mut self, value: impl Into<String>) -> Self {
        self.user_handle = Some(value.into());
        self
    }
    /// Sets or clears `user_handle`.
    #[must_use]
    pub fn set_user_handle(mut self, value: Option<String>) -> Self {
        self.user_handle = value;
        self
    }
    /// Sets `extra`.
    #[must_use]
    pub fn extra(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.extra = value;
        self
    }
}

/// Response returned by @simplewebauthn/server's `generateAuthenticationOptions`
///
/// See: <https://api.immich.app/models/PasskeyAuthenticationOptionsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PasskeyAuthenticationOptionsDTO(pub ::serde_json::Map<String, ::serde_json::Value>);

impl ::std::ops::Deref for PasskeyAuthenticationOptionsDTO {
    type Target = ::serde_json::Map<String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<String, ::serde_json::Value> {
        &self.0
    }
}

impl From<::serde_json::Map<String, ::serde_json::Value>> for PasskeyAuthenticationOptionsDTO {
    fn from(value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}

/// `PasskeyRegistrationFinishDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `name` | [`String`] |  | Passkey name |
/// | `response` | `object` | ✓ | Response returned by @simplewebauthn/browser's `startAuthentication()` |
///
/// See: <https://api.immich.app/models/PasskeyRegistrationFinishDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PasskeyRegistrationFinishDTO {
    /// Passkey name
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// Response returned by @simplewebauthn/browser's `startAuthentication()` (required)
    pub response: ::serde_json::Value,
}

impl PasskeyRegistrationFinishDTO {
    /// Creates a new [`PasskeyRegistrationFinishDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(response: ::serde_json::Value) -> Self {
        Self { name: None, response }
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
    /// Sets `response`.
    #[must_use]
    pub fn response(mut self, value: ::serde_json::Value) -> Self {
        self.response = value;
        self
    }
}

/// Response returned by @simplewebauthn/server's `generateRegistrationOptions`
///
/// See: <https://api.immich.app/models/PasskeyRegistrationOptionsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PasskeyRegistrationOptionsDTO(pub ::serde_json::Map<String, ::serde_json::Value>);

impl ::std::ops::Deref for PasskeyRegistrationOptionsDTO {
    type Target = ::serde_json::Map<String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<String, ::serde_json::Value> {
        &self.0
    }
}

impl From<::serde_json::Map<String, ::serde_json::Value>> for PasskeyRegistrationOptionsDTO {
    fn from(value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}

/// `PasskeyResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Passkey ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `name` | [`String`] | ✓ | Passkey name |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `used_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last used date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/PasskeyResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PasskeyResponseDTO {
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Passkey ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: ::uuid::Uuid,
    /// Passkey name (required)
    pub name: Option<String>,
    /// Last update date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Last used date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "usedAt")]
    pub used_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl PasskeyResponseDTO {
    /// Creates a new [`PasskeyResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            id,
            name: None,
            updated_at,
            used_at: None,
        }
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
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
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
    /// Sets `used_at`.
    #[must_use]
    pub fn used_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.used_at = Some(value);
        self
    }
    /// Sets or clears `used_at`.
    #[must_use]
    pub fn set_used_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.used_at = value;
        self
    }
}

/// `PasskeyUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `name` | [`String`] | ✓ | Passkey name |
///
/// See: <https://api.immich.app/models/PasskeyUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PasskeyUpdateDTO {
    /// Passkey name (required)
    pub name: Option<String>,
}

impl PasskeyUpdateDTO {
    /// Creates a new [`PasskeyUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { name: None }
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
}

impl Default for PasskeyUpdateDTO {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PersonUsersDeleteDTO> for Vec<PersonUsersDeleteItem> {
    fn from(value: PersonUsersDeleteDTO) -> Self {
        value.0
    }
}

impl From<PersonUsersResponseDTO> for Vec<PersonUsersResponseItem> {
    fn from(value: PersonUsersResponseDTO) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Transcode hardware acceleration
#[non_exhaustive]
pub enum TranscodeHwAccel {
    Nvenc,
    Qsv,
    Vaapi,
    Rkmpp,
    Disabled,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl TranscodeHwAccel {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Nvenc => "nvenc",
            Self::Qsv => "qsv",
            Self::Vaapi => "vaapi",
            Self::Rkmpp => "rkmpp",
            Self::Disabled => "disabled",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for TranscodeHwAccel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for TranscodeHwAccel {
    fn from(value: &str) -> Self {
        match value {
            "nvenc" => Self::Nvenc,
            "qsv" => Self::Qsv,
            "vaapi" => Self::Vaapi,
            "rkmpp" => Self::Rkmpp,
            "disabled" => Self::Disabled,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for TranscodeHwAccel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "nvenc" => Self::Nvenc,
            "qsv" => Self::Qsv,
            "vaapi" => Self::Vaapi,
            "rkmpp" => Self::Rkmpp,
            "disabled" => Self::Disabled,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for TranscodeHwAccel {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for TranscodeHwAccel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for TranscodeHwAccel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for TranscodeHwAccel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = TranscodeHwAccel;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(TranscodeHwAccel::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(TranscodeHwAccel::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Generation of default values for serde.
#[allow(clippy::missing_panics_doc)]
pub mod defaults {
    #[must_use]
    pub fn default_bool<const V: bool>() -> bool {
        V
    }
    #[must_use]
    pub fn default_nzu64<T, const V: u64>() -> T
    where
        T: TryFrom<::std::num::NonZeroU64>,
        <T as ::std::convert::TryFrom<::std::num::NonZeroU64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(::std::num::NonZeroU64::try_from(V).unwrap()).unwrap()
    }
    #[must_use]
    pub fn album_user_add_dto_role() -> super::AlbumUserRole {
        super::AlbumUserRole::Editor
    }
    #[must_use]
    pub fn search_order_direction() -> super::AssetOrder {
        super::AssetOrder::Desc
    }
    #[must_use]
    pub fn search_order_field() -> super::SearchOrderField {
        super::SearchOrderField::FileCreatedAt
    }
}

/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    #[derive(Clone, PartialEq, Eq)]
    pub struct ConversionError(pub ::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
