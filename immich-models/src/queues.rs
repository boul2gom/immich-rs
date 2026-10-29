// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `queues`.

pub use crate::jobs::{QueueName, QueueStatisticsDTO};

/// Job name
///
/// See: <https://api.immich.app/models/JobName>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum JobName {
    AssetDelete,
    AssetDeleteCheck,
    AssetDetectFacesQueueAll,
    AssetDetectFaces,
    AssetDetectDuplicatesQueueAll,
    AssetDetectDuplicates,
    AssetEditThumbnailGeneration,
    AssetEncodeVideoQueueAll,
    AssetEncodeVideo,
    AssetEmptyTrash,
    AssetExtractMetadataQueueAll,
    AssetExtractMetadata,
    AssetFileMigration,
    AssetGenerateThumbnailsQueueAll,
    AssetGenerateThumbnails,
    AuditTableCleanup,
    DatabaseBackup,
    FacialRecognitionQueueAll,
    FacialRecognition,
    FileDelete,
    FileMigrationQueueAll,
    LibraryDeleteCheck,
    LibraryDelete,
    LibraryRemoveAsset,
    LibraryScanAssetsQueueAll,
    LibrarySyncAssets,
    LibrarySyncFilesQueueAll,
    LibrarySyncFiles,
    LibraryScanQueueAll,
    HlsSessionCleanup,
    MemoryCleanup,
    MemoryGenerate,
    NotificationsCleanup,
    NotifyUserSignup,
    NotifyAlbumInvite,
    NotifyAlbumUpdate,
    UserDelete,
    UserDeleteCheck,
    UserSyncUsage,
    PersonCleanup,
    PersonFileMigration,
    PersonGenerateThumbnail,
    SessionCleanup,
    SendMail,
    SidecarQueueAll,
    SidecarCheck,
    SidecarWrite,
    SmartSearchQueueAll,
    SmartSearch,
    StorageTemplateMigration,
    StorageTemplateMigrationSingle,
    TagCleanup,
    VersionCheck,
    OcrQueueAll,
    Ocr,
    WorkflowAssetTrigger,
    IntegrityUntrackedFilesQueueAll,
    IntegrityUntrackedFiles,
    IntegrityUntrackedRefresh,
    IntegrityMissingFilesQueueAll,
    IntegrityMissingFiles,
    IntegrityMissingFilesRefresh,
    IntegrityChecksumFiles,
    IntegrityChecksumFilesRefresh,
    IntegrityDeleteReportType,
    IntegrityDeleteReports,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for JobName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AssetDelete => f.write_str("AssetDelete"),
            Self::AssetDeleteCheck => f.write_str("AssetDeleteCheck"),
            Self::AssetDetectFacesQueueAll => f.write_str("AssetDetectFacesQueueAll"),
            Self::AssetDetectFaces => f.write_str("AssetDetectFaces"),
            Self::AssetDetectDuplicatesQueueAll => f.write_str("AssetDetectDuplicatesQueueAll"),
            Self::AssetDetectDuplicates => f.write_str("AssetDetectDuplicates"),
            Self::AssetEditThumbnailGeneration => f.write_str("AssetEditThumbnailGeneration"),
            Self::AssetEncodeVideoQueueAll => f.write_str("AssetEncodeVideoQueueAll"),
            Self::AssetEncodeVideo => f.write_str("AssetEncodeVideo"),
            Self::AssetEmptyTrash => f.write_str("AssetEmptyTrash"),
            Self::AssetExtractMetadataQueueAll => f.write_str("AssetExtractMetadataQueueAll"),
            Self::AssetExtractMetadata => f.write_str("AssetExtractMetadata"),
            Self::AssetFileMigration => f.write_str("AssetFileMigration"),
            Self::AssetGenerateThumbnailsQueueAll => f.write_str("AssetGenerateThumbnailsQueueAll"),
            Self::AssetGenerateThumbnails => f.write_str("AssetGenerateThumbnails"),
            Self::AuditTableCleanup => f.write_str("AuditTableCleanup"),
            Self::DatabaseBackup => f.write_str("DatabaseBackup"),
            Self::FacialRecognitionQueueAll => f.write_str("FacialRecognitionQueueAll"),
            Self::FacialRecognition => f.write_str("FacialRecognition"),
            Self::FileDelete => f.write_str("FileDelete"),
            Self::FileMigrationQueueAll => f.write_str("FileMigrationQueueAll"),
            Self::LibraryDeleteCheck => f.write_str("LibraryDeleteCheck"),
            Self::LibraryDelete => f.write_str("LibraryDelete"),
            Self::LibraryRemoveAsset => f.write_str("LibraryRemoveAsset"),
            Self::LibraryScanAssetsQueueAll => f.write_str("LibraryScanAssetsQueueAll"),
            Self::LibrarySyncAssets => f.write_str("LibrarySyncAssets"),
            Self::LibrarySyncFilesQueueAll => f.write_str("LibrarySyncFilesQueueAll"),
            Self::LibrarySyncFiles => f.write_str("LibrarySyncFiles"),
            Self::LibraryScanQueueAll => f.write_str("LibraryScanQueueAll"),
            Self::HlsSessionCleanup => f.write_str("HlsSessionCleanup"),
            Self::MemoryCleanup => f.write_str("MemoryCleanup"),
            Self::MemoryGenerate => f.write_str("MemoryGenerate"),
            Self::NotificationsCleanup => f.write_str("NotificationsCleanup"),
            Self::NotifyUserSignup => f.write_str("NotifyUserSignup"),
            Self::NotifyAlbumInvite => f.write_str("NotifyAlbumInvite"),
            Self::NotifyAlbumUpdate => f.write_str("NotifyAlbumUpdate"),
            Self::UserDelete => f.write_str("UserDelete"),
            Self::UserDeleteCheck => f.write_str("UserDeleteCheck"),
            Self::UserSyncUsage => f.write_str("UserSyncUsage"),
            Self::PersonCleanup => f.write_str("PersonCleanup"),
            Self::PersonFileMigration => f.write_str("PersonFileMigration"),
            Self::PersonGenerateThumbnail => f.write_str("PersonGenerateThumbnail"),
            Self::SessionCleanup => f.write_str("SessionCleanup"),
            Self::SendMail => f.write_str("SendMail"),
            Self::SidecarQueueAll => f.write_str("SidecarQueueAll"),
            Self::SidecarCheck => f.write_str("SidecarCheck"),
            Self::SidecarWrite => f.write_str("SidecarWrite"),
            Self::SmartSearchQueueAll => f.write_str("SmartSearchQueueAll"),
            Self::SmartSearch => f.write_str("SmartSearch"),
            Self::StorageTemplateMigration => f.write_str("StorageTemplateMigration"),
            Self::StorageTemplateMigrationSingle => f.write_str("StorageTemplateMigrationSingle"),
            Self::TagCleanup => f.write_str("TagCleanup"),
            Self::VersionCheck => f.write_str("VersionCheck"),
            Self::OcrQueueAll => f.write_str("OcrQueueAll"),
            Self::Ocr => f.write_str("Ocr"),
            Self::WorkflowAssetTrigger => f.write_str("WorkflowAssetTrigger"),
            Self::IntegrityUntrackedFilesQueueAll => f.write_str("IntegrityUntrackedFilesQueueAll"),
            Self::IntegrityUntrackedFiles => f.write_str("IntegrityUntrackedFiles"),
            Self::IntegrityUntrackedRefresh => f.write_str("IntegrityUntrackedRefresh"),
            Self::IntegrityMissingFilesQueueAll => f.write_str("IntegrityMissingFilesQueueAll"),
            Self::IntegrityMissingFiles => f.write_str("IntegrityMissingFiles"),
            Self::IntegrityMissingFilesRefresh => f.write_str("IntegrityMissingFilesRefresh"),
            Self::IntegrityChecksumFiles => f.write_str("IntegrityChecksumFiles"),
            Self::IntegrityChecksumFilesRefresh => f.write_str("IntegrityChecksumFilesRefresh"),
            Self::IntegrityDeleteReportType => f.write_str("IntegrityDeleteReportType"),
            Self::IntegrityDeleteReports => f.write_str("IntegrityDeleteReports"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for JobName {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "AssetDelete" => Ok(Self::AssetDelete),
            "AssetDeleteCheck" => Ok(Self::AssetDeleteCheck),
            "AssetDetectFacesQueueAll" => Ok(Self::AssetDetectFacesQueueAll),
            "AssetDetectFaces" => Ok(Self::AssetDetectFaces),
            "AssetDetectDuplicatesQueueAll" => Ok(Self::AssetDetectDuplicatesQueueAll),
            "AssetDetectDuplicates" => Ok(Self::AssetDetectDuplicates),
            "AssetEditThumbnailGeneration" => Ok(Self::AssetEditThumbnailGeneration),
            "AssetEncodeVideoQueueAll" => Ok(Self::AssetEncodeVideoQueueAll),
            "AssetEncodeVideo" => Ok(Self::AssetEncodeVideo),
            "AssetEmptyTrash" => Ok(Self::AssetEmptyTrash),
            "AssetExtractMetadataQueueAll" => Ok(Self::AssetExtractMetadataQueueAll),
            "AssetExtractMetadata" => Ok(Self::AssetExtractMetadata),
            "AssetFileMigration" => Ok(Self::AssetFileMigration),
            "AssetGenerateThumbnailsQueueAll" => Ok(Self::AssetGenerateThumbnailsQueueAll),
            "AssetGenerateThumbnails" => Ok(Self::AssetGenerateThumbnails),
            "AuditTableCleanup" => Ok(Self::AuditTableCleanup),
            "DatabaseBackup" => Ok(Self::DatabaseBackup),
            "FacialRecognitionQueueAll" => Ok(Self::FacialRecognitionQueueAll),
            "FacialRecognition" => Ok(Self::FacialRecognition),
            "FileDelete" => Ok(Self::FileDelete),
            "FileMigrationQueueAll" => Ok(Self::FileMigrationQueueAll),
            "LibraryDeleteCheck" => Ok(Self::LibraryDeleteCheck),
            "LibraryDelete" => Ok(Self::LibraryDelete),
            "LibraryRemoveAsset" => Ok(Self::LibraryRemoveAsset),
            "LibraryScanAssetsQueueAll" => Ok(Self::LibraryScanAssetsQueueAll),
            "LibrarySyncAssets" => Ok(Self::LibrarySyncAssets),
            "LibrarySyncFilesQueueAll" => Ok(Self::LibrarySyncFilesQueueAll),
            "LibrarySyncFiles" => Ok(Self::LibrarySyncFiles),
            "LibraryScanQueueAll" => Ok(Self::LibraryScanQueueAll),
            "HlsSessionCleanup" => Ok(Self::HlsSessionCleanup),
            "MemoryCleanup" => Ok(Self::MemoryCleanup),
            "MemoryGenerate" => Ok(Self::MemoryGenerate),
            "NotificationsCleanup" => Ok(Self::NotificationsCleanup),
            "NotifyUserSignup" => Ok(Self::NotifyUserSignup),
            "NotifyAlbumInvite" => Ok(Self::NotifyAlbumInvite),
            "NotifyAlbumUpdate" => Ok(Self::NotifyAlbumUpdate),
            "UserDelete" => Ok(Self::UserDelete),
            "UserDeleteCheck" => Ok(Self::UserDeleteCheck),
            "UserSyncUsage" => Ok(Self::UserSyncUsage),
            "PersonCleanup" => Ok(Self::PersonCleanup),
            "PersonFileMigration" => Ok(Self::PersonFileMigration),
            "PersonGenerateThumbnail" => Ok(Self::PersonGenerateThumbnail),
            "SessionCleanup" => Ok(Self::SessionCleanup),
            "SendMail" => Ok(Self::SendMail),
            "SidecarQueueAll" => Ok(Self::SidecarQueueAll),
            "SidecarCheck" => Ok(Self::SidecarCheck),
            "SidecarWrite" => Ok(Self::SidecarWrite),
            "SmartSearchQueueAll" => Ok(Self::SmartSearchQueueAll),
            "SmartSearch" => Ok(Self::SmartSearch),
            "StorageTemplateMigration" => Ok(Self::StorageTemplateMigration),
            "StorageTemplateMigrationSingle" => Ok(Self::StorageTemplateMigrationSingle),
            "TagCleanup" => Ok(Self::TagCleanup),
            "VersionCheck" => Ok(Self::VersionCheck),
            "OcrQueueAll" => Ok(Self::OcrQueueAll),
            "Ocr" => Ok(Self::Ocr),
            "WorkflowAssetTrigger" => Ok(Self::WorkflowAssetTrigger),
            "IntegrityUntrackedFilesQueueAll" => Ok(Self::IntegrityUntrackedFilesQueueAll),
            "IntegrityUntrackedFiles" => Ok(Self::IntegrityUntrackedFiles),
            "IntegrityUntrackedRefresh" => Ok(Self::IntegrityUntrackedRefresh),
            "IntegrityMissingFilesQueueAll" => Ok(Self::IntegrityMissingFilesQueueAll),
            "IntegrityMissingFiles" => Ok(Self::IntegrityMissingFiles),
            "IntegrityMissingFilesRefresh" => Ok(Self::IntegrityMissingFilesRefresh),
            "IntegrityChecksumFiles" => Ok(Self::IntegrityChecksumFiles),
            "IntegrityChecksumFilesRefresh" => Ok(Self::IntegrityChecksumFilesRefresh),
            "IntegrityDeleteReportType" => Ok(Self::IntegrityDeleteReportType),
            "IntegrityDeleteReports" => Ok(Self::IntegrityDeleteReports),
            _ => Err(format!("invalid value for {}: {}", "JobName", value).into()),
        }
    }
}

