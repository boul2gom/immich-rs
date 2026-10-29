// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `sync`.

pub use crate::activities::UserAvatarColor;
pub use crate::albums::{AlbumUserRole, AssetOrder};
pub use crate::assets::{AssetEditAction, AssetTypeEnum, AssetVisibility};
pub use crate::memories::{MemoryType, MemoryTypeV1};
pub use crate::users::UserMetadataKey;

/// `SyncAckDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `types` | [`Vec<SyncEntityType>`](crate::sync::SyncEntityType) |  | Sync entity types to delete acks for |
///
/// See: <https://api.immich.app/models/SyncAckDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct SyncAckDeleteDTO {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub types: Vec<SyncEntityType>,
}

impl SyncAckDeleteDTO {
    ///Creates a new [`SyncAckDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    ///Sets `types`.
    #[must_use]
    pub fn types(mut self, value: Vec<SyncEntityType>) -> Self {
        self.types = value;
        self
    }
}

/// `SyncAckDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ack` | [`String`] | ✓ | Acknowledgment ID |
/// | `type` | [`SyncEntityType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/SyncAckDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAckDTO {
    pub ack: String,
    /// (required)
    #[serde(rename = "type")]
    pub type_: SyncEntityType,
}

impl SyncAckDTO {
    ///Creates a new [`SyncAckDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ack: impl Into<String>, type_: SyncEntityType) -> Self {
        Self { ack: ack.into(), type_ }
    }

    ///Sets `ack`.
    #[must_use]
    pub fn ack(mut self, value: impl Into<String>) -> Self {
        self.ack = value.into();
        self
    }

    ///Sets `type_`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAckSetDTO {
    pub acks: Vec<String>,
}

impl SyncAckSetDTO {
    ///Creates a new [`SyncAckSetDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(acks: Vec<String>) -> Self {
        Self { acks }
    }

    ///Sets `acks`.
    #[must_use]
    pub fn acks(mut self, value: Vec<String>) -> Self {
        self.acks = value;
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
/// | `album_id` | [`Uuid`](::uuid::Uuid) | ✓ | Album ID |
///
/// See: <https://api.immich.app/models/SyncAlbumDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAlbumDeleteV1 {
    /// Album ID (required)
    #[serde(rename = "albumId")]
    pub album_id: ::uuid::Uuid,
}

impl SyncAlbumDeleteV1 {
    ///Creates a new [`SyncAlbumDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: ::uuid::Uuid) -> Self {
        Self { album_id }
    }

    ///Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: ::uuid::Uuid) -> Self {
        self.album_id = value;
        self
    }
}

/// `SyncAlbumToAssetDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`Uuid`](::uuid::Uuid) | ✓ | Album ID |
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
///
/// See: <https://api.immich.app/models/SyncAlbumToAssetDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAlbumToAssetDeleteV1 {
    /// Album ID (required)
    #[serde(rename = "albumId")]
    pub album_id: ::uuid::Uuid,
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
}

impl SyncAlbumToAssetDeleteV1 {
    ///Creates a new [`SyncAlbumToAssetDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: ::uuid::Uuid, asset_id: ::uuid::Uuid) -> Self {
        Self { album_id, asset_id }
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
        self.asset_id = value;
        self
    }
}

/// `SyncAlbumToAssetV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`Uuid`](::uuid::Uuid) | ✓ | Album ID |
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
///
/// See: <https://api.immich.app/models/SyncAlbumToAssetV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAlbumToAssetV1 {
    /// Album ID (required)
    #[serde(rename = "albumId")]
    pub album_id: ::uuid::Uuid,
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
}

impl SyncAlbumToAssetV1 {
    ///Creates a new [`SyncAlbumToAssetV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: ::uuid::Uuid, asset_id: ::uuid::Uuid) -> Self {
        Self { album_id, asset_id }
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
        self.asset_id = value;
        self
    }
}

/// `SyncAlbumUserDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`Uuid`](::uuid::Uuid) | ✓ | Album ID |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
///
/// See: <https://api.immich.app/models/SyncAlbumUserDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAlbumUserDeleteV1 {
    /// Album ID (required)
    #[serde(rename = "albumId")]
    pub album_id: ::uuid::Uuid,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl SyncAlbumUserDeleteV1 {
    ///Creates a new [`SyncAlbumUserDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: ::uuid::Uuid, user_id: ::uuid::Uuid) -> Self {
        Self { album_id, user_id }
    }

    ///Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: ::uuid::Uuid) -> Self {
        self.album_id = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }
}

/// `SyncAlbumUserV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`Uuid`](::uuid::Uuid) | ✓ | Album ID |
/// | `role` | [`AlbumUserRole`] | ✓ |  |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
///
/// See: <https://api.immich.app/models/SyncAlbumUserV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAlbumUserV1 {
    /// Album ID (required)
    #[serde(rename = "albumId")]
    pub album_id: ::uuid::Uuid,
    pub role: AlbumUserRole,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl SyncAlbumUserV1 {
    ///Creates a new [`SyncAlbumUserV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_id: ::uuid::Uuid, role: AlbumUserRole, user_id: ::uuid::Uuid) -> Self {
        Self {
            album_id,
            role,
            user_id,
        }
    }

    ///Sets `album_id`.
    #[must_use]
    pub fn album_id(mut self, value: ::uuid::Uuid) -> Self {
        self.album_id = value;
        self
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

/// `SyncAlbumV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at |
/// | `description` | [`String`] | ✓ | Album description |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Album ID |
/// | `is_activity_enabled` | `bool` | ✓ | Is activity enabled |
/// | `name` | [`String`] | ✓ | Album name |
/// | `order` | [`AssetOrder`] | ✓ |  |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner ID |
/// | `thumbnail_asset_id` | [`String`] | ✓ | Thumbnail asset ID |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at |
///
/// See: <https://api.immich.app/models/SyncAlbumV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAlbumV1 {
    /// Created at (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub description: String,
    pub id: ::uuid::Uuid,
    /// Is activity enabled (required)
    #[serde(rename = "isActivityEnabled")]
    pub is_activity_enabled: bool,
    pub name: String,
    pub order: AssetOrder,
    /// Owner ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    /// Thumbnail asset ID (required)
    #[serde(rename = "thumbnailAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_asset_id: Option<String>,
    /// Updated at (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncAlbumV1 {
    ///Creates a new [`SyncAlbumV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        description: impl Into<String>,
        id: ::uuid::Uuid,
        is_activity_enabled: bool,
        name: impl Into<String>,
        order: AssetOrder,
        owner_id: ::uuid::Uuid,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            description: description.into(),
            id,
            is_activity_enabled,
            name: name.into(),
            order,
            owner_id,
            thumbnail_asset_id: None,
            updated_at,
        }
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

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `order`.
    #[must_use]
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = value;
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `thumbnail_asset_id`.
    #[must_use]
    pub fn thumbnail_asset_id(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_asset_id = Some(value.into());
        self
    }

    ///Sets or clears `thumbnail_asset_id`.
    #[must_use]
    pub fn set_thumbnail_asset_id(mut self, value: Option<String>) -> Self {
        self.thumbnail_asset_id = value;
        self
    }

    ///Sets `updated_at`.
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
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at |
/// | `description` | [`String`] | ✓ | Album description |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Album ID |
/// | `is_activity_enabled` | `bool` | ✓ | Is activity enabled |
/// | `name` | [`String`] | ✓ | Album name |
/// | `order` | [`AssetOrder`] | ✓ |  |
/// | `thumbnail_asset_id` | [`String`] | ✓ | Thumbnail asset ID |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at |
///
/// See: <https://api.immich.app/models/SyncAlbumV2>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAlbumV2 {
    /// Created at (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub description: String,
    pub id: ::uuid::Uuid,
    /// Is activity enabled (required)
    #[serde(rename = "isActivityEnabled")]
    pub is_activity_enabled: bool,
    pub name: String,
    pub order: AssetOrder,
    /// Thumbnail asset ID (required)
    #[serde(rename = "thumbnailAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_asset_id: Option<String>,
    /// Updated at (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncAlbumV2 {
    ///Creates a new [`SyncAlbumV2`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        description: impl Into<String>,
        id: ::uuid::Uuid,
        is_activity_enabled: bool,
        name: impl Into<String>,
        order: AssetOrder,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            description: description.into(),
            id,
            is_activity_enabled,
            name: name.into(),
            order,
            thumbnail_asset_id: None,
            updated_at,
        }
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

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `order`.
    #[must_use]
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = value;
        self
    }

    ///Sets `thumbnail_asset_id`.
    #[must_use]
    pub fn thumbnail_asset_id(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_asset_id = Some(value.into());
        self
    }

    ///Sets or clears `thumbnail_asset_id`.
    #[must_use]
    pub fn set_thumbnail_asset_id(mut self, value: Option<String>) -> Self {
        self.thumbnail_asset_id = value;
        self
    }

    ///Sets `updated_at`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
///
/// See: <https://api.immich.app/models/SyncAssetDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetDeleteV1 {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
}

impl SyncAssetDeleteV1 {
    ///Creates a new [`SyncAssetDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: ::uuid::Uuid) -> Self {
        Self { asset_id }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }
}

/// `SyncAssetEditDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `edit_id` | [`Uuid`](::uuid::Uuid) | ✓ | Edit ID |
///
/// See: <https://api.immich.app/models/SyncAssetEditDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetEditDeleteV1 {
    /// Edit ID (required)
    #[serde(rename = "editId")]
    pub edit_id: ::uuid::Uuid,
}

impl SyncAssetEditDeleteV1 {
    ///Creates a new [`SyncAssetEditDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(edit_id: ::uuid::Uuid) -> Self {
        Self { edit_id }
    }

    ///Sets `edit_id`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Edit ID |
