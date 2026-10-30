// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `activities`.

pub use crate::id::{ActivityId, AlbumId, AssetId, UserId};

/// Activity create
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`AlbumId`](crate::id::AlbumId) | ✓ | Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `asset_id` | [`AssetId`](crate::id::AssetId) |  | Asset ID (if activity is for an asset) (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `comment` | [`String`] |  | Comment text (required if type is comment) |
/// | `type_` | [`ReactionType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/ActivityCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ActivityCreateDTO {
    /// Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "albumId")]
    pub album_id: AlbumId,
    /// Asset ID (if activity is for an asset) (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(rename = "assetId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub asset_id: Option<AssetId>,
    /// Comment text (required if type is comment)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: ReactionType,
}

impl ActivityCreateDTO {
    /// Creates a new [`ActivityCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: impl Into<AlbumId>, type_: ReactionType) -> Self {
        Self {
            album_id: album_id.into(),
            asset_id: None,
            comment: None,
            type_,
        }
    }
    /// Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: impl Into<AlbumId>) -> Self {
        self.album_id = value.into();
        self
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = Some(value.into());
        self
    }
    /// Sets or clears `asset_id`.
    #[must_use]
    pub fn set_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.asset_id = value;
        self
    }
    /// Sets `comment`.
    #[must_use]
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }
    /// Sets or clears `comment`.
    #[must_use]
    pub fn set_comment(mut self, value: Option<String>) -> Self {
        self.comment = value;
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: ReactionType) -> Self {
        self.type_ = value;
        self
    }
}

/// `ActivityResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (if activity is for an asset) (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `comment` | [`String`] |  | Comment text (for comment activities) |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`ActivityId`](crate::id::ActivityId) | ✓ | Activity ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `type_` | [`ReactionType`] | ✓ |  |
/// | `user` | [`UserResponseDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/ActivityResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ActivityResponseDTO {
    /// Asset ID (if activity is for an asset) (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: Option<AssetId>,
    /// Comment text (for comment activities)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: Option<String>,
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Activity ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: ActivityId,
    /// (required)
    #[serde(rename = "type")]
    pub type_: ReactionType,
    /// (required)
    pub user: UserResponseDTO,
}

impl ActivityResponseDTO {
    /// Creates a new [`ActivityResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<ActivityId>,
        type_: ReactionType,
        user: UserResponseDTO,
    ) -> Self {
        Self {
            asset_id: None,
            comment: None,
            created_at,
            id: id.into(),
            type_,
            user,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = Some(value.into());
        self
    }
    /// Sets or clears `asset_id`.
    #[must_use]
    pub fn set_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.asset_id = value;
        self
    }
    /// Sets `comment`.
    #[must_use]
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }
    /// Sets or clears `comment`.
    #[must_use]
    pub fn set_comment(mut self, value: Option<String>) -> Self {
        self.comment = value;
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<ActivityId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: ReactionType) -> Self {
        self.type_ = value;
        self
    }
    /// Sets `user`.
    #[must_use]
    pub fn user(mut self, value: UserResponseDTO) -> Self {
        self.user = value;
        self
    }
}

/// `ActivityStatisticsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `comments` | `i64` | ✓ | Number of comments |
/// | `likes` | `i64` | ✓ | Number of likes |
///
/// See: <https://api.immich.app/models/ActivityStatisticsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ActivityStatisticsResponseDTO {
    /// Number of comments (required)
    pub comments: i64,
    /// Number of likes (required)
    pub likes: i64,
}

impl ActivityStatisticsResponseDTO {
    /// Creates a new [`ActivityStatisticsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(comments: i64, likes: i64) -> Self {
        Self { comments, likes }
    }
    /// Sets `comments`.
    #[must_use]
    pub fn comments(mut self, value: i64) -> Self {
        self.comments = value;
        self
    }
    /// Sets `likes`.
    #[must_use]
    pub fn likes(mut self, value: i64) -> Self {
        self.likes = value;
        self
    }
}

/// Reaction level
///
/// See: <https://api.immich.app/models/ReactionLevel>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReactionLevel {
    Album,
    Asset,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl ReactionLevel {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Album => "album",
            Self::Asset => "asset",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for ReactionLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for ReactionLevel {
    fn from(value: &str) -> Self {
        match value {
            "album" => Self::Album,
            "asset" => Self::Asset,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for ReactionLevel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "album" => Self::Album,
            "asset" => Self::Asset,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for ReactionLevel {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for ReactionLevel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for ReactionLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for ReactionLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = ReactionLevel;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ReactionLevel::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ReactionLevel::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Reaction type
///
/// See: <https://api.immich.app/models/ReactionType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReactionType {
    Comment,
    Like,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl ReactionType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Comment => "comment",
            Self::Like => "like",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for ReactionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for ReactionType {
    fn from(value: &str) -> Self {
        match value {
            "comment" => Self::Comment,
            "like" => Self::Like,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for ReactionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "comment" => Self::Comment,
            "like" => Self::Like,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for ReactionType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for ReactionType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for ReactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for ReactionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = ReactionType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ReactionType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ReactionType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// User avatar color
///
/// See: <https://api.immich.app/models/UserAvatarColor>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum UserAvatarColor {
    Primary,
    Pink,
    Red,
    Yellow,
    Blue,
    Green,
    Purple,
    Orange,
    Gray,
    Amber,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl UserAvatarColor {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Primary => "primary",
            Self::Pink => "pink",
            Self::Red => "red",
            Self::Yellow => "yellow",
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Purple => "purple",
            Self::Orange => "orange",
            Self::Gray => "gray",
            Self::Amber => "amber",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for UserAvatarColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for UserAvatarColor {
    fn from(value: &str) -> Self {
        match value {
            "primary" => Self::Primary,
            "pink" => Self::Pink,
            "red" => Self::Red,
            "yellow" => Self::Yellow,
            "blue" => Self::Blue,
            "green" => Self::Green,
            "purple" => Self::Purple,
            "orange" => Self::Orange,
            "gray" => Self::Gray,
            "amber" => Self::Amber,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for UserAvatarColor {
    fn from(value: String) -> Self {
        match value.as_str() {
            "primary" => Self::Primary,
            "pink" => Self::Pink,
            "red" => Self::Red,
            "yellow" => Self::Yellow,
            "blue" => Self::Blue,
            "green" => Self::Green,
            "purple" => Self::Purple,
            "orange" => Self::Orange,
            "gray" => Self::Gray,
            "amber" => Self::Amber,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for UserAvatarColor {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for UserAvatarColor {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for UserAvatarColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for UserAvatarColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = UserAvatarColor;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(UserAvatarColor::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(UserAvatarColor::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `UserResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] | ✓ |  |
/// | `email` | [`String`] | ✓ | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `name` | [`String`] | ✓ | User name |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile change date |
/// | `profile_image_path` | [`String`] | ✓ | Profile image path |
///
/// See: <https://api.immich.app/models/UserResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserResponseDTO {
    /// (required)
    #[serde(rename = "avatarColor")]
    pub avatar_color: UserAvatarColor,
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) (required)
    pub email: String,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: UserId,
    /// User name (required)
    pub name: String,
    /// Profile change date (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Profile image path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
}

impl UserResponseDTO {
    /// Creates a new [`UserResponseDTO`].
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
