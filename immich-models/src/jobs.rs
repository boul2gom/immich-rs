// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `jobs`.

/// `JobCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `name` | [`ManualJobName`] | ✓ |  |
///
/// See: <https://api.immich.app/models/JobCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct JobCreateDTO {
    /// (required)
    pub name: ManualJobName,
}

impl JobCreateDTO {
    /// Creates a new [`JobCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(name: ManualJobName) -> Self {
        Self { name }
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: ManualJobName) -> Self {
        self.name = value;
        self
    }
}

/// Manual job name
///
/// See: <https://api.immich.app/models/ManualJobName>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ManualJobName {
    PersonCleanup,
    TagCleanup,
    UserCleanup,
    MemoryCleanup,
    MemoryCreate,
    BackupDatabase,
    IntegrityMissingFiles,
    IntegrityUntrackedFiles,
    IntegrityChecksumMismatch,
    IntegrityMissingFilesRefresh,
    IntegrityUntrackedFilesRefresh,
    IntegrityChecksumMismatchRefresh,
    IntegrityMissingFilesDeleteAll,
    IntegrityUntrackedFilesDeleteAll,
    IntegrityChecksumMismatchDeleteAll,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl ManualJobName {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::PersonCleanup => "person-cleanup",
            Self::TagCleanup => "tag-cleanup",
            Self::UserCleanup => "user-cleanup",
            Self::MemoryCleanup => "memory-cleanup",
            Self::MemoryCreate => "memory-create",
            Self::BackupDatabase => "backup-database",
            Self::IntegrityMissingFiles => "integrity-missing-files",
            Self::IntegrityUntrackedFiles => "integrity-untracked-files",
            Self::IntegrityChecksumMismatch => "integrity-checksum-mismatch",
            Self::IntegrityMissingFilesRefresh => "integrity-missing-files-refresh",
            Self::IntegrityUntrackedFilesRefresh => "integrity-untracked-files-refresh",
            Self::IntegrityChecksumMismatchRefresh => "integrity-checksum-mismatch-refresh",
            Self::IntegrityMissingFilesDeleteAll => "integrity-missing-files-delete-all",
            Self::IntegrityUntrackedFilesDeleteAll => "integrity-untracked-files-delete-all",
            Self::IntegrityChecksumMismatchDeleteAll => "integrity-checksum-mismatch-delete-all",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for ManualJobName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for ManualJobName {
    fn from(value: &str) -> Self {
        match value {
            "person-cleanup" => Self::PersonCleanup,
            "tag-cleanup" => Self::TagCleanup,
            "user-cleanup" => Self::UserCleanup,
            "memory-cleanup" => Self::MemoryCleanup,
            "memory-create" => Self::MemoryCreate,
            "backup-database" => Self::BackupDatabase,
            "integrity-missing-files" => Self::IntegrityMissingFiles,
            "integrity-untracked-files" => Self::IntegrityUntrackedFiles,
            "integrity-checksum-mismatch" => Self::IntegrityChecksumMismatch,
            "integrity-missing-files-refresh" => Self::IntegrityMissingFilesRefresh,
            "integrity-untracked-files-refresh" => Self::IntegrityUntrackedFilesRefresh,
            "integrity-checksum-mismatch-refresh" => Self::IntegrityChecksumMismatchRefresh,
            "integrity-missing-files-delete-all" => Self::IntegrityMissingFilesDeleteAll,
            "integrity-untracked-files-delete-all" => Self::IntegrityUntrackedFilesDeleteAll,
            "integrity-checksum-mismatch-delete-all" => Self::IntegrityChecksumMismatchDeleteAll,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for ManualJobName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "person-cleanup" => Self::PersonCleanup,
            "tag-cleanup" => Self::TagCleanup,
            "user-cleanup" => Self::UserCleanup,
            "memory-cleanup" => Self::MemoryCleanup,
            "memory-create" => Self::MemoryCreate,
            "backup-database" => Self::BackupDatabase,
            "integrity-missing-files" => Self::IntegrityMissingFiles,
            "integrity-untracked-files" => Self::IntegrityUntrackedFiles,
            "integrity-checksum-mismatch" => Self::IntegrityChecksumMismatch,
            "integrity-missing-files-refresh" => Self::IntegrityMissingFilesRefresh,
            "integrity-untracked-files-refresh" => Self::IntegrityUntrackedFilesRefresh,
            "integrity-checksum-mismatch-refresh" => Self::IntegrityChecksumMismatchRefresh,
            "integrity-missing-files-delete-all" => Self::IntegrityMissingFilesDeleteAll,
            "integrity-untracked-files-delete-all" => Self::IntegrityUntrackedFilesDeleteAll,
            "integrity-checksum-mismatch-delete-all" => Self::IntegrityChecksumMismatchDeleteAll,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for ManualJobName {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for ManualJobName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for ManualJobName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for ManualJobName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = ManualJobName;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ManualJobName::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ManualJobName::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Queue command to execute
///
/// See: <https://api.immich.app/models/QueueCommand>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum QueueCommand {
    Start,
    Pause,
    Resume,
    Empty,
    ClearFailed,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl QueueCommand {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Start => "start",
            Self::Pause => "pause",
            Self::Resume => "resume",
            Self::Empty => "empty",
            Self::ClearFailed => "clear-failed",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for QueueCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for QueueCommand {
    fn from(value: &str) -> Self {
        match value {
            "start" => Self::Start,
            "pause" => Self::Pause,
            "resume" => Self::Resume,
            "empty" => Self::Empty,
            "clear-failed" => Self::ClearFailed,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for QueueCommand {
    fn from(value: String) -> Self {
        match value.as_str() {
            "start" => Self::Start,
            "pause" => Self::Pause,
            "resume" => Self::Resume,
            "empty" => Self::Empty,
            "clear-failed" => Self::ClearFailed,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for QueueCommand {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for QueueCommand {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for QueueCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for QueueCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = QueueCommand;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(QueueCommand::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(QueueCommand::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `QueueCommandDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `command` | [`QueueCommand`] | ✓ |  |
/// | `force` | `bool` |  | Force the command execution (if applicable) |
///
/// See: <https://api.immich.app/models/QueueCommandDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct QueueCommandDTO {
    /// (required)
    pub command: QueueCommand,
    /// Force the command execution (if applicable)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub force: Option<bool>,
}

impl QueueCommandDTO {
    /// Creates a new [`QueueCommandDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(command: QueueCommand) -> Self {
        Self { command, force: None }
    }
    /// Sets `command`.
    #[must_use]
    pub fn command(mut self, value: QueueCommand) -> Self {
        self.command = value;
        self
    }
    /// Sets `force`.
    #[must_use]
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }
    /// Sets or clears `force`.
    #[must_use]
    pub fn set_force(mut self, value: Option<bool>) -> Self {
        self.force = value;
        self
    }
}

/// Queue name
///
/// See: <https://api.immich.app/models/QueueName>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum QueueName {
    ThumbnailGeneration,
    MetadataExtraction,
    VideoConversion,
    FaceDetection,
    FacialRecognition,
    SmartSearch,
    DuplicateDetection,
    BackgroundTask,
    StorageTemplateMigration,
    Migration,
    Search,
    Sidecar,
    Library,
    Notifications,
    BackupDatabase,
    Ocr,
    Workflow,
    IntegrityCheck,
    Editor,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl QueueName {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::ThumbnailGeneration => "thumbnailGeneration",
            Self::MetadataExtraction => "metadataExtraction",
            Self::VideoConversion => "videoConversion",
            Self::FaceDetection => "faceDetection",
            Self::FacialRecognition => "facialRecognition",
            Self::SmartSearch => "smartSearch",
            Self::DuplicateDetection => "duplicateDetection",
            Self::BackgroundTask => "backgroundTask",
            Self::StorageTemplateMigration => "storageTemplateMigration",
            Self::Migration => "migration",
            Self::Search => "search",
            Self::Sidecar => "sidecar",
            Self::Library => "library",
            Self::Notifications => "notifications",
            Self::BackupDatabase => "backupDatabase",
            Self::Ocr => "ocr",
            Self::Workflow => "workflow",
            Self::IntegrityCheck => "integrityCheck",
            Self::Editor => "editor",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for QueueName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for QueueName {
    fn from(value: &str) -> Self {
        match value {
            "thumbnailGeneration" => Self::ThumbnailGeneration,
            "metadataExtraction" => Self::MetadataExtraction,
            "videoConversion" => Self::VideoConversion,
            "faceDetection" => Self::FaceDetection,
            "facialRecognition" => Self::FacialRecognition,
            "smartSearch" => Self::SmartSearch,
            "duplicateDetection" => Self::DuplicateDetection,
            "backgroundTask" => Self::BackgroundTask,
            "storageTemplateMigration" => Self::StorageTemplateMigration,
            "migration" => Self::Migration,
            "search" => Self::Search,
            "sidecar" => Self::Sidecar,
            "library" => Self::Library,
            "notifications" => Self::Notifications,
            "backupDatabase" => Self::BackupDatabase,
            "ocr" => Self::Ocr,
            "workflow" => Self::Workflow,
            "integrityCheck" => Self::IntegrityCheck,
            "editor" => Self::Editor,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for QueueName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "thumbnailGeneration" => Self::ThumbnailGeneration,
            "metadataExtraction" => Self::MetadataExtraction,
            "videoConversion" => Self::VideoConversion,
            "faceDetection" => Self::FaceDetection,
            "facialRecognition" => Self::FacialRecognition,
            "smartSearch" => Self::SmartSearch,
            "duplicateDetection" => Self::DuplicateDetection,
            "backgroundTask" => Self::BackgroundTask,
            "storageTemplateMigration" => Self::StorageTemplateMigration,
            "migration" => Self::Migration,
            "search" => Self::Search,
            "sidecar" => Self::Sidecar,
            "library" => Self::Library,
            "notifications" => Self::Notifications,
            "backupDatabase" => Self::BackupDatabase,
            "ocr" => Self::Ocr,
            "workflow" => Self::Workflow,
            "integrityCheck" => Self::IntegrityCheck,
            "editor" => Self::Editor,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for QueueName {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for QueueName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for QueueName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for QueueName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = QueueName;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(QueueName::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(QueueName::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `QueueResponseLegacyDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `job_counts` | [`QueueStatisticsDTO`] | ✓ |  |
/// | `queue_status` | [`QueueStatusLegacyDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/QueueResponseLegacyDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct QueueResponseLegacyDTO {
    /// (required)
    #[serde(rename = "jobCounts")]
    pub job_counts: QueueStatisticsDTO,
    /// (required)
    #[serde(rename = "queueStatus")]
    pub queue_status: QueueStatusLegacyDTO,
}

impl QueueResponseLegacyDTO {
    /// Creates a new [`QueueResponseLegacyDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(job_counts: QueueStatisticsDTO, queue_status: QueueStatusLegacyDTO) -> Self {
        Self {
            job_counts,
            queue_status,
        }
    }
    /// Sets `job_counts`.
    #[must_use]
    pub fn job_counts(mut self, value: QueueStatisticsDTO) -> Self {
        self.job_counts = value;
        self
    }
    /// Sets `queue_status`.
    #[must_use]
    pub fn queue_status(mut self, value: QueueStatusLegacyDTO) -> Self {
        self.queue_status = value;
        self
    }
}

/// `QueueStatisticsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `active` | `i64` | ✓ | Number of active jobs |
/// | `completed` | `i64` | ✓ | Number of completed jobs |
/// | `delayed` | `i64` | ✓ | Number of delayed jobs |
/// | `failed` | `i64` | ✓ | Number of failed jobs |
/// | `paused` | `i64` | ✓ | Number of paused jobs |
/// | `waiting` | `i64` | ✓ | Number of waiting jobs |
///
/// See: <https://api.immich.app/models/QueueStatisticsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct QueueStatisticsDTO {
    /// Number of active jobs (required)
    pub active: i64,
    /// Number of completed jobs (required)
    pub completed: i64,
    /// Number of delayed jobs (required)
    pub delayed: i64,
    /// Number of failed jobs (required)
    pub failed: i64,
    /// Number of paused jobs (required)
    pub paused: i64,
    /// Number of waiting jobs (required)
    pub waiting: i64,
}

impl QueueStatisticsDTO {
    /// Creates a new [`QueueStatisticsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(active: i64, completed: i64, delayed: i64, failed: i64, paused: i64, waiting: i64) -> Self {
        Self {
            active,
            completed,
            delayed,
            failed,
            paused,
            waiting,
        }
    }
    /// Sets `active`.
    #[must_use]
    pub fn active(mut self, value: i64) -> Self {
        self.active = value;
        self
    }
    /// Sets `completed`.
    #[must_use]
    pub fn completed(mut self, value: i64) -> Self {
        self.completed = value;
        self
    }
    /// Sets `delayed`.
    #[must_use]
    pub fn delayed(mut self, value: i64) -> Self {
        self.delayed = value;
        self
    }
    /// Sets `failed`.
    #[must_use]
    pub fn failed(mut self, value: i64) -> Self {
        self.failed = value;
        self
    }
    /// Sets `paused`.
    #[must_use]
    pub fn paused(mut self, value: i64) -> Self {
        self.paused = value;
        self
    }
    /// Sets `waiting`.
    #[must_use]
    pub fn waiting(mut self, value: i64) -> Self {
        self.waiting = value;
        self
    }
}

/// `QueueStatusLegacyDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `is_active` | `bool` | ✓ | Whether the queue is currently active (has running jobs) |
/// | `is_paused` | `bool` | ✓ | Whether the queue is paused |
///
/// See: <https://api.immich.app/models/QueueStatusLegacyDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct QueueStatusLegacyDTO {
    /// Whether the queue is currently active (has running jobs) (required)
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Whether the queue is paused (required)
    #[serde(rename = "isPaused")]
    pub is_paused: bool,
}

impl QueueStatusLegacyDTO {
    /// Creates a new [`QueueStatusLegacyDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_active: bool, is_paused: bool) -> Self {
        Self { is_active, is_paused }
    }
    /// Sets `is_active`.
    #[must_use]
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = value;
        self
    }
    /// Sets `is_paused`.
    #[must_use]
    pub fn is_paused(mut self, value: bool) -> Self {
        self.is_paused = value;
        self
    }
}

/// `QueuesResponseLegacyDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `background_task` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `backup_database` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `duplicate_detection` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `editor` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `face_detection` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `facial_recognition` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `integrity_check` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `library` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `metadata_extraction` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `migration` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `notifications` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `ocr` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `search` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `sidecar` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `smart_search` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `storage_template_migration` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `thumbnail_generation` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `video_conversion` | [`QueueResponseLegacyDTO`] | ✓ |  |
/// | `workflow` | [`QueueResponseLegacyDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/QueuesResponseLegacyDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct QueuesResponseLegacyDTO {
    /// (required)
    #[serde(rename = "backgroundTask")]
    pub background_task: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "backupDatabase")]
    pub backup_database: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "duplicateDetection")]
    pub duplicate_detection: QueueResponseLegacyDTO,
    /// (required)
    pub editor: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "faceDetection")]
    pub face_detection: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "facialRecognition")]
    pub facial_recognition: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "integrityCheck")]
    pub integrity_check: QueueResponseLegacyDTO,
    /// (required)
    pub library: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "metadataExtraction")]
    pub metadata_extraction: QueueResponseLegacyDTO,
    /// (required)
    pub migration: QueueResponseLegacyDTO,
    /// (required)
    pub notifications: QueueResponseLegacyDTO,
    /// (required)
    pub ocr: QueueResponseLegacyDTO,
    /// (required)
    pub search: QueueResponseLegacyDTO,
    /// (required)
    pub sidecar: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "smartSearch")]
    pub smart_search: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "storageTemplateMigration")]
    pub storage_template_migration: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "thumbnailGeneration")]
    pub thumbnail_generation: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "videoConversion")]
    pub video_conversion: QueueResponseLegacyDTO,
    /// (required)
    pub workflow: QueueResponseLegacyDTO,
}

