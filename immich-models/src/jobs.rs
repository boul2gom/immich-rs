// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `jobs`.

/// `JobCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `name` | [`ManualJobName`] | ✓ |  |
///
/// See: <https://api.immich.app/models/JobCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct JobCreateDTO {
    pub name: ManualJobName,
}

impl JobCreateDTO {
    ///Creates a new [`JobCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(name: ManualJobName) -> Self {
        Self { name }
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: ManualJobName) -> Self {
        self.name = value;
        self
    }
}

/// Manual job name
///
/// See: <https://api.immich.app/models/ManualJobName>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ManualJobName {
    #[serde(rename = "person-cleanup")]
    PersonCleanup,
    #[serde(rename = "tag-cleanup")]
    TagCleanup,
    #[serde(rename = "user-cleanup")]
    UserCleanup,
    #[serde(rename = "memory-cleanup")]
    MemoryCleanup,
    #[serde(rename = "memory-create")]
    MemoryCreate,
    #[serde(rename = "backup-database")]
    BackupDatabase,
    #[serde(rename = "integrity-missing-files")]
    IntegrityMissingFiles,
    #[serde(rename = "integrity-untracked-files")]
    IntegrityUntrackedFiles,
    #[serde(rename = "integrity-checksum-mismatch")]
    IntegrityChecksumMismatch,
    #[serde(rename = "integrity-missing-files-refresh")]
    IntegrityMissingFilesRefresh,
    #[serde(rename = "integrity-untracked-files-refresh")]
    IntegrityUntrackedFilesRefresh,
    #[serde(rename = "integrity-checksum-mismatch-refresh")]
    IntegrityChecksumMismatchRefresh,
    #[serde(rename = "integrity-missing-files-delete-all")]
    IntegrityMissingFilesDeleteAll,
    #[serde(rename = "integrity-untracked-files-delete-all")]
    IntegrityUntrackedFilesDeleteAll,
    #[serde(rename = "integrity-checksum-mismatch-delete-all")]
    IntegrityChecksumMismatchDeleteAll,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for ManualJobName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PersonCleanup => f.write_str("person-cleanup"),
            Self::TagCleanup => f.write_str("tag-cleanup"),
            Self::UserCleanup => f.write_str("user-cleanup"),
            Self::MemoryCleanup => f.write_str("memory-cleanup"),
            Self::MemoryCreate => f.write_str("memory-create"),
            Self::BackupDatabase => f.write_str("backup-database"),
            Self::IntegrityMissingFiles => f.write_str("integrity-missing-files"),
            Self::IntegrityUntrackedFiles => f.write_str("integrity-untracked-files"),
            Self::IntegrityChecksumMismatch => f.write_str("integrity-checksum-mismatch"),
            Self::IntegrityMissingFilesRefresh => f.write_str("integrity-missing-files-refresh"),
            Self::IntegrityUntrackedFilesRefresh => f.write_str("integrity-untracked-files-refresh"),
            Self::IntegrityChecksumMismatchRefresh => f.write_str("integrity-checksum-mismatch-refresh"),
            Self::IntegrityMissingFilesDeleteAll => f.write_str("integrity-missing-files-delete-all"),
            Self::IntegrityUntrackedFilesDeleteAll => f.write_str("integrity-untracked-files-delete-all"),
            Self::IntegrityChecksumMismatchDeleteAll => f.write_str("integrity-checksum-mismatch-delete-all"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for ManualJobName {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "person-cleanup" => Ok(Self::PersonCleanup),
            "tag-cleanup" => Ok(Self::TagCleanup),
            "user-cleanup" => Ok(Self::UserCleanup),
            "memory-cleanup" => Ok(Self::MemoryCleanup),
            "memory-create" => Ok(Self::MemoryCreate),
            "backup-database" => Ok(Self::BackupDatabase),
            "integrity-missing-files" => Ok(Self::IntegrityMissingFiles),
            "integrity-untracked-files" => Ok(Self::IntegrityUntrackedFiles),
            "integrity-checksum-mismatch" => Ok(Self::IntegrityChecksumMismatch),
            "integrity-missing-files-refresh" => Ok(Self::IntegrityMissingFilesRefresh),
            "integrity-untracked-files-refresh" => Ok(Self::IntegrityUntrackedFilesRefresh),
            "integrity-checksum-mismatch-refresh" => Ok(Self::IntegrityChecksumMismatchRefresh),
            "integrity-missing-files-delete-all" => Ok(Self::IntegrityMissingFilesDeleteAll),
            "integrity-untracked-files-delete-all" => Ok(Self::IntegrityUntrackedFilesDeleteAll),
            "integrity-checksum-mismatch-delete-all" => Ok(Self::IntegrityChecksumMismatchDeleteAll),
            _ => Err(format!("invalid value for {}: {}", "ManualJobName", value).into()),
        }
    }
}

