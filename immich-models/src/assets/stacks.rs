// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `stacks`.

pub use crate::assets::AssetResponseDTO;

/// `StackCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs (first becomes primary, min 2) |
///
/// See: <https://api.immich.app/models/StackCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StackCreateDTO {
    /// Asset IDs (first becomes primary, min 2) (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<::uuid::Uuid>,
}

impl StackCreateDTO {
    ///Creates a new [`StackCreateDTO`].
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

/// Stack response
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `assets` | [`Vec<AssetResponseDTO>`](crate::assets::AssetResponseDTO) | ✓ |  |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Stack ID |
/// | `primary_asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Primary asset ID |
///
/// See: <https://api.immich.app/models/StackResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StackResponseDTO {
    pub assets: Vec<AssetResponseDTO>,
    pub id: ::uuid::Uuid,
    /// Primary asset ID (required)
    #[serde(rename = "primaryAssetId")]
    pub primary_asset_id: ::uuid::Uuid,
}

impl StackResponseDTO {
    ///Creates a new [`StackResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(assets: Vec<AssetResponseDTO>, id: ::uuid::Uuid, primary_asset_id: ::uuid::Uuid) -> Self {
        Self {
            assets,
            id,
            primary_asset_id,
        }
    }

    ///Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: Vec<AssetResponseDTO>) -> Self {
        self.assets = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `primary_asset_id`.
    #[must_use]
    pub fn primary_asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.primary_asset_id = value;
        self
    }
}

/// `StackUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `primary_asset_id` | [`Uuid`](::uuid::Uuid) |  | Primary asset ID |
///
/// See: <https://api.immich.app/models/StackUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct StackUpdateDTO {
    /// Primary asset ID
    #[serde(
        rename = "primaryAssetId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub primary_asset_id: Option<::uuid::Uuid>,
}

impl StackUpdateDTO {
    ///Creates a new [`StackUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { primary_asset_id: None }
    }

    ///Sets `primary_asset_id`.
    #[must_use]
    pub fn primary_asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.primary_asset_id = Some(value);
        self
    }

    ///Sets or clears `primary_asset_id`.
    #[must_use]
    pub fn set_primary_asset_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.primary_asset_id = value;
        self
    }
}
