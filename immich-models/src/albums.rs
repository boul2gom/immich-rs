// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `albums`.

pub use crate::activities::UserResponseDTO;
pub use crate::id::{AlbumId, AssetId, UserId};

/// `AddUsersDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_users` | [`Vec<AlbumUserAddDTO>`](crate::albums::AlbumUserAddDTO) | ✓ | Album users to add |
///
/// See: <https://api.immich.app/models/AddUsersDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AddUsersDTO {
    /// Album users to add (required)
    #[serde(rename = "albumUsers")]
    pub album_users: Vec<AlbumUserAddDTO>,
}

impl AddUsersDTO {
    /// Creates a new [`AddUsersDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_users: impl IntoIterator<Item = AlbumUserAddDTO>) -> Self {
        Self {
            album_users: album_users.into_iter().collect(),
        }
    }
    /// Sets `album_users`.
    #[must_use]
    pub fn album_users(mut self, value: impl IntoIterator<Item = AlbumUserAddDTO>) -> Self {
        self.album_users = value.into_iter().collect();
        self
    }
}

/// `AlbumResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_name` | [`String`] | ✓ | Album name |
/// | `album_thumbnail_asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Thumbnail asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `album_users` | [`Vec<AlbumUserResponseDTO>`](crate::albums::AlbumUserResponseDTO) | ✓ | First entry is always the album owner. Second entry is the auth user, if it differs from the owner. The rest are ordered alphabetically. |
/// | `asset_count` | `i64` | ✓ | Number of assets |
/// | `contributor_counts` | [`Vec<ContributorCountResponseDTO>`](crate::albums::ContributorCountResponseDTO) |  |  |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `description` | [`String`] | ✓ | Album description |
/// | `end_date` | [`DateTime`](::chrono::DateTime) |  | UTC representation of (local) end date (latest asset) |
/// | `has_shared_link` | `bool` | ✓ | Has shared link |
/// | `id` | [`AlbumId`](crate::id::AlbumId) | ✓ | Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_activity_enabled` | `bool` | ✓ | Activity feed enabled |
/// | `last_modified_asset_timestamp` | [`DateTime`](::chrono::DateTime) |  | Last modified asset timestamp |
/// | `order` | [`AssetOrder`] |  |  |
/// | `shared` | `bool` | ✓ | Is shared album |
/// | `start_date` | [`DateTime`](::chrono::DateTime) |  | UTC representation of (local) start date (earliest asset) |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
///
/// See: <https://api.immich.app/models/AlbumResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AlbumResponseDTO {
    /// Album name (required)
    #[serde(rename = "albumName")]
    pub album_name: String,
    /// Thumbnail asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "albumThumbnailAssetId")]
    pub album_thumbnail_asset_id: Option<AssetId>,
    /// First entry is always the album owner. Second entry is the auth user, if it differs from the owner. The rest are ordered alphabetically. (required)
    #[serde(rename = "albumUsers")]
    pub album_users: Vec<AlbumUserResponseDTO>,
    /// Number of assets (required)
    #[serde(rename = "assetCount")]
    pub asset_count: i64,
    #[serde(
        rename = "contributorCounts",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub contributor_counts: Vec<ContributorCountResponseDTO>,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Album description (required)
    pub description: String,
    /// UTC representation of (local) end date (latest asset)
    #[serde(rename = "endDate", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Has shared link (required)
    #[serde(rename = "hasSharedLink")]
    pub has_shared_link: bool,
    /// Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: AlbumId,
    /// Activity feed enabled (required)
    #[serde(rename = "isActivityEnabled")]
    pub is_activity_enabled: bool,
    /// Last modified asset timestamp
    #[serde(
        rename = "lastModifiedAssetTimestamp",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_modified_asset_timestamp: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub order: Option<AssetOrder>,
    /// Is shared album (required)
    pub shared: bool,
    /// UTC representation of (local) start date (earliest asset)
    #[serde(
        rename = "startDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub start_date: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl AlbumResponseDTO {
    /// Creates a new [`AlbumResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        album_name: impl Into<String>,
        album_users: impl IntoIterator<Item = AlbumUserResponseDTO>,
        asset_count: i64,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        description: impl Into<String>,
        has_shared_link: bool,
        id: impl Into<AlbumId>,
        is_activity_enabled: bool,
        shared: bool,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            album_name: album_name.into(),
            album_thumbnail_asset_id: None,
            album_users: album_users.into_iter().collect(),
            asset_count,
            contributor_counts: Vec::new(),
            created_at,
            description: description.into(),
            end_date: None,
            has_shared_link,
            id: id.into(),
            is_activity_enabled,
            last_modified_asset_timestamp: None,
            order: None,
            shared,
            start_date: None,
            updated_at,
        }
    }
    /// Sets `album_name`.
    #[must_use]
    pub fn album_name(mut self, value: impl Into<String>) -> Self {
        self.album_name = value.into();
        self
    }
    /// Sets `album_thumbnail_asset_id`.
    #[must_use]
    pub fn album_thumbnail_asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.album_thumbnail_asset_id = Some(value.into());
        self
    }
    /// Sets or clears `album_thumbnail_asset_id`.
    #[must_use]
    pub fn set_album_thumbnail_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.album_thumbnail_asset_id = value;
        self
    }
    /// Sets `album_users`.
    #[must_use]
    pub fn album_users(mut self, value: impl IntoIterator<Item = AlbumUserResponseDTO>) -> Self {
        self.album_users = value.into_iter().collect();
        self
    }
    /// Sets `asset_count`.
    #[must_use]
    pub fn asset_count(mut self, value: i64) -> Self {
        self.asset_count = value;
        self
    }
    /// Sets `contributor_counts`.
    #[must_use]
    pub fn contributor_counts(mut self, value: impl IntoIterator<Item = ContributorCountResponseDTO>) -> Self {
        self.contributor_counts = value.into_iter().collect();
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = value.into();
        self
    }
    /// Sets `end_date`.
    #[must_use]
    pub fn end_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.end_date = Some(value);
        self
    }
    /// Sets or clears `end_date`.
    #[must_use]
    pub fn set_end_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.end_date = value;
        self
    }
    /// Sets `has_shared_link`.
    #[must_use]
    pub fn has_shared_link(mut self, value: bool) -> Self {
        self.has_shared_link = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<AlbumId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_activity_enabled`.
    #[must_use]
    pub fn is_activity_enabled(mut self, value: bool) -> Self {
        self.is_activity_enabled = value;
        self
    }
    /// Sets `last_modified_asset_timestamp`.
    #[must_use]
    pub fn last_modified_asset_timestamp(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.last_modified_asset_timestamp = Some(value);
        self
    }
    /// Sets or clears `last_modified_asset_timestamp`.
    #[must_use]
    pub fn set_last_modified_asset_timestamp(
        mut self,
        value: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ) -> Self {
        self.last_modified_asset_timestamp = value;
        self
    }
    /// Sets `order`.
    #[must_use]
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = Some(value);
        self
    }
    /// Sets or clears `order`.
    #[must_use]
    pub fn set_order(mut self, value: Option<AssetOrder>) -> Self {
        self.order = value;
        self
    }
    /// Sets `shared`.
    #[must_use]
    pub fn shared(mut self, value: bool) -> Self {
        self.shared = value;
        self
    }
    /// Sets `start_date`.
    #[must_use]
    pub fn start_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.start_date = Some(value);
        self
    }
    /// Sets or clears `start_date`.
    #[must_use]
    pub fn set_start_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.start_date = value;
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `AlbumStatisticsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `not_shared` | `i64` | ✓ | Number of non-shared albums |
/// | `owned` | `i64` | ✓ | Number of owned albums |
/// | `shared` | `i64` | ✓ | Number of shared albums |
///
/// See: <https://api.immich.app/models/AlbumStatisticsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AlbumStatisticsResponseDTO {
    /// Number of non-shared albums (required)
    #[serde(rename = "notShared")]
    pub not_shared: i64,
    /// Number of owned albums (required)
    pub owned: i64,
    /// Number of shared albums (required)
    pub shared: i64,
}