impl TryFrom<&str> for ManualJobName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for ManualJobName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for ManualJobName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Queue command to execute
///
/// See: <https://api.immich.app/models/QueueCommand>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum QueueCommand {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "resume")]
    Resume,
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "clear-failed")]
    ClearFailed,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for QueueCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Start => f.write_str("start"),
            Self::Pause => f.write_str("pause"),
            Self::Resume => f.write_str("resume"),
            Self::Empty => f.write_str("empty"),
            Self::ClearFailed => f.write_str("clear-failed"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for QueueCommand {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "start" => Ok(Self::Start),
            "pause" => Ok(Self::Pause),
            "resume" => Ok(Self::Resume),
            "empty" => Ok(Self::Empty),
            "clear-failed" => Ok(Self::ClearFailed),
            _ => Err(format!("invalid value for {}: {}", "QueueCommand", value).into()),
        }
    }
}

impl TryFrom<&str> for QueueCommand {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for QueueCommand {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for QueueCommand {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct QueueCommandDTO {
    pub command: QueueCommand,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub force: Option<bool>,
}

impl QueueCommandDTO {
    ///Creates a new [`QueueCommandDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(command: QueueCommand) -> Self {
        Self { command, force: None }
    }

    ///Sets `command`.
    #[must_use]
    pub fn command(mut self, value: QueueCommand) -> Self {
        self.command = value;
        self
    }

