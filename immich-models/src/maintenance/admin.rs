// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `admin`.

/// Integrity report type
///
/// See: <https://api.immich.app/models/IntegrityReport>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum IntegrityReport {
    #[serde(rename = "untracked_file")]
    UntrackedFile,
    #[serde(rename = "missing_file")]
    MissingFile,
    #[serde(rename = "checksum_mismatch")]
    ChecksumMismatch,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for IntegrityReport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UntrackedFile => f.write_str("untracked_file"),
            Self::MissingFile => f.write_str("missing_file"),
            Self::ChecksumMismatch => f.write_str("checksum_mismatch"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for IntegrityReport {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "untracked_file" => Ok(Self::UntrackedFile),
            "missing_file" => Ok(Self::MissingFile),
            "checksum_mismatch" => Ok(Self::ChecksumMismatch),
            _ => Err(format!("invalid value for {}: {}", "IntegrityReport", value).into()),
        }
    }
}

impl TryFrom<&str> for IntegrityReport {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for IntegrityReport {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for IntegrityReport {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `IntegrityReportResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `items` | `Vec<object>` | ✓ |  |
/// | `next_cursor` | [`String`] |  |  |
///
/// See: <https://api.immich.app/models/IntegrityReportResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct IntegrityReportResponseDTO {
    pub items: Vec<IntegrityReportResponseItemsItem>,
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: Option<String>,
}

impl IntegrityReportResponseDTO {
    ///Creates a new [`IntegrityReportResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(items: Vec<IntegrityReportResponseItemsItem>) -> Self {
        Self {
            items,
            next_cursor: None,
        }
    }

    ///Sets `items`.
    #[must_use]
    pub fn items(mut self, value: Vec<IntegrityReportResponseItemsItem>) -> Self {
        self.items = value;
        self
    }

    ///Sets `next_cursor`.
    #[must_use]
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    ///Sets or clears `next_cursor`.
    #[must_use]
    pub fn set_next_cursor(mut self, value: Option<String>) -> Self {
        self.next_cursor = value;
        self
    }
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct IntegrityReportResponseItemsItem {
    ///Integrity report item id
    pub id: ::uuid::Uuid,
    ///Integrity report item path
    pub path: String,
    #[serde(rename = "type")]
    pub type_: IntegrityReport,
}

impl IntegrityReportResponseItemsItem {
    ///Creates a new [`IntegrityReportResponseItemsItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: ::uuid::Uuid, path: impl Into<String>, type_: IntegrityReport) -> Self {
        Self {
            id,
            path: path.into(),
            type_,
        }
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `path`.
    #[must_use]
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = value.into();
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: IntegrityReport) -> Self {
        self.type_ = value;
        self
    }
}

/// `IntegrityReportSummaryResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `checksum_mismatch` | `i64` | ✓ |  |
/// | `missing_file` | `i64` | ✓ |  |
/// | `untracked_file` | `i64` | ✓ |  |
///
/// See: <https://api.immich.app/models/IntegrityReportSummaryResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct IntegrityReportSummaryResponseDTO {
    pub checksum_mismatch: i64,
    pub missing_file: i64,
    pub untracked_file: i64,
}

impl IntegrityReportSummaryResponseDTO {
    ///Creates a new [`IntegrityReportSummaryResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(checksum_mismatch: i64, missing_file: i64, untracked_file: i64) -> Self {
        Self {
            checksum_mismatch,
            missing_file,
            untracked_file,
        }
    }

    ///Sets `checksum_mismatch`.
    #[must_use]
    pub fn checksum_mismatch(mut self, value: i64) -> Self {
        self.checksum_mismatch = value;
        self
    }

    ///Sets `missing_file`.
    #[must_use]
    pub fn missing_file(mut self, value: i64) -> Self {
        self.missing_file = value;
        self
    }

    ///Sets `untracked_file`.
    #[must_use]
    pub fn untracked_file(mut self, value: i64) -> Self {
        self.untracked_file = value;
        self
    }
}

