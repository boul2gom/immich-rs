// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `stacks`.

pub use crate::assets::AssetResponseDTO;
pub use crate::id::{AssetId, StackId};

/// `StackCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs (first becomes primary, min 2) |
///
/// See: <https://api.immich.app/models/StackCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct StackCreateDTO {
    /// Asset IDs (first becomes primary, min 2) (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<AssetId>,
}

impl StackCreateDTO {
    /// Creates a new [`StackCreateDTO`].
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

/// Stack response
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `assets` | [`Vec<AssetResponseDTO>`](crate::assets::AssetResponseDTO) | ✓ |  |
/// | `id` | [`StackId`](crate::id::StackId) | ✓ | Stack ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `primary_asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Primary asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/StackResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct StackResponseDTO {
    /// (required)
    pub assets: Vec<AssetResponseDTO>,
    /// Stack ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: StackId,
    /// Primary asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "primaryAssetId")]
    pub primary_asset_id: AssetId,
}

impl StackResponseDTO {
    /// Creates a new [`StackResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        assets: impl IntoIterator<Item = AssetResponseDTO>,
        id: impl Into<StackId>,
        primary_asset_id: impl Into<AssetId>,
    ) -> Self {
        Self {
            assets: assets.into_iter().collect(),
            id: id.into(),
            primary_asset_id: primary_asset_id.into(),
        }
    }
    /// Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: impl IntoIterator<Item = AssetResponseDTO>) -> Self {
        self.assets = value.into_iter().collect();
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<StackId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `primary_asset_id`.
    #[must_use]
    pub fn primary_asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.primary_asset_id = value.into();
        self
    }
}

/// `StackUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `primary_asset_id` | [`AssetId`](crate::id::AssetId) |  | Primary asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/StackUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct StackUpdateDTO {
    /// Primary asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(
        rename = "primaryAssetId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub primary_asset_id: Option<AssetId>,
}

impl StackUpdateDTO {
    /// Creates a new [`StackUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { primary_asset_id: None }
    }
    /// Sets `primary_asset_id`.
    #[must_use]
    pub fn primary_asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.primary_asset_id = Some(value.into());
        self
    }
    /// Sets or clears `primary_asset_id`.
    #[must_use]
    pub fn set_primary_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.primary_asset_id = value;
        self
    }
}
