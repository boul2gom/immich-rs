// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `albums`.

pub use crate::activities::UserResponseDTO;

/// `AddUsersDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_users` | [`Vec<AlbumUserAddDTO>`](crate::albums::AlbumUserAddDTO) | ✓ | Album users to add |
///
/// See: <https://api.immich.app/models/AddUsersDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AddUsersDTO {
    /// Album users to add (required)
    #[serde(rename = "albumUsers")]
    pub album_users: Vec<AlbumUserAddDTO>,
}

impl AddUsersDTO {
    ///Creates a new [`AddUsersDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_users: Vec<AlbumUserAddDTO>) -> Self {
        Self { album_users }
    }

    ///Sets `album_users`.
    #[must_use]
    pub fn album_users(mut self, value: Vec<AlbumUserAddDTO>) -> Self {
        self.album_users = value;
        self
    }
}

/// `AlbumResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_name` | [`String`] | ✓ | Album name |
/// | `album_thumbnail_asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Thumbnail asset ID |
/// | `album_users` | [`Vec<AlbumUserResponseDTO>`](crate::albums::AlbumUserResponseDTO) | ✓ | First entry is always the album owner. Second entry is the auth user, if it differs from the owner. The rest are ordered alphabetically. |
/// | `asset_count` | `i64` | ✓ | Number of assets |
/// | `contributor_counts` | [`Vec<ContributorCountResponseDTO>`](crate::albums::ContributorCountResponseDTO) |  |  |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `description` | [`String`] | ✓ | Album description |
/// | `end_date` | [`DateTime`](::chrono::DateTime) |  | UTC representation of (local) end date (latest asset) |
/// | `has_shared_link` | `bool` | ✓ | Has shared link |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Album ID |
/// | `is_activity_enabled` | `bool` | ✓ | Activity feed enabled |
/// | `last_modified_asset_timestamp` | [`DateTime`](::chrono::DateTime) |  | Last modified asset timestamp |
/// | `order` | [`AssetOrder`] |  |  |
/// | `shared` | `bool` | ✓ | Is shared album |
/// | `start_date` | [`DateTime`](::chrono::DateTime) |  | UTC representation of (local) start date (earliest asset) |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
///
/// See: <https://api.immich.app/models/AlbumResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AlbumResponseDTO {
    /// Album name (required)
    #[serde(rename = "albumName")]
    pub album_name: String,
    /// Thumbnail asset ID (required)
    #[serde(rename = "albumThumbnailAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_thumbnail_asset_id: Option<::uuid::Uuid>,
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
    pub description: String,
    /// UTC representation of (local) end date (latest asset)
    #[serde(rename = "endDate", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Has shared link (required)
    #[serde(rename = "hasSharedLink")]
    pub has_shared_link: bool,
    pub id: ::uuid::Uuid,
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
    ///Creates a new [`AlbumResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        album_name: impl Into<String>,
        album_users: Vec<AlbumUserResponseDTO>,
        asset_count: i64,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        description: impl Into<String>,
        has_shared_link: bool,
        id: ::uuid::Uuid,
        is_activity_enabled: bool,
        shared: bool,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            album_name: album_name.into(),
            album_thumbnail_asset_id: None,
            album_users,
            asset_count,
            contributor_counts: Vec::new(),
            created_at,
            description: description.into(),
            end_date: None,
            has_shared_link,
            id,
            is_activity_enabled,
            last_modified_asset_timestamp: None,
            order: None,
            shared,
            start_date: None,
            updated_at,
        }
    }

    ///Sets `album_name`.
    #[must_use]
    pub fn album_name(mut self, value: impl Into<String>) -> Self {
        self.album_name = value.into();
        self
    }

    ///Sets `album_thumbnail_asset_id`.
    #[must_use]
    pub fn album_thumbnail_asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.album_thumbnail_asset_id = Some(value);
        self
    }

    ///Sets or clears `album_thumbnail_asset_id`.
    #[must_use]
    pub fn set_album_thumbnail_asset_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.album_thumbnail_asset_id = value;
        self
    }

    ///Sets `album_users`.
    #[must_use]
    pub fn album_users(mut self, value: Vec<AlbumUserResponseDTO>) -> Self {
        self.album_users = value;
        self
    }

    ///Sets `asset_count`.
    #[must_use]
    pub fn asset_count(mut self, value: i64) -> Self {
        self.asset_count = value;
        self
    }

    ///Sets `contributor_counts`.
    #[must_use]
    pub fn contributor_counts(mut self, value: Vec<ContributorCountResponseDTO>) -> Self {
        self.contributor_counts = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = value.into();
        self
    }

    ///Sets `end_date`.
    #[must_use]
    pub fn end_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.end_date = Some(value);
        self
    }

    ///Sets or clears `end_date`.
    #[must_use]
    pub fn set_end_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.end_date = value;
        self
    }

    ///Sets `has_shared_link`.
    #[must_use]
    pub fn has_shared_link(mut self, value: bool) -> Self {
        self.has_shared_link = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_activity_enabled`.
    #[must_use]
    pub fn is_activity_enabled(mut self, value: bool) -> Self {
        self.is_activity_enabled = value;
        self
    }

    ///Sets `last_modified_asset_timestamp`.
    #[must_use]
    pub fn last_modified_asset_timestamp(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.last_modified_asset_timestamp = Some(value);
        self
    }

    ///Sets or clears `last_modified_asset_timestamp`.
    #[must_use]
    pub fn set_last_modified_asset_timestamp(
        mut self,
        value: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ) -> Self {
        self.last_modified_asset_timestamp = value;
        self
    }

    ///Sets `order`.
    #[must_use]
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = Some(value);
        self
    }

    ///Sets or clears `order`.
    #[must_use]
    pub fn set_order(mut self, value: Option<AssetOrder>) -> Self {
        self.order = value;
        self
    }

    ///Sets `shared`.
    #[must_use]
    pub fn shared(mut self, value: bool) -> Self {
        self.shared = value;
        self
    }

    ///Sets `start_date`.
    #[must_use]
    pub fn start_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.start_date = Some(value);
        self
    }

    ///Sets or clears `start_date`.
    #[must_use]
    pub fn set_start_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.start_date = value;
        self
    }

    ///Sets `updated_at`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AlbumStatisticsResponseDTO {
    /// Number of non-shared albums (required)
    #[serde(rename = "notShared")]
    pub not_shared: i64,
    pub owned: i64,
    pub shared: i64,
}

impl AlbumStatisticsResponseDTO {
    ///Creates a new [`AlbumStatisticsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(not_shared: i64, owned: i64, shared: i64) -> Self {
        Self {
            not_shared,
            owned,
            shared,
        }
    }

    ///Sets `not_shared`.
    #[must_use]
    pub fn not_shared(mut self, value: i64) -> Self {
        self.not_shared = value;
        self
    }

    ///Sets `owned`.
    #[must_use]
    pub fn owned(mut self, value: i64) -> Self {
        self.owned = value;
        self
    }

    ///Sets `shared`.
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
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
///
/// See: <https://api.immich.app/models/AlbumUserAddDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AlbumUserAddDTO {
    #[serde(default = "crate::common::defaults::album_user_add_dto_role")]
    pub role: AlbumUserRole,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl AlbumUserAddDTO {
    ///Creates a new [`AlbumUserAddDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(user_id: ::uuid::Uuid) -> Self {
        Self {
            role: crate::common::defaults::album_user_add_dto_role(),
            user_id,
        }
    }

    ///Sets `role`.
    #[must_use]
    pub fn role(mut self, value: AlbumUserRole) -> Self {
        self.role = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }
}

