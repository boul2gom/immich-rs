// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `sync`.

pub use crate::activities::UserAvatarColor;
pub use crate::albums::{AlbumUserRole, AssetOrder};
pub use crate::assets::{AssetEditAction, AssetTypeEnum, AssetVisibility};
pub use crate::id::{AlbumId, AssetId, FaceId, MemoryId, PersonId, StackId, UserId};
pub use crate::memories::{MemoryType, MemoryTypeV1};
pub use crate::users::UserMetadataKey;

/// `SyncAckDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `types` | [`Vec<SyncEntityType>`](crate::sync::SyncEntityType) |  | Sync entity types to delete acks for |
///
/// See: <https://api.immich.app/models/SyncAckDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct SyncAckDeleteDTO {
    /// Sync entity types to delete acks for
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub types: Vec<SyncEntityType>,
}

impl SyncAckDeleteDTO {
    /// Creates a new [`SyncAckDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }
    /// Sets `types`.
    #[must_use]
    pub fn types(mut self, value: impl IntoIterator<Item = SyncEntityType>) -> Self {
        self.types = value.into_iter().collect();
        self
    }
}

/// `SyncAckDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ack` | [`String`] | ✓ | Acknowledgment ID |
/// | `type_` | [`SyncEntityType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/SyncAckDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAckDTO {
    /// Acknowledgment ID (required)
    pub ack: String,
    /// (required)
    #[serde(rename = "type")]
    pub type_: SyncEntityType,
}

impl SyncAckDTO {
    /// Creates a new [`SyncAckDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ack: impl Into<String>, type_: SyncEntityType) -> Self {
        Self { ack: ack.into(), type_ }
    }
    /// Sets `ack`.
    #[must_use]
    pub fn ack(mut self, value: impl Into<String>) -> Self {
        self.ack = value.into();
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: SyncEntityType) -> Self {
        self.type_ = value;
        self
    }
}

/// `SyncAckSetDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `acks` | [`Vec<String>`](::std::vec::Vec) | ✓ | Acknowledgment IDs (max 1000) |
///
/// See: <https://api.immich.app/models/SyncAckSetDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAckSetDTO {
    /// Acknowledgment IDs (max 1000) (required)
    pub acks: Vec<String>,
}

impl SyncAckSetDTO {
    /// Creates a new [`SyncAckSetDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(acks: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            acks: acks.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `acks`.
    #[must_use]
    pub fn acks(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.acks = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `SyncAckV1`
///
/// See: <https://api.immich.app/models/SyncAckV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct SyncAckV1(pub ::serde_json::Map<String, ::serde_json::Value>);

impl ::std::ops::Deref for SyncAckV1 {
    type Target = ::serde_json::Map<String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<String, ::serde_json::Value> {
        &self.0
    }
}

impl From<::serde_json::Map<String, ::serde_json::Value>> for SyncAckV1 {
    fn from(value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}

/// `SyncAlbumDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`AlbumId`](crate::id::AlbumId) | ✓ | Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncAlbumDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAlbumDeleteV1 {
    /// Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "albumId")]
    pub album_id: AlbumId,
}

impl SyncAlbumDeleteV1 {
    /// Creates a new [`SyncAlbumDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: impl Into<AlbumId>) -> Self {
        Self {
            album_id: album_id.into(),
        }
    }
    /// Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: impl Into<AlbumId>) -> Self {
        self.album_id = value.into();
        self
    }
}

/// `SyncAlbumToAssetDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`AlbumId`](crate::id::AlbumId) | ✓ | Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncAlbumToAssetDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAlbumToAssetDeleteV1 {
    /// Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "albumId")]
    pub album_id: AlbumId,
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
}

impl SyncAlbumToAssetDeleteV1 {
    /// Creates a new [`SyncAlbumToAssetDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: impl Into<AlbumId>, asset_id: impl Into<AssetId>) -> Self {
        Self {
            album_id: album_id.into(),
            asset_id: asset_id.into(),
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
        self.asset_id = value.into();
        self
    }
}

/// `SyncAlbumToAssetV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`AlbumId`](crate::id::AlbumId) | ✓ | Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncAlbumToAssetV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAlbumToAssetV1 {
    /// Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "albumId")]
    pub album_id: AlbumId,
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
}

impl SyncAlbumToAssetV1 {
    /// Creates a new [`SyncAlbumToAssetV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: impl Into<AlbumId>, asset_id: impl Into<AssetId>) -> Self {
        Self {
            album_id: album_id.into(),
            asset_id: asset_id.into(),
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
        self.asset_id = value.into();
        self
    }
}

/// `SyncAlbumUserDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`AlbumId`](crate::id::AlbumId) | ✓ | Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncAlbumUserDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAlbumUserDeleteV1 {
    /// Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "albumId")]
    pub album_id: AlbumId,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl SyncAlbumUserDeleteV1 {
    /// Creates a new [`SyncAlbumUserDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: impl Into<AlbumId>, user_id: impl Into<UserId>) -> Self {
        Self {
            album_id: album_id.into(),
            user_id: user_id.into(),
        }
    }
    /// Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: impl Into<AlbumId>) -> Self {
        self.album_id = value.into();
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
    }
}

/// `SyncAlbumUserV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`AlbumId`](crate::id::AlbumId) | ✓ | Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `role` | [`AlbumUserRole`] | ✓ |  |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncAlbumUserV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAlbumUserV1 {
    /// Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "albumId")]
    pub album_id: AlbumId,
    /// (required)
    pub role: AlbumUserRole,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl SyncAlbumUserV1 {
    /// Creates a new [`SyncAlbumUserV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: impl Into<AlbumId>, role: AlbumUserRole, user_id: impl Into<UserId>) -> Self {
        Self {
            album_id: album_id.into(),
            role,
            user_id: user_id.into(),
        }
    }
    /// Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: impl Into<AlbumId>) -> Self {
        self.album_id = value.into();
        self
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

/// `SyncAlbumV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `description` | [`String`] | ✓ | Album description |
/// | `id` | [`AlbumId`](crate::id::AlbumId) | ✓ | Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_activity_enabled` | `bool` | ✓ | Is activity enabled |
/// | `name` | [`String`] | ✓ | Album name |
/// | `order` | [`AssetOrder`] | ✓ |  |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `thumbnail_asset_id` | [`String`] | ✓ | Thumbnail asset ID |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/SyncAlbumV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAlbumV1 {
    /// Created at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Album description (required)
    pub description: String,
    /// Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: AlbumId,
    /// Is activity enabled (required)
    #[serde(rename = "isActivityEnabled")]
    pub is_activity_enabled: bool,
    /// Album name (required)
    pub name: String,
    /// (required)
    pub order: AssetOrder,
    /// Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    /// Thumbnail asset ID (required)
    #[serde(rename = "thumbnailAssetId")]
    pub thumbnail_asset_id: Option<String>,
    /// Updated at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncAlbumV1 {
    /// Creates a new [`SyncAlbumV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        description: impl Into<String>,
        id: impl Into<AlbumId>,
        is_activity_enabled: bool,
        name: impl Into<String>,
        order: AssetOrder,
        owner_id: impl Into<UserId>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            description: description.into(),
            id: id.into(),
            is_activity_enabled,
            name: name.into(),
            order,
            owner_id: owner_id.into(),
            thumbnail_asset_id: None,
            updated_at,
        }
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
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `order`.
    #[must_use]
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = value;
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `thumbnail_asset_id`.
    #[must_use]
    pub fn thumbnail_asset_id(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_asset_id = Some(value.into());
        self
    }
    /// Sets or clears `thumbnail_asset_id`.
    #[must_use]
    pub fn set_thumbnail_asset_id(mut self, value: Option<String>) -> Self {
        self.thumbnail_asset_id = value;
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `SyncAlbumV2`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `description` | [`String`] | ✓ | Album description |
/// | `id` | [`AlbumId`](crate::id::AlbumId) | ✓ | Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_activity_enabled` | `bool` | ✓ | Is activity enabled |
/// | `name` | [`String`] | ✓ | Album name |
/// | `order` | [`AssetOrder`] | ✓ |  |
/// | `thumbnail_asset_id` | [`String`] | ✓ | Thumbnail asset ID |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/SyncAlbumV2>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAlbumV2 {
    /// Created at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Album description (required)
    pub description: String,
    /// Album ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: AlbumId,
    /// Is activity enabled (required)
    #[serde(rename = "isActivityEnabled")]
    pub is_activity_enabled: bool,
    /// Album name (required)
    pub name: String,
    /// (required)
    pub order: AssetOrder,
    /// Thumbnail asset ID (required)
    #[serde(rename = "thumbnailAssetId")]
    pub thumbnail_asset_id: Option<String>,
    /// Updated at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncAlbumV2 {
    /// Creates a new [`SyncAlbumV2`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        description: impl Into<String>,
        id: impl Into<AlbumId>,
        is_activity_enabled: bool,
        name: impl Into<String>,
        order: AssetOrder,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            description: description.into(),
            id: id.into(),
            is_activity_enabled,
            name: name.into(),
            order,
            thumbnail_asset_id: None,
            updated_at,
        }
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
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `order`.
    #[must_use]
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = value;
        self
    }
    /// Sets `thumbnail_asset_id`.
    #[must_use]
    pub fn thumbnail_asset_id(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_asset_id = Some(value.into());
        self
    }
    /// Sets or clears `thumbnail_asset_id`.
    #[must_use]
    pub fn set_thumbnail_asset_id(mut self, value: Option<String>) -> Self {
        self.thumbnail_asset_id = value;
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `SyncAssetDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncAssetDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAssetDeleteV1 {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
}

impl SyncAssetDeleteV1 {
    /// Creates a new [`SyncAssetDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: impl Into<AssetId>) -> Self {
        Self {
            asset_id: asset_id.into(),
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
}

/// `SyncAssetEditDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `edit_id` | [`Uuid`](::uuid::Uuid) | ✓ | Edit ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncAssetEditDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAssetEditDeleteV1 {
    /// Edit ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "editId")]
    pub edit_id: ::uuid::Uuid,
}

impl SyncAssetEditDeleteV1 {
    /// Creates a new [`SyncAssetEditDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(edit_id: ::uuid::Uuid) -> Self {
        Self { edit_id }
    }
    /// Sets `edit_id`.
    #[must_use]
    pub fn edit_id(mut self, value: ::uuid::Uuid) -> Self {
        self.edit_id = value;
        self
    }
}

/// `SyncAssetEditV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `action` | [`AssetEditAction`] | ✓ |  |
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Edit ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `parameters` | `object` | ✓ | Edit parameters |
/// | `sequence` | `i64` | ✓ | Edit sequence |
///
/// See: <https://api.immich.app/models/SyncAssetEditV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SyncAssetEditV1 {
    /// (required)
    pub action: AssetEditAction,
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Edit ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: ::uuid::Uuid,
    /// Edit parameters (required)
    pub parameters: ::serde_json::Map<String, ::serde_json::Value>,
    /// Edit sequence (required)
    pub sequence: i64,
}

impl SyncAssetEditV1 {
    /// Creates a new [`SyncAssetEditV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        action: AssetEditAction,
        asset_id: impl Into<AssetId>,
        id: ::uuid::Uuid,
        parameters: ::serde_json::Map<String, ::serde_json::Value>,
        sequence: i64,
    ) -> Self {
        Self {
            action,
            asset_id: asset_id.into(),
            id,
            parameters,
            sequence,
        }
    }
    /// Sets `action`.
    #[must_use]
    pub fn action(mut self, value: AssetEditAction) -> Self {
        self.action = value;
        self
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }
    /// Sets `parameters`.
    #[must_use]
    pub fn parameters(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.parameters = value;
        self
    }
    /// Sets `sequence`.
    #[must_use]
    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = value;
        self
    }
}