impl QueuesResponseLegacyDTO {
    /// Creates a new [`QueuesResponseLegacyDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        background_task: QueueResponseLegacyDTO,
        backup_database: QueueResponseLegacyDTO,
        duplicate_detection: QueueResponseLegacyDTO,
        editor: QueueResponseLegacyDTO,
        face_detection: QueueResponseLegacyDTO,
        facial_recognition: QueueResponseLegacyDTO,
        integrity_check: QueueResponseLegacyDTO,
        library: QueueResponseLegacyDTO,
        metadata_extraction: QueueResponseLegacyDTO,
        migration: QueueResponseLegacyDTO,
        notifications: QueueResponseLegacyDTO,
        ocr: QueueResponseLegacyDTO,
        search: QueueResponseLegacyDTO,
        sidecar: QueueResponseLegacyDTO,
        smart_search: QueueResponseLegacyDTO,
        storage_template_migration: QueueResponseLegacyDTO,
        thumbnail_generation: QueueResponseLegacyDTO,
        video_conversion: QueueResponseLegacyDTO,
        workflow: QueueResponseLegacyDTO,
    ) -> Self {
        Self {
            background_task,
            backup_database,
            duplicate_detection,
            editor,
            face_detection,
            facial_recognition,
            integrity_check,
            library,
            metadata_extraction,
            migration,
            notifications,
            ocr,
            search,
            sidecar,
            smart_search,
            storage_template_migration,
            thumbnail_generation,
            video_conversion,
            workflow,
        }
    }
    /// Sets `background_task`.
    #[must_use]
    pub fn background_task(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.background_task = value;
        self
    }
    /// Sets `backup_database`.
    #[must_use]
    pub fn backup_database(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.backup_database = value;
        self
    }
    /// Sets `duplicate_detection`.
    #[must_use]
    pub fn duplicate_detection(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.duplicate_detection = value;
        self
    }
    /// Sets `editor`.
    #[must_use]
    pub fn editor(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.editor = value;
        self
    }
    /// Sets `face_detection`.
    #[must_use]
    pub fn face_detection(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.face_detection = value;
        self
    }
    /// Sets `facial_recognition`.
    #[must_use]
    pub fn facial_recognition(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.facial_recognition = value;
        self
    }
    /// Sets `integrity_check`.
    #[must_use]
    pub fn integrity_check(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.integrity_check = value;
        self
    }
    /// Sets `library`.
    #[must_use]
    pub fn library(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.library = value;
        self
    }
    /// Sets `metadata_extraction`.
    #[must_use]
    pub fn metadata_extraction(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.metadata_extraction = value;
        self
    }
    /// Sets `migration`.
    #[must_use]
    pub fn migration(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.migration = value;
        self
    }
    /// Sets `notifications`.
    #[must_use]
    pub fn notifications(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.notifications = value;
        self
    }
    /// Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.ocr = value;
        self
    }
    /// Sets `search`.
    #[must_use]
    pub fn search(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.search = value;
        self
    }
    /// Sets `sidecar`.
    #[must_use]
    pub fn sidecar(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.sidecar = value;
        self
    }
    /// Sets `smart_search`.
    #[must_use]
    pub fn smart_search(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.smart_search = value;
        self
    }
    /// Sets `storage_template_migration`.
    #[must_use]
    pub fn storage_template_migration(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.storage_template_migration = value;
        self
    }
    /// Sets `thumbnail_generation`.
    #[must_use]
    pub fn thumbnail_generation(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.thumbnail_generation = value;
        self
    }
    /// Sets `video_conversion`.
    #[must_use]
    pub fn video_conversion(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.video_conversion = value;
        self
    }
    /// Sets `workflow`.
    #[must_use]
    pub fn workflow(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.workflow = value;
        self
    }
}