/// `AlbumUserCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `role` | [`AlbumUserRole`] | ✓ |  |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
///
/// See: <https://api.immich.app/models/AlbumUserCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AlbumUserCreateDTO {
    pub role: AlbumUserRole,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl AlbumUserCreateDTO {
    ///Creates a new [`AlbumUserCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(role: AlbumUserRole, user_id: ::uuid::Uuid) -> Self {
        Self { role, user_id }
    }

    ///Sets `role`.
    #[must_use]
    pub fn role(mut self, value: AlbumUserRole) -> Self {
        self.role = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AlbumUserResponseDTO {
    pub role: AlbumUserRole,
    pub user: UserResponseDTO,
}

impl AlbumUserResponseDTO {
    ///Creates a new [`AlbumUserResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(role: AlbumUserRole, user: UserResponseDTO) -> Self {
        Self { role, user }
    }

    ///Sets `role`.
    #[must_use]
    pub fn role(mut self, value: AlbumUserRole) -> Self {
        self.role = value;
        self
    }

    ///Sets `user`.
    #[must_use]
    pub fn user(mut self, value: UserResponseDTO) -> Self {
        self.user = value;
        self
    }
}

/// Album user role
///
/// See: <https://api.immich.app/models/AlbumUserRole>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AlbumUserRole {
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "viewer")]
    Viewer,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AlbumUserRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Editor => f.write_str("editor"),
            Self::Owner => f.write_str("owner"),
            Self::Viewer => f.write_str("viewer"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AlbumUserRole {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "editor" => Ok(Self::Editor),
            "owner" => Ok(Self::Owner),
            "viewer" => Ok(Self::Viewer),
            _ => Err(format!("invalid value for {}: {}", "AlbumUserRole", value).into()),
        }
    }
}