/// Maintenance action
///
/// See: <https://api.immich.app/models/MaintenanceAction>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MaintenanceAction {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "select_database_restore")]
    SelectDatabaseRestore,
    #[serde(rename = "restore_database")]
    RestoreDatabase,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for MaintenanceAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Start => f.write_str("start"),
            Self::End => f.write_str("end"),
            Self::SelectDatabaseRestore => f.write_str("select_database_restore"),
            Self::RestoreDatabase => f.write_str("restore_database"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for MaintenanceAction {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "select_database_restore" => Ok(Self::SelectDatabaseRestore),
            "restore_database" => Ok(Self::RestoreDatabase),
            _ => Err(format!("invalid value for {}: {}", "MaintenanceAction", value).into()),
        }
    }
}

impl TryFrom<&str> for MaintenanceAction {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MaintenanceAction {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MaintenanceAction {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `MaintenanceAuthDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `username` | [`String`] | ✓ | Maintenance username |
///
/// See: <https://api.immich.app/models/MaintenanceAuthDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MaintenanceAuthDTO {
    pub username: String,
}

impl MaintenanceAuthDTO {
    ///Creates a new [`MaintenanceAuthDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
        }
    }

    ///Sets `username`.
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = value.into();
        self
    }
}

/// `MaintenanceDetectInstallResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `storage` | [`Vec<MaintenanceDetectInstallStorageFolderDTO>`](crate::maintenance::admin::MaintenanceDetectInstallStorageFolderDTO) | ✓ |  |
///
/// See: <https://api.immich.app/models/MaintenanceDetectInstallResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MaintenanceDetectInstallResponseDTO {
    pub storage: Vec<MaintenanceDetectInstallStorageFolderDTO>,
}

impl MaintenanceDetectInstallResponseDTO {
    ///Creates a new [`MaintenanceDetectInstallResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(storage: Vec<MaintenanceDetectInstallStorageFolderDTO>) -> Self {
        Self { storage }
    }

    ///Sets `storage`.
    #[must_use]
    pub fn storage(mut self, value: Vec<MaintenanceDetectInstallStorageFolderDTO>) -> Self {
        self.storage = value;
        self
    }
}

/// `MaintenanceDetectInstallStorageFolderDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `files` | `i64` | ✓ | Number of files in the folder |
/// | `folder` | [`StorageFolder`] | ✓ |  |
/// | `readable` | `bool` | ✓ | Whether the folder is readable |
/// | `writable` | `bool` | ✓ | Whether the folder is writable |
///
/// See: <https://api.immich.app/models/MaintenanceDetectInstallStorageFolderDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MaintenanceDetectInstallStorageFolderDTO {
    pub files: i64,
    pub folder: StorageFolder,
    pub readable: bool,
    pub writable: bool,
}

impl MaintenanceDetectInstallStorageFolderDTO {
    ///Creates a new [`MaintenanceDetectInstallStorageFolderDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(files: i64, folder: StorageFolder, readable: bool, writable: bool) -> Self {
        Self {
            files,
            folder,
            readable,
            writable,
        }
    }

    ///Sets `files`.
    #[must_use]
    pub fn files(mut self, value: i64) -> Self {
        self.files = value;
        self
    }

    ///Sets `folder`.
    #[must_use]
    pub fn folder(mut self, value: StorageFolder) -> Self {
        self.folder = value;
        self
    }

    ///Sets `readable`.
    #[must_use]
    pub fn readable(mut self, value: bool) -> Self {
        self.readable = value;
        self
    }

    ///Sets `writable`.
    #[must_use]
    pub fn writable(mut self, value: bool) -> Self {
        self.writable = value;
        self
    }
}

/// `MaintenanceLoginDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `token` | [`String`] |  | Maintenance token |
///
/// See: <https://api.immich.app/models/MaintenanceLoginDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct MaintenanceLoginDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: Option<String>,
}

impl MaintenanceLoginDTO {
    ///Creates a new [`MaintenanceLoginDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { token: None }
    }

    ///Sets `token`.
    #[must_use]
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    ///Sets or clears `token`.
    #[must_use]
    pub fn set_token(mut self, value: Option<String>) -> Self {
        self.token = value;
        self
    }
}