/// `SyncAssetExifV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `city` | [`String`] | ✓ | City |
/// | `country` | [`String`] | ✓ | Country |
/// | `date_time_original` | [`DateTime`](::chrono::DateTime) | ✓ | Date time original (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `description` | [`String`] | ✓ | Description |
/// | `exif_image_height` | `i64` | ✓ | Exif image height |
/// | `exif_image_width` | `i64` | ✓ | Exif image width |
/// | `exposure_time` | [`String`] | ✓ | Exposure time |
/// | `f_number` | `f64` | ✓ | F number |
/// | `file_size_in_byte` | `i64` | ✓ | File size in byte |
/// | `focal_length` | `f64` | ✓ | Focal length |
/// | `fps` | `f64` | ✓ | FPS |
/// | `iso` | `i64` | ✓ | ISO |
/// | `latitude` | `f64` | ✓ | Latitude |
/// | `lens_model` | [`String`] | ✓ | Lens model |
/// | `longitude` | `f64` | ✓ | Longitude |
/// | `make` | [`String`] | ✓ | Make |
/// | `model` | [`String`] | ✓ | Model |
/// | `modify_date` | [`DateTime`](::chrono::DateTime) | ✓ | Modify date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `orientation` | [`String`] | ✓ | Orientation |
/// | `profile_description` | [`String`] | ✓ | Profile description |
/// | `projection_type` | [`String`] | ✓ | Projection type |
/// | `rating` | `i64` | ✓ | Rating |
/// | `state` | [`String`] | ✓ | State |
/// | `time_zone` | [`String`] | ✓ | Time zone |
///
/// See: <https://api.immich.app/models/SyncAssetExifV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SyncAssetExifV1 {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// City (required)
    pub city: Option<String>,
    /// Country (required)
    pub country: Option<String>,
    /// Date time original (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "dateTimeOriginal")]
    pub date_time_original: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Description (required)
    pub description: Option<String>,
    /// Exif image height (required)
    #[serde(rename = "exifImageHeight")]
    pub exif_image_height: Option<i64>,
    /// Exif image width (required)
    #[serde(rename = "exifImageWidth")]
    pub exif_image_width: Option<i64>,
    /// Exposure time (required)
    #[serde(rename = "exposureTime")]
    pub exposure_time: Option<String>,
    /// F number (required)
    #[serde(rename = "fNumber")]
    pub f_number: Option<f64>,
    /// File size in byte (required)
    #[serde(rename = "fileSizeInByte")]
    pub file_size_in_byte: Option<i64>,
    /// Focal length (required)
    #[serde(rename = "focalLength")]
    pub focal_length: Option<f64>,
    /// FPS (required)
    pub fps: Option<f64>,
    /// ISO (required)
    pub iso: Option<i64>,
    /// Latitude (required)
    pub latitude: Option<f64>,
    /// Lens model (required)
    #[serde(rename = "lensModel")]
    pub lens_model: Option<String>,
    /// Longitude (required)
    pub longitude: Option<f64>,
    /// Make (required)
    pub make: Option<String>,
    /// Model (required)
    pub model: Option<String>,
    /// Modify date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "modifyDate")]
    pub modify_date: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Orientation (required)
    pub orientation: Option<String>,
    /// Profile description (required)
    #[serde(rename = "profileDescription")]
    pub profile_description: Option<String>,
    /// Projection type (required)
    #[serde(rename = "projectionType")]
    pub projection_type: Option<String>,
    /// Rating (required)
    pub rating: Option<i64>,
    /// State (required)
    pub state: Option<String>,
    /// Time zone (required)
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
}

impl SyncAssetExifV1 {
    /// Creates a new [`SyncAssetExifV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: impl Into<AssetId>) -> Self {
        Self {
            asset_id: asset_id.into(),
            city: None,
            country: None,
            date_time_original: None,
            description: None,
            exif_image_height: None,
            exif_image_width: None,
            exposure_time: None,
            f_number: None,
            file_size_in_byte: None,
            focal_length: None,
            fps: None,
            iso: None,
            latitude: None,
            lens_model: None,
            longitude: None,
            make: None,
            model: None,
            modify_date: None,
            orientation: None,
            profile_description: None,
            projection_type: None,
            rating: None,
            state: None,
            time_zone: None,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
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
    /// Sets `date_time_original`.
    #[must_use]
    pub fn date_time_original(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.date_time_original = Some(value);
        self
    }
    /// Sets or clears `date_time_original`.
    #[must_use]
    pub fn set_date_time_original(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.date_time_original = value;
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
    /// Sets `exif_image_height`.
    #[must_use]
    pub fn exif_image_height(mut self, value: i64) -> Self {
        self.exif_image_height = Some(value);
        self
    }
    /// Sets or clears `exif_image_height`.
    #[must_use]
    pub fn set_exif_image_height(mut self, value: Option<i64>) -> Self {
        self.exif_image_height = value;
        self
    }
    /// Sets `exif_image_width`.
    #[must_use]
    pub fn exif_image_width(mut self, value: i64) -> Self {
        self.exif_image_width = Some(value);
        self
    }
    /// Sets or clears `exif_image_width`.
    #[must_use]
    pub fn set_exif_image_width(mut self, value: Option<i64>) -> Self {
        self.exif_image_width = value;
        self
    }
    /// Sets `exposure_time`.
    #[must_use]
    pub fn exposure_time(mut self, value: impl Into<String>) -> Self {
        self.exposure_time = Some(value.into());
        self
    }
    /// Sets or clears `exposure_time`.
    #[must_use]
    pub fn set_exposure_time(mut self, value: Option<String>) -> Self {
        self.exposure_time = value;
        self
    }
    /// Sets `f_number`.
    #[must_use]
    pub fn f_number(mut self, value: f64) -> Self {
        self.f_number = Some(value);
        self
    }
    /// Sets or clears `f_number`.
    #[must_use]
    pub fn set_f_number(mut self, value: Option<f64>) -> Self {
        self.f_number = value;
        self
    }
    /// Sets `file_size_in_byte`.
    #[must_use]
    pub fn file_size_in_byte(mut self, value: i64) -> Self {
        self.file_size_in_byte = Some(value);
        self
    }
    /// Sets or clears `file_size_in_byte`.
    #[must_use]
    pub fn set_file_size_in_byte(mut self, value: Option<i64>) -> Self {
        self.file_size_in_byte = value;
        self
    }
    /// Sets `focal_length`.
    #[must_use]
    pub fn focal_length(mut self, value: f64) -> Self {
        self.focal_length = Some(value);
        self
    }
    /// Sets or clears `focal_length`.
    #[must_use]
    pub fn set_focal_length(mut self, value: Option<f64>) -> Self {
        self.focal_length = value;
        self
    }
    /// Sets `fps`.
    #[must_use]
    pub fn fps(mut self, value: f64) -> Self {
        self.fps = Some(value);
        self
    }
    /// Sets or clears `fps`.
    #[must_use]
    pub fn set_fps(mut self, value: Option<f64>) -> Self {
        self.fps = value;
        self
    }
    /// Sets `iso`.
    #[must_use]
    pub fn iso(mut self, value: i64) -> Self {
        self.iso = Some(value);
        self
    }
    /// Sets or clears `iso`.
    #[must_use]
    pub fn set_iso(mut self, value: Option<i64>) -> Self {
        self.iso = value;
        self
    }
    /// Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }
    /// Sets or clears `latitude`.
    #[must_use]
    pub fn set_latitude(mut self, value: Option<f64>) -> Self {
        self.latitude = value;
        self
    }
    /// Sets `lens_model`.
    #[must_use]
    pub fn lens_model(mut self, value: impl Into<String>) -> Self {
        self.lens_model = Some(value.into());
        self
    }
    /// Sets or clears `lens_model`.
    #[must_use]
    pub fn set_lens_model(mut self, value: Option<String>) -> Self {
        self.lens_model = value;
        self
    }
    /// Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }
    /// Sets or clears `longitude`.
    #[must_use]
    pub fn set_longitude(mut self, value: Option<f64>) -> Self {
        self.longitude = value;
        self
    }
    /// Sets `make`.
    #[must_use]
    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }
    /// Sets or clears `make`.
    #[must_use]
    pub fn set_make(mut self, value: Option<String>) -> Self {
        self.make = value;
        self
    }
    /// Sets `model`.
    #[must_use]
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }
    /// Sets or clears `model`.
    #[must_use]
    pub fn set_model(mut self, value: Option<String>) -> Self {
        self.model = value;
        self
    }
    /// Sets `modify_date`.
    #[must_use]
    pub fn modify_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.modify_date = Some(value);
        self
    }
    /// Sets or clears `modify_date`.
    #[must_use]
    pub fn set_modify_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.modify_date = value;
        self
    }
    /// Sets `orientation`.
    #[must_use]
    pub fn orientation(mut self, value: impl Into<String>) -> Self {
        self.orientation = Some(value.into());
        self
    }
    /// Sets or clears `orientation`.
    #[must_use]
    pub fn set_orientation(mut self, value: Option<String>) -> Self {
        self.orientation = value;
        self
    }
    /// Sets `profile_description`.
    #[must_use]
    pub fn profile_description(mut self, value: impl Into<String>) -> Self {
        self.profile_description = Some(value.into());
        self
    }
    /// Sets or clears `profile_description`.
    #[must_use]
    pub fn set_profile_description(mut self, value: Option<String>) -> Self {
        self.profile_description = value;
        self
    }
    /// Sets `projection_type`.
    #[must_use]
    pub fn projection_type(mut self, value: impl Into<String>) -> Self {
        self.projection_type = Some(value.into());
        self
    }
    /// Sets or clears `projection_type`.
    #[must_use]
    pub fn set_projection_type(mut self, value: Option<String>) -> Self {
        self.projection_type = value;
        self
    }
    /// Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: i64) -> Self {
        self.rating = Some(value);
        self
    }
    /// Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<i64>) -> Self {
        self.rating = value;
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
    /// Sets `time_zone`.
    #[must_use]
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }
    /// Sets or clears `time_zone`.
    #[must_use]
    pub fn set_time_zone(mut self, value: Option<String>) -> Self {
        self.time_zone = value;
        self
    }
}

/// `SyncAssetFaceDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_face_id` | [`FaceId`](crate::id::FaceId) | ✓ | Asset face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncAssetFaceDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAssetFaceDeleteV1 {
    /// Asset face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetFaceId")]
    pub asset_face_id: FaceId,
}

impl SyncAssetFaceDeleteV1 {
    /// Creates a new [`SyncAssetFaceDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_face_id: impl Into<FaceId>) -> Self {
        Self {
            asset_face_id: asset_face_id.into(),
        }
    }
    /// Sets `asset_face_id`.
    #[must_use]
    pub fn asset_face_id(mut self, value: impl Into<FaceId>) -> Self {
        self.asset_face_id = value.into();
        self
    }
}

/// `SyncAssetFaceV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `bounding_box_x1` | `i64` | ✓ | Bounding box X1 |
/// | `bounding_box_x2` | `i64` | ✓ | Bounding box X2 |
/// | `bounding_box_y1` | `i64` | ✓ | Bounding box Y1 |
/// | `bounding_box_y2` | `i64` | ✓ | Bounding box Y2 |
/// | `id` | [`FaceId`](crate::id::FaceId) | ✓ | Asset face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `image_height` | `i64` | ✓ | Image height |
/// | `image_width` | `i64` | ✓ | Image width |
/// | `person_id` | [`PersonId`](crate::id::PersonId) | ✓ | Person ID |
/// | `source_type` | [`String`] | ✓ | Source type |
///
/// See: <https://api.immich.app/models/SyncAssetFaceV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAssetFaceV1 {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Bounding box X1 (required)
    #[serde(rename = "boundingBoxX1")]
    pub bounding_box_x1: i64,
    /// Bounding box X2 (required)
    #[serde(rename = "boundingBoxX2")]
    pub bounding_box_x2: i64,
    /// Bounding box Y1 (required)
    #[serde(rename = "boundingBoxY1")]
    pub bounding_box_y1: i64,
    /// Bounding box Y2 (required)
    #[serde(rename = "boundingBoxY2")]
    pub bounding_box_y2: i64,
    /// Asset face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: FaceId,
    /// Image height (required)
    #[serde(rename = "imageHeight")]
    pub image_height: i64,
    /// Image width (required)
    #[serde(rename = "imageWidth")]
    pub image_width: i64,
    /// Person ID (required)
    #[serde(rename = "personId")]
    pub person_id: Option<String>,
    /// Source type (required)
    #[serde(rename = "sourceType")]
    pub source_type: String,
}