/// | `parameters` | `object` | ✓ | Edit parameters |
/// | `sequence` | `i64` | ✓ | Edit sequence |
///
/// See: <https://api.immich.app/models/SyncAssetEditV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetEditV1 {
    pub action: AssetEditAction,
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    pub id: ::uuid::Uuid,
    pub parameters: ::serde_json::Map<String, ::serde_json::Value>,
    pub sequence: i64,
}

impl SyncAssetEditV1 {
    ///Creates a new [`SyncAssetEditV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        action: AssetEditAction,
        asset_id: ::uuid::Uuid,
        id: ::uuid::Uuid,
        parameters: ::serde_json::Map<String, ::serde_json::Value>,
        sequence: i64,
    ) -> Self {
        Self {
            action,
            asset_id,
            id,
            parameters,
            sequence,
        }
    }

    ///Sets `action`.
    #[must_use]
    pub fn action(mut self, value: AssetEditAction) -> Self {
        self.action = value;
        self
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `parameters`.
    #[must_use]
    pub fn parameters(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.parameters = value;
        self
    }

    ///Sets `sequence`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `city` | [`String`] | ✓ | City |
/// | `country` | [`String`] | ✓ | Country |
/// | `date_time_original` | [`DateTime`](::chrono::DateTime) | ✓ | Date time original |
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
/// | `modify_date` | [`DateTime`](::chrono::DateTime) | ✓ | Modify date |
/// | `orientation` | [`String`] | ✓ | Orientation |
/// | `profile_description` | [`String`] | ✓ | Profile description |
/// | `projection_type` | [`String`] | ✓ | Projection type |
/// | `rating` | `i64` | ✓ | Rating |
/// | `state` | [`String`] | ✓ | State |
/// | `time_zone` | [`String`] | ✓ | Time zone |
///
/// See: <https://api.immich.app/models/SyncAssetExifV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetExifV1 {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Date time original (required)
    #[serde(rename = "dateTimeOriginal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_original: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Exif image height (required)
    #[serde(rename = "exifImageHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exif_image_height: Option<i64>,
    /// Exif image width (required)
    #[serde(rename = "exifImageWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exif_image_width: Option<i64>,
    /// Exposure time (required)
    #[serde(rename = "exposureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposure_time: Option<String>,
    /// F number (required)
    #[serde(rename = "fNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_number: Option<f64>,
    /// File size in byte (required)
    #[serde(rename = "fileSizeInByte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_in_byte: Option<i64>,
    /// Focal length (required)
    #[serde(rename = "focalLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focal_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// Lens model (required)
    #[serde(rename = "lensModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lens_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Modify date (required)
    #[serde(rename = "modifyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_date: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    /// Profile description (required)
    #[serde(rename = "profileDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_description: Option<String>,
    /// Projection type (required)
    #[serde(rename = "projectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Time zone (required)
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl SyncAssetExifV1 {
    ///Creates a new [`SyncAssetExifV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: ::uuid::Uuid) -> Self {
        Self {
            asset_id,
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

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
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

    ///Sets `date_time_original`.
    #[must_use]
    pub fn date_time_original(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.date_time_original = Some(value);
        self
    }

    ///Sets or clears `date_time_original`.
    #[must_use]
    pub fn set_date_time_original(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.date_time_original = value;
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

    ///Sets `exif_image_height`.
    #[must_use]
    pub fn exif_image_height(mut self, value: i64) -> Self {
        self.exif_image_height = Some(value);
        self
    }

    ///Sets or clears `exif_image_height`.
    #[must_use]
    pub fn set_exif_image_height(mut self, value: Option<i64>) -> Self {
        self.exif_image_height = value;
        self
    }

    ///Sets `exif_image_width`.
    #[must_use]
    pub fn exif_image_width(mut self, value: i64) -> Self {
        self.exif_image_width = Some(value);
        self
    }

    ///Sets or clears `exif_image_width`.
    #[must_use]
    pub fn set_exif_image_width(mut self, value: Option<i64>) -> Self {
        self.exif_image_width = value;
        self
    }

    ///Sets `exposure_time`.
    #[must_use]
    pub fn exposure_time(mut self, value: impl Into<String>) -> Self {
        self.exposure_time = Some(value.into());
        self
    }

    ///Sets or clears `exposure_time`.
    #[must_use]
    pub fn set_exposure_time(mut self, value: Option<String>) -> Self {
        self.exposure_time = value;
        self
    }

    ///Sets `f_number`.
    #[must_use]
    pub fn f_number(mut self, value: f64) -> Self {
        self.f_number = Some(value);
        self
    }

    ///Sets or clears `f_number`.
    #[must_use]
    pub fn set_f_number(mut self, value: Option<f64>) -> Self {
        self.f_number = value;
        self
    }

    ///Sets `file_size_in_byte`.
    #[must_use]
    pub fn file_size_in_byte(mut self, value: i64) -> Self {
        self.file_size_in_byte = Some(value);
        self
    }

    ///Sets or clears `file_size_in_byte`.
    #[must_use]
    pub fn set_file_size_in_byte(mut self, value: Option<i64>) -> Self {
        self.file_size_in_byte = value;
        self
    }

    ///Sets `focal_length`.
    #[must_use]
    pub fn focal_length(mut self, value: f64) -> Self {
        self.focal_length = Some(value);
        self
    }

    ///Sets or clears `focal_length`.
    #[must_use]
    pub fn set_focal_length(mut self, value: Option<f64>) -> Self {
        self.focal_length = value;
        self
    }

    ///Sets `fps`.
    #[must_use]
    pub fn fps(mut self, value: f64) -> Self {
        self.fps = Some(value);
        self
    }

    ///Sets or clears `fps`.
    #[must_use]
    pub fn set_fps(mut self, value: Option<f64>) -> Self {
        self.fps = value;
        self
    }

    ///Sets `iso`.
    #[must_use]
    pub fn iso(mut self, value: i64) -> Self {
        self.iso = Some(value);
        self
    }

    ///Sets or clears `iso`.
    #[must_use]
    pub fn set_iso(mut self, value: Option<i64>) -> Self {
        self.iso = value;
        self
    }

    ///Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    ///Sets or clears `latitude`.
    #[must_use]
    pub fn set_latitude(mut self, value: Option<f64>) -> Self {
        self.latitude = value;
        self
    }

    ///Sets `lens_model`.
    #[must_use]
    pub fn lens_model(mut self, value: impl Into<String>) -> Self {
        self.lens_model = Some(value.into());
        self
    }

    ///Sets or clears `lens_model`.
    #[must_use]
    pub fn set_lens_model(mut self, value: Option<String>) -> Self {
        self.lens_model = value;
        self
    }

    ///Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    ///Sets or clears `longitude`.
    #[must_use]
    pub fn set_longitude(mut self, value: Option<f64>) -> Self {
        self.longitude = value;
        self
    }

    ///Sets `make`.
    #[must_use]
    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }

    ///Sets or clears `make`.
    #[must_use]
    pub fn set_make(mut self, value: Option<String>) -> Self {
        self.make = value;
        self
    }

    ///Sets `model`.
    #[must_use]
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    ///Sets or clears `model`.
    #[must_use]
    pub fn set_model(mut self, value: Option<String>) -> Self {
        self.model = value;
        self
    }

    ///Sets `modify_date`.
    #[must_use]
    pub fn modify_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.modify_date = Some(value);
        self
    }

    ///Sets or clears `modify_date`.
    #[must_use]
    pub fn set_modify_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.modify_date = value;
        self
    }

    ///Sets `orientation`.
    #[must_use]
    pub fn orientation(mut self, value: impl Into<String>) -> Self {
        self.orientation = Some(value.into());
        self
    }

    ///Sets or clears `orientation`.
    #[must_use]
    pub fn set_orientation(mut self, value: Option<String>) -> Self {
        self.orientation = value;
        self
    }

    ///Sets `profile_description`.
    #[must_use]
    pub fn profile_description(mut self, value: impl Into<String>) -> Self {
        self.profile_description = Some(value.into());
        self
    }

    ///Sets or clears `profile_description`.
    #[must_use]
    pub fn set_profile_description(mut self, value: Option<String>) -> Self {
        self.profile_description = value;
        self
    }

    ///Sets `projection_type`.
    #[must_use]
    pub fn projection_type(mut self, value: impl Into<String>) -> Self {
        self.projection_type = Some(value.into());
        self
    }

    ///Sets or clears `projection_type`.
    #[must_use]
    pub fn set_projection_type(mut self, value: Option<String>) -> Self {
        self.projection_type = value;
        self
    }

    ///Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: i64) -> Self {
        self.rating = Some(value);
        self
    }

    ///Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<i64>) -> Self {
        self.rating = value;
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

    ///Sets `time_zone`.
    #[must_use]
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    ///Sets or clears `time_zone`.
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
/// | `asset_face_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset face ID |
///
/// See: <https://api.immich.app/models/SyncAssetFaceDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetFaceDeleteV1 {
    /// Asset face ID (required)
    #[serde(rename = "assetFaceId")]
    pub asset_face_id: ::uuid::Uuid,
}

impl SyncAssetFaceDeleteV1 {
    ///Creates a new [`SyncAssetFaceDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_face_id: ::uuid::Uuid) -> Self {
        Self { asset_face_id }
    }

    ///Sets `asset_face_id`.
    #[must_use]
    pub fn asset_face_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_face_id = value;
        self
    }
}