impl TryFrom<&str> for AlbumUserRole {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AlbumUserRole {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AlbumUserRole {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `AlbumsAddAssetsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Album IDs |
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs |
///
/// See: <https://api.immich.app/models/AlbumsAddAssetsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AlbumsAddAssetsDTO {
    /// Album IDs (required)
    #[serde(rename = "albumIds")]
    pub album_ids: Vec<::uuid::Uuid>,
    /// Asset IDs (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<::uuid::Uuid>,
}

impl AlbumsAddAssetsDTO {
    ///Creates a new [`AlbumsAddAssetsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_ids: Vec<::uuid::Uuid>, asset_ids: Vec<::uuid::Uuid>) -> Self {
        Self { album_ids, asset_ids }
    }

    ///Sets `album_ids`.
    #[must_use]
    pub fn album_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.album_ids = value;
        self
    }

    ///Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.asset_ids = value;
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AlbumsAddAssetsResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: Option<BulkIdErrorReason>,
    pub success: bool,
}

impl AlbumsAddAssetsResponseDTO {
    ///Creates a new [`AlbumsAddAssetsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(success: bool) -> Self {
        Self { error: None, success }
    }

    ///Sets `error`.
    #[must_use]
    pub fn error(mut self, value: BulkIdErrorReason) -> Self {
        self.error = Some(value);
        self
    }

    ///Sets or clears `error`.
    #[must_use]
    pub fn set_error(mut self, value: Option<BulkIdErrorReason>) -> Self {
        self.error = value;
        self
    }

    ///Sets `success`.
    #[must_use]
    pub fn success(mut self, value: bool) -> Self {
        self.success = value;
        self
    }
}

/// Asset sort order
///
/// See: <https://api.immich.app/models/AssetOrder>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetOrder {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err(format!("invalid value for {}: {}", "AssetOrder", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetOrder {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetOrder {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetOrder {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Error reason
///
/// See: <https://api.immich.app/models/BulkIdErrorReason>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum BulkIdErrorReason {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "no_permission")]
    NoPermission,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "validation")]
    Validation,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for BulkIdErrorReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Duplicate => f.write_str("duplicate"),
            Self::NoPermission => f.write_str("no_permission"),
            Self::NotFound => f.write_str("not_found"),
            Self::Unknown => f.write_str("unknown"),
            Self::Validation => f.write_str("validation"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for BulkIdErrorReason {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "duplicate" => Ok(Self::Duplicate),
            "no_permission" => Ok(Self::NoPermission),
            "not_found" => Ok(Self::NotFound),
            "unknown" => Ok(Self::Unknown),
            "validation" => Ok(Self::Validation),
            _ => Err(format!("invalid value for {}: {}", "BulkIdErrorReason", value).into()),
        }
    }
}

