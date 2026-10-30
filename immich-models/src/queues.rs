// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `queues`.

pub use crate::jobs::{QueueName, QueueStatisticsDTO};

/// Job name
///
/// See: <https://api.immich.app/models/JobName>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl JobName {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::AssetDelete => "AssetDelete",
            Self::AssetDeleteCheck => "AssetDeleteCheck",
            Self::AssetDetectFacesQueueAll => "AssetDetectFacesQueueAll",
            Self::AssetDetectFaces => "AssetDetectFaces",
            Self::AssetDetectDuplicatesQueueAll => "AssetDetectDuplicatesQueueAll",
            Self::AssetDetectDuplicates => "AssetDetectDuplicates",
            Self::AssetEditThumbnailGeneration => "AssetEditThumbnailGeneration",
            Self::AssetEncodeVideoQueueAll => "AssetEncodeVideoQueueAll",
            Self::AssetEncodeVideo => "AssetEncodeVideo",
            Self::AssetEmptyTrash => "AssetEmptyTrash",
            Self::AssetExtractMetadataQueueAll => "AssetExtractMetadataQueueAll",
            Self::AssetExtractMetadata => "AssetExtractMetadata",
            Self::AssetFileMigration => "AssetFileMigration",
            Self::AssetGenerateThumbnailsQueueAll => "AssetGenerateThumbnailsQueueAll",
            Self::AssetGenerateThumbnails => "AssetGenerateThumbnails",
            Self::AuditTableCleanup => "AuditTableCleanup",
            Self::DatabaseBackup => "DatabaseBackup",
            Self::FacialRecognitionQueueAll => "FacialRecognitionQueueAll",
            Self::FacialRecognition => "FacialRecognition",
            Self::FileDelete => "FileDelete",
            Self::FileMigrationQueueAll => "FileMigrationQueueAll",
            Self::LibraryDeleteCheck => "LibraryDeleteCheck",
            Self::LibraryDelete => "LibraryDelete",
            Self::LibraryRemoveAsset => "LibraryRemoveAsset",
            Self::LibraryScanAssetsQueueAll => "LibraryScanAssetsQueueAll",
            Self::LibrarySyncAssets => "LibrarySyncAssets",
            Self::LibrarySyncFilesQueueAll => "LibrarySyncFilesQueueAll",
            Self::LibrarySyncFiles => "LibrarySyncFiles",
            Self::LibraryScanQueueAll => "LibraryScanQueueAll",
            Self::HlsSessionCleanup => "HlsSessionCleanup",
            Self::MemoryCleanup => "MemoryCleanup",
            Self::MemoryGenerate => "MemoryGenerate",
            Self::NotificationsCleanup => "NotificationsCleanup",
            Self::NotifyUserSignup => "NotifyUserSignup",
            Self::NotifyAlbumInvite => "NotifyAlbumInvite",
            Self::NotifyAlbumUpdate => "NotifyAlbumUpdate",
            Self::UserDelete => "UserDelete",
            Self::UserDeleteCheck => "UserDeleteCheck",
            Self::UserSyncUsage => "UserSyncUsage",
            Self::PersonCleanup => "PersonCleanup",
            Self::PersonFileMigration => "PersonFileMigration",
            Self::PersonGenerateThumbnail => "PersonGenerateThumbnail",
            Self::SessionCleanup => "SessionCleanup",
            Self::SendMail => "SendMail",
            Self::SidecarQueueAll => "SidecarQueueAll",
            Self::SidecarCheck => "SidecarCheck",
            Self::SidecarWrite => "SidecarWrite",
            Self::SmartSearchQueueAll => "SmartSearchQueueAll",
            Self::SmartSearch => "SmartSearch",
            Self::StorageTemplateMigration => "StorageTemplateMigration",
            Self::StorageTemplateMigrationSingle => "StorageTemplateMigrationSingle",
            Self::TagCleanup => "TagCleanup",
            Self::VersionCheck => "VersionCheck",
            Self::OcrQueueAll => "OcrQueueAll",
            Self::Ocr => "Ocr",
            Self::WorkflowAssetTrigger => "WorkflowAssetTrigger",
            Self::IntegrityUntrackedFilesQueueAll => "IntegrityUntrackedFilesQueueAll",
            Self::IntegrityUntrackedFiles => "IntegrityUntrackedFiles",
            Self::IntegrityUntrackedRefresh => "IntegrityUntrackedRefresh",
            Self::IntegrityMissingFilesQueueAll => "IntegrityMissingFilesQueueAll",
            Self::IntegrityMissingFiles => "IntegrityMissingFiles",
            Self::IntegrityMissingFilesRefresh => "IntegrityMissingFilesRefresh",
            Self::IntegrityChecksumFiles => "IntegrityChecksumFiles",
            Self::IntegrityChecksumFilesRefresh => "IntegrityChecksumFilesRefresh",
            Self::IntegrityDeleteReportType => "IntegrityDeleteReportType",
            Self::IntegrityDeleteReports => "IntegrityDeleteReports",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for JobName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for JobName {
    fn from(value: &str) -> Self {
        match value {
            "AssetDelete" => Self::AssetDelete,
            "AssetDeleteCheck" => Self::AssetDeleteCheck,
            "AssetDetectFacesQueueAll" => Self::AssetDetectFacesQueueAll,
            "AssetDetectFaces" => Self::AssetDetectFaces,
            "AssetDetectDuplicatesQueueAll" => Self::AssetDetectDuplicatesQueueAll,
            "AssetDetectDuplicates" => Self::AssetDetectDuplicates,
            "AssetEditThumbnailGeneration" => Self::AssetEditThumbnailGeneration,
            "AssetEncodeVideoQueueAll" => Self::AssetEncodeVideoQueueAll,
            "AssetEncodeVideo" => Self::AssetEncodeVideo,
            "AssetEmptyTrash" => Self::AssetEmptyTrash,
            "AssetExtractMetadataQueueAll" => Self::AssetExtractMetadataQueueAll,
            "AssetExtractMetadata" => Self::AssetExtractMetadata,
            "AssetFileMigration" => Self::AssetFileMigration,
            "AssetGenerateThumbnailsQueueAll" => Self::AssetGenerateThumbnailsQueueAll,
            "AssetGenerateThumbnails" => Self::AssetGenerateThumbnails,
            "AuditTableCleanup" => Self::AuditTableCleanup,
            "DatabaseBackup" => Self::DatabaseBackup,
            "FacialRecognitionQueueAll" => Self::FacialRecognitionQueueAll,
            "FacialRecognition" => Self::FacialRecognition,
            "FileDelete" => Self::FileDelete,
            "FileMigrationQueueAll" => Self::FileMigrationQueueAll,
            "LibraryDeleteCheck" => Self::LibraryDeleteCheck,
            "LibraryDelete" => Self::LibraryDelete,
            "LibraryRemoveAsset" => Self::LibraryRemoveAsset,
            "LibraryScanAssetsQueueAll" => Self::LibraryScanAssetsQueueAll,
            "LibrarySyncAssets" => Self::LibrarySyncAssets,
            "LibrarySyncFilesQueueAll" => Self::LibrarySyncFilesQueueAll,
            "LibrarySyncFiles" => Self::LibrarySyncFiles,
            "LibraryScanQueueAll" => Self::LibraryScanQueueAll,
            "HlsSessionCleanup" => Self::HlsSessionCleanup,
            "MemoryCleanup" => Self::MemoryCleanup,
            "MemoryGenerate" => Self::MemoryGenerate,
            "NotificationsCleanup" => Self::NotificationsCleanup,
            "NotifyUserSignup" => Self::NotifyUserSignup,
            "NotifyAlbumInvite" => Self::NotifyAlbumInvite,
            "NotifyAlbumUpdate" => Self::NotifyAlbumUpdate,
            "UserDelete" => Self::UserDelete,
            "UserDeleteCheck" => Self::UserDeleteCheck,
            "UserSyncUsage" => Self::UserSyncUsage,
            "PersonCleanup" => Self::PersonCleanup,
            "PersonFileMigration" => Self::PersonFileMigration,
            "PersonGenerateThumbnail" => Self::PersonGenerateThumbnail,
            "SessionCleanup" => Self::SessionCleanup,
            "SendMail" => Self::SendMail,
            "SidecarQueueAll" => Self::SidecarQueueAll,
            "SidecarCheck" => Self::SidecarCheck,
            "SidecarWrite" => Self::SidecarWrite,
            "SmartSearchQueueAll" => Self::SmartSearchQueueAll,
            "SmartSearch" => Self::SmartSearch,
            "StorageTemplateMigration" => Self::StorageTemplateMigration,
            "StorageTemplateMigrationSingle" => Self::StorageTemplateMigrationSingle,
            "TagCleanup" => Self::TagCleanup,
            "VersionCheck" => Self::VersionCheck,
            "OcrQueueAll" => Self::OcrQueueAll,
            "Ocr" => Self::Ocr,
            "WorkflowAssetTrigger" => Self::WorkflowAssetTrigger,
            "IntegrityUntrackedFilesQueueAll" => Self::IntegrityUntrackedFilesQueueAll,
            "IntegrityUntrackedFiles" => Self::IntegrityUntrackedFiles,
            "IntegrityUntrackedRefresh" => Self::IntegrityUntrackedRefresh,
            "IntegrityMissingFilesQueueAll" => Self::IntegrityMissingFilesQueueAll,
            "IntegrityMissingFiles" => Self::IntegrityMissingFiles,
            "IntegrityMissingFilesRefresh" => Self::IntegrityMissingFilesRefresh,
            "IntegrityChecksumFiles" => Self::IntegrityChecksumFiles,
            "IntegrityChecksumFilesRefresh" => Self::IntegrityChecksumFilesRefresh,
            "IntegrityDeleteReportType" => Self::IntegrityDeleteReportType,
            "IntegrityDeleteReports" => Self::IntegrityDeleteReports,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for JobName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AssetDelete" => Self::AssetDelete,
            "AssetDeleteCheck" => Self::AssetDeleteCheck,
            "AssetDetectFacesQueueAll" => Self::AssetDetectFacesQueueAll,
            "AssetDetectFaces" => Self::AssetDetectFaces,
            "AssetDetectDuplicatesQueueAll" => Self::AssetDetectDuplicatesQueueAll,
            "AssetDetectDuplicates" => Self::AssetDetectDuplicates,
            "AssetEditThumbnailGeneration" => Self::AssetEditThumbnailGeneration,
            "AssetEncodeVideoQueueAll" => Self::AssetEncodeVideoQueueAll,
            "AssetEncodeVideo" => Self::AssetEncodeVideo,
            "AssetEmptyTrash" => Self::AssetEmptyTrash,
            "AssetExtractMetadataQueueAll" => Self::AssetExtractMetadataQueueAll,
            "AssetExtractMetadata" => Self::AssetExtractMetadata,
            "AssetFileMigration" => Self::AssetFileMigration,
            "AssetGenerateThumbnailsQueueAll" => Self::AssetGenerateThumbnailsQueueAll,
            "AssetGenerateThumbnails" => Self::AssetGenerateThumbnails,
            "AuditTableCleanup" => Self::AuditTableCleanup,
            "DatabaseBackup" => Self::DatabaseBackup,
            "FacialRecognitionQueueAll" => Self::FacialRecognitionQueueAll,
            "FacialRecognition" => Self::FacialRecognition,
            "FileDelete" => Self::FileDelete,
            "FileMigrationQueueAll" => Self::FileMigrationQueueAll,
            "LibraryDeleteCheck" => Self::LibraryDeleteCheck,
            "LibraryDelete" => Self::LibraryDelete,
            "LibraryRemoveAsset" => Self::LibraryRemoveAsset,
            "LibraryScanAssetsQueueAll" => Self::LibraryScanAssetsQueueAll,
            "LibrarySyncAssets" => Self::LibrarySyncAssets,
            "LibrarySyncFilesQueueAll" => Self::LibrarySyncFilesQueueAll,
            "LibrarySyncFiles" => Self::LibrarySyncFiles,
            "LibraryScanQueueAll" => Self::LibraryScanQueueAll,
            "HlsSessionCleanup" => Self::HlsSessionCleanup,
            "MemoryCleanup" => Self::MemoryCleanup,
            "MemoryGenerate" => Self::MemoryGenerate,
            "NotificationsCleanup" => Self::NotificationsCleanup,
            "NotifyUserSignup" => Self::NotifyUserSignup,
            "NotifyAlbumInvite" => Self::NotifyAlbumInvite,
            "NotifyAlbumUpdate" => Self::NotifyAlbumUpdate,
            "UserDelete" => Self::UserDelete,
            "UserDeleteCheck" => Self::UserDeleteCheck,
            "UserSyncUsage" => Self::UserSyncUsage,
            "PersonCleanup" => Self::PersonCleanup,
            "PersonFileMigration" => Self::PersonFileMigration,
            "PersonGenerateThumbnail" => Self::PersonGenerateThumbnail,
            "SessionCleanup" => Self::SessionCleanup,
            "SendMail" => Self::SendMail,
            "SidecarQueueAll" => Self::SidecarQueueAll,
            "SidecarCheck" => Self::SidecarCheck,
            "SidecarWrite" => Self::SidecarWrite,
            "SmartSearchQueueAll" => Self::SmartSearchQueueAll,
            "SmartSearch" => Self::SmartSearch,
            "StorageTemplateMigration" => Self::StorageTemplateMigration,
            "StorageTemplateMigrationSingle" => Self::StorageTemplateMigrationSingle,
            "TagCleanup" => Self::TagCleanup,
            "VersionCheck" => Self::VersionCheck,
            "OcrQueueAll" => Self::OcrQueueAll,
            "Ocr" => Self::Ocr,
            "WorkflowAssetTrigger" => Self::WorkflowAssetTrigger,
            "IntegrityUntrackedFilesQueueAll" => Self::IntegrityUntrackedFilesQueueAll,
            "IntegrityUntrackedFiles" => Self::IntegrityUntrackedFiles,
            "IntegrityUntrackedRefresh" => Self::IntegrityUntrackedRefresh,
            "IntegrityMissingFilesQueueAll" => Self::IntegrityMissingFilesQueueAll,
            "IntegrityMissingFiles" => Self::IntegrityMissingFiles,
            "IntegrityMissingFilesRefresh" => Self::IntegrityMissingFilesRefresh,
            "IntegrityChecksumFiles" => Self::IntegrityChecksumFiles,
            "IntegrityChecksumFilesRefresh" => Self::IntegrityChecksumFilesRefresh,
            "IntegrityDeleteReportType" => Self::IntegrityDeleteReportType,
            "IntegrityDeleteReports" => Self::IntegrityDeleteReports,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for JobName {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for JobName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for JobName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for JobName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = JobName;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(JobName::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(JobName::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `QueueDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `failed` | `bool` |  | If true, will also remove failed jobs from the queue. |
///
/// See: <https://api.immich.app/models/QueueDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct QueueDeleteDTO {
    /// If true, will also remove failed jobs from the queue.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub failed: Option<bool>,
}

impl QueueDeleteDTO {
    /// Creates a new [`QueueDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { failed: None }
    }
    /// Sets `failed`.
    #[must_use]
    pub fn failed(mut self, value: bool) -> Self {
        self.failed = Some(value);
        self
    }
    /// Sets or clears `failed`.
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
#[non_exhaustive]
pub struct QueueJobResponseDTO {
    /// Job data payload (required)
    pub data: ::serde_json::Map<String, ::serde_json::Value>,
    /// Job ID
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: Option<String>,
    /// (required)
    pub name: JobName,
    /// Job creation timestamp (required)
    pub timestamp: i64,
}

impl QueueJobResponseDTO {
    /// Creates a new [`QueueJobResponseDTO`].
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
    /// Sets `data`.
    #[must_use]
    pub fn data(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.data = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets or clears `id`.
    #[must_use]
    pub fn set_id(mut self, value: Option<String>) -> Self {
        self.id = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: JobName) -> Self {
        self.name = value;
        self
    }
    /// Sets `timestamp`.
    #[must_use]
    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = value;
        self
    }
}

/// Queue job status
///
/// See: <https://api.immich.app/models/QueueJobStatus>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum QueueJobStatus {
    Active,
    Failed,
    Completed,
    Delayed,
    Waiting,
    Paused,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl QueueJobStatus {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Failed => "failed",
            Self::Completed => "completed",
            Self::Delayed => "delayed",
            Self::Waiting => "waiting",
            Self::Paused => "paused",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for QueueJobStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for QueueJobStatus {
    fn from(value: &str) -> Self {
        match value {
            "active" => Self::Active,
            "failed" => Self::Failed,
            "completed" => Self::Completed,
            "delayed" => Self::Delayed,
            "waiting" => Self::Waiting,
            "paused" => Self::Paused,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for QueueJobStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "active" => Self::Active,
            "failed" => Self::Failed,
            "completed" => Self::Completed,
            "delayed" => Self::Delayed,
            "waiting" => Self::Waiting,
            "paused" => Self::Paused,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for QueueJobStatus {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for QueueJobStatus {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for QueueJobStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for QueueJobStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = QueueJobStatus;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(QueueJobStatus::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(QueueJobStatus::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct QueueResponseDTO {
    /// Whether the queue is paused (required)
    #[serde(rename = "isPaused")]
    pub is_paused: bool,
    /// (required)
    pub name: QueueName,
    /// (required)
    pub statistics: QueueStatisticsDTO,
}

impl QueueResponseDTO {
    /// Creates a new [`QueueResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_paused: bool, name: QueueName, statistics: QueueStatisticsDTO) -> Self {
        Self {
            is_paused,
            name,
            statistics,
        }
    }
    /// Sets `is_paused`.
    #[must_use]
    pub fn is_paused(mut self, value: bool) -> Self {
        self.is_paused = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: QueueName) -> Self {
        self.name = value;
        self
    }
    /// Sets `statistics`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct QueueUpdateDTO {
    /// Whether to pause the queue
    #[serde(rename = "isPaused", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_paused: Option<bool>,
}

impl QueueUpdateDTO {
    /// Creates a new [`QueueUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { is_paused: None }
    }
    /// Sets `is_paused`.
    #[must_use]
    pub fn is_paused(mut self, value: bool) -> Self {
        self.is_paused = Some(value);
        self
    }
    /// Sets or clears `is_paused`.
    #[must_use]
    pub fn set_is_paused(mut self, value: Option<bool>) -> Self {
        self.is_paused = value;
        self
    }
}