impl SyncAssetFaceV1 {
    /// Creates a new [`SyncAssetFaceV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: impl Into<AssetId>,
        bounding_box_x1: i64,
        bounding_box_x2: i64,
        bounding_box_y1: i64,
        bounding_box_y2: i64,
        id: impl Into<FaceId>,
        image_height: i64,
        image_width: i64,
        source_type: impl Into<String>,
    ) -> Self {
        Self {
            asset_id: asset_id.into(),
            bounding_box_x1,
            bounding_box_x2,
            bounding_box_y1,
            bounding_box_y2,
            id: id.into(),
            image_height,
            image_width,
            person_id: None,
            source_type: source_type.into(),
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `bounding_box_x1`.
    #[must_use]
    pub fn bounding_box_x1(mut self, value: i64) -> Self {
        self.bounding_box_x1 = value;
        self
    }
    /// Sets `bounding_box_x2`.
    #[must_use]
    pub fn bounding_box_x2(mut self, value: i64) -> Self {
        self.bounding_box_x2 = value;
        self
    }
    /// Sets `bounding_box_y1`.
    #[must_use]
    pub fn bounding_box_y1(mut self, value: i64) -> Self {
        self.bounding_box_y1 = value;
        self
    }
    /// Sets `bounding_box_y2`.
    #[must_use]
    pub fn bounding_box_y2(mut self, value: i64) -> Self {
        self.bounding_box_y2 = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<FaceId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `image_height`.
    #[must_use]
    pub fn image_height(mut self, value: i64) -> Self {
        self.image_height = value;
        self
    }
    /// Sets `image_width`.
    #[must_use]
    pub fn image_width(mut self, value: i64) -> Self {
        self.image_width = value;
        self
    }
    /// Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: impl Into<String>) -> Self {
        self.person_id = Some(value.into());
        self
    }
    /// Sets or clears `person_id`.
    #[must_use]
    pub fn set_person_id(mut self, value: Option<String>) -> Self {
        self.person_id = value;
        self
    }
    /// Sets `source_type`.
    #[must_use]
    pub fn source_type(mut self, value: impl Into<String>) -> Self {
        self.source_type = value.into();
        self
    }
}

/// `SyncAssetFaceV3`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `bounding_box_x1` | `i64` | ✓ | Bounding box X1 |
/// | `bounding_box_x2` | `i64` | ✓ | Bounding box X2 |
/// | `bounding_box_y1` | `i64` | ✓ | Bounding box Y1 |
/// | `bounding_box_y2` | `i64` | ✓ | Bounding box Y2 |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Face deleted at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`FaceId`](crate::id::FaceId) | ✓ | Asset face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `image_height` | `i64` | ✓ | Image height |
/// | `image_width` | `i64` | ✓ | Image width |
/// | `is_visible` | `bool` | ✓ | Is the face visible in the asset |
/// | `person_id` | [`PersonId`](crate::id::PersonId) | ✓ | Person ID |
/// | `source_type` | [`String`] | ✓ | Source type |
///
/// See: <https://api.immich.app/models/SyncAssetFaceV3>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAssetFaceV3 {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Bounding box X1 (required)
    #[serde(rename = "boundingBoxX1")]
    pub bounding_box_x1: i64,
    /// Bounding box X2 (required)
    #[serde(rename = "boundingBoxX2")]
    pub bounding_box_x2: i64,
    /// Bounding box Y1 (required)
    #[serde(rename = "boundingBoxY1")]
    pub bounding_box_y1: i64,
    /// Bounding box Y2 (required)
    #[serde(rename = "boundingBoxY2")]
    pub bounding_box_y2: i64,
    /// Face deleted at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Asset face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: FaceId,
    /// Image height (required)
    #[serde(rename = "imageHeight")]
    pub image_height: i64,
    /// Image width (required)
    #[serde(rename = "imageWidth")]
    pub image_width: i64,
    /// Is the face visible in the asset (required)
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    /// Person ID (required)
    #[serde(rename = "personId")]
    pub person_id: Option<String>,
    /// Source type (required)
    #[serde(rename = "sourceType")]
    pub source_type: String,
}

impl SyncAssetFaceV3 {
    /// Creates a new [`SyncAssetFaceV3`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: impl Into<AssetId>,
        bounding_box_x1: i64,
        bounding_box_x2: i64,
        bounding_box_y1: i64,
        bounding_box_y2: i64,
        id: impl Into<FaceId>,
        image_height: i64,
        image_width: i64,
        is_visible: bool,
        source_type: impl Into<String>,
    ) -> Self {
        Self {
            asset_id: asset_id.into(),
            bounding_box_x1,
            bounding_box_x2,
            bounding_box_y1,
            bounding_box_y2,
            deleted_at: None,
            id: id.into(),
            image_height,
            image_width,
            is_visible,
            person_id: None,
            source_type: source_type.into(),
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `bounding_box_x1`.
    #[must_use]
    pub fn bounding_box_x1(mut self, value: i64) -> Self {
        self.bounding_box_x1 = value;
        self
    }
    /// Sets `bounding_box_x2`.
    #[must_use]
    pub fn bounding_box_x2(mut self, value: i64) -> Self {
        self.bounding_box_x2 = value;
        self
    }
    /// Sets `bounding_box_y1`.
    #[must_use]
    pub fn bounding_box_y1(mut self, value: i64) -> Self {
        self.bounding_box_y1 = value;
        self
    }
    /// Sets `bounding_box_y2`.
    #[must_use]
    pub fn bounding_box_y2(mut self, value: i64) -> Self {
        self.bounding_box_y2 = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<FaceId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `image_height`.
    #[must_use]
    pub fn image_height(mut self, value: i64) -> Self {
        self.image_height = value;
        self
    }
    /// Sets `image_width`.
    #[must_use]
    pub fn image_width(mut self, value: i64) -> Self {
        self.image_width = value;
        self
    }
    /// Sets `is_visible`.
    #[must_use]
    pub fn is_visible(mut self, value: bool) -> Self {
        self.is_visible = value;
        self
    }
    /// Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: impl Into<String>) -> Self {
        self.person_id = Some(value.into());
        self
    }
    /// Sets or clears `person_id`.
    #[must_use]
    pub fn set_person_id(mut self, value: Option<String>) -> Self {
        self.person_id = value;
        self
    }
    /// Sets `source_type`.
    #[must_use]
    pub fn source_type(mut self, value: impl Into<String>) -> Self {
        self.source_type = value.into();
        self
    }
}

/// `SyncAssetMetadataDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `key` | [`String`] | ✓ | Key |
///
/// See: <https://api.immich.app/models/SyncAssetMetadataDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAssetMetadataDeleteV1 {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Key (required)
    pub key: String,
}

impl SyncAssetMetadataDeleteV1 {
    /// Creates a new [`SyncAssetMetadataDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: impl Into<AssetId>, key: impl Into<String>) -> Self {
        Self {
            asset_id: asset_id.into(),
            key: key.into(),
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }
}

/// `SyncAssetMetadataV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `key` | [`String`] | ✓ | Key |
/// | `value` | `object` | ✓ | Value |
///
/// See: <https://api.immich.app/models/SyncAssetMetadataV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SyncAssetMetadataV1 {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Key (required)
    pub key: String,
    /// Value (required)
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl SyncAssetMetadataV1 {
    /// Creates a new [`SyncAssetMetadataV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: impl Into<AssetId>,
        key: impl Into<String>,
        value: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self {
            asset_id: asset_id.into(),
            key: key.into(),
            value,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }
    /// Sets `value`.
    #[must_use]
    pub fn value(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.value = value;
        self
    }
}

/// `SyncAssetOcrDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Original asset ID of the deleted OCR entry |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Timestamp when the OCR entry was deleted (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`String`] | ✓ | Audit row ID of the deleted OCR entry |
///
/// See: <https://api.immich.app/models/SyncAssetOcrDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAssetOcrDeleteV1 {
    /// Original asset ID of the deleted OCR entry (required)
    #[serde(rename = "assetId")]
    pub asset_id: String,
    /// Timestamp when the OCR entry was deleted (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Audit row ID of the deleted OCR entry (required)
    pub id: String,
}

impl SyncAssetOcrDeleteV1 {
    /// Creates a new [`SyncAssetOcrDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: impl Into<String>,
        deleted_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<String>,
    ) -> Self {
        Self {
            asset_id: asset_id.into(),
            deleted_at,
            id: id.into(),
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

/// `SyncAssetOcrV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `box_score` | `f64` | ✓ | Confidence score of the bounding box |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | OCR entry ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_visible` | `bool` | ✓ | Whether the OCR entry is visible |
/// | `text` | [`String`] | ✓ | Recognized text content |
/// | `text_score` | `f64` | ✓ | Confidence score of the recognized text |
/// | `x1` | `f64` | ✓ | Top-left X coordinate (normalized 0–1) |
/// | `x2` | `f64` | ✓ | Top-right X coordinate (normalized 0–1) |
/// | `x3` | `f64` | ✓ | Bottom-right X coordinate (normalized 0–1) |
/// | `x4` | `f64` | ✓ | Bottom-left X coordinate (normalized 0–1) |
/// | `y1` | `f64` | ✓ | Top-left Y coordinate (normalized 0–1) |
/// | `y2` | `f64` | ✓ | Top-right Y coordinate (normalized 0–1) |
/// | `y3` | `f64` | ✓ | Bottom-right Y coordinate (normalized 0–1) |
/// | `y4` | `f64` | ✓ | Bottom-left Y coordinate (normalized 0–1) |
///
/// See: <https://api.immich.app/models/SyncAssetOcrV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SyncAssetOcrV1 {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Confidence score of the bounding box (required)
    #[serde(rename = "boxScore")]
    pub box_score: f64,
    /// OCR entry ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: ::uuid::Uuid,
    /// Whether the OCR entry is visible (required)
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    /// Recognized text content (required)
    pub text: String,
    /// Confidence score of the recognized text (required)
    #[serde(rename = "textScore")]
    pub text_score: f64,
    /// Top-left X coordinate (normalized 0–1) (required)
    pub x1: f64,
    /// Top-right X coordinate (normalized 0–1) (required)
    pub x2: f64,
    /// Bottom-right X coordinate (normalized 0–1) (required)
    pub x3: f64,
    /// Bottom-left X coordinate (normalized 0–1) (required)
    pub x4: f64,
    /// Top-left Y coordinate (normalized 0–1) (required)
    pub y1: f64,
    /// Top-right Y coordinate (normalized 0–1) (required)
    pub y2: f64,
    /// Bottom-right Y coordinate (normalized 0–1) (required)
    pub y3: f64,
    /// Bottom-left Y coordinate (normalized 0–1) (required)
    pub y4: f64,
}

impl SyncAssetOcrV1 {
    /// Creates a new [`SyncAssetOcrV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: impl Into<AssetId>,
        box_score: f64,
        id: ::uuid::Uuid,
        is_visible: bool,
        text: impl Into<String>,
        text_score: f64,
        x1: f64,
        x2: f64,
        x3: f64,
        x4: f64,
        y1: f64,
        y2: f64,
        y3: f64,
        y4: f64,
    ) -> Self {
        Self {
            asset_id: asset_id.into(),
            box_score,
            id,
            is_visible,
            text: text.into(),
            text_score,
            x1,
            x2,
            x3,
            x4,
            y1,
            y2,
            y3,
            y4,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `box_score`.
    #[must_use]
    pub fn box_score(mut self, value: f64) -> Self {
        self.box_score = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }
    /// Sets `is_visible`.
    #[must_use]
    pub fn is_visible(mut self, value: bool) -> Self {
        self.is_visible = value;
        self
    }
    /// Sets `text`.
    #[must_use]
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = value.into();
        self
    }
    /// Sets `text_score`.
    #[must_use]
    pub fn text_score(mut self, value: f64) -> Self {
        self.text_score = value;
        self
    }
    /// Sets `x1`.
    #[must_use]
    pub fn x1(mut self, value: f64) -> Self {
        self.x1 = value;
        self
    }
    /// Sets `x2`.
    #[must_use]
    pub fn x2(mut self, value: f64) -> Self {
        self.x2 = value;
        self
    }
    /// Sets `x3`.
    #[must_use]
    pub fn x3(mut self, value: f64) -> Self {
        self.x3 = value;
        self
    }
    /// Sets `x4`.
    #[must_use]
    pub fn x4(mut self, value: f64) -> Self {
        self.x4 = value;
        self
    }
    /// Sets `y1`.
    #[must_use]
    pub fn y1(mut self, value: f64) -> Self {
        self.y1 = value;
        self
    }
    /// Sets `y2`.
    #[must_use]
    pub fn y2(mut self, value: f64) -> Self {
        self.y2 = value;
        self
    }
    /// Sets `y3`.
    #[must_use]
    pub fn y3(mut self, value: f64) -> Self {
        self.y3 = value;
        self
    }
    /// Sets `y4`.
    #[must_use]
    pub fn y4(mut self, value: f64) -> Self {
        self.y4 = value;
        self
    }
}

/// `SyncAssetV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `checksum` | [`String`] | ✓ | Checksum |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Uploaded to Immich at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deleted at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `duration` | [`String`] | ✓ | Duration |
/// | `file_created_at` | [`DateTime`](::chrono::DateTime) | ✓ | File created at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `file_modified_at` | [`DateTime`](::chrono::DateTime) | ✓ | File modified at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `height` | `i64` | ✓ | Asset height |
/// | `id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_edited` | `bool` | ✓ | Is edited |
/// | `is_favorite` | `bool` | ✓ | Is favorite |
/// | `library_id` | [`LibraryId`](crate::id::LibraryId) | ✓ | Library ID |
/// | `live_photo_video_id` | [`AssetId`](crate::id::AssetId) | ✓ | Live photo video ID |
/// | `local_date_time` | [`DateTime`](::chrono::DateTime) | ✓ | Local date time (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `original_file_name` | [`String`] | ✓ | Original file name |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `stack_id` | [`StackId`](crate::id::StackId) | ✓ | Stack ID |
/// | `thumbhash` | [`String`] | ✓ | Thumbhash |
/// | `type_` | [`AssetTypeEnum`] | ✓ |  |
/// | `visibility` | [`AssetVisibility`] | ✓ |  |
/// | `width` | `i64` | ✓ | Asset width |
///
/// See: <https://api.immich.app/models/SyncAssetV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAssetV1 {
    /// Checksum (required)
    pub checksum: String,
    /// Uploaded to Immich at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Deleted at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Duration (required)
    pub duration: Option<String>,
    /// File created at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "fileCreatedAt")]
    pub file_created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// File modified at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "fileModifiedAt")]
    pub file_modified_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Asset height (required)
    pub height: Option<i64>,
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: AssetId,
    /// Is edited (required)
    #[serde(rename = "isEdited")]
    pub is_edited: bool,
    /// Is favorite (required)
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    /// Library ID (required)
    #[serde(rename = "libraryId")]
    pub library_id: Option<String>,
    /// Live photo video ID (required)
    #[serde(rename = "livePhotoVideoId")]
    pub live_photo_video_id: Option<String>,
    /// Local date time (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "localDateTime")]
    pub local_date_time: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Original file name (required)
    #[serde(rename = "originalFileName")]
    pub original_file_name: String,
    /// Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    /// Stack ID (required)
    #[serde(rename = "stackId")]
    pub stack_id: Option<String>,
    /// Thumbhash (required)
    pub thumbhash: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: AssetTypeEnum,
    /// (required)
    pub visibility: AssetVisibility,
    /// Asset width (required)
    pub width: Option<i64>,
}