impl AlbumStatisticsResponseDTO {
    /// Creates a new [`AlbumStatisticsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(not_shared: i64, owned: i64, shared: i64) -> Self {
        Self {
            not_shared,
            owned,
            shared,
        }
    }
    /// Sets `not_shared`.
    #[must_use]
    pub fn not_shared(mut self, value: i64) -> Self {
        self.not_shared = value;
        self
    }
    /// Sets `owned`.
    #[must_use]
    pub fn owned(mut self, value: i64) -> Self {
        self.owned = value;
        self
    }
    /// Sets `shared`.
    #[must_use]
    pub fn shared(mut self, value: i64) -> Self {
        self.shared = value;
        self
    }
}

/// `AlbumUserAddDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `role` | [`AlbumUserRole`] |  | Album user role |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/AlbumUserAddDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AlbumUserAddDTO {
    /// Album user role
    #[serde(default = "crate::common::defaults::album_user_add_dto_role")]
    pub role: AlbumUserRole,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl AlbumUserAddDTO {
    /// Creates a new [`AlbumUserAddDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(user_id: impl Into<UserId>) -> Self {
        Self {
            role: crate::common::defaults::album_user_add_dto_role(),
            user_id: user_id.into(),
        }
    }
    /// Sets `role`.
    #[must_use]
    pub fn role(mut self, value: AlbumUserRole) -> Self {
        self.role = value;
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
    }
}