/// `SyncAssetFaceV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `bounding_box_x1` | `i64` | ✓ | Bounding box X1 |
/// | `bounding_box_x2` | `i64` | ✓ | Bounding box X2 |
/// | `bounding_box_y1` | `i64` | ✓ | Bounding box Y1 |
/// | `bounding_box_y2` | `i64` | ✓ | Bounding box Y2 |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset face ID |
/// | `image_height` | `i64` | ✓ | Image height |
/// | `image_width` | `i64` | ✓ | Image width |
/// | `person_id` | [`String`] | ✓ | Person ID |
/// | `source_type` | [`String`] | ✓ | Source type |
///
/// See: <https://api.immich.app/models/SyncAssetFaceV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetFaceV1 {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
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
    pub id: ::uuid::Uuid,
    /// Image height (required)
    #[serde(rename = "imageHeight")]
    pub image_height: i64,
    /// Image width (required)
    #[serde(rename = "imageWidth")]
    pub image_width: i64,
    /// Person ID (required)
    #[serde(rename = "personId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    /// Source type (required)
    #[serde(rename = "sourceType")]
    pub source_type: String,
}

impl SyncAssetFaceV1 {
    ///Creates a new [`SyncAssetFaceV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: ::uuid::Uuid,
        bounding_box_x1: i64,
        bounding_box_x2: i64,
        bounding_box_y1: i64,
        bounding_box_y2: i64,
        id: ::uuid::Uuid,
        image_height: i64,
        image_width: i64,
        source_type: impl Into<String>,
    ) -> Self {
        Self {
            asset_id,
            bounding_box_x1,
            bounding_box_x2,
            bounding_box_y1,
            bounding_box_y2,
            id,
            image_height,
            image_width,
            person_id: None,
            source_type: source_type.into(),
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `bounding_box_x1`.
    #[must_use]
    pub fn bounding_box_x1(mut self, value: i64) -> Self {
        self.bounding_box_x1 = value;
        self
    }

    ///Sets `bounding_box_x2`.
    #[must_use]
    pub fn bounding_box_x2(mut self, value: i64) -> Self {
        self.bounding_box_x2 = value;
        self
    }

    ///Sets `bounding_box_y1`.
    #[must_use]
    pub fn bounding_box_y1(mut self, value: i64) -> Self {
        self.bounding_box_y1 = value;
        self
    }

    ///Sets `bounding_box_y2`.
    #[must_use]
    pub fn bounding_box_y2(mut self, value: i64) -> Self {
        self.bounding_box_y2 = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `image_height`.
    #[must_use]
    pub fn image_height(mut self, value: i64) -> Self {
        self.image_height = value;
        self
    }

    ///Sets `image_width`.
    #[must_use]
    pub fn image_width(mut self, value: i64) -> Self {
        self.image_width = value;
        self
    }

    ///Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: impl Into<String>) -> Self {
        self.person_id = Some(value.into());
        self
    }

    ///Sets or clears `person_id`.
    #[must_use]
    pub fn set_person_id(mut self, value: Option<String>) -> Self {
        self.person_id = value;
        self
    }

    ///Sets `source_type`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `bounding_box_x1` | `i64` | ✓ | Bounding box X1 |
/// | `bounding_box_x2` | `i64` | ✓ | Bounding box X2 |
/// | `bounding_box_y1` | `i64` | ✓ | Bounding box Y1 |
/// | `bounding_box_y2` | `i64` | ✓ | Bounding box Y2 |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Face deleted at |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset face ID |
/// | `image_height` | `i64` | ✓ | Image height |
/// | `image_width` | `i64` | ✓ | Image width |
/// | `is_visible` | `bool` | ✓ | Is the face visible in the asset |
/// | `person_id` | [`String`] | ✓ | Person ID |
/// | `source_type` | [`String`] | ✓ | Source type |
///
/// See: <https://api.immich.app/models/SyncAssetFaceV3>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetFaceV3 {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
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
    /// Face deleted at (required)
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub id: ::uuid::Uuid,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    /// Source type (required)
    #[serde(rename = "sourceType")]
    pub source_type: String,
}

impl SyncAssetFaceV3 {
    ///Creates a new [`SyncAssetFaceV3`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: ::uuid::Uuid,
        bounding_box_x1: i64,
        bounding_box_x2: i64,
        bounding_box_y1: i64,
        bounding_box_y2: i64,
        id: ::uuid::Uuid,
        image_height: i64,
        image_width: i64,
        is_visible: bool,
        source_type: impl Into<String>,
    ) -> Self {
        Self {
            asset_id,
            bounding_box_x1,
            bounding_box_x2,
            bounding_box_y1,
            bounding_box_y2,
            deleted_at: None,
            id,
            image_height,
            image_width,
            is_visible,
            person_id: None,
            source_type: source_type.into(),
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `bounding_box_x1`.
    #[must_use]
    pub fn bounding_box_x1(mut self, value: i64) -> Self {
        self.bounding_box_x1 = value;
        self
    }

    ///Sets `bounding_box_x2`.
    #[must_use]
    pub fn bounding_box_x2(mut self, value: i64) -> Self {
        self.bounding_box_x2 = value;
        self
    }

    ///Sets `bounding_box_y1`.
    #[must_use]
    pub fn bounding_box_y1(mut self, value: i64) -> Self {
        self.bounding_box_y1 = value;
        self
    }

    ///Sets `bounding_box_y2`.
    #[must_use]
    pub fn bounding_box_y2(mut self, value: i64) -> Self {
        self.bounding_box_y2 = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `image_height`.
    #[must_use]
    pub fn image_height(mut self, value: i64) -> Self {
        self.image_height = value;
        self
    }

    ///Sets `image_width`.
    #[must_use]
    pub fn image_width(mut self, value: i64) -> Self {
        self.image_width = value;
        self
    }

    ///Sets `is_visible`.
    #[must_use]
    pub fn is_visible(mut self, value: bool) -> Self {
        self.is_visible = value;
        self
    }

    ///Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: impl Into<String>) -> Self {
        self.person_id = Some(value.into());
        self
    }

    ///Sets or clears `person_id`.
    #[must_use]
    pub fn set_person_id(mut self, value: Option<String>) -> Self {
        self.person_id = value;
        self
    }

    ///Sets `source_type`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `key` | [`String`] | ✓ | Key |
///
/// See: <https://api.immich.app/models/SyncAssetMetadataDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetMetadataDeleteV1 {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    pub key: String,
}

impl SyncAssetMetadataDeleteV1 {
    ///Creates a new [`SyncAssetMetadataDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: ::uuid::Uuid, key: impl Into<String>) -> Self {
        Self {
            asset_id,
            key: key.into(),
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `key`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `key` | [`String`] | ✓ | Key |
/// | `value` | `object` | ✓ | Value |
///
/// See: <https://api.immich.app/models/SyncAssetMetadataV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetMetadataV1 {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    pub key: String,
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl SyncAssetMetadataV1 {
    ///Creates a new [`SyncAssetMetadataV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: ::uuid::Uuid,
        key: impl Into<String>,
        value: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self {
            asset_id,
            key: key.into(),
            value,
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }

    ///Sets `value`.
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
/// | `asset_id` | [`String`] | ✓ | Original asset ID of the deleted OCR entry |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Timestamp when the OCR entry was deleted |
/// | `id` | [`String`] | ✓ | Audit row ID of the deleted OCR entry |
///
/// See: <https://api.immich.app/models/SyncAssetOcrDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetOcrDeleteV1 {
    /// Original asset ID of the deleted OCR entry (required)
    #[serde(rename = "assetId")]
    pub asset_id: String,
    /// Timestamp when the OCR entry was deleted (required)
    #[serde(rename = "deletedAt")]
    pub deleted_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: String,
}

impl SyncAssetOcrDeleteV1 {
    ///Creates a new [`SyncAssetOcrDeleteV1`].
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

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = value.into();
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `id`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `box_score` | `f64` | ✓ | Confidence score of the bounding box |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | OCR entry ID |
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
pub struct SyncAssetOcrV1 {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    /// Confidence score of the bounding box (required)
    #[serde(rename = "boxScore")]
    pub box_score: f64,
    pub id: ::uuid::Uuid,
    /// Whether the OCR entry is visible (required)
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    pub text: String,
    /// Confidence score of the recognized text (required)
    #[serde(rename = "textScore")]
    pub text_score: f64,
    pub x1: f64,
    pub x2: f64,
    pub x3: f64,
    pub x4: f64,
    pub y1: f64,
    pub y2: f64,
    pub y3: f64,
    pub y4: f64,
}

impl SyncAssetOcrV1 {
    ///Creates a new [`SyncAssetOcrV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: ::uuid::Uuid,
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
            asset_id,
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

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `box_score`.
    #[must_use]
    pub fn box_score(mut self, value: f64) -> Self {
        self.box_score = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_visible`.
    #[must_use]
    pub fn is_visible(mut self, value: bool) -> Self {
        self.is_visible = value;
        self
    }

    ///Sets `text`.
    #[must_use]
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = value.into();
        self
    }

    ///Sets `text_score`.
    #[must_use]
    pub fn text_score(mut self, value: f64) -> Self {
        self.text_score = value;
        self
    }

    ///Sets `x1`.
    #[must_use]
    pub fn x1(mut self, value: f64) -> Self {
        self.x1 = value;
        self
    }

    ///Sets `x2`.
    #[must_use]
    pub fn x2(mut self, value: f64) -> Self {
        self.x2 = value;
        self
    }

    ///Sets `x3`.
    #[must_use]
    pub fn x3(mut self, value: f64) -> Self {
        self.x3 = value;
        self
    }

    ///Sets `x4`.
    #[must_use]
    pub fn x4(mut self, value: f64) -> Self {
        self.x4 = value;
        self
    }

    ///Sets `y1`.
    #[must_use]
    pub fn y1(mut self, value: f64) -> Self {
        self.y1 = value;
        self
    }

    ///Sets `y2`.
    #[must_use]
    pub fn y2(mut self, value: f64) -> Self {
        self.y2 = value;
        self
    }

    ///Sets `y3`.
    #[must_use]
    pub fn y3(mut self, value: f64) -> Self {
        self.y3 = value;
        self
    }

    ///Sets `y4`.
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
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Uploaded to Immich at |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deleted at |
/// | `duration` | [`String`] | ✓ | Duration |
/// | `file_created_at` | [`DateTime`](::chrono::DateTime) | ✓ | File created at |
/// | `file_modified_at` | [`DateTime`](::chrono::DateTime) | ✓ | File modified at |
/// | `height` | `i64` | ✓ | Asset height |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `is_edited` | `bool` | ✓ | Is edited |
/// | `is_favorite` | `bool` | ✓ | Is favorite |
/// | `library_id` | [`String`] | ✓ | Library ID |
/// | `live_photo_video_id` | [`String`] | ✓ | Live photo video ID |
/// | `local_date_time` | [`DateTime`](::chrono::DateTime) | ✓ | Local date time |
/// | `original_file_name` | [`String`] | ✓ | Original file name |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner ID |
/// | `stack_id` | [`String`] | ✓ | Stack ID |
/// | `thumbhash` | [`String`] | ✓ | Thumbhash |
/// | `type` | [`AssetTypeEnum`] | ✓ |  |
/// | `visibility` | [`AssetVisibility`] | ✓ |  |
/// | `width` | `i64` | ✓ | Asset width |
///
/// See: <https://api.immich.app/models/SyncAssetV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetV1 {
    pub checksum: String,
    /// Uploaded to Immich at (required)
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Deleted at (required)
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// File created at (required)
    #[serde(rename = "fileCreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// File modified at (required)
    #[serde(rename = "fileModifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_modified_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    pub id: ::uuid::Uuid,
    /// Is edited (required)
    #[serde(rename = "isEdited")]
    pub is_edited: bool,
    /// Is favorite (required)
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    /// Library ID (required)
    #[serde(rename = "libraryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_id: Option<String>,
    /// Live photo video ID (required)
    #[serde(rename = "livePhotoVideoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_photo_video_id: Option<String>,
    /// Local date time (required)
    #[serde(rename = "localDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_date_time: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Original file name (required)
    #[serde(rename = "originalFileName")]
    pub original_file_name: String,
    /// Owner ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    /// Stack ID (required)
    #[serde(rename = "stackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbhash: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: AssetTypeEnum,
    pub visibility: AssetVisibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl SyncAssetV1 {
    ///Creates a new [`SyncAssetV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        checksum: impl Into<String>,
        id: ::uuid::Uuid,
        is_edited: bool,
        is_favorite: bool,
        original_file_name: impl Into<String>,
        owner_id: ::uuid::Uuid,
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
            id,
            is_edited,
            is_favorite,
            library_id: None,
            live_photo_video_id: None,
            local_date_time: None,
            original_file_name: original_file_name.into(),
            owner_id,
            stack_id: None,
            thumbhash: None,
            type_,
            visibility,
            width: None,
        }
    }

    ///Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = value.into();
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    ///Sets or clears `created_at`.
    #[must_use]
    pub fn set_created_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: impl Into<String>) -> Self {
        self.duration = Some(value.into());
        self
    }

    ///Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<String>) -> Self {
        self.duration = value;
        self
    }

    ///Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_created_at = Some(value);
        self
    }

    ///Sets or clears `file_created_at`.
    #[must_use]
    pub fn set_file_created_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.file_created_at = value;
        self
    }

    ///Sets `file_modified_at`.
    #[must_use]
    pub fn file_modified_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_modified_at = Some(value);
        self
    }

    ///Sets or clears `file_modified_at`.
    #[must_use]
    pub fn set_file_modified_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.file_modified_at = value;
        self
    }

    ///Sets `height`.
    #[must_use]
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    ///Sets or clears `height`.
    #[must_use]
    pub fn set_height(mut self, value: Option<i64>) -> Self {
        self.height = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_edited`.
    #[must_use]
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: impl Into<String>) -> Self {
        self.library_id = Some(value.into());
        self
    }

    ///Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<String>) -> Self {
        self.library_id = value;
        self
    }

    ///Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: impl Into<String>) -> Self {
        self.live_photo_video_id = Some(value.into());
        self
    }

    ///Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<String>) -> Self {
        self.live_photo_video_id = value;
        self
    }

    ///Sets `local_date_time`.
    #[must_use]
    pub fn local_date_time(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.local_date_time = Some(value);
        self
    }

    ///Sets or clears `local_date_time`.
    #[must_use]
    pub fn set_local_date_time(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.local_date_time = value;
        self
    }

    ///Sets `original_file_name`.
    #[must_use]
    pub fn original_file_name(mut self, value: impl Into<String>) -> Self {
        self.original_file_name = value.into();
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `stack_id`.
    #[must_use]
    pub fn stack_id(mut self, value: impl Into<String>) -> Self {
        self.stack_id = Some(value.into());
        self
    }

    ///Sets or clears `stack_id`.
    #[must_use]
    pub fn set_stack_id(mut self, value: Option<String>) -> Self {
        self.stack_id = value;
        self
    }

    ///Sets `thumbhash`.
    #[must_use]
    pub fn thumbhash(mut self, value: impl Into<String>) -> Self {
        self.thumbhash = Some(value.into());
        self
    }

    ///Sets or clears `thumbhash`.
    #[must_use]
    pub fn set_thumbhash(mut self, value: Option<String>) -> Self {
        self.thumbhash = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = value;
        self
    }

    ///Sets `width`.
    #[must_use]
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    ///Sets or clears `width`.
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
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Uploaded to Immich at |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deleted at |
/// | `duration` | `i64` | ✓ | Duration |
/// | `file_created_at` | [`DateTime`](::chrono::DateTime) | ✓ | File created at |
/// | `file_modified_at` | [`DateTime`](::chrono::DateTime) | ✓ | File modified at |
/// | `height` | `i64` | ✓ | Asset height |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `is_edited` | `bool` | ✓ | Is edited |
/// | `is_favorite` | `bool` | ✓ | Is favorite |
/// | `library_id` | [`String`] | ✓ | Library ID |
/// | `live_photo_video_id` | [`String`] | ✓ | Live photo video ID |
/// | `local_date_time` | [`DateTime`](::chrono::DateTime) | ✓ | Local date time |
/// | `original_file_name` | [`String`] | ✓ | Original file name |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner ID |
/// | `stack_id` | [`String`] | ✓ | Stack ID |
/// | `thumbhash` | [`String`] | ✓ | Thumbhash |
/// | `type` | [`AssetTypeEnum`] | ✓ |  |
/// | `visibility` | [`AssetVisibility`] | ✓ |  |
/// | `width` | `i64` | ✓ | Asset width |
///
/// See: <https://api.immich.app/models/SyncAssetV2>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAssetV2 {
    pub checksum: String,
    /// Uploaded to Immich at (required)
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Deleted at (required)
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// File created at (required)
    #[serde(rename = "fileCreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// File modified at (required)
    #[serde(rename = "fileModifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_modified_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    pub id: ::uuid::Uuid,
    /// Is edited (required)
    #[serde(rename = "isEdited")]
    pub is_edited: bool,
    /// Is favorite (required)
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    /// Library ID (required)
    #[serde(rename = "libraryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_id: Option<String>,
    /// Live photo video ID (required)
    #[serde(rename = "livePhotoVideoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_photo_video_id: Option<String>,
    /// Local date time (required)
    #[serde(rename = "localDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_date_time: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Original file name (required)
    #[serde(rename = "originalFileName")]
    pub original_file_name: String,
    /// Owner ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    /// Stack ID (required)
    #[serde(rename = "stackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbhash: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: AssetTypeEnum,
    pub visibility: AssetVisibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl SyncAssetV2 {
    ///Creates a new [`SyncAssetV2`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        checksum: impl Into<String>,
        id: ::uuid::Uuid,
        is_edited: bool,
        is_favorite: bool,
        original_file_name: impl Into<String>,
        owner_id: ::uuid::Uuid,
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
            id,
            is_edited,
            is_favorite,
            library_id: None,
            live_photo_video_id: None,
            local_date_time: None,
            original_file_name: original_file_name.into(),
            owner_id,
            stack_id: None,
            thumbhash: None,
            type_,
            visibility,
            width: None,
        }
    }

    ///Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = value.into();
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    ///Sets or clears `created_at`.
    #[must_use]
    pub fn set_created_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    ///Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<i64>) -> Self {
        self.duration = value;
        self
    }

    ///Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_created_at = Some(value);
        self
    }

    ///Sets or clears `file_created_at`.
    #[must_use]
    pub fn set_file_created_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.file_created_at = value;
        self
    }

    ///Sets `file_modified_at`.
    #[must_use]
    pub fn file_modified_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_modified_at = Some(value);
        self
    }

    ///Sets or clears `file_modified_at`.
    #[must_use]
    pub fn set_file_modified_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.file_modified_at = value;
        self
    }

    ///Sets `height`.
    #[must_use]
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    ///Sets or clears `height`.
    #[must_use]
    pub fn set_height(mut self, value: Option<i64>) -> Self {
        self.height = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_edited`.
    #[must_use]
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: impl Into<String>) -> Self {
        self.library_id = Some(value.into());
        self
    }

    ///Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<String>) -> Self {
        self.library_id = value;
        self
    }

    ///Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: impl Into<String>) -> Self {
        self.live_photo_video_id = Some(value.into());
        self
    }

    ///Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<String>) -> Self {
        self.live_photo_video_id = value;
        self
    }

    ///Sets `local_date_time`.
    #[must_use]
    pub fn local_date_time(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.local_date_time = Some(value);
        self
    }

    ///Sets or clears `local_date_time`.
    #[must_use]
    pub fn set_local_date_time(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.local_date_time = value;
        self
    }

    ///Sets `original_file_name`.
    #[must_use]
    pub fn original_file_name(mut self, value: impl Into<String>) -> Self {
        self.original_file_name = value.into();
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `stack_id`.
    #[must_use]
    pub fn stack_id(mut self, value: impl Into<String>) -> Self {
        self.stack_id = Some(value.into());
        self
    }

    ///Sets or clears `stack_id`.
    #[must_use]
    pub fn set_stack_id(mut self, value: Option<String>) -> Self {
        self.stack_id = value;
        self
    }

    ///Sets `thumbhash`.
    #[must_use]
    pub fn thumbhash(mut self, value: impl Into<String>) -> Self {
        self.thumbhash = Some(value.into());
        self
    }

    ///Sets or clears `thumbhash`.
    #[must_use]
    pub fn set_thumbhash(mut self, value: Option<String>) -> Self {
        self.thumbhash = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = value;
        self
    }

    ///Sets `width`.
    #[must_use]
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    ///Sets or clears `width`.
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
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | User deleted at |
/// | `email` | [`String`] | ✓ | User email |
/// | `has_profile_image` | `bool` | ✓ | User has profile image |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
/// | `is_admin` | `bool` | ✓ | User is admin |
/// | `name` | [`String`] | ✓ | User name |
/// | `oauth_id` | [`String`] | ✓ | User OAuth ID |
/// | `pin_code` | [`String`] | ✓ | User pin code |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | User profile changed at |
/// | `quota_size_in_bytes` | `i64` | ✓ | Quota size in bytes |
/// | `quota_usage_in_bytes` | `i64` | ✓ | Quota usage in bytes |
/// | `storage_label` | [`String`] | ✓ | User storage label |
///
/// See: <https://api.immich.app/models/SyncAuthUserV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAuthUserV1 {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    /// User deleted at (required)
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub email: String,
    /// User has profile image (required)
    #[serde(rename = "hasProfileImage")]
    pub has_profile_image: bool,
    pub id: ::uuid::Uuid,
    /// User is admin (required)
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    pub name: String,
    /// User OAuth ID (required)
    #[serde(rename = "oauthId")]
    pub oauth_id: String,
    /// User pin code (required)
    #[serde(rename = "pinCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_code: Option<String>,
    /// User profile changed at (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Quota size in bytes (required)
    #[serde(rename = "quotaSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_size_in_bytes: Option<i64>,
    /// Quota usage in bytes (required)
    #[serde(rename = "quotaUsageInBytes")]
    pub quota_usage_in_bytes: i64,
    /// User storage label (required)
    #[serde(rename = "storageLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_label: Option<String>,
}