    ///Sets `force`.
    #[must_use]
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    ///Sets or clears `force`.
    #[must_use]
    pub fn set_force(mut self, value: Option<bool>) -> Self {
        self.force = value;
        self
    }
}

/// Queue name
///
/// See: <https://api.immich.app/models/QueueName>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum QueueName {
    #[serde(rename = "thumbnailGeneration")]
    ThumbnailGeneration,
    #[serde(rename = "metadataExtraction")]
    MetadataExtraction,
    #[serde(rename = "videoConversion")]
    VideoConversion,
    #[serde(rename = "faceDetection")]
    FaceDetection,
    #[serde(rename = "facialRecognition")]
    FacialRecognition,
    #[serde(rename = "smartSearch")]
    SmartSearch,
    #[serde(rename = "duplicateDetection")]
    DuplicateDetection,
    #[serde(rename = "backgroundTask")]
    BackgroundTask,
    #[serde(rename = "storageTemplateMigration")]
    StorageTemplateMigration,
    #[serde(rename = "migration")]
    Migration,
    #[serde(rename = "search")]
    Search,
    #[serde(rename = "sidecar")]
    Sidecar,
    #[serde(rename = "library")]
    Library,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "backupDatabase")]
    BackupDatabase,
    #[serde(rename = "ocr")]
    Ocr,
    #[serde(rename = "workflow")]
    Workflow,
    #[serde(rename = "integrityCheck")]
    IntegrityCheck,
    #[serde(rename = "editor")]
    Editor,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for QueueName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ThumbnailGeneration => f.write_str("thumbnailGeneration"),
            Self::MetadataExtraction => f.write_str("metadataExtraction"),
            Self::VideoConversion => f.write_str("videoConversion"),
            Self::FaceDetection => f.write_str("faceDetection"),
            Self::FacialRecognition => f.write_str("facialRecognition"),
            Self::SmartSearch => f.write_str("smartSearch"),
            Self::DuplicateDetection => f.write_str("duplicateDetection"),
            Self::BackgroundTask => f.write_str("backgroundTask"),
            Self::StorageTemplateMigration => f.write_str("storageTemplateMigration"),
            Self::Migration => f.write_str("migration"),
            Self::Search => f.write_str("search"),
            Self::Sidecar => f.write_str("sidecar"),
            Self::Library => f.write_str("library"),
            Self::Notifications => f.write_str("notifications"),
            Self::BackupDatabase => f.write_str("backupDatabase"),
            Self::Ocr => f.write_str("ocr"),
            Self::Workflow => f.write_str("workflow"),
            Self::IntegrityCheck => f.write_str("integrityCheck"),
            Self::Editor => f.write_str("editor"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for QueueName {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "thumbnailGeneration" => Ok(Self::ThumbnailGeneration),
            "metadataExtraction" => Ok(Self::MetadataExtraction),
            "videoConversion" => Ok(Self::VideoConversion),
            "faceDetection" => Ok(Self::FaceDetection),
            "facialRecognition" => Ok(Self::FacialRecognition),
            "smartSearch" => Ok(Self::SmartSearch),
            "duplicateDetection" => Ok(Self::DuplicateDetection),
            "backgroundTask" => Ok(Self::BackgroundTask),
            "storageTemplateMigration" => Ok(Self::StorageTemplateMigration),
            "migration" => Ok(Self::Migration),
            "search" => Ok(Self::Search),
            "sidecar" => Ok(Self::Sidecar),
            "library" => Ok(Self::Library),
            "notifications" => Ok(Self::Notifications),
            "backupDatabase" => Ok(Self::BackupDatabase),
            "ocr" => Ok(Self::Ocr),
            "workflow" => Ok(Self::Workflow),
            "integrityCheck" => Ok(Self::IntegrityCheck),
            "editor" => Ok(Self::Editor),
            _ => Err(format!("invalid value for {}: {}", "QueueName", value).into()),
        }
    }
}

impl TryFrom<&str> for QueueName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for QueueName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for QueueName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct QueueResponseLegacyDTO {
    /// (required)
    #[serde(rename = "jobCounts")]
    pub job_counts: QueueStatisticsDTO,
    /// (required)
    #[serde(rename = "queueStatus")]
    pub queue_status: QueueStatusLegacyDTO,
}

impl QueueResponseLegacyDTO {
    ///Creates a new [`QueueResponseLegacyDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(job_counts: QueueStatisticsDTO, queue_status: QueueStatusLegacyDTO) -> Self {
        Self {
            job_counts,
            queue_status,
        }
    }

    ///Sets `job_counts`.
    #[must_use]
    pub fn job_counts(mut self, value: QueueStatisticsDTO) -> Self {
        self.job_counts = value;
        self
    }

    ///Sets `queue_status`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct QueueStatisticsDTO {
    pub active: i64,
    pub completed: i64,
    pub delayed: i64,
    pub failed: i64,
    pub paused: i64,
    pub waiting: i64,
}

impl QueueStatisticsDTO {
    ///Creates a new [`QueueStatisticsDTO`].
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

    ///Sets `active`.
    #[must_use]
    pub fn active(mut self, value: i64) -> Self {
        self.active = value;
        self
    }

    ///Sets `completed`.
    #[must_use]
    pub fn completed(mut self, value: i64) -> Self {
        self.completed = value;
        self
    }