/// `AlbumUserCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `role` | [`AlbumUserRole`] | ✓ |  |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/AlbumUserCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AlbumUserCreateDTO {
    /// (required)
    pub role: AlbumUserRole,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl AlbumUserCreateDTO {
    /// Creates a new [`AlbumUserCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(role: AlbumUserRole, user_id: impl Into<UserId>) -> Self {
        Self {
            role,
            user_id: user_id.into(),
        }
    }
    /// Sets `role`.
    #[must_use]
    pub fn role(mut self, value: AlbumUserRole) -> Self {
        self.role = value;
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
    }
}

/// `AlbumUserResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `role` | [`AlbumUserRole`] | ✓ |  |
/// | `user` | [`UserResponseDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AlbumUserResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AlbumUserResponseDTO {
    /// (required)
    pub role: AlbumUserRole,
    /// (required)
    pub user: UserResponseDTO,
}

impl AlbumUserResponseDTO {
    /// Creates a new [`AlbumUserResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(role: AlbumUserRole, user: UserResponseDTO) -> Self {
        Self { role, user }
    }
    /// Sets `role`.
    #[must_use]
    pub fn role(mut self, value: AlbumUserRole) -> Self {
        self.role = value;
        self
    }
    /// Sets `user`.
    #[must_use]
    pub fn user(mut self, value: UserResponseDTO) -> Self {
        self.user = value;
        self
    }
}