impl SyncAuthUserV1 {
    ///Creates a new [`SyncAuthUserV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        email: impl Into<String>,
        has_profile_image: bool,
        id: ::uuid::Uuid,
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
            id,
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

    ///Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }

    ///Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }

    ///Sets `has_profile_image`.
    #[must_use]
    pub fn has_profile_image(mut self, value: bool) -> Self {
        self.has_profile_image = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `oauth_id`.
    #[must_use]
    pub fn oauth_id(mut self, value: impl Into<String>) -> Self {
        self.oauth_id = value.into();
        self
    }

    ///Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = Some(value.into());
        self
    }

    ///Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<String>) -> Self {
        self.pin_code = value;
        self
    }

    ///Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }

    ///Sets `quota_size_in_bytes`.
    #[must_use]
    pub fn quota_size_in_bytes(mut self, value: i64) -> Self {
        self.quota_size_in_bytes = Some(value);
        self
    }

    ///Sets or clears `quota_size_in_bytes`.
    #[must_use]
    pub fn set_quota_size_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_size_in_bytes = value;
        self
    }

    ///Sets `quota_usage_in_bytes`.
    #[must_use]
    pub fn quota_usage_in_bytes(mut self, value: i64) -> Self {
        self.quota_usage_in_bytes = value;
        self
    }

    ///Sets `storage_label`.
    #[must_use]
    pub fn storage_label(mut self, value: impl Into<String>) -> Self {
        self.storage_label = Some(value.into());
        self
    }

    ///Sets or clears `storage_label`.
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
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | User deleted at |
/// | `email` | [`String`] | ✓ | User email |
/// | `has_profile_image` | `bool` | ✓ | User has profile image |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
/// | `is_admin` | `bool` | ✓ | User is admin |
/// | `name` | [`String`] | ✓ | User name |
/// | `oauth_id` | [`String`] | ✓ | User OAuth ID |
/// | `pin_code` | [`String`] | ✓ | User pin code |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | User profile changed at |
/// | `quota_size_in_bytes` | `i64` | ✓ | Quota size in bytes |
/// | `quota_usage_in_bytes` | `i64` | ✓ | Quota usage in bytes |
/// | `storage_label` | [`String`] | ✓ | User storage label |
///
/// See: <https://api.immich.app/models/SyncAuthUserV2>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncAuthUserV2 {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    /// User deleted at (required)
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub email: String,
    /// User has profile image (required)
    #[serde(rename = "hasProfileImage")]
    pub has_profile_image: bool,
    pub id: ::uuid::Uuid,
    /// User is admin (required)
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    pub name: String,
    /// User OAuth ID (required)
    #[serde(rename = "oauthId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_id: Option<String>,
    /// User pin code (required)
    #[serde(rename = "pinCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_code: Option<String>,
    /// User profile changed at (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Quota size in bytes (required)
    #[serde(rename = "quotaSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_size_in_bytes: Option<i64>,
    /// Quota usage in bytes (required)
    #[serde(rename = "quotaUsageInBytes")]
    pub quota_usage_in_bytes: i64,
    /// User storage label (required)
    #[serde(rename = "storageLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_label: Option<String>,
}

