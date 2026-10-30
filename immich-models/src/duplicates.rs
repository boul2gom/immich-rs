// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `duplicates`.

pub use crate::assets::AssetResponseDTO;
pub use crate::id::{AssetId, DuplicateId};

/// `DuplicateResolveDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `groups` | [`Vec<DuplicateResolveGroupDTO>`](crate::duplicates::DuplicateResolveGroupDTO) | ✓ | List of duplicate groups to resolve |
///
/// See: <https://api.immich.app/models/DuplicateResolveDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DuplicateResolveDTO {
    /// List of duplicate groups to resolve (required)
    pub groups: Vec<DuplicateResolveGroupDTO>,
}

impl DuplicateResolveDTO {
    /// Creates a new [`DuplicateResolveDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(groups: impl IntoIterator<Item = DuplicateResolveGroupDTO>) -> Self {
        Self {
            groups: groups.into_iter().collect(),
        }
    }
    /// Sets `groups`.
    #[must_use]
    pub fn groups(mut self, value: impl IntoIterator<Item = DuplicateResolveGroupDTO>) -> Self {
        self.groups = value.into_iter().collect();
        self
    }
}

/// `DuplicateResolveGroupDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `duplicate_id` | [`DuplicateId`](crate::id::DuplicateId) | ✓ | Pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$` |
/// | `keep_asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs to keep |
/// | `trash_asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs to trash or delete |
///
/// See: <https://api.immich.app/models/DuplicateResolveGroupDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DuplicateResolveGroupDTO {
    /// Pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$` (required)
    #[serde(rename = "duplicateId")]
    pub duplicate_id: DuplicateId,
    /// Asset IDs to keep (required)
    #[serde(rename = "keepAssetIds")]
    pub keep_asset_ids: Vec<AssetId>,
    /// Asset IDs to trash or delete (required)
    #[serde(rename = "trashAssetIds")]
    pub trash_asset_ids: Vec<AssetId>,
}

impl DuplicateResolveGroupDTO {
    /// Creates a new [`DuplicateResolveGroupDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        duplicate_id: impl Into<DuplicateId>,
        keep_asset_ids: impl IntoIterator<Item = impl Into<AssetId>>,
        trash_asset_ids: impl IntoIterator<Item = impl Into<AssetId>>,
    ) -> Self {
        Self {
            duplicate_id: duplicate_id.into(),
            keep_asset_ids: keep_asset_ids.into_iter().map(Into::into).collect(),
            trash_asset_ids: trash_asset_ids.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `duplicate_id`.
    #[must_use]
    pub fn duplicate_id(mut self, value: impl Into<DuplicateId>) -> Self {
        self.duplicate_id = value.into();
        self
    }
    /// Sets `keep_asset_ids`.
    #[must_use]
    pub fn keep_asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.keep_asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `trash_asset_ids`.
    #[must_use]
    pub fn trash_asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.trash_asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `DuplicateResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `assets` | [`Vec<AssetResponseDTO>`](crate::assets::AssetResponseDTO) | ✓ | Duplicate assets |
/// | `duplicate_id` | [`DuplicateId`](crate::id::DuplicateId) | ✓ | Duplicate group ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `suggested_keep_asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Suggested asset IDs to keep based on file size and EXIF data |
///
/// See: <https://api.immich.app/models/DuplicateResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct DuplicateResponseDTO {
    /// Duplicate assets (required)
    pub assets: Vec<AssetResponseDTO>,
    /// Duplicate group ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "duplicateId")]
    pub duplicate_id: DuplicateId,
    /// Suggested asset IDs to keep based on file size and EXIF data (required)
    #[serde(rename = "suggestedKeepAssetIds")]
    pub suggested_keep_asset_ids: Vec<AssetId>,
}

impl DuplicateResponseDTO {
    /// Creates a new [`DuplicateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        assets: impl IntoIterator<Item = AssetResponseDTO>,
        duplicate_id: impl Into<DuplicateId>,
        suggested_keep_asset_ids: impl IntoIterator<Item = impl Into<AssetId>>,
    ) -> Self {
        Self {
            assets: assets.into_iter().collect(),
            duplicate_id: duplicate_id.into(),
            suggested_keep_asset_ids: suggested_keep_asset_ids.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: impl IntoIterator<Item = AssetResponseDTO>) -> Self {
        self.assets = value.into_iter().collect();
        self
    }
    /// Sets `duplicate_id`.
    #[must_use]
    pub fn duplicate_id(mut self, value: impl Into<DuplicateId>) -> Self {
        self.duplicate_id = value.into();
        self
    }
    /// Sets `suggested_keep_asset_ids`.
    #[must_use]
    pub fn suggested_keep_asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.suggested_keep_asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
}
