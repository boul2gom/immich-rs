// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `partners`.

pub use crate::activities::UserAvatarColor;

/// `PartnerCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `shared_with_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID to share with |
///
/// See: <https://api.immich.app/models/PartnerCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PartnerCreateDTO {
    /// User ID to share with (required)
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: ::uuid::Uuid,
}

impl PartnerCreateDTO {
    ///Creates a new [`PartnerCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(shared_with_id: ::uuid::Uuid) -> Self {
        Self { shared_with_id }
    }

    ///Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_with_id = value;
        self
    }
}

/// Partner direction
///
/// See: <https://api.immich.app/models/PartnerDirection>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PartnerDirection {
    #[serde(rename = "shared-by")]
    SharedBy,
    #[serde(rename = "shared-with")]
    SharedWith,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for PartnerDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SharedBy => f.write_str("shared-by"),
            Self::SharedWith => f.write_str("shared-with"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for PartnerDirection {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "shared-by" => Ok(Self::SharedBy),
            "shared-with" => Ok(Self::SharedWith),
            _ => Err(format!("invalid value for {}: {}", "PartnerDirection", value).into()),
        }
    }
}

impl TryFrom<&str> for PartnerDirection {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PartnerDirection {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PartnerDirection {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Partner response
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] | ✓ |  |
/// | `email` | [`String`] | ✓ | User email |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
/// | `in_timeline` | `bool` |  | Show in timeline |
/// | `name` | [`String`] | ✓ | User name |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile change date |
/// | `profile_image_path` | [`String`] | ✓ | Profile image path |
///
/// See: <https://api.immich.app/models/PartnerResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PartnerResponseDTO {
    /// (required)
    #[serde(rename = "avatarColor")]
    pub avatar_color: UserAvatarColor,
    pub email: PartnerResponseEmail,
    pub id: ::uuid::Uuid,
    /// Show in timeline
    #[serde(
        rename = "inTimeline",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub in_timeline: Option<bool>,
    pub name: String,
    /// Profile change date (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Profile image path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
}

impl PartnerResponseDTO {
    ///Creates a new [`PartnerResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        avatar_color: UserAvatarColor,
        email: PartnerResponseEmail,
        id: ::uuid::Uuid,
        name: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        profile_image_path: impl Into<String>,
    ) -> Self {
        Self {
            avatar_color,
            email,
            id,
            in_timeline: None,
            name: name.into(),
            profile_changed_at,
            profile_image_path: profile_image_path.into(),
        }
    }

    ///Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = value;
        self
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: PartnerResponseEmail) -> Self {
        self.email = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `in_timeline`.
    #[must_use]
    pub fn in_timeline(mut self, value: bool) -> Self {
        self.in_timeline = Some(value);
        self
    }

    ///Sets or clears `in_timeline`.
    #[must_use]
    pub fn set_in_timeline(mut self, value: Option<bool>) -> Self {
        self.in_timeline = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }

    ///Sets `profile_image_path`.
    #[must_use]
    pub fn profile_image_path(mut self, value: impl Into<String>) -> Self {
        self.profile_image_path = value.into();
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PartnerResponseEmail(pub String);

impl ::std::ops::Deref for PartnerResponseEmail {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for PartnerResponseEmail {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new(
                    "^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for PartnerResponseEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PartnerResponseEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PartnerResponseEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for PartnerResponseEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `PartnerUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `in_timeline` | `bool` | ✓ | Show partner assets in timeline |
///
/// See: <https://api.immich.app/models/PartnerUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PartnerUpdateDTO {
    /// Show partner assets in timeline (required)
    #[serde(rename = "inTimeline")]
    pub in_timeline: bool,
}

impl PartnerUpdateDTO {
    ///Creates a new [`PartnerUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(in_timeline: bool) -> Self {
        Self { in_timeline }
    }

    ///Sets `in_timeline`.
    #[must_use]
    pub fn in_timeline(mut self, value: bool) -> Self {
        self.in_timeline = value;
        self
    }
}