/// `MaintenanceStatusResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `action` | [`MaintenanceAction`] | ✓ |  |
/// | `active` | `bool` | ✓ |  |
/// | `error` | [`String`] |  |  |
/// | `progress` | `i64` |  |  |
/// | `task` | [`String`] |  |  |
///
/// See: <https://api.immich.app/models/MaintenanceStatusResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MaintenanceStatusResponseDTO {
    pub action: MaintenanceAction,
    pub active: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub task: Option<String>,
}

impl MaintenanceStatusResponseDTO {
    ///Creates a new [`MaintenanceStatusResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(action: MaintenanceAction, active: bool) -> Self {
        Self {
            action,
            active,
            error: None,
            progress: None,
            task: None,
        }
    }

    ///Sets `action`.
    #[must_use]
    pub fn action(mut self, value: MaintenanceAction) -> Self {
        self.action = value;
        self
    }

    ///Sets `active`.
    #[must_use]
    pub fn active(mut self, value: bool) -> Self {
        self.active = value;
        self
    }

    ///Sets `error`.
    #[must_use]
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    ///Sets or clears `error`.
    #[must_use]
    pub fn set_error(mut self, value: Option<String>) -> Self {
        self.error = value;
        self
    }

    ///Sets `progress`.
    #[must_use]
    pub fn progress(mut self, value: i64) -> Self {
        self.progress = Some(value);
        self
    }

    ///Sets or clears `progress`.
    #[must_use]
    pub fn set_progress(mut self, value: Option<i64>) -> Self {
        self.progress = value;
        self
    }

    ///Sets `task`.
    #[must_use]
    pub fn task(mut self, value: impl Into<String>) -> Self {
        self.task = Some(value.into());
        self
    }

    ///Sets or clears `task`.
    #[must_use]
    pub fn set_task(mut self, value: Option<String>) -> Self {
        self.task = value;
        self
    }
}

/// `SetMaintenanceModeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `action` | [`MaintenanceAction`] | ✓ |  |
/// | `restore_backup_filename` | [`String`] |  | Restore backup filename |
///
/// See: <https://api.immich.app/models/SetMaintenanceModeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SetMaintenanceModeDTO {
    pub action: MaintenanceAction,
    /// Restore backup filename
    #[serde(
        rename = "restoreBackupFilename",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub restore_backup_filename: Option<String>,
}

impl SetMaintenanceModeDTO {
    ///Creates a new [`SetMaintenanceModeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(action: MaintenanceAction) -> Self {
        Self {
            action,
            restore_backup_filename: None,
        }
    }

    ///Sets `action`.
    #[must_use]
    pub fn action(mut self, value: MaintenanceAction) -> Self {
        self.action = value;
        self
    }

    ///Sets `restore_backup_filename`.
    #[must_use]
    pub fn restore_backup_filename(mut self, value: impl Into<String>) -> Self {
        self.restore_backup_filename = Some(value.into());
        self
    }

    ///Sets or clears `restore_backup_filename`.
    #[must_use]
    pub fn set_restore_backup_filename(mut self, value: Option<String>) -> Self {
        self.restore_backup_filename = value;
        self
    }
}

/// Storage folder
///
/// See: <https://api.immich.app/models/StorageFolder>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum StorageFolder {
    #[serde(rename = "encoded-video")]
    EncodedVideo,
    #[serde(rename = "library")]
    Library,
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "thumbs")]
    Thumbs,
    #[serde(rename = "backups")]
    Backups,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for StorageFolder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::EncodedVideo => f.write_str("encoded-video"),
            Self::Library => f.write_str("library"),
            Self::Upload => f.write_str("upload"),
            Self::Profile => f.write_str("profile"),
            Self::Thumbs => f.write_str("thumbs"),
            Self::Backups => f.write_str("backups"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for StorageFolder {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "encoded-video" => Ok(Self::EncodedVideo),
            "library" => Ok(Self::Library),
            "upload" => Ok(Self::Upload),
            "profile" => Ok(Self::Profile),
            "thumbs" => Ok(Self::Thumbs),
            "backups" => Ok(Self::Backups),
            _ => Err(format!("invalid value for {}: {}", "StorageFolder", value).into()),
        }
    }
}

impl TryFrom<&str> for StorageFolder {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for StorageFolder {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for StorageFolder {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}