impl SyncAssetV1 {
    /// Creates a new [`SyncAssetV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        checksum: impl Into<String>,
        id: impl Into<AssetId>,
        is_edited: bool,
        is_favorite: bool,
        original_file_name: impl Into<String>,
        owner_id: impl Into<UserId>,
        type_: AssetTypeEnum,
        visibility: AssetVisibility,
    ) -> Self {
        Self {
            checksum: checksum.into(),
            created_at: None,
            deleted_at: None,
            duration: None,
            file_created_at: None,
            file_modified_at: None,
            height: None,
            id: id.into(),
            is_edited,
            is_favorite,
            library_id: None,
            live_photo_video_id: None,
            local_date_time: None,
            original_file_name: original_file_name.into(),
            owner_id: owner_id.into(),
            stack_id: None,
            thumbhash: None,
            type_,
            visibility,
            width: None,
        }
    }
    /// Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = value.into();
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }
    /// Sets or clears `created_at`.
    #[must_use]
    pub fn set_created_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: impl Into<String>) -> Self {
        self.duration = Some(value.into());
        self
    }
    /// Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<String>) -> Self {
        self.duration = value;
        self
    }
    /// Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_created_at = Some(value);
        self
    }
    /// Sets or clears `file_created_at`.
    #[must_use]
    pub fn set_file_created_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.file_created_at = value;
        self
    }
    /// Sets `file_modified_at`.
    #[must_use]
    pub fn file_modified_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_modified_at = Some(value);
        self
    }
    /// Sets or clears `file_modified_at`.
    #[must_use]
    pub fn set_file_modified_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.file_modified_at = value;
        self
    }
    /// Sets `height`.
    #[must_use]
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }
    /// Sets or clears `height`.
    #[must_use]
    pub fn set_height(mut self, value: Option<i64>) -> Self {
        self.height = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<AssetId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_edited`.
    #[must_use]
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = value;
        self
    }
    /// Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = value;
        self
    }
    /// Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: impl Into<String>) -> Self {
        self.library_id = Some(value.into());
        self
    }
    /// Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<String>) -> Self {
        self.library_id = value;
        self
    }
    /// Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: impl Into<String>) -> Self {
        self.live_photo_video_id = Some(value.into());
        self
    }
    /// Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<String>) -> Self {
        self.live_photo_video_id = value;
        self
    }
    /// Sets `local_date_time`.
    #[must_use]
    pub fn local_date_time(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.local_date_time = Some(value);
        self
    }
    /// Sets or clears `local_date_time`.
    #[must_use]
    pub fn set_local_date_time(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.local_date_time = value;
        self
    }
    /// Sets `original_file_name`.
    #[must_use]
    pub fn original_file_name(mut self, value: impl Into<String>) -> Self {
        self.original_file_name = value.into();
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `stack_id`.
    #[must_use]
    pub fn stack_id(mut self, value: impl Into<String>) -> Self {
        self.stack_id = Some(value.into());
        self
    }
    /// Sets or clears `stack_id`.
    #[must_use]
    pub fn set_stack_id(mut self, value: Option<String>) -> Self {
        self.stack_id = value;
        self
    }
    /// Sets `thumbhash`.
    #[must_use]
    pub fn thumbhash(mut self, value: impl Into<String>) -> Self {
        self.thumbhash = Some(value.into());
        self
    }
    /// Sets or clears `thumbhash`.
    #[must_use]
    pub fn set_thumbhash(mut self, value: Option<String>) -> Self {
        self.thumbhash = value;
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = value;
        self
    }
    /// Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = value;
        self
    }
    /// Sets `width`.
    #[must_use]
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }
    /// Sets or clears `width`.
    #[must_use]
    pub fn set_width(mut self, value: Option<i64>) -> Self {
        self.width = value;
        self
    }
}

/// `SyncAssetV2`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `checksum` | [`String`] | ✓ | Checksum |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Uploaded to Immich at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deleted at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `duration` | `i64` | ✓ | Duration |
/// | `file_created_at` | [`DateTime`](::chrono::DateTime) | ✓ | File created at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `file_modified_at` | [`DateTime`](::chrono::DateTime) | ✓ | File modified at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `height` | `i64` | ✓ | Asset height |
/// | `id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_edited` | `bool` | ✓ | Is edited |
/// | `is_favorite` | `bool` | ✓ | Is favorite |
/// | `library_id` | [`LibraryId`](crate::id::LibraryId) | ✓ | Library ID |
/// | `live_photo_video_id` | [`AssetId`](crate::id::AssetId) | ✓ | Live photo video ID |
/// | `local_date_time` | [`DateTime`](::chrono::DateTime) | ✓ | Local date time (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `original_file_name` | [`String`] | ✓ | Original file name |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `stack_id` | [`StackId`](crate::id::StackId) | ✓ | Stack ID |
/// | `thumbhash` | [`String`] | ✓ | Thumbhash |
/// | `type_` | [`AssetTypeEnum`] | ✓ |  |
/// | `visibility` | [`AssetVisibility`] | ✓ |  |
/// | `width` | `i64` | ✓ | Asset width |
///
/// See: <https://api.immich.app/models/SyncAssetV2>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAssetV2 {
    /// Checksum (required)
    pub checksum: String,
    /// Uploaded to Immich at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Deleted at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Duration (required)
    pub duration: Option<i64>,
    /// File created at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "fileCreatedAt")]
    pub file_created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// File modified at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "fileModifiedAt")]
    pub file_modified_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Asset height (required)
    pub height: Option<i64>,
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: AssetId,
    /// Is edited (required)
    #[serde(rename = "isEdited")]
    pub is_edited: bool,
    /// Is favorite (required)
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    /// Library ID (required)
    #[serde(rename = "libraryId")]
    pub library_id: Option<String>,
    /// Live photo video ID (required)
    #[serde(rename = "livePhotoVideoId")]
    pub live_photo_video_id: Option<String>,
    /// Local date time (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "localDateTime")]
    pub local_date_time: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Original file name (required)
    #[serde(rename = "originalFileName")]
    pub original_file_name: String,
    /// Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    /// Stack ID (required)
    #[serde(rename = "stackId")]
    pub stack_id: Option<String>,
    /// Thumbhash (required)
    pub thumbhash: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: AssetTypeEnum,
    /// (required)
    pub visibility: AssetVisibility,
    /// Asset width (required)
    pub width: Option<i64>,
}

impl SyncAssetV2 {
    /// Creates a new [`SyncAssetV2`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        checksum: impl Into<String>,
        id: impl Into<AssetId>,
        is_edited: bool,
        is_favorite: bool,
        original_file_name: impl Into<String>,
        owner_id: impl Into<UserId>,
        type_: AssetTypeEnum,
        visibility: AssetVisibility,
    ) -> Self {
        Self {
            checksum: checksum.into(),
            created_at: None,
            deleted_at: None,
            duration: None,
            file_created_at: None,
            file_modified_at: None,
            height: None,
            id: id.into(),
            is_edited,
            is_favorite,
            library_id: None,
            live_photo_video_id: None,
            local_date_time: None,
            original_file_name: original_file_name.into(),
            owner_id: owner_id.into(),
            stack_id: None,
            thumbhash: None,
            type_,
            visibility,
            width: None,
        }
    }
    /// Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = value.into();
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }
    /// Sets or clears `created_at`.
    #[must_use]
    pub fn set_created_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }
    /// Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<i64>) -> Self {
        self.duration = value;
        self
    }
    /// Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_created_at = Some(value);
        self
    }
    /// Sets or clears `file_created_at`.
    #[must_use]
    pub fn set_file_created_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.file_created_at = value;
        self
    }
    /// Sets `file_modified_at`.
    #[must_use]
    pub fn file_modified_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_modified_at = Some(value);
        self
    }
    /// Sets or clears `file_modified_at`.
    #[must_use]
    pub fn set_file_modified_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.file_modified_at = value;
        self
    }
    /// Sets `height`.
    #[must_use]
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }
    /// Sets or clears `height`.
    #[must_use]
    pub fn set_height(mut self, value: Option<i64>) -> Self {
        self.height = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<AssetId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_edited`.
    #[must_use]
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = value;
        self
    }
    /// Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = value;
        self
    }
    /// Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: impl Into<String>) -> Self {
        self.library_id = Some(value.into());
        self
    }
    /// Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<String>) -> Self {
        self.library_id = value;
        self
    }
    /// Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: impl Into<String>) -> Self {
        self.live_photo_video_id = Some(value.into());
        self
    }
    /// Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<String>) -> Self {
        self.live_photo_video_id = value;
        self
    }
    /// Sets `local_date_time`.
    #[must_use]
    pub fn local_date_time(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.local_date_time = Some(value);
        self
    }
    /// Sets or clears `local_date_time`.
    #[must_use]
    pub fn set_local_date_time(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.local_date_time = value;
        self
    }
    /// Sets `original_file_name`.
    #[must_use]
    pub fn original_file_name(mut self, value: impl Into<String>) -> Self {
        self.original_file_name = value.into();
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `stack_id`.
    #[must_use]
    pub fn stack_id(mut self, value: impl Into<String>) -> Self {
        self.stack_id = Some(value.into());
        self
    }
    /// Sets or clears `stack_id`.
    #[must_use]
    pub fn set_stack_id(mut self, value: Option<String>) -> Self {
        self.stack_id = value;
        self
    }
    /// Sets `thumbhash`.
    #[must_use]
    pub fn thumbhash(mut self, value: impl Into<String>) -> Self {
        self.thumbhash = Some(value.into());
        self
    }
    /// Sets or clears `thumbhash`.
    #[must_use]
    pub fn set_thumbhash(mut self, value: Option<String>) -> Self {
        self.thumbhash = value;
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = value;
        self
    }
    /// Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = value;
        self
    }
    /// Sets `width`.
    #[must_use]
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }
    /// Sets or clears `width`.
    #[must_use]
    pub fn set_width(mut self, value: Option<i64>) -> Self {
        self.width = value;
        self
    }
}

