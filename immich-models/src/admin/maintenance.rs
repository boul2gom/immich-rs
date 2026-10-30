// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `maintenance`.

/// Integrity report type
///
/// See: <https://api.immich.app/models/IntegrityReport>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum IntegrityReport {
    UntrackedFile,
    MissingFile,
    ChecksumMismatch,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl IntegrityReport {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::UntrackedFile => "untracked_file",
            Self::MissingFile => "missing_file",
            Self::ChecksumMismatch => "checksum_mismatch",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for IntegrityReport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for IntegrityReport {
    fn from(value: &str) -> Self {
        match value {
            "untracked_file" => Self::UntrackedFile,
            "missing_file" => Self::MissingFile,
            "checksum_mismatch" => Self::ChecksumMismatch,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for IntegrityReport {
    fn from(value: String) -> Self {
        match value.as_str() {
            "untracked_file" => Self::UntrackedFile,
            "missing_file" => Self::MissingFile,
            "checksum_mismatch" => Self::ChecksumMismatch,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for IntegrityReport {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for IntegrityReport {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for IntegrityReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for IntegrityReport {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = IntegrityReport;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(IntegrityReport::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(IntegrityReport::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct IntegrityReportResponseDTO {
    /// (required)
    pub items: Vec<IntegrityReportResponseItemsItem>,
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: Option<String>,
}

impl IntegrityReportResponseDTO {
    /// Creates a new [`IntegrityReportResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(items: impl IntoIterator<Item = IntegrityReportResponseItemsItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
            next_cursor: None,
        }
    }
    /// Sets `items`.
    #[must_use]
    pub fn items(mut self, value: impl IntoIterator<Item = IntegrityReportResponseItemsItem>) -> Self {
        self.items = value.into_iter().collect();
        self
    }
    /// Sets `next_cursor`.
    #[must_use]
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }
    /// Sets or clears `next_cursor`.
    #[must_use]
    pub fn set_next_cursor(mut self, value: Option<String>) -> Self {
        self.next_cursor = value;
        self
    }
}

/// `IntegrityReportResponseDtoItemsItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct IntegrityReportResponseItemsItem {
    /// Integrity report item id
    pub id: ::uuid::Uuid,
    /// Integrity report item path
    pub path: String,
    #[serde(rename = "type")]
    pub type_: IntegrityReport,
}

impl IntegrityReportResponseItemsItem {
    /// Creates a new [`IntegrityReportResponseItemsItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: ::uuid::Uuid, path: impl Into<String>, type_: IntegrityReport) -> Self {
        Self {
            id,
            path: path.into(),
            type_,
        }
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }
    /// Sets `path`.
    #[must_use]
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = value.into();
        self
    }
    /// Sets `type_`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct IntegrityReportSummaryResponseDTO {
    /// (required)
    pub checksum_mismatch: i64,
    /// (required)
    pub missing_file: i64,
    /// (required)
    pub untracked_file: i64,
}

impl IntegrityReportSummaryResponseDTO {
    /// Creates a new [`IntegrityReportSummaryResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(checksum_mismatch: i64, missing_file: i64, untracked_file: i64) -> Self {
        Self {
            checksum_mismatch,
            missing_file,
            untracked_file,
        }
    }
    /// Sets `checksum_mismatch`.
    #[must_use]
    pub fn checksum_mismatch(mut self, value: i64) -> Self {
        self.checksum_mismatch = value;
        self
    }
    /// Sets `missing_file`.
    #[must_use]
    pub fn missing_file(mut self, value: i64) -> Self {
        self.missing_file = value;
        self
    }
    /// Sets `untracked_file`.
    #[must_use]
    pub fn untracked_file(mut self, value: i64) -> Self {
        self.untracked_file = value;
        self
    }
}

/// Maintenance action
///
/// See: <https://api.immich.app/models/MaintenanceAction>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MaintenanceAction {
    Start,
    End,
    SelectDatabaseRestore,
    RestoreDatabase,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl MaintenanceAction {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::SelectDatabaseRestore => "select_database_restore",
            Self::RestoreDatabase => "restore_database",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for MaintenanceAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for MaintenanceAction {
    fn from(value: &str) -> Self {
        match value {
            "start" => Self::Start,
            "end" => Self::End,
            "select_database_restore" => Self::SelectDatabaseRestore,
            "restore_database" => Self::RestoreDatabase,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for MaintenanceAction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "start" => Self::Start,
            "end" => Self::End,
            "select_database_restore" => Self::SelectDatabaseRestore,
            "restore_database" => Self::RestoreDatabase,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for MaintenanceAction {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for MaintenanceAction {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for MaintenanceAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for MaintenanceAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = MaintenanceAction;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MaintenanceAction::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MaintenanceAction::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `MaintenanceAuthDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `username` | [`String`] | ✓ | Maintenance username |
///
/// See: <https://api.immich.app/models/MaintenanceAuthDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MaintenanceAuthDTO {
    /// Maintenance username (required)
    pub username: String,
}

impl MaintenanceAuthDTO {
    /// Creates a new [`MaintenanceAuthDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
        }
    }
    /// Sets `username`.
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
/// | `storage` | [`Vec<MaintenanceDetectInstallStorageFolderDTO>`](crate::admin::maintenance::MaintenanceDetectInstallStorageFolderDTO) | ✓ |  |
///
/// See: <https://api.immich.app/models/MaintenanceDetectInstallResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MaintenanceDetectInstallResponseDTO {
    /// (required)
    pub storage: Vec<MaintenanceDetectInstallStorageFolderDTO>,
}

impl MaintenanceDetectInstallResponseDTO {
    /// Creates a new [`MaintenanceDetectInstallResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(storage: impl IntoIterator<Item = MaintenanceDetectInstallStorageFolderDTO>) -> Self {
        Self {
            storage: storage.into_iter().collect(),
        }
    }
    /// Sets `storage`.
    #[must_use]
    pub fn storage(mut self, value: impl IntoIterator<Item = MaintenanceDetectInstallStorageFolderDTO>) -> Self {
        self.storage = value.into_iter().collect();
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MaintenanceDetectInstallStorageFolderDTO {
    /// Number of files in the folder (required)
    pub files: i64,
    /// (required)
    pub folder: StorageFolder,
    /// Whether the folder is readable (required)
    pub readable: bool,
    /// Whether the folder is writable (required)
    pub writable: bool,
}

impl MaintenanceDetectInstallStorageFolderDTO {
    /// Creates a new [`MaintenanceDetectInstallStorageFolderDTO`].
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
    /// Sets `files`.
    #[must_use]
    pub fn files(mut self, value: i64) -> Self {
        self.files = value;
        self
    }
    /// Sets `folder`.
    #[must_use]
    pub fn folder(mut self, value: StorageFolder) -> Self {
        self.folder = value;
        self
    }
    /// Sets `readable`.
    #[must_use]
    pub fn readable(mut self, value: bool) -> Self {
        self.readable = value;
        self
    }
    /// Sets `writable`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct MaintenanceLoginDTO {
    /// Maintenance token
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: Option<String>,
}

impl MaintenanceLoginDTO {
    /// Creates a new [`MaintenanceLoginDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { token: None }
    }
    /// Sets `token`.
    #[must_use]
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }
    /// Sets or clears `token`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MaintenanceStatusResponseDTO {
    /// (required)
    pub action: MaintenanceAction,
    /// (required)
    pub active: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub task: Option<String>,
}

impl MaintenanceStatusResponseDTO {
    /// Creates a new [`MaintenanceStatusResponseDTO`].
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
    /// Sets `action`.
    #[must_use]
    pub fn action(mut self, value: MaintenanceAction) -> Self {
        self.action = value;
        self
    }
    /// Sets `active`.
    #[must_use]
    pub fn active(mut self, value: bool) -> Self {
        self.active = value;
        self
    }
    /// Sets `error`.
    #[must_use]
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }
    /// Sets or clears `error`.
    #[must_use]
    pub fn set_error(mut self, value: Option<String>) -> Self {
        self.error = value;
        self
    }
    /// Sets `progress`.
    #[must_use]
    pub fn progress(mut self, value: i64) -> Self {
        self.progress = Some(value);
        self
    }
    /// Sets or clears `progress`.
    #[must_use]
    pub fn set_progress(mut self, value: Option<i64>) -> Self {
        self.progress = value;
        self
    }
    /// Sets `task`.
    #[must_use]
    pub fn task(mut self, value: impl Into<String>) -> Self {
        self.task = Some(value.into());
        self
    }
    /// Sets or clears `task`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SetMaintenanceModeDTO {
    /// (required)
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
    /// Creates a new [`SetMaintenanceModeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(action: MaintenanceAction) -> Self {
        Self {
            action,
            restore_backup_filename: None,
        }
    }
    /// Sets `action`.
    #[must_use]
    pub fn action(mut self, value: MaintenanceAction) -> Self {
        self.action = value;
        self
    }
    /// Sets `restore_backup_filename`.
    #[must_use]
    pub fn restore_backup_filename(mut self, value: impl Into<String>) -> Self {
        self.restore_backup_filename = Some(value.into());
        self
    }
    /// Sets or clears `restore_backup_filename`.
    #[must_use]
    pub fn set_restore_backup_filename(mut self, value: Option<String>) -> Self {
        self.restore_backup_filename = value;
        self
    }
}

/// Storage folder
///
/// See: <https://api.immich.app/models/StorageFolder>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum StorageFolder {
    EncodedVideo,
    Library,
    Upload,
    Profile,
    Thumbs,
    Backups,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl StorageFolder {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::EncodedVideo => "encoded-video",
            Self::Library => "library",
            Self::Upload => "upload",
            Self::Profile => "profile",
            Self::Thumbs => "thumbs",
            Self::Backups => "backups",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for StorageFolder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for StorageFolder {
    fn from(value: &str) -> Self {
        match value {
            "encoded-video" => Self::EncodedVideo,
            "library" => Self::Library,
            "upload" => Self::Upload,
            "profile" => Self::Profile,
            "thumbs" => Self::Thumbs,
            "backups" => Self::Backups,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for StorageFolder {
    fn from(value: String) -> Self {
        match value.as_str() {
            "encoded-video" => Self::EncodedVideo,
            "library" => Self::Library,
            "upload" => Self::Upload,
            "profile" => Self::Profile,
            "thumbs" => Self::Thumbs,
            "backups" => Self::Backups,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for StorageFolder {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for StorageFolder {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for StorageFolder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for StorageFolder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = StorageFolder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(StorageFolder::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(StorageFolder::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}
