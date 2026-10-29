// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `admin`.

/// `DatabaseBackupDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `backups` | [`Vec<String>`](::std::vec::Vec) | ✓ | Backup filenames to delete |
///
/// See: <https://api.immich.app/models/DatabaseBackupDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DatabaseBackupDeleteDTO {
    pub backups: Vec<String>,
}

impl DatabaseBackupDeleteDTO {
    ///Creates a new [`DatabaseBackupDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(backups: Vec<String>) -> Self {
        Self { backups }
    }

    ///Sets `backups`.
    #[must_use]
    pub fn backups(mut self, value: Vec<String>) -> Self {
        self.backups = value;
        self
    }
}

/// `DatabaseBackupDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `filename` | [`String`] | ✓ | Backup filename |
/// | `filesize` | `i64` | ✓ | Backup file size |
/// | `timezone` | [`String`] | ✓ | Backup timezone |
///
/// See: <https://api.immich.app/models/DatabaseBackupDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DatabaseBackupDTO {
    pub filename: String,
    pub filesize: i64,
    pub timezone: String,
}

impl DatabaseBackupDTO {
    ///Creates a new [`DatabaseBackupDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(filename: impl Into<String>, filesize: i64, timezone: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            filesize,
            timezone: timezone.into(),
        }
    }

    ///Sets `filename`.
    #[must_use]
    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = value.into();
        self
    }

    ///Sets `filesize`.
    #[must_use]
    pub fn filesize(mut self, value: i64) -> Self {
        self.filesize = value;
        self
    }

    ///Sets `timezone`.
    #[must_use]
    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = value.into();
        self
    }
}

/// `DatabaseBackupListResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `backups` | [`Vec<DatabaseBackupDTO>`](crate::database::admin::DatabaseBackupDTO) | ✓ | List of backups |
///
/// See: <https://api.immich.app/models/DatabaseBackupListResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DatabaseBackupListResponseDTO {
    pub backups: Vec<DatabaseBackupDTO>,
}

impl DatabaseBackupListResponseDTO {
    ///Creates a new [`DatabaseBackupListResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(backups: Vec<DatabaseBackupDTO>) -> Self {
        Self { backups }
    }

    ///Sets `backups`.
    #[must_use]
    pub fn backups(mut self, value: Vec<DatabaseBackupDTO>) -> Self {
        self.backups = value;
        self
    }
}

/// `DatabaseBackupUploadDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `file` | `String` |  | Database backup file |
///
/// See: <https://api.immich.app/models/DatabaseBackupUploadDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct DatabaseBackupUploadDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub file: Option<String>,
}

impl DatabaseBackupUploadDTO {
    ///Creates a new [`DatabaseBackupUploadDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { file: None }
    }

    ///Sets `file`.
    #[must_use]
    pub fn file(mut self, value: impl Into<String>) -> Self {
        self.file = Some(value.into());
        self
    }

    ///Sets or clears `file`.
    #[must_use]
    pub fn set_file(mut self, value: Option<String>) -> Self {
        self.file = value;
        self
    }
}