    ///Sets `delayed`.
    #[must_use]
    pub fn delayed(mut self, value: i64) -> Self {
        self.delayed = value;
        self
    }

    ///Sets `failed`.
    #[must_use]
    pub fn failed(mut self, value: i64) -> Self {
        self.failed = value;
        self
    }

    ///Sets `paused`.
    #[must_use]
    pub fn paused(mut self, value: i64) -> Self {
        self.paused = value;
        self
    }

    ///Sets `waiting`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct QueueStatusLegacyDTO {
    /// Whether the queue is currently active (has running jobs) (required)
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Whether the queue is paused (required)
    #[serde(rename = "isPaused")]
    pub is_paused: bool,
}

impl QueueStatusLegacyDTO {
    ///Creates a new [`QueueStatusLegacyDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_active: bool, is_paused: bool) -> Self {
        Self { is_active, is_paused }
    }

    ///Sets `is_active`.
    #[must_use]
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = value;
        self
    }

    ///Sets `is_paused`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
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
    pub library: QueueResponseLegacyDTO,
    /// (required)
    #[serde(rename = "metadataExtraction")]
    pub metadata_extraction: QueueResponseLegacyDTO,
    pub migration: QueueResponseLegacyDTO,
    pub notifications: QueueResponseLegacyDTO,
    pub ocr: QueueResponseLegacyDTO,
    pub search: QueueResponseLegacyDTO,
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
    pub workflow: QueueResponseLegacyDTO,
}

impl QueuesResponseLegacyDTO {
    ///Creates a new [`QueuesResponseLegacyDTO`].
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

    ///Sets `background_task`.
    #[must_use]
    pub fn background_task(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.background_task = value;
        self
    }

    ///Sets `backup_database`.
    #[must_use]
    pub fn backup_database(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.backup_database = value;
        self
    }

    ///Sets `duplicate_detection`.
    #[must_use]
    pub fn duplicate_detection(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.duplicate_detection = value;
        self
    }

    ///Sets `editor`.
    #[must_use]
    pub fn editor(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.editor = value;
        self
    }

    ///Sets `face_detection`.
    #[must_use]
    pub fn face_detection(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.face_detection = value;
        self
    }

    ///Sets `facial_recognition`.
    #[must_use]
    pub fn facial_recognition(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.facial_recognition = value;
        self
    }

    ///Sets `integrity_check`.
    #[must_use]
    pub fn integrity_check(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.integrity_check = value;
        self
    }

    ///Sets `library`.
    #[must_use]
    pub fn library(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.library = value;
        self
    }

    ///Sets `metadata_extraction`.
    #[must_use]
    pub fn metadata_extraction(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.metadata_extraction = value;
        self
    }

    ///Sets `migration`.
    #[must_use]
    pub fn migration(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.migration = value;
        self
    }

    ///Sets `notifications`.
    #[must_use]
    pub fn notifications(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.notifications = value;
        self
    }

    ///Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.ocr = value;
        self
    }

    ///Sets `search`.
    #[must_use]
    pub fn search(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.search = value;
        self
    }

    ///Sets `sidecar`.
    #[must_use]
    pub fn sidecar(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.sidecar = value;
        self
    }

    ///Sets `smart_search`.
    #[must_use]
    pub fn smart_search(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.smart_search = value;
        self
    }

    ///Sets `storage_template_migration`.
    #[must_use]
    pub fn storage_template_migration(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.storage_template_migration = value;
        self
    }

    ///Sets `thumbnail_generation`.
    #[must_use]
    pub fn thumbnail_generation(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.thumbnail_generation = value;
        self
    }

    ///Sets `video_conversion`.
    #[must_use]
    pub fn video_conversion(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.video_conversion = value;
        self
    }

    ///Sets `workflow`.
    #[must_use]
    pub fn workflow(mut self, value: QueueResponseLegacyDTO) -> Self {
        self.workflow = value;
        self
    }
}