impl SyncAuthUserV2 {
    ///Creates a new [`SyncAuthUserV2`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        email: impl Into<String>,
        has_profile_image: bool,
        id: ::uuid::Uuid,
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
            id,
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

    ///Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }

    ///Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }

    ///Sets `has_profile_image`.
    #[must_use]
    pub fn has_profile_image(mut self, value: bool) -> Self {
        self.has_profile_image = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_admin`.
    #[must_use]
    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `oauth_id`.
    #[must_use]
    pub fn oauth_id(mut self, value: impl Into<String>) -> Self {
        self.oauth_id = Some(value.into());
        self
    }

    ///Sets or clears `oauth_id`.
    #[must_use]
    pub fn set_oauth_id(mut self, value: Option<String>) -> Self {
        self.oauth_id = value;
        self
    }

    ///Sets `pin_code`.
    #[must_use]
    pub fn pin_code(mut self, value: impl Into<String>) -> Self {
        self.pin_code = Some(value.into());
        self
    }

    ///Sets or clears `pin_code`.
    #[must_use]
    pub fn set_pin_code(mut self, value: Option<String>) -> Self {
        self.pin_code = value;
        self
    }

    ///Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }

    ///Sets `quota_size_in_bytes`.
    #[must_use]
    pub fn quota_size_in_bytes(mut self, value: i64) -> Self {
        self.quota_size_in_bytes = Some(value);
        self
    }

    ///Sets or clears `quota_size_in_bytes`.
    #[must_use]
    pub fn set_quota_size_in_bytes(mut self, value: Option<i64>) -> Self {
        self.quota_size_in_bytes = value;
        self
    }

    ///Sets `quota_usage_in_bytes`.
    #[must_use]
    pub fn quota_usage_in_bytes(mut self, value: i64) -> Self {
        self.quota_usage_in_bytes = value;
        self
    }

    ///Sets `storage_label`.
    #[must_use]
    pub fn storage_label(mut self, value: impl Into<String>) -> Self {
        self.storage_label = Some(value.into());
        self
    }

    ///Sets or clears `storage_label`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for SyncEntityType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AuthUserV1 => f.write_str("AuthUserV1"),
            Self::AuthUserV2 => f.write_str("AuthUserV2"),
            Self::UserV1 => f.write_str("UserV1"),
            Self::UserDeleteV1 => f.write_str("UserDeleteV1"),
            Self::AssetV1 => f.write_str("AssetV1"),
            Self::AssetV2 => f.write_str("AssetV2"),
            Self::AssetDeleteV1 => f.write_str("AssetDeleteV1"),
            Self::AssetExifV1 => f.write_str("AssetExifV1"),
            Self::AssetEditV1 => f.write_str("AssetEditV1"),
            Self::AssetEditDeleteV1 => f.write_str("AssetEditDeleteV1"),
            Self::AssetMetadataV1 => f.write_str("AssetMetadataV1"),
            Self::AssetMetadataDeleteV1 => f.write_str("AssetMetadataDeleteV1"),
            Self::AssetOcrV1 => f.write_str("AssetOcrV1"),
            Self::AssetOcrDeleteV1 => f.write_str("AssetOcrDeleteV1"),
            Self::PartnerV1 => f.write_str("PartnerV1"),
            Self::PartnerDeleteV1 => f.write_str("PartnerDeleteV1"),
            Self::PartnerAssetV1 => f.write_str("PartnerAssetV1"),
            Self::PartnerAssetV2 => f.write_str("PartnerAssetV2"),
            Self::PartnerAssetBackfillV1 => f.write_str("PartnerAssetBackfillV1"),
            Self::PartnerAssetBackfillV2 => f.write_str("PartnerAssetBackfillV2"),
            Self::PartnerAssetDeleteV1 => f.write_str("PartnerAssetDeleteV1"),
            Self::PartnerAssetExifV1 => f.write_str("PartnerAssetExifV1"),
            Self::PartnerAssetExifBackfillV1 => f.write_str("PartnerAssetExifBackfillV1"),
            Self::PartnerStackBackfillV1 => f.write_str("PartnerStackBackfillV1"),
            Self::PartnerStackDeleteV1 => f.write_str("PartnerStackDeleteV1"),
            Self::PartnerStackV1 => f.write_str("PartnerStackV1"),
            Self::AlbumV1 => f.write_str("AlbumV1"),
            Self::AlbumV2 => f.write_str("AlbumV2"),
            Self::AlbumDeleteV1 => f.write_str("AlbumDeleteV1"),
            Self::AlbumUserV1 => f.write_str("AlbumUserV1"),
            Self::AlbumUserBackfillV1 => f.write_str("AlbumUserBackfillV1"),
            Self::AlbumUserDeleteV1 => f.write_str("AlbumUserDeleteV1"),
            Self::AlbumAssetCreateV1 => f.write_str("AlbumAssetCreateV1"),
            Self::AlbumAssetCreateV2 => f.write_str("AlbumAssetCreateV2"),
            Self::AlbumAssetUpdateV1 => f.write_str("AlbumAssetUpdateV1"),
            Self::AlbumAssetUpdateV2 => f.write_str("AlbumAssetUpdateV2"),
            Self::AlbumAssetBackfillV1 => f.write_str("AlbumAssetBackfillV1"),
            Self::AlbumAssetBackfillV2 => f.write_str("AlbumAssetBackfillV2"),
            Self::AlbumAssetExifCreateV1 => f.write_str("AlbumAssetExifCreateV1"),
            Self::AlbumAssetExifUpdateV1 => f.write_str("AlbumAssetExifUpdateV1"),
            Self::AlbumAssetExifBackfillV1 => f.write_str("AlbumAssetExifBackfillV1"),
            Self::AlbumToAssetV1 => f.write_str("AlbumToAssetV1"),
            Self::AlbumToAssetDeleteV1 => f.write_str("AlbumToAssetDeleteV1"),
            Self::AlbumToAssetBackfillV1 => f.write_str("AlbumToAssetBackfillV1"),
            Self::MemoryV1 => f.write_str("MemoryV1"),
            Self::MemoryV2 => f.write_str("MemoryV2"),
            Self::MemoryDeleteV1 => f.write_str("MemoryDeleteV1"),
            Self::MemoryToAssetV1 => f.write_str("MemoryToAssetV1"),
            Self::MemoryToAssetV2 => f.write_str("MemoryToAssetV2"),
            Self::MemoryToAssetDeleteV1 => f.write_str("MemoryToAssetDeleteV1"),
            Self::StackV1 => f.write_str("StackV1"),
            Self::StackDeleteV1 => f.write_str("StackDeleteV1"),
            Self::PersonV1 => f.write_str("PersonV1"),
            Self::PersonDeleteV1 => f.write_str("PersonDeleteV1"),
            Self::AssetFaceV1 => f.write_str("AssetFaceV1"),
            Self::AssetFaceV2 => f.write_str("AssetFaceV2"),
            Self::AssetFaceV3 => f.write_str("AssetFaceV3"),
            Self::AssetFaceDeleteV1 => f.write_str("AssetFaceDeleteV1"),
            Self::UserMetadataV1 => f.write_str("UserMetadataV1"),
            Self::UserMetadataDeleteV1 => f.write_str("UserMetadataDeleteV1"),
            Self::SyncAckV1 => f.write_str("SyncAckV1"),
            Self::SyncResetV1 => f.write_str("SyncResetV1"),
            Self::SyncCompleteV1 => f.write_str("SyncCompleteV1"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for SyncEntityType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "AuthUserV1" => Ok(Self::AuthUserV1),
            "AuthUserV2" => Ok(Self::AuthUserV2),
            "UserV1" => Ok(Self::UserV1),
            "UserDeleteV1" => Ok(Self::UserDeleteV1),
            "AssetV1" => Ok(Self::AssetV1),
            "AssetV2" => Ok(Self::AssetV2),
            "AssetDeleteV1" => Ok(Self::AssetDeleteV1),
            "AssetExifV1" => Ok(Self::AssetExifV1),
            "AssetEditV1" => Ok(Self::AssetEditV1),
            "AssetEditDeleteV1" => Ok(Self::AssetEditDeleteV1),
            "AssetMetadataV1" => Ok(Self::AssetMetadataV1),
            "AssetMetadataDeleteV1" => Ok(Self::AssetMetadataDeleteV1),
            "AssetOcrV1" => Ok(Self::AssetOcrV1),
            "AssetOcrDeleteV1" => Ok(Self::AssetOcrDeleteV1),
            "PartnerV1" => Ok(Self::PartnerV1),
            "PartnerDeleteV1" => Ok(Self::PartnerDeleteV1),
            "PartnerAssetV1" => Ok(Self::PartnerAssetV1),
            "PartnerAssetV2" => Ok(Self::PartnerAssetV2),
            "PartnerAssetBackfillV1" => Ok(Self::PartnerAssetBackfillV1),
            "PartnerAssetBackfillV2" => Ok(Self::PartnerAssetBackfillV2),
            "PartnerAssetDeleteV1" => Ok(Self::PartnerAssetDeleteV1),
            "PartnerAssetExifV1" => Ok(Self::PartnerAssetExifV1),
            "PartnerAssetExifBackfillV1" => Ok(Self::PartnerAssetExifBackfillV1),
            "PartnerStackBackfillV1" => Ok(Self::PartnerStackBackfillV1),
            "PartnerStackDeleteV1" => Ok(Self::PartnerStackDeleteV1),
            "PartnerStackV1" => Ok(Self::PartnerStackV1),
            "AlbumV1" => Ok(Self::AlbumV1),
            "AlbumV2" => Ok(Self::AlbumV2),
            "AlbumDeleteV1" => Ok(Self::AlbumDeleteV1),
            "AlbumUserV1" => Ok(Self::AlbumUserV1),
            "AlbumUserBackfillV1" => Ok(Self::AlbumUserBackfillV1),
            "AlbumUserDeleteV1" => Ok(Self::AlbumUserDeleteV1),
            "AlbumAssetCreateV1" => Ok(Self::AlbumAssetCreateV1),
            "AlbumAssetCreateV2" => Ok(Self::AlbumAssetCreateV2),
            "AlbumAssetUpdateV1" => Ok(Self::AlbumAssetUpdateV1),
            "AlbumAssetUpdateV2" => Ok(Self::AlbumAssetUpdateV2),
            "AlbumAssetBackfillV1" => Ok(Self::AlbumAssetBackfillV1),
            "AlbumAssetBackfillV2" => Ok(Self::AlbumAssetBackfillV2),
            "AlbumAssetExifCreateV1" => Ok(Self::AlbumAssetExifCreateV1),
            "AlbumAssetExifUpdateV1" => Ok(Self::AlbumAssetExifUpdateV1),
            "AlbumAssetExifBackfillV1" => Ok(Self::AlbumAssetExifBackfillV1),
            "AlbumToAssetV1" => Ok(Self::AlbumToAssetV1),
            "AlbumToAssetDeleteV1" => Ok(Self::AlbumToAssetDeleteV1),
            "AlbumToAssetBackfillV1" => Ok(Self::AlbumToAssetBackfillV1),
            "MemoryV1" => Ok(Self::MemoryV1),
            "MemoryV2" => Ok(Self::MemoryV2),
            "MemoryDeleteV1" => Ok(Self::MemoryDeleteV1),
            "MemoryToAssetV1" => Ok(Self::MemoryToAssetV1),
            "MemoryToAssetV2" => Ok(Self::MemoryToAssetV2),
            "MemoryToAssetDeleteV1" => Ok(Self::MemoryToAssetDeleteV1),
            "StackV1" => Ok(Self::StackV1),
            "StackDeleteV1" => Ok(Self::StackDeleteV1),
            "PersonV1" => Ok(Self::PersonV1),
            "PersonDeleteV1" => Ok(Self::PersonDeleteV1),
            "AssetFaceV1" => Ok(Self::AssetFaceV1),
            "AssetFaceV2" => Ok(Self::AssetFaceV2),
            "AssetFaceV3" => Ok(Self::AssetFaceV3),
            "AssetFaceDeleteV1" => Ok(Self::AssetFaceDeleteV1),
            "UserMetadataV1" => Ok(Self::UserMetadataV1),
            "UserMetadataDeleteV1" => Ok(Self::UserMetadataDeleteV1),
            "SyncAckV1" => Ok(Self::SyncAckV1),
            "SyncResetV1" => Ok(Self::SyncResetV1),
            "SyncCompleteV1" => Ok(Self::SyncCompleteV1),
            _ => Err(format!("invalid value for {}: {}", "SyncEntityType", value).into()),
        }
    }
}

impl TryFrom<&str> for SyncEntityType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SyncEntityType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SyncEntityType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `SyncMemoryAssetDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `memory_id` | [`Uuid`](::uuid::Uuid) | ✓ | Memory ID |
///
/// See: <https://api.immich.app/models/SyncMemoryAssetDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncMemoryAssetDeleteV1 {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    /// Memory ID (required)
    #[serde(rename = "memoryId")]
    pub memory_id: ::uuid::Uuid,
}

impl SyncMemoryAssetDeleteV1 {
    ///Creates a new [`SyncMemoryAssetDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: ::uuid::Uuid, memory_id: ::uuid::Uuid) -> Self {
        Self { asset_id, memory_id }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `memory_id`.
    #[must_use]
    pub fn memory_id(mut self, value: ::uuid::Uuid) -> Self {
        self.memory_id = value;
        self
    }
}

/// `SyncMemoryAssetV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `memory_id` | [`Uuid`](::uuid::Uuid) | ✓ | Memory ID |
///
/// See: <https://api.immich.app/models/SyncMemoryAssetV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncMemoryAssetV1 {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    /// Memory ID (required)
    #[serde(rename = "memoryId")]
    pub memory_id: ::uuid::Uuid,
}

