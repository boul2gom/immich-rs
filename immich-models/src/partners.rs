// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `partners`.

pub use crate::activities::UserAvatarColor;
pub use crate::id::UserId;

/// `PartnerCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `shared_with_id` | [`UserId`](crate::id::UserId) | ✓ | User ID to share with (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/PartnerCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PartnerCreateDTO {
    /// User ID to share with (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: UserId,
}

impl PartnerCreateDTO {
    /// Creates a new [`PartnerCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(shared_with_id: impl Into<UserId>) -> Self {
        Self {
            shared_with_id: shared_with_id.into(),
        }
    }
    /// Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_with_id = value.into();
        self
    }
}

/// Partner direction
///
/// See: <https://api.immich.app/models/PartnerDirection>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PartnerDirection {
    SharedBy,
    SharedWith,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl PartnerDirection {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::SharedBy => "shared-by",
            Self::SharedWith => "shared-with",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for PartnerDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for PartnerDirection {
    fn from(value: &str) -> Self {
        match value {
            "shared-by" => Self::SharedBy,
            "shared-with" => Self::SharedWith,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for PartnerDirection {
    fn from(value: String) -> Self {
        match value.as_str() {
            "shared-by" => Self::SharedBy,
            "shared-with" => Self::SharedWith,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for PartnerDirection {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for PartnerDirection {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for PartnerDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for PartnerDirection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = PartnerDirection;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(PartnerDirection::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(PartnerDirection::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Partner response
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] | ✓ |  |
/// | `email` | [`String`] | ✓ | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `in_timeline` | `bool` |  | Show in timeline |
/// | `name` | [`String`] | ✓ | User name |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile change date |
/// | `profile_image_path` | [`String`] | ✓ | Profile image path |
///
/// See: <https://api.immich.app/models/PartnerResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PartnerResponseDTO {
    /// (required)
    #[serde(rename = "avatarColor")]
    pub avatar_color: UserAvatarColor,
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) (required)
    pub email: String,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: UserId,
    /// Show in timeline
    #[serde(
        rename = "inTimeline",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub in_timeline: Option<bool>,
    /// User name (required)
    pub name: String,
    /// Profile change date (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Profile image path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
}

impl PartnerResponseDTO {
    /// Creates a new [`PartnerResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        avatar_color: UserAvatarColor,
        email: impl Into<String>,
        id: impl Into<UserId>,
        name: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        profile_image_path: impl Into<String>,
    ) -> Self {
        Self {
            avatar_color,
            email: email.into(),
            id: id.into(),
            in_timeline: None,
            name: name.into(),
            profile_changed_at,
            profile_image_path: profile_image_path.into(),
        }
    }
    /// Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = value;
        self
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<UserId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `in_timeline`.
    #[must_use]
    pub fn in_timeline(mut self, value: bool) -> Self {
        self.in_timeline = Some(value);
        self
    }
    /// Sets or clears `in_timeline`.
    #[must_use]
    pub fn set_in_timeline(mut self, value: Option<bool>) -> Self {
        self.in_timeline = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }
    /// Sets `profile_image_path`.
    #[must_use]
    pub fn profile_image_path(mut self, value: impl Into<String>) -> Self {
        self.profile_image_path = value.into();
        self
    }
}

/// `PartnerUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `in_timeline` | `bool` | ✓ | Show partner assets in timeline |
///
/// See: <https://api.immich.app/models/PartnerUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PartnerUpdateDTO {
    /// Show partner assets in timeline (required)
    #[serde(rename = "inTimeline")]
    pub in_timeline: bool,
}

impl PartnerUpdateDTO {
    /// Creates a new [`PartnerUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(in_timeline: bool) -> Self {
        Self { in_timeline }
    }
    /// Sets `in_timeline`.
    #[must_use]
    pub fn in_timeline(mut self, value: bool) -> Self {
        self.in_timeline = value;
        self
    }
}
