// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `download`.

/// `DownloadArchiveDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `archive_name` | [`String`] |  | The name of the archive to download, without extension |
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs |
/// | `edited` | `bool` |  | Download edited asset if available |
///
/// See: <https://api.immich.app/models/DownloadArchiveDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
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
    pub asset_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub edited: Option<bool>,
}

impl DownloadArchiveDTO {
    ///Creates a new [`DownloadArchiveDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_ids: Vec<::uuid::Uuid>) -> Self {
        Self {
            archive_name: None,
            asset_ids,
            edited: None,
        }
    }

    ///Sets `archive_name`.
    #[must_use]
    pub fn archive_name(mut self, value: impl Into<String>) -> Self {
        self.archive_name = Some(value.into());
        self
    }

    ///Sets or clears `archive_name`.
    #[must_use]
    pub fn set_archive_name(mut self, value: Option<String>) -> Self {
        self.archive_name = value;
        self
    }

    ///Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.asset_ids = value;
        self
    }

    ///Sets `edited`.
    #[must_use]
    pub fn edited(mut self, value: bool) -> Self {
        self.edited = Some(value);
        self
    }

    ///Sets or clears `edited`.
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
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs in this archive |
/// | `size` | `i64` | ✓ | Archive size in bytes |
///
/// See: <https://api.immich.app/models/DownloadArchiveInfo>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DownloadArchiveInfo {
    /// Asset IDs in this archive (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<::uuid::Uuid>,
    pub size: i64,
}

impl DownloadArchiveInfo {
    ///Creates a new [`DownloadArchiveInfo`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_ids: Vec<::uuid::Uuid>, size: i64) -> Self {
        Self { asset_ids, size }
    }

    ///Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.asset_ids = value;
        self
    }

    ///Sets `size`.
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
/// | `album_id` | [`Uuid`](::uuid::Uuid) |  | Album ID to download |
/// | `archive_size` | `i64` |  | Archive size limit in bytes |
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Asset IDs to download |
/// | `user_id` | [`Uuid`](::uuid::Uuid) |  | User ID to download assets from |
///
/// See: <https://api.immich.app/models/DownloadInfoDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct DownloadInfoDTO {
    /// Album ID to download
    #[serde(rename = "albumId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub album_id: Option<::uuid::Uuid>,
    /// Archive size limit in bytes
    #[serde(
        rename = "archiveSize",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub archive_size: Option<::std::num::NonZeroU64>,
    /// Asset IDs to download
    #[serde(rename = "assetIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub asset_ids: Vec<::uuid::Uuid>,
    /// User ID to download assets from
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<::uuid::Uuid>,
}

impl DownloadInfoDTO {
    ///Creates a new [`DownloadInfoDTO`].
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

    ///Sets `archive_size`.
    #[must_use]
    pub fn archive_size(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.archive_size = Some(value);
        self
    }

    ///Sets or clears `archive_size`.
    #[must_use]
    pub fn set_archive_size(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.archive_size = value;
        self
    }

    ///Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.asset_ids = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    ///Sets or clears `user_id`.
    #[must_use]
    pub fn set_user_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.user_id = value;
        self
    }
}

/// `DownloadResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `archives` | [`Vec<DownloadArchiveInfo>`](crate::assets::download::DownloadArchiveInfo) | ✓ | Archive information |
/// | `total_size` | `i64` | ✓ | Total size in bytes |
///
/// See: <https://api.immich.app/models/DownloadResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DownloadResponseDTO {
    pub archives: Vec<DownloadArchiveInfo>,
    /// Total size in bytes (required)
    #[serde(rename = "totalSize")]
    pub total_size: i64,
}

impl DownloadResponseDTO {
    ///Creates a new [`DownloadResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(archives: Vec<DownloadArchiveInfo>, total_size: i64) -> Self {
        Self { archives, total_size }
    }

    ///Sets `archives`.
    #[must_use]
    pub fn archives(mut self, value: Vec<DownloadArchiveInfo>) -> Self {
        self.archives = value;
        self
    }

    ///Sets `total_size`.
    #[must_use]
    pub fn total_size(mut self, value: i64) -> Self {
        self.total_size = value;
        self
    }
}