/// `SyncAuthUserV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] |  |  |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | User deleted at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `email` | [`String`] | ✓ | User email |
/// | `has_profile_image` | `bool` | ✓ | User has profile image |
/// | `id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_admin` | `bool` | ✓ | User is admin |
/// | `name` | [`String`] | ✓ | User name |
/// | `oauth_id` | [`String`] | ✓ | User `OAuth` ID |
/// | `pin_code` | [`String`] | ✓ | User pin code |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | User profile changed at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `quota_size_in_bytes` | `i64` | ✓ | Quota size in bytes |
/// | `quota_usage_in_bytes` | `i64` | ✓ | Quota usage in bytes |
/// | `storage_label` | [`String`] | ✓ | User storage label |
///
/// See: <https://api.immich.app/models/SyncAuthUserV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAuthUserV1 {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    /// User deleted at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// User email (required)
    pub email: String,
    /// User has profile image (required)
    #[serde(rename = "hasProfileImage")]
    pub has_profile_image: bool,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: UserId,
    /// User is admin (required)
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    /// User name (required)
    pub name: String,
    /// User `OAuth` ID (required)
    #[serde(rename = "oauthId")]
    pub oauth_id: String,
    /// User pin code (required)
    #[serde(rename = "pinCode")]
    pub pin_code: Option<String>,
    /// User profile changed at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Quota size in bytes (required)
    #[serde(rename = "quotaSizeInBytes")]
    pub quota_size_in_bytes: Option<i64>,
    /// Quota usage in bytes (required)
    #[serde(rename = "quotaUsageInBytes")]
    pub quota_usage_in_bytes: i64,
    /// User storage label (required)
    #[serde(rename = "storageLabel")]
    pub storage_label: Option<String>,
}

impl SyncAuthUserV1 {
    /// Creates a new [`SyncAuthUserV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        email: impl Into<String>,
        has_profile_image: bool,
        id: impl Into<UserId>,
        is_admin: bool,
        name: impl Into<String>,
        oauth_id: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        quota_usage_in_bytes: i64,
    ) -> Self {
        Self {
            avatar_color: None,
            deleted_at: None,
            email: email.into(),
            has_profile_image,
            id: id.into(),
            is_admin,
            name: name.into(),
            oauth_id: oauth_id.into(),
            pin_code: None,
            profile_changed_at,
            quota_size_in_bytes: None,
            quota_usage_in_bytes,
            storage_label: None,
        }
    }
    /// Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }
    /// Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }
    /// Sets `has_profile_image`.
    #[must_use]
    pub fn has_profile_image(mut self, value: bool) -> Self {
        self.has_profile_image = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<UserId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `oauth_id`.
    #[must_use]
    pub fn oauth_id(mut self, value: impl Into<String>) -> Self {
        self.oauth_id = value.into();
        self
    }
    /// Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = Some(value.into());
        self
    }
    /// Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<String>) -> Self {
        self.pin_code = value;
        self
    }
    /// Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }
    /// Sets `quota_size_in_bytes`.
    #[must_use]
    pub fn quota_size_in_bytes(mut self, value: i64) -> Self {
        self.quota_size_in_bytes = Some(value);
        self
    }
    /// Sets or clears `quota_size_in_bytes`.
    #[must_use]
    pub fn set_quota_size_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_size_in_bytes = value;
        self
    }
    /// Sets `quota_usage_in_bytes`.
    #[must_use]
    pub fn quota_usage_in_bytes(mut self, value: i64) -> Self {
        self.quota_usage_in_bytes = value;
        self
    }
    /// Sets `storage_label`.
    #[must_use]
    pub fn storage_label(mut self, value: impl Into<String>) -> Self {
        self.storage_label = Some(value.into());
        self
    }
    /// Sets or clears `storage_label`.
    #[must_use]
    pub fn set_storage_label(mut self, value: Option<String>) -> Self {
        self.storage_label = value;
        self
    }
}

/// `SyncAuthUserV2`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] |  |  |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | User deleted at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `email` | [`String`] | ✓ | User email |
/// | `has_profile_image` | `bool` | ✓ | User has profile image |
/// | `id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_admin` | `bool` | ✓ | User is admin |
/// | `name` | [`String`] | ✓ | User name |
/// | `oauth_id` | [`String`] | ✓ | User `OAuth` ID |
/// | `pin_code` | [`String`] | ✓ | User pin code |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | User profile changed at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `quota_size_in_bytes` | `i64` | ✓ | Quota size in bytes |
/// | `quota_usage_in_bytes` | `i64` | ✓ | Quota usage in bytes |
/// | `storage_label` | [`String`] | ✓ | User storage label |
///
/// See: <https://api.immich.app/models/SyncAuthUserV2>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncAuthUserV2 {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    /// User deleted at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// User email (required)
    pub email: String,
    /// User has profile image (required)
    #[serde(rename = "hasProfileImage")]
    pub has_profile_image: bool,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: UserId,
    /// User is admin (required)
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    /// User name (required)
    pub name: String,
    /// User `OAuth` ID (required)
    #[serde(rename = "oauthId")]
    pub oauth_id: Option<String>,
    /// User pin code (required)
    #[serde(rename = "pinCode")]
    pub pin_code: Option<String>,
    /// User profile changed at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Quota size in bytes (required)
    #[serde(rename = "quotaSizeInBytes")]
    pub quota_size_in_bytes: Option<i64>,
    /// Quota usage in bytes (required)
    #[serde(rename = "quotaUsageInBytes")]
    pub quota_usage_in_bytes: i64,
    /// User storage label (required)
    #[serde(rename = "storageLabel")]
    pub storage_label: Option<String>,
}

impl SyncAuthUserV2 {
    /// Creates a new [`SyncAuthUserV2`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        email: impl Into<String>,
        has_profile_image: bool,
        id: impl Into<UserId>,
        is_admin: bool,
        name: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        quota_usage_in_bytes: i64,
    ) -> Self {
        Self {
            avatar_color: None,
            deleted_at: None,
            email: email.into(),
            has_profile_image,
            id: id.into(),
            is_admin,
            name: name.into(),
            oauth_id: None,
            pin_code: None,
            profile_changed_at,
            quota_size_in_bytes: None,
            quota_usage_in_bytes,
            storage_label: None,
        }
    }
    /// Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }
    /// Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }
    /// Sets `has_profile_image`.
    #[must_use]
    pub fn has_profile_image(mut self, value: bool) -> Self {
        self.has_profile_image = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<UserId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `oauth_id`.
    #[must_use]
    pub fn oauth_id(mut self, value: impl Into<String>) -> Self {
        self.oauth_id = Some(value.into());
        self
    }
    /// Sets or clears `oauth_id`.
    #[must_use]
    pub fn set_oauth_id(mut self, value: Option<String>) -> Self {
        self.oauth_id = value;
        self
    }
    /// Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = Some(value.into());
        self
    }
    /// Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<String>) -> Self {
        self.pin_code = value;
        self
    }
    /// Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }
    /// Sets `quota_size_in_bytes`.
    #[must_use]
    pub fn quota_size_in_bytes(mut self, value: i64) -> Self {
        self.quota_size_in_bytes = Some(value);
        self
    }
    /// Sets or clears `quota_size_in_bytes`.
    #[must_use]
    pub fn set_quota_size_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_size_in_bytes = value;
        self
    }
    /// Sets `quota_usage_in_bytes`.
    #[must_use]
    pub fn quota_usage_in_bytes(mut self, value: i64) -> Self {
        self.quota_usage_in_bytes = value;
        self
    }
    /// Sets `storage_label`.
    #[must_use]
    pub fn storage_label(mut self, value: impl Into<String>) -> Self {
        self.storage_label = Some(value.into());
        self
    }
    /// Sets or clears `storage_label`.
    #[must_use]
    pub fn set_storage_label(mut self, value: Option<String>) -> Self {
        self.storage_label = value;
        self
    }
}

/// `SyncCompleteV1`
///
/// See: <https://api.immich.app/models/SyncCompleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct SyncCompleteV1(pub ::serde_json::Map<String, ::serde_json::Value>);

impl ::std::ops::Deref for SyncCompleteV1 {
    type Target = ::serde_json::Map<String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<String, ::serde_json::Value> {
        &self.0
    }
}

