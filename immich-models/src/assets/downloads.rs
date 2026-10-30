// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `downloads`.

pub use crate::id::{AlbumId, AssetId, UserId};

/// `DownloadArchiveDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `archive_name` | [`String`] |  | The name of the archive to download, without extension |
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs |
/// | `edited` | `bool` |  | Download edited asset if available |
///
/// See: <https://api.immich.app/models/DownloadArchiveDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DownloadArchiveDTO {
    /// The name of the archive to download, without extension
    #[serde(
        rename = "archiveName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub archive_name: Option<String>,
    /// Asset IDs (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<AssetId>,
    /// Download edited asset if available
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub edited: Option<bool>,
}

impl DownloadArchiveDTO {
    /// Creates a new [`DownloadArchiveDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_ids: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        Self {
            archive_name: None,
            asset_ids: asset_ids.into_iter().map(Into::into).collect(),
            edited: None,
        }
    }
    /// Sets `archive_name`.
    #[must_use]
    pub fn archive_name(mut self, value: impl Into<String>) -> Self {
        self.archive_name = Some(value.into());
        self
    }
    /// Sets or clears `archive_name`.
    #[must_use]
    pub fn set_archive_name(mut self, value: Option<String>) -> Self {
        self.archive_name = value;
        self
    }
    /// Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `edited`.
    #[must_use]
    pub fn edited(mut self, value: bool) -> Self {
        self.edited = Some(value);
        self
    }
    /// Sets or clears `edited`.
    #[must_use]
    pub fn set_edited(mut self, value: Option<bool>) -> Self {
        self.edited = value;
        self
    }
}

/// `DownloadArchiveInfo`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs in this archive |
/// | `size` | `i64` | ✓ | Archive size in bytes |
///
/// See: <https://api.immich.app/models/DownloadArchiveInfo>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DownloadArchiveInfo {
    /// Asset IDs in this archive (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<AssetId>,
    /// Archive size in bytes (required)
    pub size: i64,
}

impl DownloadArchiveInfo {
    /// Creates a new [`DownloadArchiveInfo`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_ids: impl IntoIterator<Item = impl Into<AssetId>>, size: i64) -> Self {
        Self {
            asset_ids: asset_ids.into_iter().map(Into::into).collect(),
            size,
        }
    }
    /// Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `size`.
    #[must_use]
    pub fn size(mut self, value: i64) -> Self {
        self.size = value;
        self
    }
}

/// `DownloadInfoDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_id` | [`AlbumId`](crate::id::AlbumId) |  | Album ID to download (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `archive_size` | `i64` |  | Archive size limit in bytes |
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) |  | Asset IDs to download |
/// | `user_id` | [`UserId`](crate::id::UserId) |  | User ID to download assets from (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/DownloadInfoDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct DownloadInfoDTO {
    /// Album ID to download (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(rename = "albumId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub album_id: Option<AlbumId>,
    /// Archive size limit in bytes
    #[serde(
        rename = "archiveSize",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub archive_size: Option<::std::num::NonZeroU64>,
    /// Asset IDs to download
    #[serde(rename = "assetIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub asset_ids: Vec<AssetId>,
    /// User ID to download assets from (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<UserId>,
}

impl DownloadInfoDTO {
    /// Creates a new [`DownloadInfoDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_id: None,
            archive_size: None,
            asset_ids: Vec::new(),
            user_id: None,
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
    /// Sets `archive_size`.
    #[must_use]
    pub fn archive_size(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.archive_size = Some(value);
        self
    }
    /// Sets or clears `archive_size`.
    #[must_use]
    pub fn set_archive_size(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.archive_size = value;
        self
    }
    /// Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = Some(value.into());
        self
    }
    /// Sets or clears `user_id`.
    #[must_use]
    pub fn set_user_id(mut self, value: Option<UserId>) -> Self {
        self.user_id = value;
        self
    }
}

/// `DownloadResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `archives` | [`Vec<DownloadArchiveInfo>`](crate::assets::downloads::DownloadArchiveInfo) | ✓ | Archive information |
/// | `total_size` | `i64` | ✓ | Total size in bytes |
///
/// See: <https://api.immich.app/models/DownloadResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DownloadResponseDTO {
    /// Archive information (required)
    pub archives: Vec<DownloadArchiveInfo>,
    /// Total size in bytes (required)
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

impl DownloadResponseDTO {
    /// Creates a new [`DownloadResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(archives: impl IntoIterator<Item = DownloadArchiveInfo>, total_size: i64) -> Self {
        Self {
            archives: archives.into_iter().collect(),
            total_size,
        }
    }
    /// Sets `archives`.
    #[must_use]
    pub fn archives(mut self, value: impl IntoIterator<Item = DownloadArchiveInfo>) -> Self {
        self.archives = value.into_iter().collect();
        self
    }
    /// Sets `total_size`.
    #[must_use]
    pub fn total_size(mut self, value: i64) -> Self {
        self.total_size = value;
        self
    }
}