impl TryFrom<&str> for JobName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for JobName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for JobName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `QueueDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `failed` | `bool` |  | If true, will also remove failed jobs from the queue. |
///
/// See: <https://api.immich.app/models/QueueDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct QueueDeleteDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub failed: Option<bool>,
}

impl QueueDeleteDTO {
    ///Creates a new [`QueueDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { failed: None }
    }

    ///Sets `failed`.
    #[must_use]
    pub fn failed(mut self, value: bool) -> Self {
        self.failed = Some(value);
        self
    }

    ///Sets or clears `failed`.
    #[must_use]
    pub fn set_failed(mut self, value: Option<bool>) -> Self {
        self.failed = value;
        self
    }
}

/// `QueueJobResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `data` | `object` | ✓ | Job data payload |
/// | `id` | [`String`] |  | Job ID |
/// | `name` | [`JobName`] | ✓ |  |
/// | `timestamp` | `i64` | ✓ | Job creation timestamp |
///
/// See: <https://api.immich.app/models/QueueJobResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct QueueJobResponseDTO {
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: Option<String>,
    pub name: JobName,
    pub timestamp: i64,
}

impl QueueJobResponseDTO {
    ///Creates a new [`QueueJobResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(data: ::serde_json::Map<String, ::serde_json::Value>, name: JobName, timestamp: i64) -> Self {
        Self {
            data,
            id: None,
            name,
            timestamp,
        }
    }

    ///Sets `data`.
    #[must_use]
    pub fn data(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.data = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    ///Sets or clears `id`.
    #[must_use]
    pub fn set_id(mut self, value: Option<String>) -> Self {
        self.id = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: JobName) -> Self {
        self.name = value;
        self
    }

    ///Sets `timestamp`.
    #[must_use]
    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = value;
        self
    }
}