impl From<::serde_json::Map<String, ::serde_json::Value>> for SyncCompleteV1 {
    fn from(value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}

/// Sync entity type
///
/// See: <https://api.immich.app/models/SyncEntityType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SyncEntityType {
    AuthUserV1,
    AuthUserV2,
    UserV1,
    UserDeleteV1,
    AssetV1,
    AssetV2,
    AssetDeleteV1,
    AssetExifV1,
    AssetEditV1,
    AssetEditDeleteV1,
    AssetMetadataV1,
    AssetMetadataDeleteV1,
    AssetOcrV1,
    AssetOcrDeleteV1,
    PartnerV1,
    PartnerDeleteV1,
    PartnerAssetV1,
    PartnerAssetV2,
    PartnerAssetBackfillV1,
    PartnerAssetBackfillV2,
    PartnerAssetDeleteV1,
    PartnerAssetExifV1,
    PartnerAssetExifBackfillV1,
    PartnerStackBackfillV1,
    PartnerStackDeleteV1,
    PartnerStackV1,
    AlbumV1,
    AlbumV2,
    AlbumDeleteV1,
    AlbumUserV1,
    AlbumUserBackfillV1,
    AlbumUserDeleteV1,
    AlbumAssetCreateV1,
    AlbumAssetCreateV2,
    AlbumAssetUpdateV1,
    AlbumAssetUpdateV2,
    AlbumAssetBackfillV1,
    AlbumAssetBackfillV2,
    AlbumAssetExifCreateV1,
    AlbumAssetExifUpdateV1,
    AlbumAssetExifBackfillV1,
    AlbumToAssetV1,
    AlbumToAssetDeleteV1,
    AlbumToAssetBackfillV1,
    MemoryV1,
    MemoryV2,
    MemoryDeleteV1,
    MemoryToAssetV1,
    MemoryToAssetV2,
    MemoryToAssetDeleteV1,
    StackV1,
    StackDeleteV1,
    PersonV1,
    PersonDeleteV1,
    AssetFaceV1,
    AssetFaceV2,
    AssetFaceV3,
    AssetFaceDeleteV1,
    UserMetadataV1,
    UserMetadataDeleteV1,
    SyncAckV1,
    SyncResetV1,
    SyncCompleteV1,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl SyncEntityType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AuthUserV1 => "AuthUserV1",
            Self::AuthUserV2 => "AuthUserV2",
            Self::UserV1 => "UserV1",
            Self::UserDeleteV1 => "UserDeleteV1",
            Self::AssetV1 => "AssetV1",
            Self::AssetV2 => "AssetV2",
            Self::AssetDeleteV1 => "AssetDeleteV1",
            Self::AssetExifV1 => "AssetExifV1",
            Self::AssetEditV1 => "AssetEditV1",
            Self::AssetEditDeleteV1 => "AssetEditDeleteV1",
            Self::AssetMetadataV1 => "AssetMetadataV1",
            Self::AssetMetadataDeleteV1 => "AssetMetadataDeleteV1",
            Self::AssetOcrV1 => "AssetOcrV1",
            Self::AssetOcrDeleteV1 => "AssetOcrDeleteV1",
            Self::PartnerV1 => "PartnerV1",
            Self::PartnerDeleteV1 => "PartnerDeleteV1",
            Self::PartnerAssetV1 => "PartnerAssetV1",
            Self::PartnerAssetV2 => "PartnerAssetV2",
            Self::PartnerAssetBackfillV1 => "PartnerAssetBackfillV1",
            Self::PartnerAssetBackfillV2 => "PartnerAssetBackfillV2",
            Self::PartnerAssetDeleteV1 => "PartnerAssetDeleteV1",
            Self::PartnerAssetExifV1 => "PartnerAssetExifV1",
            Self::PartnerAssetExifBackfillV1 => "PartnerAssetExifBackfillV1",
            Self::PartnerStackBackfillV1 => "PartnerStackBackfillV1",
            Self::PartnerStackDeleteV1 => "PartnerStackDeleteV1",
            Self::PartnerStackV1 => "PartnerStackV1",
            Self::AlbumV1 => "AlbumV1",
            Self::AlbumV2 => "AlbumV2",
            Self::AlbumDeleteV1 => "AlbumDeleteV1",
            Self::AlbumUserV1 => "AlbumUserV1",
            Self::AlbumUserBackfillV1 => "AlbumUserBackfillV1",
            Self::AlbumUserDeleteV1 => "AlbumUserDeleteV1",
            Self::AlbumAssetCreateV1 => "AlbumAssetCreateV1",
            Self::AlbumAssetCreateV2 => "AlbumAssetCreateV2",
            Self::AlbumAssetUpdateV1 => "AlbumAssetUpdateV1",
            Self::AlbumAssetUpdateV2 => "AlbumAssetUpdateV2",
            Self::AlbumAssetBackfillV1 => "AlbumAssetBackfillV1",
            Self::AlbumAssetBackfillV2 => "AlbumAssetBackfillV2",
            Self::AlbumAssetExifCreateV1 => "AlbumAssetExifCreateV1",
            Self::AlbumAssetExifUpdateV1 => "AlbumAssetExifUpdateV1",
            Self::AlbumAssetExifBackfillV1 => "AlbumAssetExifBackfillV1",
            Self::AlbumToAssetV1 => "AlbumToAssetV1",
            Self::AlbumToAssetDeleteV1 => "AlbumToAssetDeleteV1",
            Self::AlbumToAssetBackfillV1 => "AlbumToAssetBackfillV1",
            Self::MemoryV1 => "MemoryV1",
            Self::MemoryV2 => "MemoryV2",
            Self::MemoryDeleteV1 => "MemoryDeleteV1",
            Self::MemoryToAssetV1 => "MemoryToAssetV1",
            Self::MemoryToAssetV2 => "MemoryToAssetV2",
            Self::MemoryToAssetDeleteV1 => "MemoryToAssetDeleteV1",
            Self::StackV1 => "StackV1",
            Self::StackDeleteV1 => "StackDeleteV1",
            Self::PersonV1 => "PersonV1",
            Self::PersonDeleteV1 => "PersonDeleteV1",
            Self::AssetFaceV1 => "AssetFaceV1",
            Self::AssetFaceV2 => "AssetFaceV2",
            Self::AssetFaceV3 => "AssetFaceV3",
            Self::AssetFaceDeleteV1 => "AssetFaceDeleteV1",
            Self::UserMetadataV1 => "UserMetadataV1",
            Self::UserMetadataDeleteV1 => "UserMetadataDeleteV1",
            Self::SyncAckV1 => "SyncAckV1",
            Self::SyncResetV1 => "SyncResetV1",
            Self::SyncCompleteV1 => "SyncCompleteV1",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for SyncEntityType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for SyncEntityType {
    fn from(value: &str) -> Self {
        match value {
            "AuthUserV1" => Self::AuthUserV1,
            "AuthUserV2" => Self::AuthUserV2,
            "UserV1" => Self::UserV1,
            "UserDeleteV1" => Self::UserDeleteV1,
            "AssetV1" => Self::AssetV1,
            "AssetV2" => Self::AssetV2,
            "AssetDeleteV1" => Self::AssetDeleteV1,
            "AssetExifV1" => Self::AssetExifV1,
            "AssetEditV1" => Self::AssetEditV1,
            "AssetEditDeleteV1" => Self::AssetEditDeleteV1,
            "AssetMetadataV1" => Self::AssetMetadataV1,
            "AssetMetadataDeleteV1" => Self::AssetMetadataDeleteV1,
            "AssetOcrV1" => Self::AssetOcrV1,
            "AssetOcrDeleteV1" => Self::AssetOcrDeleteV1,
            "PartnerV1" => Self::PartnerV1,
            "PartnerDeleteV1" => Self::PartnerDeleteV1,
            "PartnerAssetV1" => Self::PartnerAssetV1,
            "PartnerAssetV2" => Self::PartnerAssetV2,
            "PartnerAssetBackfillV1" => Self::PartnerAssetBackfillV1,
            "PartnerAssetBackfillV2" => Self::PartnerAssetBackfillV2,
            "PartnerAssetDeleteV1" => Self::PartnerAssetDeleteV1,
            "PartnerAssetExifV1" => Self::PartnerAssetExifV1,
            "PartnerAssetExifBackfillV1" => Self::PartnerAssetExifBackfillV1,
            "PartnerStackBackfillV1" => Self::PartnerStackBackfillV1,
            "PartnerStackDeleteV1" => Self::PartnerStackDeleteV1,
            "PartnerStackV1" => Self::PartnerStackV1,
            "AlbumV1" => Self::AlbumV1,
            "AlbumV2" => Self::AlbumV2,
            "AlbumDeleteV1" => Self::AlbumDeleteV1,
            "AlbumUserV1" => Self::AlbumUserV1,
            "AlbumUserBackfillV1" => Self::AlbumUserBackfillV1,
            "AlbumUserDeleteV1" => Self::AlbumUserDeleteV1,
            "AlbumAssetCreateV1" => Self::AlbumAssetCreateV1,
            "AlbumAssetCreateV2" => Self::AlbumAssetCreateV2,
            "AlbumAssetUpdateV1" => Self::AlbumAssetUpdateV1,
            "AlbumAssetUpdateV2" => Self::AlbumAssetUpdateV2,
            "AlbumAssetBackfillV1" => Self::AlbumAssetBackfillV1,
            "AlbumAssetBackfillV2" => Self::AlbumAssetBackfillV2,
            "AlbumAssetExifCreateV1" => Self::AlbumAssetExifCreateV1,
            "AlbumAssetExifUpdateV1" => Self::AlbumAssetExifUpdateV1,
            "AlbumAssetExifBackfillV1" => Self::AlbumAssetExifBackfillV1,
            "AlbumToAssetV1" => Self::AlbumToAssetV1,
            "AlbumToAssetDeleteV1" => Self::AlbumToAssetDeleteV1,
            "AlbumToAssetBackfillV1" => Self::AlbumToAssetBackfillV1,
            "MemoryV1" => Self::MemoryV1,
            "MemoryV2" => Self::MemoryV2,
            "MemoryDeleteV1" => Self::MemoryDeleteV1,
            "MemoryToAssetV1" => Self::MemoryToAssetV1,
            "MemoryToAssetV2" => Self::MemoryToAssetV2,
            "MemoryToAssetDeleteV1" => Self::MemoryToAssetDeleteV1,
            "StackV1" => Self::StackV1,
            "StackDeleteV1" => Self::StackDeleteV1,
            "PersonV1" => Self::PersonV1,
            "PersonDeleteV1" => Self::PersonDeleteV1,
            "AssetFaceV1" => Self::AssetFaceV1,
            "AssetFaceV2" => Self::AssetFaceV2,
            "AssetFaceV3" => Self::AssetFaceV3,
            "AssetFaceDeleteV1" => Self::AssetFaceDeleteV1,
            "UserMetadataV1" => Self::UserMetadataV1,
            "UserMetadataDeleteV1" => Self::UserMetadataDeleteV1,
            "SyncAckV1" => Self::SyncAckV1,
            "SyncResetV1" => Self::SyncResetV1,
            "SyncCompleteV1" => Self::SyncCompleteV1,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for SyncEntityType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AuthUserV1" => Self::AuthUserV1,
            "AuthUserV2" => Self::AuthUserV2,
            "UserV1" => Self::UserV1,
            "UserDeleteV1" => Self::UserDeleteV1,
            "AssetV1" => Self::AssetV1,
            "AssetV2" => Self::AssetV2,
            "AssetDeleteV1" => Self::AssetDeleteV1,
            "AssetExifV1" => Self::AssetExifV1,
            "AssetEditV1" => Self::AssetEditV1,
            "AssetEditDeleteV1" => Self::AssetEditDeleteV1,
            "AssetMetadataV1" => Self::AssetMetadataV1,
            "AssetMetadataDeleteV1" => Self::AssetMetadataDeleteV1,
            "AssetOcrV1" => Self::AssetOcrV1,
            "AssetOcrDeleteV1" => Self::AssetOcrDeleteV1,
            "PartnerV1" => Self::PartnerV1,
            "PartnerDeleteV1" => Self::PartnerDeleteV1,
            "PartnerAssetV1" => Self::PartnerAssetV1,
            "PartnerAssetV2" => Self::PartnerAssetV2,
            "PartnerAssetBackfillV1" => Self::PartnerAssetBackfillV1,
            "PartnerAssetBackfillV2" => Self::PartnerAssetBackfillV2,
            "PartnerAssetDeleteV1" => Self::PartnerAssetDeleteV1,
            "PartnerAssetExifV1" => Self::PartnerAssetExifV1,
            "PartnerAssetExifBackfillV1" => Self::PartnerAssetExifBackfillV1,
            "PartnerStackBackfillV1" => Self::PartnerStackBackfillV1,
            "PartnerStackDeleteV1" => Self::PartnerStackDeleteV1,
            "PartnerStackV1" => Self::PartnerStackV1,
            "AlbumV1" => Self::AlbumV1,
            "AlbumV2" => Self::AlbumV2,
            "AlbumDeleteV1" => Self::AlbumDeleteV1,
            "AlbumUserV1" => Self::AlbumUserV1,
            "AlbumUserBackfillV1" => Self::AlbumUserBackfillV1,
            "AlbumUserDeleteV1" => Self::AlbumUserDeleteV1,
            "AlbumAssetCreateV1" => Self::AlbumAssetCreateV1,
            "AlbumAssetCreateV2" => Self::AlbumAssetCreateV2,
            "AlbumAssetUpdateV1" => Self::AlbumAssetUpdateV1,
            "AlbumAssetUpdateV2" => Self::AlbumAssetUpdateV2,
            "AlbumAssetBackfillV1" => Self::AlbumAssetBackfillV1,
            "AlbumAssetBackfillV2" => Self::AlbumAssetBackfillV2,
            "AlbumAssetExifCreateV1" => Self::AlbumAssetExifCreateV1,
            "AlbumAssetExifUpdateV1" => Self::AlbumAssetExifUpdateV1,
            "AlbumAssetExifBackfillV1" => Self::AlbumAssetExifBackfillV1,
            "AlbumToAssetV1" => Self::AlbumToAssetV1,
            "AlbumToAssetDeleteV1" => Self::AlbumToAssetDeleteV1,
            "AlbumToAssetBackfillV1" => Self::AlbumToAssetBackfillV1,
            "MemoryV1" => Self::MemoryV1,
            "MemoryV2" => Self::MemoryV2,
            "MemoryDeleteV1" => Self::MemoryDeleteV1,
            "MemoryToAssetV1" => Self::MemoryToAssetV1,
            "MemoryToAssetV2" => Self::MemoryToAssetV2,
            "MemoryToAssetDeleteV1" => Self::MemoryToAssetDeleteV1,
            "StackV1" => Self::StackV1,
            "StackDeleteV1" => Self::StackDeleteV1,
            "PersonV1" => Self::PersonV1,
            "PersonDeleteV1" => Self::PersonDeleteV1,
            "AssetFaceV1" => Self::AssetFaceV1,
            "AssetFaceV2" => Self::AssetFaceV2,
            "AssetFaceV3" => Self::AssetFaceV3,
            "AssetFaceDeleteV1" => Self::AssetFaceDeleteV1,
            "UserMetadataV1" => Self::UserMetadataV1,
            "UserMetadataDeleteV1" => Self::UserMetadataDeleteV1,
            "SyncAckV1" => Self::SyncAckV1,
            "SyncResetV1" => Self::SyncResetV1,
            "SyncCompleteV1" => Self::SyncCompleteV1,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for SyncEntityType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for SyncEntityType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for SyncEntityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for SyncEntityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = SyncEntityType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SyncEntityType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SyncEntityType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `SyncMemoryAssetDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `memory_id` | [`MemoryId`](crate::id::MemoryId) | ✓ | Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncMemoryAssetDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncMemoryAssetDeleteV1 {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "memoryId")]
    pub memory_id: MemoryId,
}

impl SyncMemoryAssetDeleteV1 {
    /// Creates a new [`SyncMemoryAssetDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: impl Into<AssetId>, memory_id: impl Into<MemoryId>) -> Self {
        Self {
            asset_id: asset_id.into(),
            memory_id: memory_id.into(),
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `memory_id`.
    #[must_use]
    pub fn memory_id(mut self, value: impl Into<MemoryId>) -> Self {
        self.memory_id = value.into();
        self
    }
}

/// `SyncMemoryAssetV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `memory_id` | [`MemoryId`](crate::id::MemoryId) | ✓ | Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncMemoryAssetV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncMemoryAssetV1 {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "memoryId")]
    pub memory_id: MemoryId,
}

impl SyncMemoryAssetV1 {
    /// Creates a new [`SyncMemoryAssetV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: impl Into<AssetId>, memory_id: impl Into<MemoryId>) -> Self {
        Self {
            asset_id: asset_id.into(),
            memory_id: memory_id.into(),
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `memory_id`.
    #[must_use]
    pub fn memory_id(mut self, value: impl Into<MemoryId>) -> Self {
        self.memory_id = value.into();
        self
    }
}

/// `SyncMemoryDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `memory_id` | [`MemoryId`](crate::id::MemoryId) | ✓ | Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncMemoryDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncMemoryDeleteV1 {
    /// Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "memoryId")]
    pub memory_id: MemoryId,
}

impl SyncMemoryDeleteV1 {
    /// Creates a new [`SyncMemoryDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(memory_id: impl Into<MemoryId>) -> Self {
        Self {
            memory_id: memory_id.into(),
        }
    }
    /// Sets `memory_id`.
    #[must_use]
    pub fn memory_id(mut self, value: impl Into<MemoryId>) -> Self {
        self.memory_id = value.into();
        self
    }
}

/// `SyncMemoryV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `data` | `object` | ✓ | Data |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deleted at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `hide_at` | [`DateTime`](::chrono::DateTime) | ✓ | Hide at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`MemoryId`](crate::id::MemoryId) | ✓ | Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_saved` | `bool` | ✓ | Is saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) | ✓ | Memory at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) | ✓ | Seen at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `show_at` | [`DateTime`](::chrono::DateTime) | ✓ | Show at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `type_` | [`MemoryTypeV1`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/SyncMemoryV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SyncMemoryV1 {
    /// Created at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Data (required)
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    /// Deleted at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Hide at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "hideAt")]
    pub hide_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: MemoryId,
    /// Is saved (required)
    #[serde(rename = "isSaved")]
    pub is_saved: bool,
    /// Memory at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "memoryAt")]
    pub memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    /// Seen at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "seenAt")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Show at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "showAt")]
    pub show_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: MemoryTypeV1,
    /// Updated at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncMemoryV1 {
    /// Creates a new [`SyncMemoryV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        data: ::serde_json::Map<String, ::serde_json::Value>,
        id: impl Into<MemoryId>,
        is_saved: bool,
        memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
        owner_id: impl Into<UserId>,
        type_: MemoryTypeV1,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            data,
            deleted_at: None,
            hide_at: None,
            id: id.into(),
            is_saved,
            memory_at,
            owner_id: owner_id.into(),
            seen_at: None,
            show_at: None,
            type_,
            updated_at,
        }
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `data`.
    #[must_use]
    pub fn data(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.data = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `hide_at`.
    #[must_use]
    pub fn hide_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.hide_at = Some(value);
        self
    }
    /// Sets or clears `hide_at`.
    #[must_use]
    pub fn set_hide_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.hide_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<MemoryId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = value;
        self
    }
    /// Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = value;
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }
    /// Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }
    /// Sets `show_at`.
    #[must_use]
    pub fn show_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.show_at = Some(value);
        self
    }
    /// Sets or clears `show_at`.
    #[must_use]
    pub fn set_show_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.show_at = value;
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: MemoryTypeV1) -> Self {
        self.type_ = value;
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `SyncMemoryV2`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `data` | `object` | ✓ | Data |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deleted at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `hide_at` | [`DateTime`](::chrono::DateTime) | ✓ | Hide at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`MemoryId`](crate::id::MemoryId) | ✓ | Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_saved` | `bool` | ✓ | Is saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) | ✓ | Memory at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) | ✓ | Seen at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `show_at` | [`DateTime`](::chrono::DateTime) | ✓ | Show at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `type_` | [`MemoryType`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/SyncMemoryV2>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SyncMemoryV2 {
    /// Created at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Data (required)
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    /// Deleted at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Hide at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "hideAt")]
    pub hide_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: MemoryId,
    /// Is saved (required)
    #[serde(rename = "isSaved")]
    pub is_saved: bool,
    /// Memory at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "memoryAt")]
    pub memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    /// Seen at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "seenAt")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Show at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "showAt")]
    pub show_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: MemoryType,
    /// Updated at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncMemoryV2 {
    /// Creates a new [`SyncMemoryV2`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        data: ::serde_json::Map<String, ::serde_json::Value>,
        id: impl Into<MemoryId>,
        is_saved: bool,
        memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
        owner_id: impl Into<UserId>,
        type_: MemoryType,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            data,
            deleted_at: None,
            hide_at: None,
            id: id.into(),
            is_saved,
            memory_at,
            owner_id: owner_id.into(),
            seen_at: None,
            show_at: None,
            type_,
            updated_at,
        }
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `data`.
    #[must_use]
    pub fn data(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.data = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `hide_at`.
    #[must_use]
    pub fn hide_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.hide_at = Some(value);
        self
    }
    /// Sets or clears `hide_at`.
    #[must_use]
    pub fn set_hide_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.hide_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<MemoryId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = value;
        self
    }
    /// Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = value;
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }
    /// Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }
    /// Sets `show_at`.
    #[must_use]
    pub fn show_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.show_at = Some(value);
        self
    }
    /// Sets or clears `show_at`.
    #[must_use]
    pub fn set_show_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.show_at = value;
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: MemoryType) -> Self {
        self.type_ = value;
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `SyncPartnerDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `shared_by_id` | [`UserId`](crate::id::UserId) | ✓ | Shared by ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `shared_with_id` | [`UserId`](crate::id::UserId) | ✓ | Shared with ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncPartnerDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncPartnerDeleteV1 {
    /// Shared by ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "sharedById")]
    pub shared_by_id: UserId,
    /// Shared with ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: UserId,
}

