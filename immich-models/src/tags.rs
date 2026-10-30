// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `tags`.

pub use crate::id::{AssetId, TagId};

/// `TagBulkAssetsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs |
/// | `tag_ids` | [`Vec<TagId>`](crate::id::TagId) | ✓ | Tag IDs |
///
/// See: <https://api.immich.app/models/TagBulkAssetsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TagBulkAssetsDTO {
    /// Asset IDs (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<AssetId>,
    /// Tag IDs (required)
    #[serde(rename = "tagIds")]
    pub tag_ids: Vec<TagId>,
}

impl TagBulkAssetsDTO {
    /// Creates a new [`TagBulkAssetsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_ids: impl IntoIterator<Item = impl Into<AssetId>>,
        tag_ids: impl IntoIterator<Item = impl Into<TagId>>,
    ) -> Self {
        Self {
            asset_ids: asset_ids.into_iter().map(Into::into).collect(),
            tag_ids: tag_ids.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `tag_ids`.
    #[must_use]
    pub fn tag_ids(mut self, value: impl IntoIterator<Item = impl Into<TagId>>) -> Self {
        self.tag_ids = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `TagBulkAssetsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `count` | `i64` | ✓ | Number of assets tagged |
///
/// See: <https://api.immich.app/models/TagBulkAssetsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TagBulkAssetsResponseDTO {
    /// Number of assets tagged (required)
    pub count: i64,
}

impl TagBulkAssetsResponseDTO {
    /// Creates a new [`TagBulkAssetsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(count: i64) -> Self {
        Self { count }
    }
    /// Sets `count`.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = value;
        self
    }
}

/// `TagCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `color` | [`String`] |  | Tag color (hex) (pattern: `^#?([0-9A-Fa-f]{3}\|[0-9A-Fa-f]{4}\|[0-9A-Fa-f]{6}\|[0-9A-Fa-f]{8})$`) |
/// | `name` | [`String`] | ✓ | Tag name (pattern: `^[^/]*$`) |
/// | `parent_id` | [`TagId`](crate::id::TagId) |  | Parent tag ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/TagCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TagCreateDTO {
    /// Tag color (hex) (pattern: `^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$`)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<String>,
    /// Tag name (pattern: `^[^/]*$`) (required)
    pub name: String,
    /// Parent tag ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(rename = "parentId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_id: Option<TagId>,
}

impl TagCreateDTO {
    /// Creates a new [`TagCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            color: None,
            name: name.into(),
            parent_id: None,
        }
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
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `parent_id`.
    #[must_use]
    pub fn parent_id(mut self, value: impl Into<TagId>) -> Self {
        self.parent_id = Some(value.into());
        self
    }
    /// Sets or clears `parent_id`.
    #[must_use]
    pub fn set_parent_id(mut self, value: Option<TagId>) -> Self {
        self.parent_id = value;
        self
    }
}

/// `TagUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `color` | [`String`] |  | Tag color (hex) (pattern: `^#?([0-9A-Fa-f]{3}\|[0-9A-Fa-f]{4}\|[0-9A-Fa-f]{6}\|[0-9A-Fa-f]{8})$`) |
/// | `name` | [`String`] |  | Tag name (pattern: `^[^/]*$`) |
///
/// See: <https://api.immich.app/models/TagUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct TagUpdateDTO {
    /// Tag color (hex) (pattern: `^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$`)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<String>,
    /// Tag name (pattern: `^[^/]*$`)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
}

impl TagUpdateDTO {
    /// Creates a new [`TagUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            color: None,
            name: None,
        }
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
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
}

/// `TagUpsertDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `tags` | [`Vec<String>`](::std::vec::Vec) | ✓ | Tag names to upsert |
///
/// See: <https://api.immich.app/models/TagUpsertDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TagUpsertDTO {
    /// Tag names to upsert (required)
    pub tags: Vec<String>,
}

impl TagUpsertDTO {
    /// Creates a new [`TagUpsertDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            tags: tags.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `tags`.
    #[must_use]
    pub fn tags(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags = value.into_iter().map(Into::into).collect();
        self
    }
}