/// Queue job status
///
/// See: <https://api.immich.app/models/QueueJobStatus>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum QueueJobStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "delayed")]
    Delayed,
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "paused")]
    Paused,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for QueueJobStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
            Self::Failed => f.write_str("failed"),
            Self::Completed => f.write_str("completed"),
            Self::Delayed => f.write_str("delayed"),
            Self::Waiting => f.write_str("waiting"),
            Self::Paused => f.write_str("paused"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for QueueJobStatus {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "failed" => Ok(Self::Failed),
            "completed" => Ok(Self::Completed),
            "delayed" => Ok(Self::Delayed),
            "waiting" => Ok(Self::Waiting),
            "paused" => Ok(Self::Paused),
            _ => Err(format!("invalid value for {}: {}", "QueueJobStatus", value).into()),
        }
    }
}

impl TryFrom<&str> for QueueJobStatus {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for QueueJobStatus {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for QueueJobStatus {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `QueueResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `is_paused` | `bool` | ✓ | Whether the queue is paused |
/// | `name` | [`QueueName`] | ✓ |  |
/// | `statistics` | [`QueueStatisticsDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/QueueResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct QueueResponseDTO {
    /// Whether the queue is paused (required)
    #[serde(rename = "isPaused")]
    pub is_paused: bool,
    pub name: QueueName,
    pub statistics: QueueStatisticsDTO,
}

impl QueueResponseDTO {
    ///Creates a new [`QueueResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_paused: bool, name: QueueName, statistics: QueueStatisticsDTO) -> Self {
        Self {
            is_paused,
            name,
            statistics,
        }
    }

    ///Sets `is_paused`.
    #[must_use]
    pub fn is_paused(mut self, value: bool) -> Self {
        self.is_paused = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: QueueName) -> Self {
        self.name = value;
        self
    }

    ///Sets `statistics`.
    #[must_use]
    pub fn statistics(mut self, value: QueueStatisticsDTO) -> Self {
        self.statistics = value;
        self
    }
}

/// `QueueUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `is_paused` | `bool` |  | Whether to pause the queue |
///
/// See: <https://api.immich.app/models/QueueUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct QueueUpdateDTO {
    /// Whether to pause the queue
    #[serde(rename = "isPaused", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_paused: Option<bool>,
}

impl QueueUpdateDTO {
    ///Creates a new [`QueueUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { is_paused: None }
    }

    ///Sets `is_paused`.
    #[must_use]
    pub fn is_paused(mut self, value: bool) -> Self {
        self.is_paused = Some(value);
        self
    }

    ///Sets or clears `is_paused`.
    #[must_use]
    pub fn set_is_paused(mut self, value: Option<bool>) -> Self {
        self.is_paused = value;
        self
    }
}