impl TryFrom<&str> for BulkIdErrorReason {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for BulkIdErrorReason {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for BulkIdErrorReason {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `BulkIdResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `error` | [`BulkIdErrorReason`] |  |  |
/// | `error_message` | [`String`] |  |  |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | ID |
/// | `success` | `bool` | ✓ | Whether operation succeeded |
///
/// See: <https://api.immich.app/models/BulkIdResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct BulkIdResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: Option<BulkIdErrorReason>,
    #[serde(
        rename = "errorMessage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub error_message: Option<String>,
    pub id: ::uuid::Uuid,
    pub success: bool,
}

impl BulkIdResponseDTO {
    ///Creates a new [`BulkIdResponseDTO`].
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

    ///Sets `error`.
    #[must_use]
    pub fn error(mut self, value: BulkIdErrorReason) -> Self {
        self.error = Some(value);
        self
    }

    ///Sets or clears `error`.
    #[must_use]
    pub fn set_error(mut self, value: Option<BulkIdErrorReason>) -> Self {
        self.error = value;
        self
    }

    ///Sets `error_message`.
    #[must_use]
    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    ///Sets or clears `error_message`.
    #[must_use]
    pub fn set_error_message(mut self, value: Option<String>) -> Self {
        self.error_message = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `success`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct BulkIdsDTO {
    pub ids: Vec<::uuid::Uuid>,
}

impl BulkIdsDTO {
    ///Creates a new [`BulkIdsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: Vec<::uuid::Uuid>) -> Self {
        Self { ids }
    }

    ///Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.ids = value;
        self
    }
}

/// `ContributorCountResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_count` | `i64` | ✓ | Number of assets contributed |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
///
/// See: <https://api.immich.app/models/ContributorCountResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ContributorCountResponseDTO {
    /// Number of assets contributed (required)
    #[serde(rename = "assetCount")]
    pub asset_count: i64,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl ContributorCountResponseDTO {
    ///Creates a new [`ContributorCountResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_count: i64, user_id: ::uuid::Uuid) -> Self {
        Self { asset_count, user_id }
    }

    ///Sets `asset_count`.
    #[must_use]
    pub fn asset_count(mut self, value: i64) -> Self {
        self.asset_count = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }
}

/// `CreateAlbumDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_name` | [`String`] | ✓ | Album name |
/// | `album_users` | [`Vec<AlbumUserCreateDTO>`](crate::albums::AlbumUserCreateDTO) |  | Album users |
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Initial asset IDs |
/// | `description` | [`String`] |  | Album description |
///
/// See: <https://api.immich.app/models/CreateAlbumDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CreateAlbumDTO {
    /// Album name (required)
    #[serde(rename = "albumName")]
    pub album_name: String,
    /// Album users
    #[serde(rename = "albumUsers", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub album_users: Vec<AlbumUserCreateDTO>,
    /// Initial asset IDs
    #[serde(rename = "assetIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub asset_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
}

impl CreateAlbumDTO {
    ///Creates a new [`CreateAlbumDTO`].
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

    ///Sets `album_name`.
    #[must_use]
    pub fn album_name(mut self, value: impl Into<String>) -> Self {
        self.album_name = value.into();
        self
    }

    ///Sets `album_users`.
    #[must_use]
    pub fn album_users(mut self, value: Vec<AlbumUserCreateDTO>) -> Self {
        self.album_users = value;
        self
    }