impl SyncPartnerDeleteV1 {
    /// Creates a new [`SyncPartnerDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(shared_by_id: impl Into<UserId>, shared_with_id: impl Into<UserId>) -> Self {
        Self {
            shared_by_id: shared_by_id.into(),
            shared_with_id: shared_with_id.into(),
        }
    }
    /// Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_by_id = value.into();
        self
    }
    /// Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_with_id = value.into();
        self
    }
}

/// `SyncPartnerV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `in_timeline` | `bool` | ✓ | In timeline |
/// | `shared_by_id` | [`UserId`](crate::id::UserId) | ✓ | Shared by ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `shared_with_id` | [`UserId`](crate::id::UserId) | ✓ | Shared with ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncPartnerV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncPartnerV1 {
    /// In timeline (required)
    #[serde(rename = "inTimeline")]
    pub in_timeline: bool,
    /// Shared by ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "sharedById")]
    pub shared_by_id: UserId,
    /// Shared with ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: UserId,
}

impl SyncPartnerV1 {
    /// Creates a new [`SyncPartnerV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(in_timeline: bool, shared_by_id: impl Into<UserId>, shared_with_id: impl Into<UserId>) -> Self {
        Self {
            in_timeline,
            shared_by_id: shared_by_id.into(),
            shared_with_id: shared_with_id.into(),
        }
    }
    /// Sets `in_timeline`.
    #[must_use]
    pub fn in_timeline(mut self, value: bool) -> Self {
        self.in_timeline = value;
        self
    }
    /// Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_by_id = value.into();
        self
    }
    /// Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_with_id = value.into();
        self
    }
}

/// `SyncPersonDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `person_id` | [`PersonId`](crate::id::PersonId) | ✓ | Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncPersonDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncPersonDeleteV1 {
    /// Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "personId")]
    pub person_id: PersonId,
}

impl SyncPersonDeleteV1 {
    /// Creates a new [`SyncPersonDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(person_id: impl Into<PersonId>) -> Self {
        Self {
            person_id: person_id.into(),
        }
    }
    /// Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: impl Into<PersonId>) -> Self {
        self.person_id = value.into();
        self
    }
}

/// `SyncPersonV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`DateTime`](::chrono::DateTime) | ✓ | Birth date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `color` | [`String`] | ✓ | Color |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `face_asset_id` | [`String`] | ✓ | Face asset ID |
/// | `id` | [`PersonId`](crate::id::PersonId) | ✓ | Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_favorite` | `bool` | ✓ | Is favorite |
/// | `is_hidden` | `bool` | ✓ | Is hidden |
/// | `name` | [`String`] | ✓ | Person name |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/SyncPersonV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncPersonV1 {
    /// Birth date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "birthDate")]
    pub birth_date: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Color (required)
    pub color: Option<String>,
    /// Created at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Face asset ID (required)
    #[serde(rename = "faceAssetId")]
    pub face_asset_id: Option<String>,
    /// Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: PersonId,
    /// Is favorite (required)
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    /// Is hidden (required)
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    /// Person name (required)
    pub name: String,
    /// Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    /// Updated at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncPersonV1 {
    /// Creates a new [`SyncPersonV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<PersonId>,
        is_favorite: bool,
        is_hidden: bool,
        name: impl Into<String>,
        owner_id: impl Into<UserId>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            birth_date: None,
            color: None,
            created_at,
            face_asset_id: None,
            id: id.into(),
            is_favorite,
            is_hidden,
            name: name.into(),
            owner_id: owner_id.into(),
            updated_at,
        }
    }
    /// Sets `birth_date`.
    #[must_use]
    pub fn birth_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.birth_date = Some(value);
        self
    }
    /// Sets or clears `birth_date`.
    #[must_use]
    pub fn set_birth_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.birth_date = value;
        self
    }
    /// Sets `color`.
    #[must_use]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }
    /// Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<String>) -> Self {
        self.color = value;
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `face_asset_id`.
    #[must_use]
    pub fn face_asset_id(mut self, value: impl Into<String>) -> Self {
        self.face_asset_id = Some(value.into());
        self
    }
    /// Sets or clears `face_asset_id`.
    #[must_use]
    pub fn set_face_asset_id(mut self, value: Option<String>) -> Self {
        self.face_asset_id = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<PersonId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = value;
        self
    }
    /// Sets `is_hidden`.
    #[must_use]
    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// Sync request type
