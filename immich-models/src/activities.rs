// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `activities`.

pub use crate::users::UserResponseEmail;

/// Activity create
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`Uuid`](::uuid::Uuid) | ✓ | Album ID |
/// | `asset_id` | [`Uuid`](::uuid::Uuid) |  | Asset ID (if activity is for an asset) |
/// | `comment` | [`String`] |  | Comment text (required if type is comment) |
/// | `type` | [`ReactionType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/ActivityCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ActivityCreateDTO {
    /// Album ID (required)
    #[serde(rename = "albumId")]
    pub album_id: ::uuid::Uuid,
    /// Asset ID (if activity is for an asset)
    #[serde(rename = "assetId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub asset_id: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: ReactionType,
}

impl ActivityCreateDTO {
    ///Creates a new [`ActivityCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: ::uuid::Uuid, type_: ReactionType) -> Self {
        Self {
            album_id,
            asset_id: None,
            comment: None,
            type_,
        }
    }

    ///Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: ::uuid::Uuid) -> Self {
        self.album_id = value;
        self
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = Some(value);
        self
    }

    ///Sets or clears `asset_id`.
    #[must_use]
    pub fn set_asset_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `comment`.
    #[must_use]
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    ///Sets or clears `comment`.
    #[must_use]
    pub fn set_comment(mut self, value: Option<String>) -> Self {
        self.comment = value;
        self
    }

    ///Sets `type_`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID (if activity is for an asset) |
/// | `comment` | [`String`] |  | Comment text (for comment activities) |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Activity ID |
/// | `type` | [`ReactionType`] | ✓ |  |
/// | `user` | [`UserResponseDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/ActivityResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ActivityResponseDTO {
    /// Asset ID (if activity is for an asset) (required)
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub comment: Option<String>,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::uuid::Uuid,
    /// (required)
    #[serde(rename = "type")]
    pub type_: ReactionType,
    pub user: UserResponseDTO,
}

impl ActivityResponseDTO {
    ///Creates a new [`ActivityResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        type_: ReactionType,
        user: UserResponseDTO,
    ) -> Self {
        Self {
            asset_id: None,
            comment: None,
            created_at,
            id,
            type_,
            user,
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = Some(value);
        self
    }

    ///Sets or clears `asset_id`.
    #[must_use]
    pub fn set_asset_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `comment`.
    #[must_use]
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    ///Sets or clears `comment`.
    #[must_use]
    pub fn set_comment(mut self, value: Option<String>) -> Self {
        self.comment = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: ReactionType) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `user`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ActivityStatisticsResponseDTO {
    pub comments: i64,
    pub likes: i64,
}

impl ActivityStatisticsResponseDTO {
    ///Creates a new [`ActivityStatisticsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(comments: i64, likes: i64) -> Self {
        Self { comments, likes }
    }

    ///Sets `comments`.
    #[must_use]
    pub fn comments(mut self, value: i64) -> Self {
        self.comments = value;
        self
    }

    ///Sets `likes`.
    #[must_use]
    pub fn likes(mut self, value: i64) -> Self {
        self.likes = value;
        self
    }
}

/// Reaction level
///
/// See: <https://api.immich.app/models/ReactionLevel>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReactionLevel {
    #[serde(rename = "album")]
    Album,
    #[serde(rename = "asset")]
    Asset,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for ReactionLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Album => f.write_str("album"),
            Self::Asset => f.write_str("asset"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for ReactionLevel {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "album" => Ok(Self::Album),
            "asset" => Ok(Self::Asset),
            _ => Err(format!("invalid value for {}: {}", "ReactionLevel", value).into()),
        }
    }
}

impl TryFrom<&str> for ReactionLevel {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ReactionLevel {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ReactionLevel {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Reaction type
///
/// See: <https://api.immich.app/models/ReactionType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReactionType {
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "like")]
    Like,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for ReactionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Comment => f.write_str("comment"),
            Self::Like => f.write_str("like"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for ReactionType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "comment" => Ok(Self::Comment),
            "like" => Ok(Self::Like),
            _ => Err(format!("invalid value for {}: {}", "ReactionType", value).into()),
        }
    }
}

impl TryFrom<&str> for ReactionType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ReactionType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ReactionType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// User avatar color
///
/// See: <https://api.immich.app/models/UserAvatarColor>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum UserAvatarColor {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "pink")]
    Pink,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "amber")]
    Amber,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for UserAvatarColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Primary => f.write_str("primary"),
            Self::Pink => f.write_str("pink"),
            Self::Red => f.write_str("red"),
            Self::Yellow => f.write_str("yellow"),
            Self::Blue => f.write_str("blue"),
            Self::Green => f.write_str("green"),
            Self::Purple => f.write_str("purple"),
            Self::Orange => f.write_str("orange"),
            Self::Gray => f.write_str("gray"),
            Self::Amber => f.write_str("amber"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for UserAvatarColor {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "primary" => Ok(Self::Primary),
            "pink" => Ok(Self::Pink),
            "red" => Ok(Self::Red),
            "yellow" => Ok(Self::Yellow),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            "purple" => Ok(Self::Purple),
            "orange" => Ok(Self::Orange),
            "gray" => Ok(Self::Gray),
            "amber" => Ok(Self::Amber),
            _ => Err(format!("invalid value for {}: {}", "UserAvatarColor", value).into()),
        }
    }
}

impl TryFrom<&str> for UserAvatarColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for UserAvatarColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for UserAvatarColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `UserResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] | ✓ |  |
/// | `email` | [`String`] | ✓ | User email |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
/// | `name` | [`String`] | ✓ | User name |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile change date |
/// | `profile_image_path` | [`String`] | ✓ | Profile image path |
///
/// See: <https://api.immich.app/models/UserResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserResponseDTO {
    /// (required)
    #[serde(rename = "avatarColor")]
    pub avatar_color: UserAvatarColor,
    pub email: UserResponseEmail,
    pub id: ::uuid::Uuid,
    pub name: String,
    /// Profile change date (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Profile image path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
}

impl UserResponseDTO {
    ///Creates a new [`UserResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        avatar_color: UserAvatarColor,
        email: UserResponseEmail,
        id: ::uuid::Uuid,
        name: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        profile_image_path: impl Into<String>,
    ) -> Self {
        Self {
            avatar_color,
            email,
            id,
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
    pub fn email(mut self, value: UserResponseEmail) -> Self {
        self.email = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
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