impl SyncMemoryAssetV1 {
    ///Creates a new [`SyncMemoryAssetV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: ::uuid::Uuid, memory_id: ::uuid::Uuid) -> Self {
        Self { asset_id, memory_id }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `memory_id`.
    #[must_use]
    pub fn memory_id(mut self, value: ::uuid::Uuid) -> Self {
        self.memory_id = value;
        self
    }
}

/// `SyncMemoryDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `memory_id` | [`Uuid`](::uuid::Uuid) | ✓ | Memory ID |
///
/// See: <https://api.immich.app/models/SyncMemoryDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncMemoryDeleteV1 {
    /// Memory ID (required)
    #[serde(rename = "memoryId")]
    pub memory_id: ::uuid::Uuid,
}

impl SyncMemoryDeleteV1 {
    ///Creates a new [`SyncMemoryDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(memory_id: ::uuid::Uuid) -> Self {
        Self { memory_id }
    }

    ///Sets `memory_id`.
    #[must_use]
    pub fn memory_id(mut self, value: ::uuid::Uuid) -> Self {
        self.memory_id = value;
        self
    }
}

/// `SyncMemoryV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at |
/// | `data` | `object` | ✓ | Data |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deleted at |
/// | `hide_at` | [`DateTime`](::chrono::DateTime) | ✓ | Hide at |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Memory ID |
/// | `is_saved` | `bool` | ✓ | Is saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) | ✓ | Memory at |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner ID |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) | ✓ | Seen at |
/// | `show_at` | [`DateTime`](::chrono::DateTime) | ✓ | Show at |
/// | `type` | [`MemoryTypeV1`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at |
///
/// See: <https://api.immich.app/models/SyncMemoryV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncMemoryV1 {
    /// Created at (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    /// Deleted at (required)
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Hide at (required)
    #[serde(rename = "hideAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub id: ::uuid::Uuid,
    /// Is saved (required)
    #[serde(rename = "isSaved")]
    pub is_saved: bool,
    /// Memory at (required)
    #[serde(rename = "memoryAt")]
    pub memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Owner ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    /// Seen at (required)
    #[serde(rename = "seenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Show at (required)
    #[serde(rename = "showAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: MemoryTypeV1,
    /// Updated at (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncMemoryV1 {
    ///Creates a new [`SyncMemoryV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        data: ::serde_json::Map<String, ::serde_json::Value>,
        id: ::uuid::Uuid,
        is_saved: bool,
        memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
        owner_id: ::uuid::Uuid,
        type_: MemoryTypeV1,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            data,
            deleted_at: None,
            hide_at: None,
            id,
            is_saved,
            memory_at,
            owner_id,
            seen_at: None,
            show_at: None,
            type_,
            updated_at,
        }
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `data`.
    #[must_use]
    pub fn data(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.data = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `hide_at`.
    #[must_use]
    pub fn hide_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.hide_at = Some(value);
        self
    }

    ///Sets or clears `hide_at`.
    #[must_use]
    pub fn set_hide_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.hide_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = value;
        self
    }

    ///Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = value;
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }

    ///Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }

    ///Sets `show_at`.
    #[must_use]
    pub fn show_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.show_at = Some(value);
        self
    }

    ///Sets or clears `show_at`.
    #[must_use]
    pub fn set_show_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.show_at = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: MemoryTypeV1) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_at`.
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
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at |
/// | `data` | `object` | ✓ | Data |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | Deleted at |
/// | `hide_at` | [`DateTime`](::chrono::DateTime) | ✓ | Hide at |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Memory ID |
/// | `is_saved` | `bool` | ✓ | Is saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) | ✓ | Memory at |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner ID |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) | ✓ | Seen at |
/// | `show_at` | [`DateTime`](::chrono::DateTime) | ✓ | Show at |
/// | `type` | [`MemoryType`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at |
///
/// See: <https://api.immich.app/models/SyncMemoryV2>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncMemoryV2 {
    /// Created at (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    /// Deleted at (required)
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Hide at (required)
    #[serde(rename = "hideAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub id: ::uuid::Uuid,
    /// Is saved (required)
    #[serde(rename = "isSaved")]
    pub is_saved: bool,
    /// Memory at (required)
    #[serde(rename = "memoryAt")]
    pub memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Owner ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    /// Seen at (required)
    #[serde(rename = "seenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Show at (required)
    #[serde(rename = "showAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: MemoryType,
    /// Updated at (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncMemoryV2 {
    ///Creates a new [`SyncMemoryV2`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        data: ::serde_json::Map<String, ::serde_json::Value>,
        id: ::uuid::Uuid,
        is_saved: bool,
        memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
        owner_id: ::uuid::Uuid,
        type_: MemoryType,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            data,
            deleted_at: None,
            hide_at: None,
            id,
            is_saved,
            memory_at,
            owner_id,
            seen_at: None,
            show_at: None,
            type_,
            updated_at,
        }
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `data`.
    #[must_use]
    pub fn data(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.data = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `hide_at`.
    #[must_use]
    pub fn hide_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.hide_at = Some(value);
        self
    }

    ///Sets or clears `hide_at`.
    #[must_use]
    pub fn set_hide_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.hide_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = value;
        self
    }

    ///Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = value;
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }

    ///Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }

    ///Sets `show_at`.
    #[must_use]
    pub fn show_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.show_at = Some(value);
        self
    }

    ///Sets or clears `show_at`.
    #[must_use]
    pub fn set_show_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.show_at = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: MemoryType) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_at`.
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
/// | `shared_by_id` | [`Uuid`](::uuid::Uuid) | ✓ | Shared by ID |
/// | `shared_with_id` | [`Uuid`](::uuid::Uuid) | ✓ | Shared with ID |
///
/// See: <https://api.immich.app/models/SyncPartnerDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncPartnerDeleteV1 {
    /// Shared by ID (required)
    #[serde(rename = "sharedById")]
    pub shared_by_id: ::uuid::Uuid,
    /// Shared with ID (required)
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: ::uuid::Uuid,
}

