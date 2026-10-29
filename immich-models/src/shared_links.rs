// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `shared_links`.

pub use crate::albums::AlbumResponseDTO;
pub use crate::assets::AssetResponseDTO;

/// Error reason if failed
///
/// See: <https://api.immich.app/models/AssetIdErrorReason>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetIdErrorReason {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "no_permission")]
    NoPermission,
    #[serde(rename = "not_found")]
    NotFound,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetIdErrorReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Duplicate => f.write_str("duplicate"),
            Self::NoPermission => f.write_str("no_permission"),
            Self::NotFound => f.write_str("not_found"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetIdErrorReason {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "duplicate" => Ok(Self::Duplicate),
            "no_permission" => Ok(Self::NoPermission),
            "not_found" => Ok(Self::NotFound),
            _ => Err(format!("invalid value for {}: {}", "AssetIdErrorReason", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetIdErrorReason {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetIdErrorReason {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetIdErrorReason {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `AssetIdsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs |
///
/// See: <https://api.immich.app/models/AssetIdsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetIdsDTO {
    /// Asset IDs (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<::uuid::Uuid>,
}

impl AssetIdsDTO {
    ///Creates a new [`AssetIdsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_ids: Vec<::uuid::Uuid>) -> Self {
        Self { asset_ids }
    }

    ///Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.asset_ids = value;
        self
    }
}

/// `AssetIdsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `error` | [`AssetIdErrorReason`] |  |  |
/// | `success` | `bool` | ✓ | Whether operation succeeded |
///
/// See: <https://api.immich.app/models/AssetIdsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetIdsResponseDTO {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: Option<AssetIdErrorReason>,
    pub success: bool,
}

impl AssetIdsResponseDTO {
    ///Creates a new [`AssetIdsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: ::uuid::Uuid, success: bool) -> Self {
        Self {
            asset_id,
            error: None,
            success,
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `error`.
    #[must_use]
    pub fn error(mut self, value: AssetIdErrorReason) -> Self {
        self.error = Some(value);
        self
    }

    ///Sets or clears `error`.
    #[must_use]
    pub fn set_error(mut self, value: Option<AssetIdErrorReason>) -> Self {
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

/// `SharedLinkCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`Uuid`](::uuid::Uuid) |  | Album ID (for album sharing) |
/// | `allow_download` | `bool` |  | Allow downloads |
/// | `allow_upload` | `bool` |  | Allow uploads |
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Asset IDs (for individual assets) |
/// | `description` | [`String`] |  | Link description |
/// | `expires_at` | [`DateTime`](::chrono::DateTime) |  | Expiration date |
/// | `password` | [`String`] |  | Link password |
/// | `show_metadata` | `bool` |  | Show metadata |
/// | `slug` | [`String`] |  | Custom URL slug |
/// | `type` | [`SharedLinkType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/SharedLinkCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SharedLinkCreateDTO {
    /// Album ID (for album sharing)
    #[serde(rename = "albumId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub album_id: Option<::uuid::Uuid>,
    /// Allow downloads
    #[serde(rename = "allowDownload", default = "crate::common::defaults::default_bool::<true>")]
    pub allow_download: bool,
    /// Allow uploads
    #[serde(
        rename = "allowUpload",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_upload: Option<bool>,
    /// Asset IDs (for individual assets)
    #[serde(rename = "assetIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub asset_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Expiration date
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// Show metadata
    #[serde(rename = "showMetadata", default = "crate::common::defaults::default_bool::<true>")]
    pub show_metadata: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: SharedLinkType,
}

impl SharedLinkCreateDTO {
    ///Creates a new [`SharedLinkCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(type_: SharedLinkType) -> Self {
        Self {
            album_id: None,
            allow_download: crate::common::defaults::default_bool::<true>(),
            allow_upload: None,
            asset_ids: Vec::new(),
            description: None,
            expires_at: None,
            password: None,
            show_metadata: crate::common::defaults::default_bool::<true>(),
            slug: None,
            type_,
        }
    }

    ///Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: ::uuid::Uuid) -> Self {
        self.album_id = Some(value);
        self
    }

    ///Sets or clears `album_id`.
    #[must_use]
    pub fn set_album_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.album_id = value;
        self
    }

    ///Sets `allow_download`.
    #[must_use]
    pub fn allow_download(mut self, value: bool) -> Self {
        self.allow_download = value;
        self
    }

    ///Sets `allow_upload`.
    #[must_use]
    pub fn allow_upload(mut self, value: bool) -> Self {
        self.allow_upload = Some(value);
        self
    }

    ///Sets or clears `allow_upload`.
    #[must_use]
    pub fn set_allow_upload(mut self, value: Option<bool>) -> Self {
        self.allow_upload = value;
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

    ///Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    ///Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.expires_at = value;
        self
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    ///Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }

    ///Sets `show_metadata`.
    #[must_use]
    pub fn show_metadata(mut self, value: bool) -> Self {
        self.show_metadata = value;
        self
    }

    ///Sets `slug`.
    #[must_use]
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    ///Sets or clears `slug`.
    #[must_use]
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: SharedLinkType) -> Self {
        self.type_ = value;
        self
    }
}

/// `SharedLinkEditDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `allow_download` | `bool` |  | Allow downloads |
/// | `allow_upload` | `bool` |  | Allow uploads |
/// | `description` | [`String`] |  | Link description |
/// | `expires_at` | [`DateTime`](::chrono::DateTime) |  | Expiration date |
/// | `password` | [`String`] |  | Link password |
/// | `show_metadata` | `bool` |  | Show metadata |
/// | `slug` | [`String`] |  | Custom URL slug |
///
/// See: <https://api.immich.app/models/SharedLinkEditDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct SharedLinkEditDTO {
    /// Allow downloads
    #[serde(
        rename = "allowDownload",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_download: Option<bool>,
    /// Allow uploads
    #[serde(
        rename = "allowUpload",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allow_upload: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Expiration date
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// Show metadata
    #[serde(
        rename = "showMetadata",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_metadata: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: Option<String>,
}

impl SharedLinkEditDTO {
    ///Creates a new [`SharedLinkEditDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            allow_download: None,
            allow_upload: None,
            description: None,
            expires_at: None,
            password: None,
            show_metadata: None,
            slug: None,
        }
    }

    ///Sets `allow_download`.
    #[must_use]
    pub fn allow_download(mut self, value: bool) -> Self {
        self.allow_download = Some(value);
        self
    }

    ///Sets or clears `allow_download`.
    #[must_use]
    pub fn set_allow_download(mut self, value: Option<bool>) -> Self {
        self.allow_download = value;
        self
    }

    ///Sets `allow_upload`.
    #[must_use]
    pub fn allow_upload(mut self, value: bool) -> Self {
        self.allow_upload = Some(value);
        self
    }

    ///Sets or clears `allow_upload`.
    #[must_use]
    pub fn set_allow_upload(mut self, value: Option<bool>) -> Self {
        self.allow_upload = value;
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

    ///Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    ///Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.expires_at = value;
        self
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    ///Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }

    ///Sets `show_metadata`.
    #[must_use]
    pub fn show_metadata(mut self, value: bool) -> Self {
        self.show_metadata = Some(value);
        self
    }

    ///Sets or clears `show_metadata`.
    #[must_use]
    pub fn set_show_metadata(mut self, value: Option<bool>) -> Self {
        self.show_metadata = value;
        self
    }

    ///Sets `slug`.
    #[must_use]
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    ///Sets or clears `slug`.
    #[must_use]
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
}