/// Album user role
///
/// See: <https://api.immich.app/models/AlbumUserRole>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AlbumUserRole {
    Editor,
    Owner,
    Viewer,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AlbumUserRole {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Editor => "editor",
            Self::Owner => "owner",
            Self::Viewer => "viewer",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AlbumUserRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AlbumUserRole {
    fn from(value: &str) -> Self {
        match value {
            "editor" => Self::Editor,
            "owner" => Self::Owner,
            "viewer" => Self::Viewer,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AlbumUserRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "editor" => Self::Editor,
            "owner" => Self::Owner,
            "viewer" => Self::Viewer,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AlbumUserRole {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AlbumUserRole {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AlbumUserRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AlbumUserRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AlbumUserRole;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AlbumUserRole::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AlbumUserRole::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `AlbumsAddAssetsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_ids` | [`Vec<AlbumId>`](crate::id::AlbumId) | ✓ | Album IDs |
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs |
///
/// See: <https://api.immich.app/models/AlbumsAddAssetsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AlbumsAddAssetsDTO {
    /// Album IDs (required)
    #[serde(rename = "albumIds")]
    pub album_ids: Vec<AlbumId>,
    /// Asset IDs (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<AssetId>,
}

impl AlbumsAddAssetsDTO {
    /// Creates a new [`AlbumsAddAssetsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        album_ids: impl IntoIterator<Item = impl Into<AlbumId>>,
        asset_ids: impl IntoIterator<Item = impl Into<AssetId>>,
    ) -> Self {
        Self {
            album_ids: album_ids.into_iter().map(Into::into).collect(),
            asset_ids: asset_ids.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `album_ids`.
    #[must_use]
    pub fn album_ids(mut self, value: impl IntoIterator<Item = impl Into<AlbumId>>) -> Self {
        self.album_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `AlbumsAddAssetsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `error` | [`BulkIdErrorReason`] |  |  |
/// | `success` | `bool` | ✓ | Operation success |
///
/// See: <https://api.immich.app/models/AlbumsAddAssetsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AlbumsAddAssetsResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: Option<BulkIdErrorReason>,
    /// Operation success (required)
    pub success: bool,
}

impl AlbumsAddAssetsResponseDTO {
    /// Creates a new [`AlbumsAddAssetsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(success: bool) -> Self {
        Self { error: None, success }
    }
    /// Sets `error`.
    #[must_use]
    pub fn error(mut self, value: BulkIdErrorReason) -> Self {
        self.error = Some(value);
        self
    }
    /// Sets or clears `error`.
    #[must_use]
    pub fn set_error(mut self, value: Option<BulkIdErrorReason>) -> Self {
        self.error = value;
        self
    }
    /// Sets `success`.
    #[must_use]
    pub fn success(mut self, value: bool) -> Self {
        self.success = value;
        self
    }
}

/// Asset sort order
///
/// See: <https://api.immich.app/models/AssetOrder>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetOrder {
    Asc,
    Desc,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetOrder {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetOrder {
    fn from(value: &str) -> Self {
        match value {
            "asc" => Self::Asc,
            "desc" => Self::Desc,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetOrder {
    fn from(value: String) -> Self {
        match value.as_str() {
            "asc" => Self::Asc,
            "desc" => Self::Desc,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetOrder {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetOrder {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetOrder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetOrder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetOrder::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetOrder::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Error reason
///
/// See: <https://api.immich.app/models/BulkIdErrorReason>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum BulkIdErrorReason {
    Duplicate,
    NoPermission,
    NotFound,
    Unknown,
    Validation,
    /// Unknown value returned by a newer version of the Immich server.
    UnknownValue(String),
}

impl BulkIdErrorReason {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Duplicate => "duplicate",
            Self::NoPermission => "no_permission",
            Self::NotFound => "not_found",
            Self::Unknown => "unknown",
            Self::Validation => "validation",
            Self::UnknownValue(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for BulkIdErrorReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for BulkIdErrorReason {
    fn from(value: &str) -> Self {
        match value {
            "duplicate" => Self::Duplicate,
            "no_permission" => Self::NoPermission,
            "not_found" => Self::NotFound,
            "unknown" => Self::Unknown,
            "validation" => Self::Validation,
            other => Self::UnknownValue(other.to_owned()),
        }
    }
}

impl From<String> for BulkIdErrorReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "duplicate" => Self::Duplicate,
            "no_permission" => Self::NoPermission,
            "not_found" => Self::NotFound,
            "unknown" => Self::Unknown,
            "validation" => Self::Validation,
            _ => Self::UnknownValue(value),
        }
    }
}

impl From<&String> for BulkIdErrorReason {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for BulkIdErrorReason {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for BulkIdErrorReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for BulkIdErrorReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = BulkIdErrorReason;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(BulkIdErrorReason::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(BulkIdErrorReason::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `BulkIdResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `error` | [`BulkIdErrorReason`] |  |  |
/// | `error_message` | [`String`] |  |  |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `success` | `bool` | ✓ | Whether operation succeeded |
///
/// See: <https://api.immich.app/models/BulkIdResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct BulkIdResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: Option<BulkIdErrorReason>,
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub error_message: Option<String>,
    /// ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: ::uuid::Uuid,
    /// Whether operation succeeded (required)
    pub success: bool,
}

impl BulkIdResponseDTO {
    /// Creates a new [`BulkIdResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: ::uuid::Uuid, success: bool) -> Self {
        Self {
            error: None,
            error_message: None,
            id,
            success,
        }
    }
    /// Sets `error`.
    #[must_use]
    pub fn error(mut self, value: BulkIdErrorReason) -> Self {
        self.error = Some(value);
        self
    }
    /// Sets or clears `error`.
    #[must_use]
    pub fn set_error(mut self, value: Option<BulkIdErrorReason>) -> Self {
        self.error = value;
        self
    }
    /// Sets `error_message`.
    #[must_use]
    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }
    /// Sets or clears `error_message`.
    #[must_use]
    pub fn set_error_message(mut self, value: Option<String>) -> Self {
        self.error_message = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }
    /// Sets `success`.
    #[must_use]
    pub fn success(mut self, value: bool) -> Self {
        self.success = value;
        self
    }
}

/// `BulkIdsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | IDs to process |
///
/// See: <https://api.immich.app/models/BulkIdsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct BulkIdsDTO {
    /// IDs to process (required)
    pub ids: Vec<::uuid::Uuid>,
}

impl BulkIdsDTO {
    /// Creates a new [`BulkIdsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: impl IntoIterator<Item = impl Into<::uuid::Uuid>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: impl IntoIterator<Item = impl Into<::uuid::Uuid>>) -> Self {
        self.ids = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `ContributorCountResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_count` | `i64` | ✓ | Number of assets contributed |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/ContributorCountResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ContributorCountResponseDTO {
    /// Number of assets contributed (required)
    #[serde(rename = "assetCount")]
    pub asset_count: i64,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl ContributorCountResponseDTO {
    /// Creates a new [`ContributorCountResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_count: i64, user_id: impl Into<UserId>) -> Self {
        Self {
            asset_count,
            user_id: user_id.into(),
        }
    }
    /// Sets `asset_count`.
    #[must_use]
    pub fn asset_count(mut self, value: i64) -> Self {
        self.asset_count = value;
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
    }
}

/// `CreateAlbumDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_name` | [`String`] | ✓ | Album name |
/// | `album_users` | [`Vec<AlbumUserCreateDTO>`](crate::albums::AlbumUserCreateDTO) |  | Album users |
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) |  | Initial asset IDs |
/// | `description` | [`String`] |  | Album description |
///
/// See: <https://api.immich.app/models/CreateAlbumDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CreateAlbumDTO {
    /// Album name (required)
    #[serde(rename = "albumName")]
    pub album_name: String,
    /// Album users
    #[serde(rename = "albumUsers", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub album_users: Vec<AlbumUserCreateDTO>,
    /// Initial asset IDs
    #[serde(rename = "assetIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub asset_ids: Vec<AssetId>,
    /// Album description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
}

impl CreateAlbumDTO {
    /// Creates a new [`CreateAlbumDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_name: impl Into<String>) -> Self {
        Self {
            album_name: album_name.into(),
            album_users: Vec::new(),
            asset_ids: Vec::new(),
            description: None,
        }
    }
    /// Sets `album_name`.
    #[must_use]
    pub fn album_name(mut self, value: impl Into<String>) -> Self {
        self.album_name = value.into();
        self
    }
    /// Sets `album_users`.
    #[must_use]
    pub fn album_users(mut self, value: impl IntoIterator<Item = AlbumUserCreateDTO>) -> Self {
        self.album_users = value.into_iter().collect();
        self
    }
    /// Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.asset_ids = value.into_iter().map(Into::into).collect();
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
}

/// `MapMarkerResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `city` | [`String`] | ✓ | City name |
/// | `country` | [`String`] | ✓ | Country name |
/// | `id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `lat` | `f64` | ✓ | Latitude |
/// | `lon` | `f64` | ✓ | Longitude |
/// | `state` | [`String`] | ✓ | State/Province name |
///
/// See: <https://api.immich.app/models/MapMarkerResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct MapMarkerResponseDTO {
    /// City name (required)
    pub city: Option<String>,
    /// Country name (required)
    pub country: Option<String>,
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: AssetId,
    /// Latitude (required)
    pub lat: f64,
    /// Longitude (required)
    pub lon: f64,
    /// State/Province name (required)
    pub state: Option<String>,
}

impl MapMarkerResponseDTO {
    /// Creates a new [`MapMarkerResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: impl Into<AssetId>, lat: f64, lon: f64) -> Self {
        Self {
            city: None,
            country: None,
            id: id.into(),
            lat,
            lon,
            state: None,
        }
    }
    /// Sets `city`.
    #[must_use]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }
    /// Sets or clears `city`.
    #[must_use]
    pub fn set_city(mut self, value: Option<String>) -> Self {
        self.city = value;
        self
    }
    /// Sets `country`.
    #[must_use]
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }
    /// Sets or clears `country`.
    #[must_use]
    pub fn set_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<AssetId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `lat`.
    #[must_use]
    pub fn lat(mut self, value: f64) -> Self {
        self.lat = value;
        self
    }
    /// Sets `lon`.
    #[must_use]
    pub fn lon(mut self, value: f64) -> Self {
        self.lon = value;
        self
    }
    /// Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }
    /// Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }
}

/// `UpdateAlbumDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_name` | [`String`] |  | Album name |
/// | `album_thumbnail_asset_id` | [`AssetId`](crate::id::AssetId) |  | Album thumbnail asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `description` | [`String`] |  | Album description |
/// | `is_activity_enabled` | `bool` |  | Enable activity feed |
/// | `order` | [`AssetOrder`] |  |  |
///
/// See: <https://api.immich.app/models/UpdateAlbumDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct UpdateAlbumDTO {
    /// Album name
    #[serde(
        rename = "albumName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub album_name: Option<String>,
    /// Album thumbnail asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(
        rename = "albumThumbnailAssetId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub album_thumbnail_asset_id: Option<AssetId>,
    /// Album description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Enable activity feed
    #[serde(
        rename = "isActivityEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_activity_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub order: Option<AssetOrder>,
}

impl UpdateAlbumDTO {
    /// Creates a new [`UpdateAlbumDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_name: None,
            album_thumbnail_asset_id: None,
            description: None,
            is_activity_enabled: None,
            order: None,
        }
    }
    /// Sets `album_name`.
    #[must_use]
    pub fn album_name(mut self, value: impl Into<String>) -> Self {
        self.album_name = Some(value.into());
        self
    }
    /// Sets or clears `album_name`.
    #[must_use]
    pub fn set_album_name(mut self, value: Option<String>) -> Self {
        self.album_name = value;
        self
    }
    /// Sets `album_thumbnail_asset_id`.
    #[must_use]
    pub fn album_thumbnail_asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.album_thumbnail_asset_id = Some(value.into());
        self
    }
    /// Sets or clears `album_thumbnail_asset_id`.
    #[must_use]
    pub fn set_album_thumbnail_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.album_thumbnail_asset_id = value;
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
    /// Sets `is_activity_enabled`.
    #[must_use]
    pub fn is_activity_enabled(mut self, value: bool) -> Self {
        self.is_activity_enabled = Some(value);
        self
    }
    /// Sets or clears `is_activity_enabled`.
    #[must_use]
    pub fn set_is_activity_enabled(mut self, value: Option<bool>) -> Self {
        self.is_activity_enabled = value;
        self
    }
    /// Sets `order`.
    #[must_use]
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = Some(value);
        self
    }
    /// Sets or clears `order`.
    #[must_use]
    pub fn set_order(mut self, value: Option<AssetOrder>) -> Self {
        self.order = value;
        self
    }
}

/// `UpdateAlbumUserDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `role` | [`AlbumUserRole`] | ✓ |  |
///
/// See: <https://api.immich.app/models/UpdateAlbumUserDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UpdateAlbumUserDTO {
    /// (required)
    pub role: AlbumUserRole,
}

impl UpdateAlbumUserDTO {
    /// Creates a new [`UpdateAlbumUserDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(role: AlbumUserRole) -> Self {
        Self { role }
    }
    /// Sets `role`.
    #[must_use]
    pub fn role(mut self, value: AlbumUserRole) -> Self {
        self.role = value;
        self
    }
}