impl SyncPartnerDeleteV1 {
    ///Creates a new [`SyncPartnerDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(shared_by_id: ::uuid::Uuid, shared_with_id: ::uuid::Uuid) -> Self {
        Self {
            shared_by_id,
            shared_with_id,
        }
    }

    ///Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_by_id = value;
        self
    }

    ///Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_with_id = value;
        self
    }
}

/// `SyncPartnerV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `in_timeline` | `bool` | ✓ | In timeline |
/// | `shared_by_id` | [`Uuid`](::uuid::Uuid) | ✓ | Shared by ID |
/// | `shared_with_id` | [`Uuid`](::uuid::Uuid) | ✓ | Shared with ID |
///
/// See: <https://api.immich.app/models/SyncPartnerV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncPartnerV1 {
    /// In timeline (required)
    #[serde(rename = "inTimeline")]
    pub in_timeline: bool,
    /// Shared by ID (required)
    #[serde(rename = "sharedById")]
    pub shared_by_id: ::uuid::Uuid,
    /// Shared with ID (required)
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: ::uuid::Uuid,
}

impl SyncPartnerV1 {
    ///Creates a new [`SyncPartnerV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(in_timeline: bool, shared_by_id: ::uuid::Uuid, shared_with_id: ::uuid::Uuid) -> Self {
        Self {
            in_timeline,
            shared_by_id,
            shared_with_id,
        }
    }

    ///Sets `in_timeline`.
    #[must_use]
    pub fn in_timeline(mut self, value: bool) -> Self {
        self.in_timeline = value;
        self
    }

    ///Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_by_id = value;
        self
    }

    ///Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_with_id = value;
        self
    }
}

/// `SyncPersonDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `person_id` | [`Uuid`](::uuid::Uuid) | ✓ | Person ID |
///
/// See: <https://api.immich.app/models/SyncPersonDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncPersonDeleteV1 {
    /// Person ID (required)
    #[serde(rename = "personId")]
    pub person_id: ::uuid::Uuid,
}

impl SyncPersonDeleteV1 {
    ///Creates a new [`SyncPersonDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(person_id: ::uuid::Uuid) -> Self {
        Self { person_id }
    }

    ///Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: ::uuid::Uuid) -> Self {
        self.person_id = value;
        self
    }
}