    ///Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.asset_ids = value;
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
}

/// `MapMarkerResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `city` | [`String`] | ✓ | City name |
/// | `country` | [`String`] | ✓ | Country name |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `lat` | `f64` | ✓ | Latitude |
/// | `lon` | `f64` | ✓ | Longitude |
/// | `state` | [`String`] | ✓ | State/Province name |
///
/// See: <https://api.immich.app/models/MapMarkerResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MapMarkerResponseDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    pub id: ::uuid::Uuid,
    pub lat: f64,
    pub lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl MapMarkerResponseDTO {
    ///Creates a new [`MapMarkerResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: ::uuid::Uuid, lat: f64, lon: f64) -> Self {
        Self {
            city: None,
            country: None,
            id,
            lat,
            lon,
            state: None,
        }
    }

    ///Sets `city`.
    #[must_use]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    ///Sets or clears `city`.
    #[must_use]
    pub fn set_city(mut self, value: Option<String>) -> Self {
        self.city = value;
        self
    }

    ///Sets `country`.
    #[must_use]
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    ///Sets or clears `country`.
    #[must_use]
    pub fn set_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `lat`.
    #[must_use]
    pub fn lat(mut self, value: f64) -> Self {
        self.lat = value;
        self
    }

    ///Sets `lon`.
    #[must_use]
    pub fn lon(mut self, value: f64) -> Self {
        self.lon = value;
        self
    }

    ///Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    ///Sets or clears `state`.
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
/// | `album_thumbnail_asset_id` | [`Uuid`](::uuid::Uuid) |  | Album thumbnail asset ID |
/// | `description` | [`String`] |  | Album description |
/// | `is_activity_enabled` | `bool` |  | Enable activity feed |
/// | `order` | [`AssetOrder`] |  |  |
///
/// See: <https://api.immich.app/models/UpdateAlbumDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct UpdateAlbumDTO {
    /// Album name
    #[serde(
        rename = "albumName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub album_name: Option<String>,
    /// Album thumbnail asset ID
    #[serde(
        rename = "albumThumbnailAssetId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub album_thumbnail_asset_id: Option<::uuid::Uuid>,
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
    ///Creates a new [`UpdateAlbumDTO`].
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

    ///Sets `album_name`.
    #[must_use]
    pub fn album_name(mut self, value: impl Into<String>) -> Self {
        self.album_name = Some(value.into());
        self
    }

    ///Sets or clears `album_name`.
    #[must_use]
    pub fn set_album_name(mut self, value: Option<String>) -> Self {
        self.album_name = value;
        self
    }

    ///Sets `album_thumbnail_asset_id`.
    #[must_use]
    pub fn album_thumbnail_asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.album_thumbnail_asset_id = Some(value);
        self
    }

    ///Sets or clears `album_thumbnail_asset_id`.
    #[must_use]
    pub fn set_album_thumbnail_asset_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.album_thumbnail_asset_id = value;
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

    ///Sets `is_activity_enabled`.
    #[must_use]
    pub fn is_activity_enabled(mut self, value: bool) -> Self {
        self.is_activity_enabled = Some(value);
        self
    }

    ///Sets or clears `is_activity_enabled`.
    #[must_use]
    pub fn set_is_activity_enabled(mut self, value: Option<bool>) -> Self {
        self.is_activity_enabled = value;
        self
    }

    ///Sets `order`.
    #[must_use]
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = Some(value);
        self
    }

    ///Sets or clears `order`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UpdateAlbumUserDTO {
    pub role: AlbumUserRole,
}

impl UpdateAlbumUserDTO {
    ///Creates a new [`UpdateAlbumUserDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(role: AlbumUserRole) -> Self {
        Self { role }
    }

    ///Sets `role`.
    #[must_use]
    pub fn role(mut self, value: AlbumUserRole) -> Self {
        self.role = value;
        self
    }
}
