// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `duplicates`.

pub use crate::assets::AssetResponseDTO;

/// `DuplicateResolveDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `groups` | [`Vec<DuplicateResolveGroupDTO>`](crate::duplicates::DuplicateResolveGroupDTO) | ✓ | List of duplicate groups to resolve |
///
/// See: <https://api.immich.app/models/DuplicateResolveDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DuplicateResolveDTO {
    pub groups: Vec<DuplicateResolveGroupDTO>,
}

impl DuplicateResolveDTO {
    ///Creates a new [`DuplicateResolveDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(groups: Vec<DuplicateResolveGroupDTO>) -> Self {
        Self { groups }
    }

    ///Sets `groups`.
    #[must_use]
    pub fn groups(mut self, value: Vec<DuplicateResolveGroupDTO>) -> Self {
        self.groups = value;
        self
    }
}

/// `DuplicateResolveGroupDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `duplicate_id` | [`Uuid`](::uuid::Uuid) | ✓ |  |
/// | `keep_asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs to keep |
/// | `trash_asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs to trash or delete |
///
/// See: <https://api.immich.app/models/DuplicateResolveGroupDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DuplicateResolveGroupDTO {
    /// (required)
    #[serde(rename = "duplicateId")]
    pub duplicate_id: ::uuid::Uuid,
    /// Asset IDs to keep (required)
    #[serde(rename = "keepAssetIds")]
    pub keep_asset_ids: Vec<::uuid::Uuid>,
    /// Asset IDs to trash or delete (required)
    #[serde(rename = "trashAssetIds")]
    pub trash_asset_ids: Vec<::uuid::Uuid>,
}

impl DuplicateResolveGroupDTO {
    ///Creates a new [`DuplicateResolveGroupDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        duplicate_id: ::uuid::Uuid,
        keep_asset_ids: Vec<::uuid::Uuid>,
        trash_asset_ids: Vec<::uuid::Uuid>,
    ) -> Self {
        Self {
            duplicate_id,
            keep_asset_ids,
            trash_asset_ids,
        }
    }

    ///Sets `duplicate_id`.
    #[must_use]
    pub fn duplicate_id(mut self, value: ::uuid::Uuid) -> Self {
        self.duplicate_id = value;
        self
    }

    ///Sets `keep_asset_ids`.
    #[must_use]
    pub fn keep_asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.keep_asset_ids = value;
        self
    }

    ///Sets `trash_asset_ids`.
    #[must_use]
    pub fn trash_asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.trash_asset_ids = value;
        self
    }
}

/// `DuplicateResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `assets` | [`Vec<AssetResponseDTO>`](crate::assets::AssetResponseDTO) | ✓ | Duplicate assets |
/// | `duplicate_id` | [`Uuid`](::uuid::Uuid) | ✓ | Duplicate group ID |
/// | `suggested_keep_asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Suggested asset IDs to keep based on file size and EXIF data |
///
/// See: <https://api.immich.app/models/DuplicateResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DuplicateResponseDTO {
    pub assets: Vec<AssetResponseDTO>,
    /// Duplicate group ID (required)
    #[serde(rename = "duplicateId")]
    pub duplicate_id: ::uuid::Uuid,
    /// Suggested asset IDs to keep based on file size and EXIF data (required)
    #[serde(rename = "suggestedKeepAssetIds")]
    pub suggested_keep_asset_ids: Vec<::uuid::Uuid>,
}

impl DuplicateResponseDTO {
    ///Creates a new [`DuplicateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        assets: Vec<AssetResponseDTO>,
        duplicate_id: ::uuid::Uuid,
        suggested_keep_asset_ids: Vec<::uuid::Uuid>,
    ) -> Self {
        Self {
            assets,
            duplicate_id,
            suggested_keep_asset_ids,
        }
    }

    ///Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: Vec<AssetResponseDTO>) -> Self {
        self.assets = value;
        self
    }

    ///Sets `duplicate_id`.
    #[must_use]
    pub fn duplicate_id(mut self, value: ::uuid::Uuid) -> Self {
        self.duplicate_id = value;
        self
    }

    ///Sets `suggested_keep_asset_ids`.
    #[must_use]
    pub fn suggested_keep_asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.suggested_keep_asset_ids = value;
        self
    }
}