/// `SyncPersonV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`DateTime`](::chrono::DateTime) | ✓ | Birth date |
/// | `color` | [`String`] | ✓ | Color |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at |
/// | `face_asset_id` | [`String`] | ✓ | Face asset ID |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Person ID |
/// | `is_favorite` | `bool` | ✓ | Is favorite |
/// | `is_hidden` | `bool` | ✓ | Is hidden |
/// | `name` | [`String`] | ✓ | Person name |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner ID |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at |
///
/// See: <https://api.immich.app/models/SyncPersonV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncPersonV1 {
    /// Birth date (required)
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Created at (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Face asset ID (required)
    #[serde(rename = "faceAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_asset_id: Option<String>,
    pub id: ::uuid::Uuid,
    /// Is favorite (required)
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    /// Is hidden (required)
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    pub name: String,
    /// Owner ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    /// Updated at (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncPersonV1 {
    ///Creates a new [`SyncPersonV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        is_favorite: bool,
        is_hidden: bool,
        name: impl Into<String>,
        owner_id: ::uuid::Uuid,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            birth_date: None,
            color: None,
            created_at,
            face_asset_id: None,
            id,
            is_favorite,
            is_hidden,
            name: name.into(),
            owner_id,
            updated_at,
        }
    }

    ///Sets `birth_date`.
    #[must_use]
    pub fn birth_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.birth_date = Some(value);
        self
    }

    ///Sets or clears `birth_date`.
    #[must_use]
    pub fn set_birth_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.birth_date = value;
        self
    }

    ///Sets `color`.
    #[must_use]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    ///Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<String>) -> Self {
        self.color = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `face_asset_id`.
    #[must_use]
    pub fn face_asset_id(mut self, value: impl Into<String>) -> Self {
        self.face_asset_id = Some(value.into());
        self
    }

    ///Sets or clears `face_asset_id`.
    #[must_use]
    pub fn set_face_asset_id(mut self, value: Option<String>) -> Self {
        self.face_asset_id = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `is_hidden`.
    #[must_use]
    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// Sync request type
///
/// See: <https://api.immich.app/models/SyncRequestType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for SyncRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AlbumsV1 => f.write_str("AlbumsV1"),
            Self::AlbumsV2 => f.write_str("AlbumsV2"),
            Self::AlbumUsersV1 => f.write_str("AlbumUsersV1"),
            Self::AlbumToAssetsV1 => f.write_str("AlbumToAssetsV1"),
            Self::AlbumAssetsV1 => f.write_str("AlbumAssetsV1"),
            Self::AlbumAssetsV2 => f.write_str("AlbumAssetsV2"),
            Self::AlbumAssetExifsV1 => f.write_str("AlbumAssetExifsV1"),
            Self::AssetsV1 => f.write_str("AssetsV1"),
            Self::AssetsV2 => f.write_str("AssetsV2"),
            Self::AssetExifsV1 => f.write_str("AssetExifsV1"),
            Self::AssetEditsV1 => f.write_str("AssetEditsV1"),
            Self::AssetMetadataV1 => f.write_str("AssetMetadataV1"),
            Self::AssetOcrV1 => f.write_str("AssetOcrV1"),
            Self::AuthUsersV1 => f.write_str("AuthUsersV1"),
            Self::AuthUsersV2 => f.write_str("AuthUsersV2"),
            Self::MemoriesV1 => f.write_str("MemoriesV1"),
            Self::MemoriesV2 => f.write_str("MemoriesV2"),
            Self::MemoryToAssetsV1 => f.write_str("MemoryToAssetsV1"),
            Self::MemoryToAssetsV2 => f.write_str("MemoryToAssetsV2"),
            Self::PartnersV1 => f.write_str("PartnersV1"),
            Self::PartnerAssetsV1 => f.write_str("PartnerAssetsV1"),
            Self::PartnerAssetsV2 => f.write_str("PartnerAssetsV2"),
            Self::PartnerAssetExifsV1 => f.write_str("PartnerAssetExifsV1"),
            Self::PartnerStacksV1 => f.write_str("PartnerStacksV1"),
            Self::StacksV1 => f.write_str("StacksV1"),
            Self::UsersV1 => f.write_str("UsersV1"),
            Self::PeopleV1 => f.write_str("PeopleV1"),
            Self::AssetFacesV1 => f.write_str("AssetFacesV1"),
            Self::AssetFacesV2 => f.write_str("AssetFacesV2"),
            Self::AssetFacesV3 => f.write_str("AssetFacesV3"),
            Self::UserMetadataV1 => f.write_str("UserMetadataV1"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for SyncRequestType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "AlbumsV1" => Ok(Self::AlbumsV1),
            "AlbumsV2" => Ok(Self::AlbumsV2),
            "AlbumUsersV1" => Ok(Self::AlbumUsersV1),
            "AlbumToAssetsV1" => Ok(Self::AlbumToAssetsV1),
            "AlbumAssetsV1" => Ok(Self::AlbumAssetsV1),
            "AlbumAssetsV2" => Ok(Self::AlbumAssetsV2),
            "AlbumAssetExifsV1" => Ok(Self::AlbumAssetExifsV1),
            "AssetsV1" => Ok(Self::AssetsV1),
            "AssetsV2" => Ok(Self::AssetsV2),
            "AssetExifsV1" => Ok(Self::AssetExifsV1),
            "AssetEditsV1" => Ok(Self::AssetEditsV1),
            "AssetMetadataV1" => Ok(Self::AssetMetadataV1),
            "AssetOcrV1" => Ok(Self::AssetOcrV1),
            "AuthUsersV1" => Ok(Self::AuthUsersV1),
            "AuthUsersV2" => Ok(Self::AuthUsersV2),
            "MemoriesV1" => Ok(Self::MemoriesV1),
            "MemoriesV2" => Ok(Self::MemoriesV2),
            "MemoryToAssetsV1" => Ok(Self::MemoryToAssetsV1),
            "MemoryToAssetsV2" => Ok(Self::MemoryToAssetsV2),
            "PartnersV1" => Ok(Self::PartnersV1),
            "PartnerAssetsV1" => Ok(Self::PartnerAssetsV1),
            "PartnerAssetsV2" => Ok(Self::PartnerAssetsV2),
            "PartnerAssetExifsV1" => Ok(Self::PartnerAssetExifsV1),
            "PartnerStacksV1" => Ok(Self::PartnerStacksV1),
            "StacksV1" => Ok(Self::StacksV1),
            "UsersV1" => Ok(Self::UsersV1),
            "PeopleV1" => Ok(Self::PeopleV1),
            "AssetFacesV1" => Ok(Self::AssetFacesV1),
            "AssetFacesV2" => Ok(Self::AssetFacesV2),
            "AssetFacesV3" => Ok(Self::AssetFacesV3),
            "UserMetadataV1" => Ok(Self::UserMetadataV1),
            _ => Err(format!("invalid value for {}: {}", "SyncRequestType", value).into()),
        }
    }
}

impl TryFrom<&str> for SyncRequestType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SyncRequestType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SyncRequestType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
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
/// | `stack_id` | [`Uuid`](::uuid::Uuid) | ✓ | Stack ID |
///
/// See: <https://api.immich.app/models/SyncStackDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncStackDeleteV1 {
    /// Stack ID (required)
    #[serde(rename = "stackId")]
    pub stack_id: ::uuid::Uuid,
}

impl SyncStackDeleteV1 {
    ///Creates a new [`SyncStackDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(stack_id: ::uuid::Uuid) -> Self {
        Self { stack_id }
    }

    ///Sets `stack_id`.
    #[must_use]
    pub fn stack_id(mut self, value: ::uuid::Uuid) -> Self {
        self.stack_id = value;
        self
    }
}

/// `SyncStackV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Created at |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Stack ID |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner ID |
/// | `primary_asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Primary asset ID |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Updated at |
///
/// See: <https://api.immich.app/models/SyncStackV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncStackV1 {
    /// Created at (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::uuid::Uuid,
    /// Owner ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    /// Primary asset ID (required)
    #[serde(rename = "primaryAssetId")]
    pub primary_asset_id: ::uuid::Uuid,
    /// Updated at (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncStackV1 {
    ///Creates a new [`SyncStackV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        owner_id: ::uuid::Uuid,
        primary_asset_id: ::uuid::Uuid,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            id,
            owner_id,
            primary_asset_id,
            updated_at,
        }
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

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `primary_asset_id`.
    #[must_use]
    pub fn primary_asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.primary_asset_id = value;
        self
    }

    ///Sets `updated_at`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncStreamDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reset: Option<bool>,
    pub types: Vec<SyncRequestType>,
}

impl SyncStreamDTO {
    ///Creates a new [`SyncStreamDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(types: Vec<SyncRequestType>) -> Self {
        Self { reset: None, types }
    }

    ///Sets `reset`.
    #[must_use]
    pub fn reset(mut self, value: bool) -> Self {
        self.reset = Some(value);
        self
    }

    ///Sets or clears `reset`.
    #[must_use]
    pub fn set_reset(mut self, value: Option<bool>) -> Self {
        self.reset = value;
        self
    }

    ///Sets `types`.
    #[must_use]
    pub fn types(mut self, value: Vec<SyncRequestType>) -> Self {
        self.types = value;
        self
    }
}

/// `SyncUserDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
///
/// See: <https://api.immich.app/models/SyncUserDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncUserDeleteV1 {
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl SyncUserDeleteV1 {
    ///Creates a new [`SyncUserDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(user_id: ::uuid::Uuid) -> Self {
        Self { user_id }
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }
}

/// `SyncUserMetadataDeleteV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `key` | [`UserMetadataKey`] | ✓ |  |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
///
/// See: <https://api.immich.app/models/SyncUserMetadataDeleteV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncUserMetadataDeleteV1 {
    pub key: UserMetadataKey,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl SyncUserMetadataDeleteV1 {
    ///Creates a new [`SyncUserMetadataDeleteV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(key: UserMetadataKey, user_id: ::uuid::Uuid) -> Self {
        Self { key, user_id }
    }

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: UserMetadataKey) -> Self {
        self.key = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }
}

/// `SyncUserMetadataV1`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `key` | [`UserMetadataKey`] | ✓ |  |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
/// | `value` | `object` | ✓ | User metadata value |
///
/// See: <https://api.immich.app/models/SyncUserMetadataV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncUserMetadataV1 {
    pub key: UserMetadataKey,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl SyncUserMetadataV1 {
    ///Creates a new [`SyncUserMetadataV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        key: UserMetadataKey,
        user_id: ::uuid::Uuid,
        value: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self { key, user_id, value }
    }

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: UserMetadataKey) -> Self {
        self.key = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }

    ///Sets `value`.
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
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) | ✓ | User deleted at |
/// | `email` | [`String`] | ✓ | User email |
/// | `has_profile_image` | `bool` | ✓ | User has profile image |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
/// | `name` | [`String`] | ✓ | User name |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | User profile changed at |
///
/// See: <https://api.immich.app/models/SyncUserV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SyncUserV1 {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    /// User deleted at (required)
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub email: String,
    /// User has profile image (required)
    #[serde(rename = "hasProfileImage")]
    pub has_profile_image: bool,
    pub id: ::uuid::Uuid,
    pub name: String,
    /// User profile changed at (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl SyncUserV1 {
    ///Creates a new [`SyncUserV1`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        email: impl Into<String>,
        has_profile_image: bool,
        id: ::uuid::Uuid,
        name: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            avatar_color: None,
            deleted_at: None,
            email: email.into(),
            has_profile_image,
            id,
            name: name.into(),
            profile_changed_at,
        }
    }

    ///Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }

    ///Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }

    ///Sets `has_profile_image`.
    #[must_use]
    pub fn has_profile_image(mut self, value: bool) -> Self {
        self.has_profile_image = value;
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
}