///
/// See: <https://api.immich.app/models/SyncRequestType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SyncRequestType {
    AlbumsV1,
    AlbumsV2,
    AlbumUsersV1,
    AlbumToAssetsV1,
    AlbumAssetsV1,
    AlbumAssetsV2,
    AlbumAssetExifsV1,
    AssetsV1,
    AssetsV2,
    AssetExifsV1,
    AssetEditsV1,
    AssetMetadataV1,
    AssetOcrV1,
    AuthUsersV1,
    AuthUsersV2,
    MemoriesV1,
    MemoriesV2,
    MemoryToAssetsV1,
    MemoryToAssetsV2,
    PartnersV1,
    PartnerAssetsV1,
    PartnerAssetsV2,
    PartnerAssetExifsV1,
    PartnerStacksV1,
    StacksV1,
    UsersV1,
    PeopleV1,
    AssetFacesV1,
    AssetFacesV2,
    AssetFacesV3,
    UserMetadataV1,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl SyncRequestType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AlbumsV1 => "AlbumsV1",
            Self::AlbumsV2 => "AlbumsV2",
            Self::AlbumUsersV1 => "AlbumUsersV1",
            Self::AlbumToAssetsV1 => "AlbumToAssetsV1",
            Self::AlbumAssetsV1 => "AlbumAssetsV1",
            Self::AlbumAssetsV2 => "AlbumAssetsV2",
            Self::AlbumAssetExifsV1 => "AlbumAssetExifsV1",
            Self::AssetsV1 => "AssetsV1",
            Self::AssetsV2 => "AssetsV2",
            Self::AssetExifsV1 => "AssetExifsV1",
            Self::AssetEditsV1 => "AssetEditsV1",
            Self::AssetMetadataV1 => "AssetMetadataV1",
            Self::AssetOcrV1 => "AssetOcrV1",
            Self::AuthUsersV1 => "AuthUsersV1",
            Self::AuthUsersV2 => "AuthUsersV2",
            Self::MemoriesV1 => "MemoriesV1",
            Self::MemoriesV2 => "MemoriesV2",
            Self::MemoryToAssetsV1 => "MemoryToAssetsV1",
            Self::MemoryToAssetsV2 => "MemoryToAssetsV2",
            Self::PartnersV1 => "PartnersV1",
            Self::PartnerAssetsV1 => "PartnerAssetsV1",
            Self::PartnerAssetsV2 => "PartnerAssetsV2",
            Self::PartnerAssetExifsV1 => "PartnerAssetExifsV1",
            Self::PartnerStacksV1 => "PartnerStacksV1",
            Self::StacksV1 => "StacksV1",
            Self::UsersV1 => "UsersV1",
            Self::PeopleV1 => "PeopleV1",
            Self::AssetFacesV1 => "AssetFacesV1",
            Self::AssetFacesV2 => "AssetFacesV2",
            Self::AssetFacesV3 => "AssetFacesV3",
            Self::UserMetadataV1 => "UserMetadataV1",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for SyncRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for SyncRequestType {
    fn from(value: &str) -> Self {
        match value {
            "AlbumsV1" => Self::AlbumsV1,
            "AlbumsV2" => Self::AlbumsV2,
            "AlbumUsersV1" => Self::AlbumUsersV1,
            "AlbumToAssetsV1" => Self::AlbumToAssetsV1,
            "AlbumAssetsV1" => Self::AlbumAssetsV1,
            "AlbumAssetsV2" => Self::AlbumAssetsV2,
            "AlbumAssetExifsV1" => Self::AlbumAssetExifsV1,
            "AssetsV1" => Self::AssetsV1,
            "AssetsV2" => Self::AssetsV2,
            "AssetExifsV1" => Self::AssetExifsV1,
            "AssetEditsV1" => Self::AssetEditsV1,
            "AssetMetadataV1" => Self::AssetMetadataV1,
            "AssetOcrV1" => Self::AssetOcrV1,
            "AuthUsersV1" => Self::AuthUsersV1,
            "AuthUsersV2" => Self::AuthUsersV2,
            "MemoriesV1" => Self::MemoriesV1,
            "MemoriesV2" => Self::MemoriesV2,
            "MemoryToAssetsV1" => Self::MemoryToAssetsV1,
            "MemoryToAssetsV2" => Self::MemoryToAssetsV2,
            "PartnersV1" => Self::PartnersV1,
            "PartnerAssetsV1" => Self::PartnerAssetsV1,
            "PartnerAssetsV2" => Self::PartnerAssetsV2,
            "PartnerAssetExifsV1" => Self::PartnerAssetExifsV1,
            "PartnerStacksV1" => Self::PartnerStacksV1,
            "StacksV1" => Self::StacksV1,
            "UsersV1" => Self::UsersV1,
            "PeopleV1" => Self::PeopleV1,
            "AssetFacesV1" => Self::AssetFacesV1,
            "AssetFacesV2" => Self::AssetFacesV2,
            "AssetFacesV3" => Self::AssetFacesV3,
            "UserMetadataV1" => Self::UserMetadataV1,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for SyncRequestType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AlbumsV1" => Self::AlbumsV1,
            "AlbumsV2" => Self::AlbumsV2,
            "AlbumUsersV1" => Self::AlbumUsersV1,
            "AlbumToAssetsV1" => Self::AlbumToAssetsV1,
            "AlbumAssetsV1" => Self::AlbumAssetsV1,
            "AlbumAssetsV2" => Self::AlbumAssetsV2,
            "AlbumAssetExifsV1" => Self::AlbumAssetExifsV1,
            "AssetsV1" => Self::AssetsV1,
            "AssetsV2" => Self::AssetsV2,
            "AssetExifsV1" => Self::AssetExifsV1,
            "AssetEditsV1" => Self::AssetEditsV1,
            "AssetMetadataV1" => Self::AssetMetadataV1,
            "AssetOcrV1" => Self::AssetOcrV1,
            "AuthUsersV1" => Self::AuthUsersV1,
            "AuthUsersV2" => Self::AuthUsersV2,
            "MemoriesV1" => Self::MemoriesV1,
            "MemoriesV2" => Self::MemoriesV2,
            "MemoryToAssetsV1" => Self::MemoryToAssetsV1,
            "MemoryToAssetsV2" => Self::MemoryToAssetsV2,
            "PartnersV1" => Self::PartnersV1,
            "PartnerAssetsV1" => Self::PartnerAssetsV1,
            "PartnerAssetsV2" => Self::PartnerAssetsV2,
            "PartnerAssetExifsV1" => Self::PartnerAssetExifsV1,
            "PartnerStacksV1" => Self::PartnerStacksV1,
            "StacksV1" => Self::StacksV1,
            "UsersV1" => Self::UsersV1,
            "PeopleV1" => Self::PeopleV1,
            "AssetFacesV1" => Self::AssetFacesV1,
            "AssetFacesV2" => Self::AssetFacesV2,
            "AssetFacesV3" => Self::AssetFacesV3,
            "UserMetadataV1" => Self::UserMetadataV1,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for SyncRequestType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for SyncRequestType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for SyncRequestType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for SyncRequestType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = SyncRequestType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SyncRequestType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SyncRequestType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `SyncResetV1`
///
/// See: <https://api.immich.app/models/SyncResetV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct SyncResetV1(pub ::serde_json::Map<String, ::serde_json::Value>);

impl ::std::ops::Deref for SyncResetV1 {
    type Target = ::serde_json::Map<String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<String, ::serde_json::Value> {
        &self.0
    }
}

impl From<::serde_json::Map<String, ::serde_json::Value>> for SyncResetV1 {
    fn from(value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}

/// `SyncStackDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `stack_id` | [`StackId`](crate::id::StackId) | ✓ | Stack ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncStackDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncStackDeleteV1 {
    /// Stack ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "stackId")]
    pub stack_id: StackId,
}

impl SyncStackDeleteV1 {
    /// Creates a new [`SyncStackDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(stack_id: impl Into<StackId>) -> Self {
        Self {
            stack_id: stack_id.into(),
        }
    }
    /// Sets `stack_id`.
    #[must_use]
    pub fn stack_id(mut self, value: impl Into<StackId>) -> Self {
        self.stack_id = value.into();
        self
    }
}

/// `SyncStackV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`StackId`](crate::id::StackId) | ✓ | Stack ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `primary_asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Primary asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/SyncStackV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncStackV1 {
    /// Created at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Stack ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: StackId,
    /// Owner ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    /// Primary asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "primaryAssetId")]
    pub primary_asset_id: AssetId,
    /// Updated at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncStackV1 {
    /// Creates a new [`SyncStackV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<StackId>,
        owner_id: impl Into<UserId>,
        primary_asset_id: impl Into<AssetId>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            id: id.into(),
            owner_id: owner_id.into(),
            primary_asset_id: primary_asset_id.into(),
            updated_at,
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
    pub fn id(mut self, value: impl Into<StackId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `primary_asset_id`.
    #[must_use]
    pub fn primary_asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.primary_asset_id = value.into();
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `SyncStreamDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `reset` | `bool` |  | Reset sync state |
/// | `types` | [`Vec<SyncRequestType>`](crate::sync::SyncRequestType) | ✓ | Sync request types |
///
/// See: <https://api.immich.app/models/SyncStreamDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncStreamDTO {
    /// Reset sync state
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reset: Option<bool>,
    /// Sync request types (required)
    pub types: Vec<SyncRequestType>,
}

impl SyncStreamDTO {
    /// Creates a new [`SyncStreamDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(types: impl IntoIterator<Item = SyncRequestType>) -> Self {
        Self {
            reset: None,
            types: types.into_iter().collect(),
        }
    }
    /// Sets `reset`.
    #[must_use]
    pub fn reset(mut self, value: bool) -> Self {
        self.reset = Some(value);
        self
    }
    /// Sets or clears `reset`.
    #[must_use]
    pub fn set_reset(mut self, value: Option<bool>) -> Self {
        self.reset = value;
        self
    }
    /// Sets `types`.
    #[must_use]
    pub fn types(mut self, value: impl IntoIterator<Item = SyncRequestType>) -> Self {
        self.types = value.into_iter().collect();
        self
    }
}

/// `SyncUserDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncUserDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncUserDeleteV1 {
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl SyncUserDeleteV1 {
    /// Creates a new [`SyncUserDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(user_id: impl Into<UserId>) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
    }
}

/// `SyncUserMetadataDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `key` | [`UserMetadataKey`] | ✓ |  |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/SyncUserMetadataDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncUserMetadataDeleteV1 {
    /// (required)
    pub key: UserMetadataKey,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl SyncUserMetadataDeleteV1 {
    /// Creates a new [`SyncUserMetadataDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(key: UserMetadataKey, user_id: impl Into<UserId>) -> Self {
        Self {
            key,
            user_id: user_id.into(),
        }
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: UserMetadataKey) -> Self {
        self.key = value;
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
    }
}

/// `SyncUserMetadataV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `key` | [`UserMetadataKey`] | ✓ |  |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `value` | `object` | ✓ | User metadata value |
///
/// See: <https://api.immich.app/models/SyncUserMetadataV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SyncUserMetadataV1 {
    /// (required)
    pub key: UserMetadataKey,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
    /// User metadata value (required)
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl SyncUserMetadataV1 {
    /// Creates a new [`SyncUserMetadataV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        key: UserMetadataKey,
        user_id: impl Into<UserId>,
        value: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self {
            key,
            user_id: user_id.into(),
            value,
        }
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: UserMetadataKey) -> Self {
        self.key = value;
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
        self
    }
    /// Sets `value`.
    #[must_use]
    pub fn value(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.value = value;
        self
    }
}

/// `SyncUserV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] |  |  |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | User deleted at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `email` | [`String`] | ✓ | User email |
/// | `has_profile_image` | `bool` | ✓ | User has profile image |
/// | `id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `name` | [`String`] | ✓ | User name |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | User profile changed at (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/SyncUserV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncUserV1 {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    /// User deleted at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// User email (required)
    pub email: String,
    /// User has profile image (required)
    #[serde(rename = "hasProfileImage")]
    pub has_profile_image: bool,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: UserId,
    /// User name (required)
    pub name: String,
    /// User profile changed at (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncUserV1 {
    /// Creates a new [`SyncUserV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        email: impl Into<String>,
        has_profile_image: bool,
        id: impl Into<UserId>,
        name: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            avatar_color: None,
            deleted_at: None,
            email: email.into(),
            has_profile_image,
            id: id.into(),
            name: name.into(),
            profile_changed_at,
        }
    }
    /// Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }
    /// Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }
    /// Sets `has_profile_image`.
    #[must_use]
    pub fn has_profile_image(mut self, value: bool) -> Self {
        self.has_profile_image = value;
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
}