/// `SharedLinkLoginDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `password` | [`String`] | ✓ | Shared link password |
///
/// See: <https://api.immich.app/models/SharedLinkLoginDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SharedLinkLoginDTO {
    pub password: String,
}

impl SharedLinkLoginDTO {
    ///Creates a new [`SharedLinkLoginDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(password: impl Into<String>) -> Self {
        Self {
            password: password.into(),
        }
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = value.into();
        self
    }
}

/// Shared link response
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album` | [`AlbumResponseDTO`] |  |  |
/// | `allow_download` | `bool` | ✓ | Allow downloads |
/// | `allow_upload` | `bool` | ✓ | Allow uploads |
/// | `assets` | [`Vec<AssetResponseDTO>`](crate::assets::AssetResponseDTO) | ✓ |  |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `description` | [`String`] | ✓ | Link description |
/// | `expires_at` | [`DateTime`](::chrono::DateTime) | ✓ | Expiration date |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Shared link ID |
/// | `key` | [`String`] | ✓ | Encryption key (base64url) |
/// | `password` | [`String`] | ✓ | Has password |
/// | `show_metadata` | `bool` | ✓ | Show metadata |
/// | `slug` | [`String`] | ✓ | Custom URL slug |
/// | `type` | [`SharedLinkType`] | ✓ |  |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner user ID |
///
/// See: <https://api.immich.app/models/SharedLinkResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SharedLinkResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub album: Option<AlbumResponseDTO>,
    /// Allow downloads (required)
    #[serde(rename = "allowDownload")]
    pub allow_download: bool,
    /// Allow uploads (required)
    #[serde(rename = "allowUpload")]
    pub allow_upload: bool,
    pub assets: Vec<AssetResponseDTO>,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Expiration date (required)
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub id: ::uuid::Uuid,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Show metadata (required)
    #[serde(rename = "showMetadata")]
    pub show_metadata: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: SharedLinkType,
    /// Owner user ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl SharedLinkResponseDTO {
    ///Creates a new [`SharedLinkResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        allow_download: bool,
        allow_upload: bool,
        assets: Vec<AssetResponseDTO>,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        key: impl Into<String>,
        show_metadata: bool,
        type_: SharedLinkType,
        user_id: ::uuid::Uuid,
    ) -> Self {
        Self {
            album: None,
            allow_download,
            allow_upload,
            assets,
            created_at,
            description: None,
            expires_at: None,
            id,
            key: key.into(),
            password: None,
            show_metadata,
            slug: None,
            type_,
            user_id,
        }
    }

    ///Sets `album`.
    #[must_use]
    pub fn album(mut self, value: AlbumResponseDTO) -> Self {
        self.album = Some(value);
        self
    }

    ///Sets or clears `album`.
    #[must_use]
    pub fn set_album(mut self, value: Option<AlbumResponseDTO>) -> Self {
        self.album = value;
        self
    }

    ///Sets `allow_download`.
    #[must_use]
    pub fn allow_download(mut self, value: bool) -> Self {
        self.allow_download = value;
        self
    }

    ///Sets `allow_upload`.
    #[must_use]
    pub fn allow_upload(mut self, value: bool) -> Self {
        self.allow_upload = value;
        self
    }

    ///Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: Vec<AssetResponseDTO>) -> Self {
        self.assets = value;
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
        self.description = Some(value.into());
        self
    }

    ///Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }

    ///Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    ///Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.expires_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    ///Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }

    ///Sets `show_metadata`.
    #[must_use]
    pub fn show_metadata(mut self, value: bool) -> Self {
        self.show_metadata = value;
        self
    }

    ///Sets `slug`.
    #[must_use]
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    ///Sets or clears `slug`.
    #[must_use]
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: SharedLinkType) -> Self {
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

/// Shared link type
///
/// See: <https://api.immich.app/models/SharedLinkType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SharedLinkType {
    #[serde(rename = "ALBUM")]
    Album,
    #[serde(rename = "INDIVIDUAL")]
    Individual,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for SharedLinkType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Album => f.write_str("ALBUM"),
            Self::Individual => f.write_str("INDIVIDUAL"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for SharedLinkType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "ALBUM" => Ok(Self::Album),
            "INDIVIDUAL" => Ok(Self::Individual),
            _ => Err(format!("invalid value for {}: {}", "SharedLinkType", value).into()),
        }
    }
}

impl TryFrom<&str> for SharedLinkType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SharedLinkType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SharedLinkType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}
