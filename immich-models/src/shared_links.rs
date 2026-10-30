// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `shared_links`.

pub use crate::albums::AlbumResponseDTO;
pub use crate::assets::AssetResponseDTO;
pub use crate::id::{AlbumId, AssetId, SharedLinkId, UserId};

/// Error reason if failed
///
/// See: <https://api.immich.app/models/AssetIdErrorReason>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetIdErrorReason {
    Duplicate,
    NoPermission,
    NotFound,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetIdErrorReason {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Duplicate => "duplicate",
            Self::NoPermission => "no_permission",
            Self::NotFound => "not_found",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetIdErrorReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetIdErrorReason {
    fn from(value: &str) -> Self {
        match value {
            "duplicate" => Self::Duplicate,
            "no_permission" => Self::NoPermission,
            "not_found" => Self::NotFound,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetIdErrorReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "duplicate" => Self::Duplicate,
            "no_permission" => Self::NoPermission,
            "not_found" => Self::NotFound,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetIdErrorReason {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetIdErrorReason {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetIdErrorReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetIdErrorReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetIdErrorReason;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetIdErrorReason::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetIdErrorReason::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `AssetIdsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs |
///
/// See: <https://api.immich.app/models/AssetIdsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetIdsDTO {
    /// Asset IDs (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<AssetId>,
}

impl AssetIdsDTO {
    /// Creates a new [`AssetIdsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_ids: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        Self {
            asset_ids: asset_ids.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `AssetIdsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `error` | [`AssetIdErrorReason`] |  |  |
/// | `success` | `bool` | ✓ | Whether operation succeeded |
///
/// See: <https://api.immich.app/models/AssetIdsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetIdsResponseDTO {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: Option<AssetIdErrorReason>,
    /// Whether operation succeeded (required)
    pub success: bool,
}

impl AssetIdsResponseDTO {
    /// Creates a new [`AssetIdsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: impl Into<AssetId>, success: bool) -> Self {
        Self {
            asset_id: asset_id.into(),
            error: None,
            success,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `error`.
    #[must_use]
    pub fn error(mut self, value: AssetIdErrorReason) -> Self {
        self.error = Some(value);
        self
    }
    /// Sets or clears `error`.
    #[must_use]
    pub fn set_error(mut self, value: Option<AssetIdErrorReason>) -> Self {
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

/// `SharedLinkCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`AlbumId`](crate::id::AlbumId) |  | Album ID (for album sharing) (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `allow_download` | `bool` |  | Allow downloads |
/// | `allow_upload` | `bool` |  | Allow uploads |
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) |  | Asset IDs (for individual assets) |
/// | `description` | [`String`] |  | Link description |
/// | `expires_at` | [`DateTime`](::chrono::DateTime) |  | Expiration date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `password` | [`String`] |  | Link password |
/// | `show_metadata` | `bool` |  | Show metadata |
/// | `slug` | [`String`] |  | Custom URL slug |
/// | `type_` | [`SharedLinkType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/SharedLinkCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SharedLinkCreateDTO {
    /// Album ID (for album sharing) (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(rename = "albumId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub album_id: Option<AlbumId>,
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
    pub asset_ids: Vec<AssetId>,
    /// Link description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Expiration date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Link password
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// Show metadata
    #[serde(rename = "showMetadata", default = "crate::common::defaults::default_bool::<true>")]
    pub show_metadata: bool,
    /// Custom URL slug
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: SharedLinkType,
}

impl SharedLinkCreateDTO {
    /// Creates a new [`SharedLinkCreateDTO`].
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
    /// Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: impl Into<AlbumId>) -> Self {
        self.album_id = Some(value.into());
        self
    }
    /// Sets or clears `album_id`.
    #[must_use]
    pub fn set_album_id(mut self, value: Option<AlbumId>) -> Self {
        self.album_id = value;
        self
    }
    /// Sets `allow_download`.
    #[must_use]
    pub fn allow_download(mut self, value: bool) -> Self {
        self.allow_download = value;
        self
    }
    /// Sets `allow_upload`.
    #[must_use]
    pub fn allow_upload(mut self, value: bool) -> Self {
        self.allow_upload = Some(value);
        self
    }
    /// Sets or clears `allow_upload`.
    #[must_use]
    pub fn set_allow_upload(mut self, value: Option<bool>) -> Self {
        self.allow_upload = value;
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
    /// Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }
    /// Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.expires_at = value;
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
    /// Sets `show_metadata`.
    #[must_use]
    pub fn show_metadata(mut self, value: bool) -> Self {
        self.show_metadata = value;
        self
    }
    /// Sets `slug`.
    #[must_use]
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    #[must_use]
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sets `type_`.
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
/// | `expires_at` | [`DateTime`](::chrono::DateTime) |  | Expiration date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `password` | [`String`] |  | Link password |
/// | `show_metadata` | `bool` |  | Show metadata |
/// | `slug` | [`String`] |  | Custom URL slug |
///
/// See: <https://api.immich.app/models/SharedLinkEditDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Link description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Expiration date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(
        rename = "expiresAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub expires_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Link password
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
    /// Show metadata
    #[serde(
        rename = "showMetadata",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_metadata: Option<bool>,
    /// Custom URL slug
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub slug: Option<String>,
}

impl SharedLinkEditDTO {
    /// Creates a new [`SharedLinkEditDTO`].
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
    /// Sets `allow_download`.
    #[must_use]
    pub fn allow_download(mut self, value: bool) -> Self {
        self.allow_download = Some(value);
        self
    }
    /// Sets or clears `allow_download`.
    #[must_use]
    pub fn set_allow_download(mut self, value: Option<bool>) -> Self {
        self.allow_download = value;
        self
    }
    /// Sets `allow_upload`.
    #[must_use]
    pub fn allow_upload(mut self, value: bool) -> Self {
        self.allow_upload = Some(value);
        self
    }
    /// Sets or clears `allow_upload`.
    #[must_use]
    pub fn set_allow_upload(mut self, value: Option<bool>) -> Self {
        self.allow_upload = value;
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
    /// Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }
    /// Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.expires_at = value;
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
    /// Sets `show_metadata`.
    #[must_use]
    pub fn show_metadata(mut self, value: bool) -> Self {
        self.show_metadata = Some(value);
        self
    }
    /// Sets or clears `show_metadata`.
    #[must_use]
    pub fn set_show_metadata(mut self, value: Option<bool>) -> Self {
        self.show_metadata = value;
        self
    }
    /// Sets `slug`.
    #[must_use]
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SharedLinkLoginDTO {
    /// Shared link password (required)
    pub password: String,
}

impl SharedLinkLoginDTO {
    /// Creates a new [`SharedLinkLoginDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(password: impl Into<String>) -> Self {
        Self {
            password: password.into(),
        }
    }
    /// Sets `password`.
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
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `description` | [`String`] | ✓ | Link description |
/// | `expires_at` | [`DateTime`](::chrono::DateTime) | ✓ | Expiration date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`SharedLinkId`](crate::id::SharedLinkId) | ✓ | Shared link ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `key` | [`String`] | ✓ | Encryption key (base64url) |
/// | `password` | [`String`] | ✓ | Has password |
/// | `show_metadata` | `bool` | ✓ | Show metadata |
/// | `slug` | [`String`] | ✓ | Custom URL slug |
/// | `type_` | [`SharedLinkType`] | ✓ |  |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SharedLinkResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SharedLinkResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub album: Option<AlbumResponseDTO>,
    /// Allow downloads (required)
    #[serde(rename = "allowDownload")]
    pub allow_download: bool,
    /// Allow uploads (required)
    #[serde(rename = "allowUpload")]
    pub allow_upload: bool,
    /// (required)
    pub assets: Vec<AssetResponseDTO>,
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Link description (required)
    pub description: Option<String>,
    /// Expiration date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Shared link ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: SharedLinkId,
    /// Encryption key (base64url) (required)
    pub key: String,
    /// Has password (required)
    pub password: Option<String>,
    /// Show metadata (required)
    #[serde(rename = "showMetadata")]
    pub show_metadata: bool,
    /// Custom URL slug (required)
    pub slug: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: SharedLinkType,
    /// Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl SharedLinkResponseDTO {
    /// Creates a new [`SharedLinkResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        allow_download: bool,
        allow_upload: bool,
        assets: impl IntoIterator<Item = AssetResponseDTO>,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<SharedLinkId>,
        key: impl Into<String>,
        show_metadata: bool,
        type_: SharedLinkType,
        user_id: impl Into<UserId>,
    ) -> Self {
        Self {
            album: None,
            allow_download,
            allow_upload,
            assets: assets.into_iter().collect(),
            created_at,
            description: None,
            expires_at: None,
            id: id.into(),
            key: key.into(),
            password: None,
            show_metadata,
            slug: None,
            type_,
            user_id: user_id.into(),
        }
    }
    /// Sets `album`.
    #[must_use]
    pub fn album(mut self, value: AlbumResponseDTO) -> Self {
        self.album = Some(value);
        self
    }
    /// Sets or clears `album`.
    #[must_use]
    pub fn set_album(mut self, value: Option<AlbumResponseDTO>) -> Self {
        self.album = value;
        self
    }
    /// Sets `allow_download`.
    #[must_use]
    pub fn allow_download(mut self, value: bool) -> Self {
        self.allow_download = value;
        self
    }
    /// Sets `allow_upload`.
    #[must_use]
    pub fn allow_upload(mut self, value: bool) -> Self {
        self.allow_upload = value;
        self
    }
    /// Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: impl IntoIterator<Item = AssetResponseDTO>) -> Self {
        self.assets = value.into_iter().collect();
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
        self.description = Some(value.into());
        self
    }
    /// Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Sets `expires_at`.
    #[must_use]
    pub fn expires_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }
    /// Sets or clears `expires_at`.
    #[must_use]
    pub fn set_expires_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.expires_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<SharedLinkId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
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
    /// Sets `show_metadata`.
    #[must_use]
    pub fn show_metadata(mut self, value: bool) -> Self {
        self.show_metadata = value;
        self
    }
    /// Sets `slug`.
    #[must_use]
    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }
    /// Sets or clears `slug`.
    #[must_use]
    pub fn set_slug(mut self, value: Option<String>) -> Self {
        self.slug = value;
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: SharedLinkType) -> Self {
        self.type_ = value;
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
    }
}

/// Shared link type
///
/// See: <https://api.immich.app/models/SharedLinkType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SharedLinkType {
    Album,
    Individual,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl SharedLinkType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Album => "ALBUM",
            Self::Individual => "INDIVIDUAL",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for SharedLinkType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for SharedLinkType {
    fn from(value: &str) -> Self {
        match value {
            "ALBUM" => Self::Album,
            "INDIVIDUAL" => Self::Individual,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for SharedLinkType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ALBUM" => Self::Album,
            "INDIVIDUAL" => Self::Individual,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for SharedLinkType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for SharedLinkType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for SharedLinkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for SharedLinkType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = SharedLinkType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SharedLinkType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SharedLinkType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}
