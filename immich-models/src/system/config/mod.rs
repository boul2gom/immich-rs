// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `config`.

pub mod notifications;

pub use crate::common::{CqMode, TranscodeHwAccel};

/// `AdminConfigBackupsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `database` | [`AdminConfigDatabaseBackupDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigBackupsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigBackupsDTO {
    /// (required)
    pub database: AdminConfigDatabaseBackupDTO,
}

impl AdminConfigBackupsDTO {
    /// Creates a new [`AdminConfigBackupsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(database: AdminConfigDatabaseBackupDTO) -> Self {
        Self { database }
    }
    /// Sets `database`.
    #[must_use]
    pub fn database(mut self, value: AdminConfigDatabaseBackupDTO) -> Self {
        self.database = value;
        self
    }
}

/// `AdminConfigClipDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether the task is enabled |
/// | `model_name` | [`String`] | ✓ | Name of the model to use |
///
/// See: <https://api.immich.app/models/AdminConfigClipDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigClipDTO {
    /// Whether the task is enabled (required)
    pub enabled: bool,
    /// Name of the model to use (required)
    #[serde(rename = "modelName")]
    pub model_name: String,
}

impl AdminConfigClipDTO {
    /// Creates a new [`AdminConfigClipDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, model_name: impl Into<String>) -> Self {
        Self {
            enabled,
            model_name: model_name.into(),
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `model_name`.
    #[must_use]
    pub fn model_name(mut self, value: impl Into<String>) -> Self {
        self.model_name = value.into();
        self
    }
}

/// `AdminConfigDatabaseBackupDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `cron_expression` | [`String`] | ✓ | Cron expression |
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `keep_last_amount` | `i64` | ✓ | Keep last amount |
///
/// See: <https://api.immich.app/models/AdminConfigDatabaseBackupDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigDatabaseBackupDTO {
    /// Cron expression (required)
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    /// Enabled (required)
    pub enabled: bool,
    /// Keep last amount (required)
    #[serde(rename = "keepLastAmount")]
    pub keep_last_amount: ::std::num::NonZeroU64,
}

impl AdminConfigDatabaseBackupDTO {
    /// Creates a new [`AdminConfigDatabaseBackupDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(cron_expression: impl Into<String>, enabled: bool, keep_last_amount: ::std::num::NonZeroU64) -> Self {
        Self {
            cron_expression: cron_expression.into(),
            enabled,
            keep_last_amount,
        }
    }
    /// Sets `cron_expression`.
    #[must_use]
    pub fn cron_expression(mut self, value: impl Into<String>) -> Self {
        self.cron_expression = value.into();
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `keep_last_amount`.
    #[must_use]
    pub fn keep_last_amount(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.keep_last_amount = value;
        self
    }
}

/// Configuration properties that are visible to the admin
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `backup` | [`AdminConfigBackupsDTO`] | ✓ |  |
/// | `ffmpeg` | [`AdminConfigFFmpegDTO`] | ✓ |  |
/// | `image` | [`AdminConfigImageDTO`] | ✓ |  |
/// | `integrity_checks` | [`AdminConfigIntegrityChecksDTO`] | ✓ |  |
/// | `job` | [`AdminConfigJobDTO`] | ✓ |  |
/// | `library` | [`AdminConfigLibraryDTO`] | ✓ |  |
/// | `logging` | [`AdminConfigLoggingDTO`] | ✓ |  |
/// | `machine_learning` | [`AdminConfigMachineLearningDTO`] | ✓ |  |
/// | `map` | [`AdminConfigMapDTO`] | ✓ |  |
/// | `metadata` | [`AdminConfigMetadataDTO`] | ✓ |  |
/// | `new_version_check` | [`AdminConfigNewVersionCheckDTO`] | ✓ |  |
/// | `nightly_tasks` | [`AdminConfigNightlyTasksDTO`] | ✓ |  |
/// | `notifications` | [`AdminConfigNotificationsDTO`] | ✓ |  |
/// | `oauth` | [`AdminConfigOAuthDTO`] | ✓ |  |
/// | `password_login` | [`AdminConfigPasswordLoginDTO`] | ✓ |  |
/// | `reverse_geocoding` | [`AdminConfigReverseGeocodingDTO`] | ✓ |  |
/// | `server` | [`AdminConfigServerDTO`] | ✓ |  |
/// | `storage_template` | [`AdminConfigStorageTemplateDTO`] | ✓ |  |
/// | `templates` | [`AdminConfigTemplatesDTO`] | ✓ |  |
/// | `theme` | [`AdminConfigThemeDTO`] | ✓ |  |
/// | `trash` | [`AdminConfigTrashDTO`] | ✓ |  |
/// | `user` | [`AdminConfigUserDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AdminConfigDTO {
    /// (required)
    pub backup: AdminConfigBackupsDTO,
    /// (required)
    pub ffmpeg: AdminConfigFFmpegDTO,
    /// (required)
    pub image: AdminConfigImageDTO,
    /// (required)
    #[serde(rename = "integrityChecks")]
    pub integrity_checks: AdminConfigIntegrityChecksDTO,
    /// (required)
    pub job: AdminConfigJobDTO,
    /// (required)
    pub library: AdminConfigLibraryDTO,
    /// (required)
    pub logging: AdminConfigLoggingDTO,
    /// (required)
    #[serde(rename = "machineLearning")]
    pub machine_learning: AdminConfigMachineLearningDTO,
    /// (required)
    pub map: AdminConfigMapDTO,
    /// (required)
    pub metadata: AdminConfigMetadataDTO,
    /// (required)
    #[serde(rename = "newVersionCheck")]
    pub new_version_check: AdminConfigNewVersionCheckDTO,
    /// (required)
    #[serde(rename = "nightlyTasks")]
    pub nightly_tasks: AdminConfigNightlyTasksDTO,
    /// (required)
    pub notifications: AdminConfigNotificationsDTO,
    /// (required)
    pub oauth: AdminConfigOAuthDTO,
    /// (required)
    #[serde(rename = "passwordLogin")]
    pub password_login: AdminConfigPasswordLoginDTO,
    /// (required)
    #[serde(rename = "reverseGeocoding")]
    pub reverse_geocoding: AdminConfigReverseGeocodingDTO,
    /// (required)
    pub server: AdminConfigServerDTO,
    /// (required)
    #[serde(rename = "storageTemplate")]
    pub storage_template: AdminConfigStorageTemplateDTO,
    /// (required)
    pub templates: AdminConfigTemplatesDTO,
    /// (required)
    pub theme: AdminConfigThemeDTO,
    /// (required)
    pub trash: AdminConfigTrashDTO,
    /// (required)
    pub user: AdminConfigUserDTO,
}

impl AdminConfigDTO {
    /// Creates a new [`AdminConfigDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        backup: AdminConfigBackupsDTO,
        ffmpeg: AdminConfigFFmpegDTO,
        image: AdminConfigImageDTO,
        integrity_checks: AdminConfigIntegrityChecksDTO,
        job: AdminConfigJobDTO,
        library: AdminConfigLibraryDTO,
        logging: AdminConfigLoggingDTO,
        machine_learning: AdminConfigMachineLearningDTO,
        map: AdminConfigMapDTO,
        metadata: AdminConfigMetadataDTO,
        new_version_check: AdminConfigNewVersionCheckDTO,
        nightly_tasks: AdminConfigNightlyTasksDTO,
        notifications: AdminConfigNotificationsDTO,
        oauth: AdminConfigOAuthDTO,
        password_login: AdminConfigPasswordLoginDTO,
        reverse_geocoding: AdminConfigReverseGeocodingDTO,
        server: AdminConfigServerDTO,
        storage_template: AdminConfigStorageTemplateDTO,
        templates: AdminConfigTemplatesDTO,
        theme: AdminConfigThemeDTO,
        trash: AdminConfigTrashDTO,
        user: AdminConfigUserDTO,
    ) -> Self {
        Self {
            backup,
            ffmpeg,
            image,
            integrity_checks,
            job,
            library,
            logging,
            machine_learning,
            map,
            metadata,
            new_version_check,
            nightly_tasks,
            notifications,
            oauth,
            password_login,
            reverse_geocoding,
            server,
            storage_template,
            templates,
            theme,
            trash,
            user,
        }
    }
    /// Sets `backup`.
    #[must_use]
    pub fn backup(mut self, value: AdminConfigBackupsDTO) -> Self {
        self.backup = value;
        self
    }
    /// Sets `ffmpeg`.
    #[must_use]
    pub fn ffmpeg(mut self, value: AdminConfigFFmpegDTO) -> Self {
        self.ffmpeg = value;
        self
    }
    /// Sets `image`.
    #[must_use]
    pub fn image(mut self, value: AdminConfigImageDTO) -> Self {
        self.image = value;
        self
    }
    /// Sets `integrity_checks`.
    #[must_use]
    pub fn integrity_checks(mut self, value: AdminConfigIntegrityChecksDTO) -> Self {
        self.integrity_checks = value;
        self
    }
    /// Sets `job`.
    #[must_use]
    pub fn job(mut self, value: AdminConfigJobDTO) -> Self {
        self.job = value;
        self
    }
    /// Sets `library`.
    #[must_use]
    pub fn library(mut self, value: AdminConfigLibraryDTO) -> Self {
        self.library = value;
        self
    }
    /// Sets `logging`.
    #[must_use]
    pub fn logging(mut self, value: AdminConfigLoggingDTO) -> Self {
        self.logging = value;
        self
    }
    /// Sets `machine_learning`.
    #[must_use]
    pub fn machine_learning(mut self, value: AdminConfigMachineLearningDTO) -> Self {
        self.machine_learning = value;
        self
    }
    /// Sets `map`.
    #[must_use]
    pub fn map(mut self, value: AdminConfigMapDTO) -> Self {
        self.map = value;
        self
    }
    /// Sets `metadata`.
    #[must_use]
    pub fn metadata(mut self, value: AdminConfigMetadataDTO) -> Self {
        self.metadata = value;
        self
    }
    /// Sets `new_version_check`.
    #[must_use]
    pub fn new_version_check(mut self, value: AdminConfigNewVersionCheckDTO) -> Self {
        self.new_version_check = value;
        self
    }
    /// Sets `nightly_tasks`.
    #[must_use]
    pub fn nightly_tasks(mut self, value: AdminConfigNightlyTasksDTO) -> Self {
        self.nightly_tasks = value;
        self
    }
    /// Sets `notifications`.
    #[must_use]
    pub fn notifications(mut self, value: AdminConfigNotificationsDTO) -> Self {
        self.notifications = value;
        self
    }
    /// Sets `oauth`.
    #[must_use]
    pub fn oauth(mut self, value: AdminConfigOAuthDTO) -> Self {
        self.oauth = value;
        self
    }
    /// Sets `password_login`.
    #[must_use]
    pub fn password_login(mut self, value: AdminConfigPasswordLoginDTO) -> Self {
        self.password_login = value;
        self
    }
    /// Sets `reverse_geocoding`.
    #[must_use]
    pub fn reverse_geocoding(mut self, value: AdminConfigReverseGeocodingDTO) -> Self {
        self.reverse_geocoding = value;
        self
    }
    /// Sets `server`.
    #[must_use]
    pub fn server(mut self, value: AdminConfigServerDTO) -> Self {
        self.server = value;
        self
    }
    /// Sets `storage_template`.
    #[must_use]
    pub fn storage_template(mut self, value: AdminConfigStorageTemplateDTO) -> Self {
        self.storage_template = value;
        self
    }
    /// Sets `templates`.
    #[must_use]
    pub fn templates(mut self, value: AdminConfigTemplatesDTO) -> Self {
        self.templates = value;
        self
    }
    /// Sets `theme`.
    #[must_use]
    pub fn theme(mut self, value: AdminConfigThemeDTO) -> Self {
        self.theme = value;
        self
    }
    /// Sets `trash`.
    #[must_use]
    pub fn trash(mut self, value: AdminConfigTrashDTO) -> Self {
        self.trash = value;
        self
    }
    /// Sets `user`.
    #[must_use]
    pub fn user(mut self, value: AdminConfigUserDTO) -> Self {
        self.user = value;
        self
    }
}

/// `AdminConfigDuplicateDetectionDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether the task is enabled |
/// | `max_distance` | `f64` | ✓ | Maximum distance threshold for duplicate detection |
///
/// See: <https://api.immich.app/models/AdminConfigDuplicateDetectionDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AdminConfigDuplicateDetectionDTO {
    /// Whether the task is enabled (required)
    pub enabled: bool,
    /// Maximum distance threshold for duplicate detection (required)
    #[serde(rename = "maxDistance")]
    pub max_distance: f64,
}

impl AdminConfigDuplicateDetectionDTO {
    /// Creates a new [`AdminConfigDuplicateDetectionDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, max_distance: f64) -> Self {
        Self { enabled, max_distance }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `max_distance`.
    #[must_use]
    pub fn max_distance(mut self, value: f64) -> Self {
        self.max_distance = value;
        self
    }
}

/// `AdminConfigFFmpegDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `accel` | `TranscodeHWAccel` | ✓ |  |
/// | `accel_decode` | `bool` | ✓ | Accelerated decode |
/// | `accepted_audio_codecs` | [`Vec<AudioCodec>`](crate::system::config::AudioCodec) | ✓ | Accepted audio codecs |
/// | `accepted_containers` | [`Vec<VideoContainer>`](crate::system::config::VideoContainer) | ✓ | Accepted containers |
/// | `accepted_video_codecs` | [`Vec<VideoCodec>`](crate::system::config::VideoCodec) | ✓ | Accepted video codecs |
/// | `bframes` | `i64` | ✓ | B-frames |
/// | `cq_mode` | `CQMode` | ✓ |  |
/// | `crf` | `i64` | ✓ | CRF |
/// | `gop_size` | `i64` | ✓ | GOP size |
/// | `max_bitrate` | [`String`] | ✓ | Max bitrate |
/// | `preferred_hw_device` | [`String`] | ✓ | Preferred hardware device |
/// | `preset` | [`String`] | ✓ | Preset |
/// | `realtime` | [`AdminConfigFFmpegRealtimeDTO`] | ✓ |  |
/// | `refs` | `i64` | ✓ | References |
/// | `target_audio_codec` | [`AudioCodec`] | ✓ |  |
/// | `target_resolution` | [`String`] | ✓ | Target resolution |
/// | `target_video_codec` | [`VideoCodec`] | ✓ |  |
/// | `temporal_aq` | `bool` | ✓ | Temporal AQ |
/// | `threads` | `i64` | ✓ | Threads |
/// | `tonemap` | [`ToneMapping`] | ✓ |  |
/// | `transcode` | [`TranscodePolicy`] | ✓ |  |
/// | `two_pass` | `bool` | ✓ | Two pass |
///
/// See: <https://api.immich.app/models/AdminConfigFFmpegDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigFFmpegDTO {
    /// (required)
    pub accel: TranscodeHwAccel,
    /// Accelerated decode (required)
    #[serde(rename = "accelDecode")]
    pub accel_decode: bool,
    /// Accepted audio codecs (required)
    #[serde(rename = "acceptedAudioCodecs")]
    pub accepted_audio_codecs: Vec<AudioCodec>,
    /// Accepted containers (required)
    #[serde(rename = "acceptedContainers")]
    pub accepted_containers: Vec<VideoContainer>,
    /// Accepted video codecs (required)
    #[serde(rename = "acceptedVideoCodecs")]
    pub accepted_video_codecs: Vec<VideoCodec>,
    /// B-frames (required)
    pub bframes: i64,
    /// (required)
    #[serde(rename = "cqMode")]
    pub cq_mode: CqMode,
    /// CRF (required)
    pub crf: i64,
    /// GOP size (required)
    #[serde(rename = "gopSize")]
    pub gop_size: i64,
    /// Max bitrate (required)
    #[serde(rename = "maxBitrate")]
    pub max_bitrate: String,
    /// Preferred hardware device (required)
    #[serde(rename = "preferredHwDevice")]
    pub preferred_hw_device: String,
    /// Preset (required)
    pub preset: String,
    /// (required)
    pub realtime: AdminConfigFFmpegRealtimeDTO,
    /// References (required)
    pub refs: i64,
    /// (required)
    #[serde(rename = "targetAudioCodec")]
    pub target_audio_codec: AudioCodec,
    /// Target resolution (required)
    #[serde(rename = "targetResolution")]
    pub target_resolution: String,
    /// (required)
    #[serde(rename = "targetVideoCodec")]
    pub target_video_codec: VideoCodec,
    /// Temporal AQ (required)
    #[serde(rename = "temporalAQ")]
    pub temporal_aq: bool,
    /// Threads (required)
    pub threads: i64,
    /// (required)
    pub tonemap: ToneMapping,
    /// (required)
    pub transcode: TranscodePolicy,
    /// Two pass (required)
    #[serde(rename = "twoPass")]
    pub two_pass: bool,
}

impl AdminConfigFFmpegDTO {
    /// Creates a new [`AdminConfigFFmpegDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        accel: TranscodeHwAccel,
        accel_decode: bool,
        accepted_audio_codecs: impl IntoIterator<Item = AudioCodec>,
        accepted_containers: impl IntoIterator<Item = VideoContainer>,
        accepted_video_codecs: impl IntoIterator<Item = VideoCodec>,
        bframes: i64,
        cq_mode: CqMode,
        crf: i64,
        gop_size: i64,
        max_bitrate: impl Into<String>,
        preferred_hw_device: impl Into<String>,
        preset: impl Into<String>,
        realtime: AdminConfigFFmpegRealtimeDTO,
        refs: i64,
        target_audio_codec: AudioCodec,
        target_resolution: impl Into<String>,
        target_video_codec: VideoCodec,
        temporal_aq: bool,
        threads: i64,
        tonemap: ToneMapping,
        transcode: TranscodePolicy,
        two_pass: bool,
    ) -> Self {
        Self {
            accel,
            accel_decode,
            accepted_audio_codecs: accepted_audio_codecs.into_iter().collect(),
            accepted_containers: accepted_containers.into_iter().collect(),
            accepted_video_codecs: accepted_video_codecs.into_iter().collect(),
            bframes,
            cq_mode,
            crf,
            gop_size,
            max_bitrate: max_bitrate.into(),
            preferred_hw_device: preferred_hw_device.into(),
            preset: preset.into(),
            realtime,
            refs,
            target_audio_codec,
            target_resolution: target_resolution.into(),
            target_video_codec,
            temporal_aq,
            threads,
            tonemap,
            transcode,
            two_pass,
        }
    }
    /// Sets `accel`.
    #[must_use]
    pub fn accel(mut self, value: TranscodeHwAccel) -> Self {
        self.accel = value;
        self
    }
    /// Sets `accel_decode`.
    #[must_use]
    pub fn accel_decode(mut self, value: bool) -> Self {
        self.accel_decode = value;
        self
    }
    /// Sets `accepted_audio_codecs`.
    #[must_use]
    pub fn accepted_audio_codecs(mut self, value: impl IntoIterator<Item = AudioCodec>) -> Self {
        self.accepted_audio_codecs = value.into_iter().collect();
        self
    }
    /// Sets `accepted_containers`.
    #[must_use]
    pub fn accepted_containers(mut self, value: impl IntoIterator<Item = VideoContainer>) -> Self {
        self.accepted_containers = value.into_iter().collect();
        self
    }
    /// Sets `accepted_video_codecs`.
    #[must_use]
    pub fn accepted_video_codecs(mut self, value: impl IntoIterator<Item = VideoCodec>) -> Self {
        self.accepted_video_codecs = value.into_iter().collect();
        self
    }
    /// Sets `bframes`.
    #[must_use]
    pub fn bframes(mut self, value: i64) -> Self {
        self.bframes = value;
        self
    }
    /// Sets `cq_mode`.
    #[must_use]
    pub fn cq_mode(mut self, value: CqMode) -> Self {
        self.cq_mode = value;
        self
    }
    /// Sets `crf`.
    #[must_use]
    pub fn crf(mut self, value: i64) -> Self {
        self.crf = value;
        self
    }
    /// Sets `gop_size`.
    #[must_use]
    pub fn gop_size(mut self, value: i64) -> Self {
        self.gop_size = value;
        self
    }
    /// Sets `max_bitrate`.
    #[must_use]
    pub fn max_bitrate(mut self, value: impl Into<String>) -> Self {
        self.max_bitrate = value.into();
        self
    }
    /// Sets `preferred_hw_device`.
    #[must_use]
    pub fn preferred_hw_device(mut self, value: impl Into<String>) -> Self {
        self.preferred_hw_device = value.into();
        self
    }
    /// Sets `preset`.
    #[must_use]
    pub fn preset(mut self, value: impl Into<String>) -> Self {
        self.preset = value.into();
        self
    }
    /// Sets `realtime`.
    #[must_use]
    pub fn realtime(mut self, value: AdminConfigFFmpegRealtimeDTO) -> Self {
        self.realtime = value;
        self
    }
    /// Sets `refs`.
    #[must_use]
    pub fn refs(mut self, value: i64) -> Self {
        self.refs = value;
        self
    }
    /// Sets `target_audio_codec`.
    #[must_use]
    pub fn target_audio_codec(mut self, value: AudioCodec) -> Self {
        self.target_audio_codec = value;
        self
    }
    /// Sets `target_resolution`.
    #[must_use]
    pub fn target_resolution(mut self, value: impl Into<String>) -> Self {
        self.target_resolution = value.into();
        self
    }
    /// Sets `target_video_codec`.
    #[must_use]
    pub fn target_video_codec(mut self, value: VideoCodec) -> Self {
        self.target_video_codec = value;
        self
    }
    /// Sets `temporal_aq`.
    #[must_use]
    pub fn temporal_aq(mut self, value: bool) -> Self {
        self.temporal_aq = value;
        self
    }
    /// Sets `threads`.
    #[must_use]
    pub fn threads(mut self, value: i64) -> Self {
        self.threads = value;
        self
    }
    /// Sets `tonemap`.
    #[must_use]
    pub fn tonemap(mut self, value: ToneMapping) -> Self {
        self.tonemap = value;
        self
    }
    /// Sets `transcode`.
    #[must_use]
    pub fn transcode(mut self, value: TranscodePolicy) -> Self {
        self.transcode = value;
        self
    }
    /// Sets `two_pass`.
    #[must_use]
    pub fn two_pass(mut self, value: bool) -> Self {
        self.two_pass = value;
        self
    }
}

/// `AdminConfigFFmpegRealtimeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enable real-time HLS transcoding (alpha) |
/// | `resolutions` | [`Vec<HlsVideoResolution>`](crate::system::config::HlsVideoResolution) | ✓ | Resolutions to use for real-time HLS transcoding |
/// | `video_codecs` | [`Vec<VideoCodec>`](crate::system::config::VideoCodec) | ✓ | Video codecs to use for real-time HLS transcoding |
///
/// See: <https://api.immich.app/models/AdminConfigFFmpegRealtimeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigFFmpegRealtimeDTO {
    /// Enable real-time HLS transcoding (alpha) (required)
    pub enabled: bool,
    /// Resolutions to use for real-time HLS transcoding (required)
    pub resolutions: Vec<HlsVideoResolution>,
    /// Video codecs to use for real-time HLS transcoding (required)
    #[serde(rename = "videoCodecs")]
    pub video_codecs: Vec<VideoCodec>,
}

impl AdminConfigFFmpegRealtimeDTO {
    /// Creates a new [`AdminConfigFFmpegRealtimeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        enabled: bool,
        resolutions: impl IntoIterator<Item = HlsVideoResolution>,
        video_codecs: impl IntoIterator<Item = VideoCodec>,
    ) -> Self {
        Self {
            enabled,
            resolutions: resolutions.into_iter().collect(),
            video_codecs: video_codecs.into_iter().collect(),
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `resolutions`.
    #[must_use]
    pub fn resolutions(mut self, value: impl IntoIterator<Item = HlsVideoResolution>) -> Self {
        self.resolutions = value.into_iter().collect();
        self
    }
    /// Sets `video_codecs`.
    #[must_use]
    pub fn video_codecs(mut self, value: impl IntoIterator<Item = VideoCodec>) -> Self {
        self.video_codecs = value.into_iter().collect();
        self
    }
}

/// `AdminConfigFacesDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `import` | `bool` | ✓ | Import |
///
/// See: <https://api.immich.app/models/AdminConfigFacesDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigFacesDTO {
    /// Import (required)
    pub import: bool,
}

impl AdminConfigFacesDTO {
    /// Creates a new [`AdminConfigFacesDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(import: bool) -> Self {
        Self { import }
    }
    /// Sets `import`.
    #[must_use]
    pub fn import(mut self, value: bool) -> Self {
        self.import = value;
        self
    }
}

/// `AdminConfigFacialRecognitionDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether the task is enabled |
/// | `max_distance` | `f64` | ✓ | Maximum distance threshold for face recognition |
/// | `min_faces` | `i64` | ✓ | Minimum number of faces required for recognition |
/// | `min_score` | `f64` | ✓ | Minimum confidence score for face detection |
/// | `model_name` | [`String`] | ✓ | Name of the model to use |
///
/// See: <https://api.immich.app/models/AdminConfigFacialRecognitionDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AdminConfigFacialRecognitionDTO {
    /// Whether the task is enabled (required)
    pub enabled: bool,
    /// Maximum distance threshold for face recognition (required)
    #[serde(rename = "maxDistance")]
    pub max_distance: f64,
    /// Minimum number of faces required for recognition (required)
    #[serde(rename = "minFaces")]
    pub min_faces: ::std::num::NonZeroU64,
    /// Minimum confidence score for face detection (required)
    #[serde(rename = "minScore")]
    pub min_score: f64,
    /// Name of the model to use (required)
    #[serde(rename = "modelName")]
    pub model_name: String,
}

impl AdminConfigFacialRecognitionDTO {
    /// Creates a new [`AdminConfigFacialRecognitionDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        enabled: bool,
        max_distance: f64,
        min_faces: ::std::num::NonZeroU64,
        min_score: f64,
        model_name: impl Into<String>,
    ) -> Self {
        Self {
            enabled,
            max_distance,
            min_faces,
            min_score,
            model_name: model_name.into(),
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `max_distance`.
    #[must_use]
    pub fn max_distance(mut self, value: f64) -> Self {
        self.max_distance = value;
        self
    }
    /// Sets `min_faces`.
    #[must_use]
    pub fn min_faces(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.min_faces = value;
        self
    }
    /// Sets `min_score`.
    #[must_use]
    pub fn min_score(mut self, value: f64) -> Self {
        self.min_score = value;
        self
    }
    /// Sets `model_name`.
    #[must_use]
    pub fn model_name(mut self, value: impl Into<String>) -> Self {
        self.model_name = value.into();
        self
    }
}

/// `AdminConfigGeneratedFullsizeImageDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `format` | [`ImageFormat`] | ✓ |  |
/// | `progressive` | `bool` |  | Progressive |
/// | `quality` | `i64` | ✓ | Quality |
///
/// See: <https://api.immich.app/models/AdminConfigGeneratedFullsizeImageDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigGeneratedFullsizeImageDTO {
    /// Enabled (required)
    pub enabled: bool,
    /// (required)
    pub format: ImageFormat,
    /// Progressive
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub progressive: Option<bool>,
    /// Quality (required)
    pub quality: ::std::num::NonZeroU64,
}

impl AdminConfigGeneratedFullsizeImageDTO {
    /// Creates a new [`AdminConfigGeneratedFullsizeImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, format: ImageFormat, quality: ::std::num::NonZeroU64) -> Self {
        Self {
            enabled,
            format,
            progressive: None,
            quality,
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `format`.
    #[must_use]
    pub fn format(mut self, value: ImageFormat) -> Self {
        self.format = value;
        self
    }
    /// Sets `progressive`.
    #[must_use]
    pub fn progressive(mut self, value: bool) -> Self {
        self.progressive = Some(value);
        self
    }
    /// Sets or clears `progressive`.
    #[must_use]
    pub fn set_progressive(mut self, value: Option<bool>) -> Self {
        self.progressive = value;
        self
    }
    /// Sets `quality`.
    #[must_use]
    pub fn quality(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.quality = value;
        self
    }
}

/// `AdminConfigGeneratedImageDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `format` | [`ImageFormat`] | ✓ |  |
/// | `progressive` | `bool` |  | Progressive |
/// | `quality` | `i64` | ✓ | Quality |
/// | `size` | `i64` | ✓ | Size |
///
/// See: <https://api.immich.app/models/AdminConfigGeneratedImageDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigGeneratedImageDTO {
    /// (required)
    pub format: ImageFormat,
    /// Progressive
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub progressive: Option<bool>,
    /// Quality (required)
    pub quality: ::std::num::NonZeroU64,
    /// Size (required)
    pub size: ::std::num::NonZeroU64,
}

impl AdminConfigGeneratedImageDTO {
    /// Creates a new [`AdminConfigGeneratedImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(format: ImageFormat, quality: ::std::num::NonZeroU64, size: ::std::num::NonZeroU64) -> Self {
        Self {
            format,
            progressive: None,
            quality,
            size,
        }
    }
    /// Sets `format`.
    #[must_use]
    pub fn format(mut self, value: ImageFormat) -> Self {
        self.format = value;
        self
    }
    /// Sets `progressive`.
    #[must_use]
    pub fn progressive(mut self, value: bool) -> Self {
        self.progressive = Some(value);
        self
    }
    /// Sets or clears `progressive`.
    #[must_use]
    pub fn set_progressive(mut self, value: Option<bool>) -> Self {
        self.progressive = value;
        self
    }
    /// Sets `quality`.
    #[must_use]
    pub fn quality(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.quality = value;
        self
    }
    /// Sets `size`.
    #[must_use]
    pub fn size(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.size = value;
        self
    }
}

/// `AdminConfigImageDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `colorspace` | [`Colorspace`] | ✓ |  |
/// | `extract_embedded` | `bool` | ✓ | Extract embedded |
/// | `fullsize` | [`AdminConfigGeneratedFullsizeImageDTO`] | ✓ |  |
/// | `preview` | [`AdminConfigGeneratedImageDTO`] | ✓ |  |
/// | `thumbnail` | [`AdminConfigGeneratedImageDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigImageDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigImageDTO {
    /// (required)
    pub colorspace: Colorspace,
    /// Extract embedded (required)
    #[serde(rename = "extractEmbedded")]
    pub extract_embedded: bool,
    /// (required)
    pub fullsize: AdminConfigGeneratedFullsizeImageDTO,
    /// (required)
    pub preview: AdminConfigGeneratedImageDTO,
    /// (required)
    pub thumbnail: AdminConfigGeneratedImageDTO,
}

impl AdminConfigImageDTO {
    /// Creates a new [`AdminConfigImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        colorspace: Colorspace,
        extract_embedded: bool,
        fullsize: AdminConfigGeneratedFullsizeImageDTO,
        preview: AdminConfigGeneratedImageDTO,
        thumbnail: AdminConfigGeneratedImageDTO,
    ) -> Self {
        Self {
            colorspace,
            extract_embedded,
            fullsize,
            preview,
            thumbnail,
        }
    }
    /// Sets `colorspace`.
    #[must_use]
    pub fn colorspace(mut self, value: Colorspace) -> Self {
        self.colorspace = value;
        self
    }
    /// Sets `extract_embedded`.
    #[must_use]
    pub fn extract_embedded(mut self, value: bool) -> Self {
        self.extract_embedded = value;
        self
    }
    /// Sets `fullsize`.
    #[must_use]
    pub fn fullsize(mut self, value: AdminConfigGeneratedFullsizeImageDTO) -> Self {
        self.fullsize = value;
        self
    }
    /// Sets `preview`.
    #[must_use]
    pub fn preview(mut self, value: AdminConfigGeneratedImageDTO) -> Self {
        self.preview = value;
        self
    }
    /// Sets `thumbnail`.
    #[must_use]
    pub fn thumbnail(mut self, value: AdminConfigGeneratedImageDTO) -> Self {
        self.thumbnail = value;
        self
    }
}

/// Integrity checks config
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `checksum_files` | [`AdminConfigIntegrityChecksumJobDTO`] | ✓ |  |
/// | `missing_files` | [`AdminConfigIntegrityJobDTO`] | ✓ |  |
/// | `untracked_files` | [`AdminConfigIntegrityJobDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigIntegrityChecksDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AdminConfigIntegrityChecksDTO {
    /// (required)
    #[serde(rename = "checksumFiles")]
    pub checksum_files: AdminConfigIntegrityChecksumJobDTO,
    /// (required)
    #[serde(rename = "missingFiles")]
    pub missing_files: AdminConfigIntegrityJobDTO,
    /// (required)
    #[serde(rename = "untrackedFiles")]
    pub untracked_files: AdminConfigIntegrityJobDTO,
}

impl AdminConfigIntegrityChecksDTO {
    /// Creates a new [`AdminConfigIntegrityChecksDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        checksum_files: AdminConfigIntegrityChecksumJobDTO,
        missing_files: AdminConfigIntegrityJobDTO,
        untracked_files: AdminConfigIntegrityJobDTO,
    ) -> Self {
        Self {
            checksum_files,
            missing_files,
            untracked_files,
        }
    }
    /// Sets `checksum_files`.
    #[must_use]
    pub fn checksum_files(mut self, value: AdminConfigIntegrityChecksumJobDTO) -> Self {
        self.checksum_files = value;
        self
    }
    /// Sets `missing_files`.
    #[must_use]
    pub fn missing_files(mut self, value: AdminConfigIntegrityJobDTO) -> Self {
        self.missing_files = value;
        self
    }
    /// Sets `untracked_files`.
    #[must_use]
    pub fn untracked_files(mut self, value: AdminConfigIntegrityJobDTO) -> Self {
        self.untracked_files = value;
        self
    }
}

/// Integrity checksum job config
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `cron_expression` | [`String`] | ✓ | Cron expression for when the integrity check should run |
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `percentage_limit` | `f64` | ✓ | Percentage limit of the integrity checksum job |
/// | `time_limit` | `i64` | ✓ | How long the integrity checksum job may run for |
///
/// See: <https://api.immich.app/models/AdminConfigIntegrityChecksumJobDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AdminConfigIntegrityChecksumJobDTO {
    /// Cron expression for when the integrity check should run (required)
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    /// Enabled (required)
    pub enabled: bool,
    /// Percentage limit of the integrity checksum job (required)
    #[serde(rename = "percentageLimit")]
    pub percentage_limit: f64,
    /// How long the integrity checksum job may run for (required)
    #[serde(rename = "timeLimit")]
    pub time_limit: i64,
}

impl AdminConfigIntegrityChecksumJobDTO {
    /// Creates a new [`AdminConfigIntegrityChecksumJobDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(cron_expression: impl Into<String>, enabled: bool, percentage_limit: f64, time_limit: i64) -> Self {
        Self {
            cron_expression: cron_expression.into(),
            enabled,
            percentage_limit,
            time_limit,
        }
    }
    /// Sets `cron_expression`.
    #[must_use]
    pub fn cron_expression(mut self, value: impl Into<String>) -> Self {
        self.cron_expression = value.into();
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `percentage_limit`.
    #[must_use]
    pub fn percentage_limit(mut self, value: f64) -> Self {
        self.percentage_limit = value;
        self
    }
    /// Sets `time_limit`.
    #[must_use]
    pub fn time_limit(mut self, value: i64) -> Self {
        self.time_limit = value;
        self
    }
}

/// Integrity job config
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `cron_expression` | [`String`] | ✓ | Cron expression for when the integrity check should run |
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/AdminConfigIntegrityJobDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigIntegrityJobDTO {
    /// Cron expression for when the integrity check should run (required)
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    /// Enabled (required)
    pub enabled: bool,
}

impl AdminConfigIntegrityJobDTO {
    /// Creates a new [`AdminConfigIntegrityJobDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(cron_expression: impl Into<String>, enabled: bool) -> Self {
        Self {
            cron_expression: cron_expression.into(),
            enabled,
        }
    }
    /// Sets `cron_expression`.
    #[must_use]
    pub fn cron_expression(mut self, value: impl Into<String>) -> Self {
        self.cron_expression = value.into();
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `AdminConfigJobDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `background_task` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `editor` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `face_detection` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `integrity_check` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `library` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `metadata_extraction` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `migration` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `notifications` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `ocr` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `search` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `sidecar` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `smart_search` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `thumbnail_generation` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `video_conversion` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
/// | `workflow` | [`AdminConfigJobSettingsDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigJobDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigJobDTO {
    /// (required)
    #[serde(rename = "backgroundTask")]
    pub background_task: AdminConfigJobSettingsDTO,
    /// (required)
    pub editor: AdminConfigJobSettingsDTO,
    /// (required)
    #[serde(rename = "faceDetection")]
    pub face_detection: AdminConfigJobSettingsDTO,
    /// (required)
    #[serde(rename = "integrityCheck")]
    pub integrity_check: AdminConfigJobSettingsDTO,
    /// (required)
    pub library: AdminConfigJobSettingsDTO,
    /// (required)
    #[serde(rename = "metadataExtraction")]
    pub metadata_extraction: AdminConfigJobSettingsDTO,
    /// (required)
    pub migration: AdminConfigJobSettingsDTO,
    /// (required)
    pub notifications: AdminConfigJobSettingsDTO,
    /// (required)
    pub ocr: AdminConfigJobSettingsDTO,
    /// (required)
    pub search: AdminConfigJobSettingsDTO,
    /// (required)
    pub sidecar: AdminConfigJobSettingsDTO,
    /// (required)
    #[serde(rename = "smartSearch")]
    pub smart_search: AdminConfigJobSettingsDTO,
    /// (required)
    #[serde(rename = "thumbnailGeneration")]
    pub thumbnail_generation: AdminConfigJobSettingsDTO,
    /// (required)
    #[serde(rename = "videoConversion")]
    pub video_conversion: AdminConfigJobSettingsDTO,
    /// (required)
    pub workflow: AdminConfigJobSettingsDTO,
}

impl AdminConfigJobDTO {
    /// Creates a new [`AdminConfigJobDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        background_task: AdminConfigJobSettingsDTO,
        editor: AdminConfigJobSettingsDTO,
        face_detection: AdminConfigJobSettingsDTO,
        integrity_check: AdminConfigJobSettingsDTO,
        library: AdminConfigJobSettingsDTO,
        metadata_extraction: AdminConfigJobSettingsDTO,
        migration: AdminConfigJobSettingsDTO,
        notifications: AdminConfigJobSettingsDTO,
        ocr: AdminConfigJobSettingsDTO,
        search: AdminConfigJobSettingsDTO,
        sidecar: AdminConfigJobSettingsDTO,
        smart_search: AdminConfigJobSettingsDTO,
        thumbnail_generation: AdminConfigJobSettingsDTO,
        video_conversion: AdminConfigJobSettingsDTO,
        workflow: AdminConfigJobSettingsDTO,
    ) -> Self {
        Self {
            background_task,
            editor,
            face_detection,
            integrity_check,
            library,
            metadata_extraction,
            migration,
            notifications,
            ocr,
            search,
            sidecar,
            smart_search,
            thumbnail_generation,
            video_conversion,
            workflow,
        }
    }
    /// Sets `background_task`.
    #[must_use]
    pub fn background_task(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.background_task = value;
        self
    }
    /// Sets `editor`.
    #[must_use]
    pub fn editor(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.editor = value;
        self
    }
    /// Sets `face_detection`.
    #[must_use]
    pub fn face_detection(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.face_detection = value;
        self
    }
    /// Sets `integrity_check`.
    #[must_use]
    pub fn integrity_check(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.integrity_check = value;
        self
    }
    /// Sets `library`.
    #[must_use]
    pub fn library(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.library = value;
        self
    }
    /// Sets `metadata_extraction`.
    #[must_use]
    pub fn metadata_extraction(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.metadata_extraction = value;
        self
    }
    /// Sets `migration`.
    #[must_use]
    pub fn migration(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.migration = value;
        self
    }
    /// Sets `notifications`.
    #[must_use]
    pub fn notifications(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.notifications = value;
        self
    }
    /// Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.ocr = value;
        self
    }
    /// Sets `search`.
    #[must_use]
    pub fn search(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.search = value;
        self
    }
    /// Sets `sidecar`.
    #[must_use]
    pub fn sidecar(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.sidecar = value;
        self
    }
    /// Sets `smart_search`.
    #[must_use]
    pub fn smart_search(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.smart_search = value;
        self
    }
    /// Sets `thumbnail_generation`.
    #[must_use]
    pub fn thumbnail_generation(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.thumbnail_generation = value;
        self
    }
    /// Sets `video_conversion`.
    #[must_use]
    pub fn video_conversion(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.video_conversion = value;
        self
    }
    /// Sets `workflow`.
    #[must_use]
    pub fn workflow(mut self, value: AdminConfigJobSettingsDTO) -> Self {
        self.workflow = value;
        self
    }
}

/// `AdminConfigJobSettingsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `concurrency` | `i64` | ✓ | Concurrency |
///
/// See: <https://api.immich.app/models/AdminConfigJobSettingsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigJobSettingsDTO {
    /// Concurrency (required)
    pub concurrency: ::std::num::NonZeroU64,
}

impl AdminConfigJobSettingsDTO {
    /// Creates a new [`AdminConfigJobSettingsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(concurrency: ::std::num::NonZeroU64) -> Self {
        Self { concurrency }
    }
    /// Sets `concurrency`.
    #[must_use]
    pub fn concurrency(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.concurrency = value;
        self
    }
}

/// `AdminConfigLibraryDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `scan` | [`AdminConfigLibraryScanDTO`] | ✓ |  |
/// | `watch` | [`AdminConfigLibraryWatchDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigLibraryDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigLibraryDTO {
    /// (required)
    pub scan: AdminConfigLibraryScanDTO,
    /// (required)
    pub watch: AdminConfigLibraryWatchDTO,
}

impl AdminConfigLibraryDTO {
    /// Creates a new [`AdminConfigLibraryDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(scan: AdminConfigLibraryScanDTO, watch: AdminConfigLibraryWatchDTO) -> Self {
        Self { scan, watch }
    }
    /// Sets `scan`.
    #[must_use]
    pub fn scan(mut self, value: AdminConfigLibraryScanDTO) -> Self {
        self.scan = value;
        self
    }
    /// Sets `watch`.
    #[must_use]
    pub fn watch(mut self, value: AdminConfigLibraryWatchDTO) -> Self {
        self.watch = value;
        self
    }
}

/// `AdminConfigLibraryScanDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `cron_expression` | [`String`] | ✓ | Cron expression |
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/AdminConfigLibraryScanDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigLibraryScanDTO {
    /// Cron expression (required)
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    /// Enabled (required)
    pub enabled: bool,
}

impl AdminConfigLibraryScanDTO {
    /// Creates a new [`AdminConfigLibraryScanDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(cron_expression: impl Into<String>, enabled: bool) -> Self {
        Self {
            cron_expression: cron_expression.into(),
            enabled,
        }
    }
    /// Sets `cron_expression`.
    #[must_use]
    pub fn cron_expression(mut self, value: impl Into<String>) -> Self {
        self.cron_expression = value.into();
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `AdminConfigLibraryWatchDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/AdminConfigLibraryWatchDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigLibraryWatchDTO {
    /// Enabled (required)
    pub enabled: bool,
}

impl AdminConfigLibraryWatchDTO {
    /// Creates a new [`AdminConfigLibraryWatchDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `AdminConfigLoggingDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `level` | [`LogLevel`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigLoggingDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigLoggingDTO {
    /// Enabled (required)
    pub enabled: bool,
    /// (required)
    pub level: LogLevel,
}

impl AdminConfigLoggingDTO {
    /// Creates a new [`AdminConfigLoggingDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, level: LogLevel) -> Self {
        Self { enabled, level }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `level`.
    #[must_use]
    pub fn level(mut self, value: LogLevel) -> Self {
        self.level = value;
        self
    }
}

/// `AdminConfigMachineLearningAvailabilityChecksDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `interval` | `i64` | ✓ |  |
/// | `timeout` | `i64` | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigMachineLearningAvailabilityChecksDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigMachineLearningAvailabilityChecksDTO {
    /// Enabled (required)
    pub enabled: bool,
    /// (required)
    pub interval: i64,
    /// (required)
    pub timeout: i64,
}

impl AdminConfigMachineLearningAvailabilityChecksDTO {
    /// Creates a new [`AdminConfigMachineLearningAvailabilityChecksDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, interval: i64, timeout: i64) -> Self {
        Self {
            enabled,
            interval,
            timeout,
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `interval`.
    #[must_use]
    pub fn interval(mut self, value: i64) -> Self {
        self.interval = value;
        self
    }
    /// Sets `timeout`.
    #[must_use]
    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = value;
        self
    }
}

/// `AdminConfigMachineLearningDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `availability_checks` | [`AdminConfigMachineLearningAvailabilityChecksDTO`] | ✓ |  |
/// | `clip` | [`AdminConfigClipDTO`] | ✓ |  |
/// | `duplicate_detection` | [`AdminConfigDuplicateDetectionDTO`] | ✓ |  |
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `facial_recognition` | [`AdminConfigFacialRecognitionDTO`] | ✓ |  |
/// | `ocr` | [`AdminConfigOcrDTO`] | ✓ |  |
/// | `urls` | [`Vec<String>`](::std::vec::Vec) | ✓ | ML service URLs |
///
/// See: <https://api.immich.app/models/AdminConfigMachineLearningDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AdminConfigMachineLearningDTO {
    /// (required)
    #[serde(rename = "availabilityChecks")]
    pub availability_checks: AdminConfigMachineLearningAvailabilityChecksDTO,
    /// (required)
    pub clip: AdminConfigClipDTO,
    /// (required)
    #[serde(rename = "duplicateDetection")]
    pub duplicate_detection: AdminConfigDuplicateDetectionDTO,
    /// Enabled (required)
    pub enabled: bool,
    /// (required)
    #[serde(rename = "facialRecognition")]
    pub facial_recognition: AdminConfigFacialRecognitionDTO,
    /// (required)
    pub ocr: AdminConfigOcrDTO,
    /// ML service URLs (required)
    pub urls: Vec<String>,
}

impl AdminConfigMachineLearningDTO {
    /// Creates a new [`AdminConfigMachineLearningDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        availability_checks: AdminConfigMachineLearningAvailabilityChecksDTO,
        clip: AdminConfigClipDTO,
        duplicate_detection: AdminConfigDuplicateDetectionDTO,
        enabled: bool,
        facial_recognition: AdminConfigFacialRecognitionDTO,
        ocr: AdminConfigOcrDTO,
        urls: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            availability_checks,
            clip,
            duplicate_detection,
            enabled,
            facial_recognition,
            ocr,
            urls: urls.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `availability_checks`.
    #[must_use]
    pub fn availability_checks(mut self, value: AdminConfigMachineLearningAvailabilityChecksDTO) -> Self {
        self.availability_checks = value;
        self
    }
    /// Sets `clip`.
    #[must_use]
    pub fn clip(mut self, value: AdminConfigClipDTO) -> Self {
        self.clip = value;
        self
    }
    /// Sets `duplicate_detection`.
    #[must_use]
    pub fn duplicate_detection(mut self, value: AdminConfigDuplicateDetectionDTO) -> Self {
        self.duplicate_detection = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `facial_recognition`.
    #[must_use]
    pub fn facial_recognition(mut self, value: AdminConfigFacialRecognitionDTO) -> Self {
        self.facial_recognition = value;
        self
    }
    /// Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: AdminConfigOcrDTO) -> Self {
        self.ocr = value;
        self
    }
    /// Sets `urls`.
    #[must_use]
    pub fn urls(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.urls = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `AdminConfigMapDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `dark_style` | [`String`] | ✓ | Dark map style URL |
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `light_style` | [`String`] | ✓ | Light map style URL |
///
/// See: <https://api.immich.app/models/AdminConfigMapDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigMapDTO {
    /// Dark map style URL (required)
    #[serde(rename = "darkStyle")]
    pub dark_style: String,
    /// Enabled (required)
    pub enabled: bool,
    /// Light map style URL (required)
    #[serde(rename = "lightStyle")]
    pub light_style: String,
}

impl AdminConfigMapDTO {
    /// Creates a new [`AdminConfigMapDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(dark_style: impl Into<String>, enabled: bool, light_style: impl Into<String>) -> Self {
        Self {
            dark_style: dark_style.into(),
            enabled,
            light_style: light_style.into(),
        }
    }
    /// Sets `dark_style`.
    #[must_use]
    pub fn dark_style(mut self, value: impl Into<String>) -> Self {
        self.dark_style = value.into();
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `light_style`.
    #[must_use]
    pub fn light_style(mut self, value: impl Into<String>) -> Self {
        self.light_style = value.into();
        self
    }
}

/// `AdminConfigMetadataDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `faces` | [`AdminConfigFacesDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigMetadataDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigMetadataDTO {
    /// (required)
    pub faces: AdminConfigFacesDTO,
}

impl AdminConfigMetadataDTO {
    /// Creates a new [`AdminConfigMetadataDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(faces: AdminConfigFacesDTO) -> Self {
        Self { faces }
    }
    /// Sets `faces`.
    #[must_use]
    pub fn faces(mut self, value: AdminConfigFacesDTO) -> Self {
        self.faces = value;
        self
    }
}

/// `AdminConfigNewVersionCheckDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `channel` | [`ReleaseChannel`] | ✓ |  |
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/AdminConfigNewVersionCheckDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigNewVersionCheckDTO {
    /// (required)
    pub channel: ReleaseChannel,
    /// Enabled (required)
    pub enabled: bool,
}

impl AdminConfigNewVersionCheckDTO {
    /// Creates a new [`AdminConfigNewVersionCheckDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(channel: ReleaseChannel, enabled: bool) -> Self {
        Self { channel, enabled }
    }
    /// Sets `channel`.
    #[must_use]
    pub fn channel(mut self, value: ReleaseChannel) -> Self {
        self.channel = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `AdminConfigNightlyTasksDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `cluster_new_faces` | `bool` | ✓ | Cluster new faces |
/// | `database_cleanup` | `bool` | ✓ | Database cleanup |
/// | `generate_memories` | `bool` | ✓ | Generate memories |
/// | `missing_thumbnails` | `bool` | ✓ | Missing thumbnails |
/// | `start_time` | [`String`] | ✓ | Start time (HH:MM) (pattern: `^(?:[01]\d\|2[0-3]):[0-5]\d$`) |
/// | `sync_quota_usage` | `bool` | ✓ | Sync quota usage |
///
/// See: <https://api.immich.app/models/AdminConfigNightlyTasksDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigNightlyTasksDTO {
    /// Cluster new faces (required)
    #[serde(rename = "clusterNewFaces")]
    pub cluster_new_faces: bool,
    /// Database cleanup (required)
    #[serde(rename = "databaseCleanup")]
    pub database_cleanup: bool,
    /// Generate memories (required)
    #[serde(rename = "generateMemories")]
    pub generate_memories: bool,
    /// Missing thumbnails (required)
    #[serde(rename = "missingThumbnails")]
    pub missing_thumbnails: bool,
    /// Start time (HH:MM) (pattern: `^(?:[01]\d|2[0-3]):[0-5]\d$`) (required)
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// Sync quota usage (required)
    #[serde(rename = "syncQuotaUsage")]
    pub sync_quota_usage: bool,
}

impl AdminConfigNightlyTasksDTO {
    /// Creates a new [`AdminConfigNightlyTasksDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        cluster_new_faces: bool,
        database_cleanup: bool,
        generate_memories: bool,
        missing_thumbnails: bool,
        start_time: impl Into<String>,
        sync_quota_usage: bool,
    ) -> Self {
        Self {
            cluster_new_faces,
            database_cleanup,
            generate_memories,
            missing_thumbnails,
            start_time: start_time.into(),
            sync_quota_usage,
        }
    }
    /// Sets `cluster_new_faces`.
    #[must_use]
    pub fn cluster_new_faces(mut self, value: bool) -> Self {
        self.cluster_new_faces = value;
        self
    }
    /// Sets `database_cleanup`.
    #[must_use]
    pub fn database_cleanup(mut self, value: bool) -> Self {
        self.database_cleanup = value;
        self
    }
    /// Sets `generate_memories`.
    #[must_use]
    pub fn generate_memories(mut self, value: bool) -> Self {
        self.generate_memories = value;
        self
    }
    /// Sets `missing_thumbnails`.
    #[must_use]
    pub fn missing_thumbnails(mut self, value: bool) -> Self {
        self.missing_thumbnails = value;
        self
    }
    /// Sets `start_time`.
    #[must_use]
    pub fn start_time(mut self, value: impl Into<String>) -> Self {
        self.start_time = value.into();
        self
    }
    /// Sets `sync_quota_usage`.
    #[must_use]
    pub fn sync_quota_usage(mut self, value: bool) -> Self {
        self.sync_quota_usage = value;
        self
    }
}

/// `AdminConfigNotificationsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `smtp` | [`AdminConfigSmtpDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigNotificationsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigNotificationsDTO {
    /// (required)
    pub smtp: AdminConfigSmtpDTO,
}

impl AdminConfigNotificationsDTO {
    /// Creates a new [`AdminConfigNotificationsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(smtp: AdminConfigSmtpDTO) -> Self {
        Self { smtp }
    }
    /// Sets `smtp`.
    #[must_use]
    pub fn smtp(mut self, value: AdminConfigSmtpDTO) -> Self {
        self.smtp = value;
        self
    }
}

/// `AdminConfigOAuthDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `account_management_url` | [`String`] |  | Account management URL |
/// | `allow_insecure_requests` | `bool` | ✓ | Allow insecure requests |
/// | `auto_launch` | `bool` | ✓ | Auto launch |
/// | `auto_register` | `bool` | ✓ | Auto register |
/// | `button_text` | [`String`] | ✓ | Button text |
/// | `client_id` | [`String`] | ✓ | Client ID |
/// | `client_secret` | [`String`] | ✓ | Client secret |
/// | `default_storage_quota` | `i64` | ✓ | Default storage quota |
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `end_session_endpoint` | [`String`] | ✓ | End session endpoint |
/// | `issuer_url` | [`String`] | ✓ | Issuer URL |
/// | `mobile_override_enabled` | `bool` | ✓ | Mobile override enabled |
/// | `mobile_redirect_uri` | [`String`] | ✓ | Mobile redirect URI (set to empty string to disable) |
/// | `profile_signing_algorithm` | [`String`] | ✓ | Profile signing algorithm |
/// | `prompt` | [`String`] | ✓ | `OAuth` prompt parameter (e.g. `select_account`, login, consent) |
/// | `role_claim` | [`String`] | ✓ | Role claim |
/// | `scope` | [`String`] | ✓ | Scope |
/// | `signing_algorithm` | [`String`] | ✓ | Signing algorithm |
/// | `storage_label_claim` | [`String`] | ✓ | Storage label claim |
/// | `storage_quota_claim` | [`String`] | ✓ | Storage quota claim |
/// | `timeout` | `i64` | ✓ | Timeout |
/// | `token_endpoint_auth_method` | [`OAuthTokenEndpointAuthMethod`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigOAuthDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigOAuthDTO {
    /// Account management URL
    #[serde(rename = "accountManagementUrl", default)]
    pub account_management_url: String,
    /// Allow insecure requests (required)
    #[serde(rename = "allowInsecureRequests")]
    pub allow_insecure_requests: bool,
    /// Auto launch (required)
    #[serde(rename = "autoLaunch")]
    pub auto_launch: bool,
    /// Auto register (required)
    #[serde(rename = "autoRegister")]
    pub auto_register: bool,
    /// Button text (required)
    #[serde(rename = "buttonText")]
    pub button_text: String,
    /// Client ID (required)
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// Client secret (required)
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    /// Default storage quota (required)
    #[serde(rename = "defaultStorageQuota")]
    pub default_storage_quota: Option<i64>,
    /// Enabled (required)
    pub enabled: bool,
    /// End session endpoint (required)
    #[serde(rename = "endSessionEndpoint")]
    pub end_session_endpoint: String,
    /// Issuer URL (required)
    #[serde(rename = "issuerUrl")]
    pub issuer_url: String,
    /// Mobile override enabled (required)
    #[serde(rename = "mobileOverrideEnabled")]
    pub mobile_override_enabled: bool,
    /// Mobile redirect URI (set to empty string to disable) (required)
    #[serde(rename = "mobileRedirectUri")]
    pub mobile_redirect_uri: String,
    /// Profile signing algorithm (required)
    #[serde(rename = "profileSigningAlgorithm")]
    pub profile_signing_algorithm: String,
    /// `OAuth` prompt parameter (e.g. `select_account`, login, consent) (required)
    pub prompt: String,
    /// Role claim (required)
    #[serde(rename = "roleClaim")]
    pub role_claim: String,
    /// Scope (required)
    pub scope: String,
    /// Signing algorithm (required)
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: String,
    /// Storage label claim (required)
    #[serde(rename = "storageLabelClaim")]
    pub storage_label_claim: String,
    /// Storage quota claim (required)
    #[serde(rename = "storageQuotaClaim")]
    pub storage_quota_claim: String,
    /// Timeout (required)
    pub timeout: ::std::num::NonZeroU64,
    /// (required)
    #[serde(rename = "tokenEndpointAuthMethod")]
    pub token_endpoint_auth_method: OAuthTokenEndpointAuthMethod,
}

impl AdminConfigOAuthDTO {
    /// Creates a new [`AdminConfigOAuthDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        allow_insecure_requests: bool,
        auto_launch: bool,
        auto_register: bool,
        button_text: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        enabled: bool,
        end_session_endpoint: impl Into<String>,
        issuer_url: impl Into<String>,
        mobile_override_enabled: bool,
        mobile_redirect_uri: impl Into<String>,
        profile_signing_algorithm: impl Into<String>,
        prompt: impl Into<String>,
        role_claim: impl Into<String>,
        scope: impl Into<String>,
        signing_algorithm: impl Into<String>,
        storage_label_claim: impl Into<String>,
        storage_quota_claim: impl Into<String>,
        timeout: ::std::num::NonZeroU64,
        token_endpoint_auth_method: OAuthTokenEndpointAuthMethod,
    ) -> Self {
        Self {
            account_management_url: <String>::default(),
            allow_insecure_requests,
            auto_launch,
            auto_register,
            button_text: button_text.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            default_storage_quota: None,
            enabled,
            end_session_endpoint: end_session_endpoint.into(),
            issuer_url: issuer_url.into(),
            mobile_override_enabled,
            mobile_redirect_uri: mobile_redirect_uri.into(),
            profile_signing_algorithm: profile_signing_algorithm.into(),
            prompt: prompt.into(),
            role_claim: role_claim.into(),
            scope: scope.into(),
            signing_algorithm: signing_algorithm.into(),
            storage_label_claim: storage_label_claim.into(),
            storage_quota_claim: storage_quota_claim.into(),
            timeout,
            token_endpoint_auth_method,
        }
    }
    /// Sets `account_management_url`.
    #[must_use]
    pub fn account_management_url(mut self, value: impl Into<String>) -> Self {
        self.account_management_url = value.into();
        self
    }
    /// Sets `allow_insecure_requests`.
    #[must_use]
    pub fn allow_insecure_requests(mut self, value: bool) -> Self {
        self.allow_insecure_requests = value;
        self
    }
    /// Sets `auto_launch`.
    #[must_use]
    pub fn auto_launch(mut self, value: bool) -> Self {
        self.auto_launch = value;
        self
    }
    /// Sets `auto_register`.
    #[must_use]
    pub fn auto_register(mut self, value: bool) -> Self {
        self.auto_register = value;
        self
    }
    /// Sets `button_text`.
    #[must_use]
    pub fn button_text(mut self, value: impl Into<String>) -> Self {
        self.button_text = value.into();
        self
    }
    /// Sets `client_id`.
    #[must_use]
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = value.into();
        self
    }
    /// Sets `client_secret`.
    #[must_use]
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = value.into();
        self
    }
    /// Sets `default_storage_quota`.
    #[must_use]
    pub fn default_storage_quota(mut self, value: i64) -> Self {
        self.default_storage_quota = Some(value);
        self
    }
    /// Sets or clears `default_storage_quota`.
    #[must_use]
    pub fn set_default_storage_quota(mut self, value: Option<i64>) -> Self {
        self.default_storage_quota = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `end_session_endpoint`.
    #[must_use]
    pub fn end_session_endpoint(mut self, value: impl Into<String>) -> Self {
        self.end_session_endpoint = value.into();
        self
    }
    /// Sets `issuer_url`.
    #[must_use]
    pub fn issuer_url(mut self, value: impl Into<String>) -> Self {
        self.issuer_url = value.into();
        self
    }
    /// Sets `mobile_override_enabled`.
    #[must_use]
    pub fn mobile_override_enabled(mut self, value: bool) -> Self {
        self.mobile_override_enabled = value;
        self
    }
    /// Sets `mobile_redirect_uri`.
    #[must_use]
    pub fn mobile_redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.mobile_redirect_uri = value.into();
        self
    }
    /// Sets `profile_signing_algorithm`.
    #[must_use]
    pub fn profile_signing_algorithm(mut self, value: impl Into<String>) -> Self {
        self.profile_signing_algorithm = value.into();
        self
    }
    /// Sets `prompt`.
    #[must_use]
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = value.into();
        self
    }
    /// Sets `role_claim`.
    #[must_use]
    pub fn role_claim(mut self, value: impl Into<String>) -> Self {
        self.role_claim = value.into();
        self
    }
    /// Sets `scope`.
    #[must_use]
    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = value.into();
        self
    }
    /// Sets `signing_algorithm`.
    #[must_use]
    pub fn signing_algorithm(mut self, value: impl Into<String>) -> Self {
        self.signing_algorithm = value.into();
        self
    }
    /// Sets `storage_label_claim`.
    #[must_use]
    pub fn storage_label_claim(mut self, value: impl Into<String>) -> Self {
        self.storage_label_claim = value.into();
        self
    }
    /// Sets `storage_quota_claim`.
    #[must_use]
    pub fn storage_quota_claim(mut self, value: impl Into<String>) -> Self {
        self.storage_quota_claim = value.into();
        self
    }
    /// Sets `timeout`.
    #[must_use]
    pub fn timeout(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.timeout = value;
        self
    }
    /// Sets `token_endpoint_auth_method`.
    #[must_use]
    pub fn token_endpoint_auth_method(mut self, value: OAuthTokenEndpointAuthMethod) -> Self {
        self.token_endpoint_auth_method = value;
        self
    }
}

/// `AdminConfigOcrDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether the task is enabled |
/// | `max_resolution` | `i64` | ✓ | Maximum resolution for OCR processing |
/// | `min_detection_score` | `f64` | ✓ | Minimum confidence score for text detection |
/// | `min_recognition_score` | `f64` | ✓ | Minimum confidence score for text recognition |
/// | `model_name` | [`String`] | ✓ | Name of the model to use |
///
/// See: <https://api.immich.app/models/AdminConfigOcrDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AdminConfigOcrDTO {
    /// Whether the task is enabled (required)
    pub enabled: bool,
    /// Maximum resolution for OCR processing (required)
    #[serde(rename = "maxResolution")]
    pub max_resolution: ::std::num::NonZeroU64,
    /// Minimum confidence score for text detection (required)
    #[serde(rename = "minDetectionScore")]
    pub min_detection_score: f64,
    /// Minimum confidence score for text recognition (required)
    #[serde(rename = "minRecognitionScore")]
    pub min_recognition_score: f64,
    /// Name of the model to use (required)
    #[serde(rename = "modelName")]
    pub model_name: String,
}

impl AdminConfigOcrDTO {
    /// Creates a new [`AdminConfigOcrDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        enabled: bool,
        max_resolution: ::std::num::NonZeroU64,
        min_detection_score: f64,
        min_recognition_score: f64,
        model_name: impl Into<String>,
    ) -> Self {
        Self {
            enabled,
            max_resolution,
            min_detection_score,
            min_recognition_score,
            model_name: model_name.into(),
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `max_resolution`.
    #[must_use]
    pub fn max_resolution(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.max_resolution = value;
        self
    }
    /// Sets `min_detection_score`.
    #[must_use]
    pub fn min_detection_score(mut self, value: f64) -> Self {
        self.min_detection_score = value;
        self
    }
    /// Sets `min_recognition_score`.
    #[must_use]
    pub fn min_recognition_score(mut self, value: f64) -> Self {
        self.min_recognition_score = value;
        self
    }
    /// Sets `model_name`.
    #[must_use]
    pub fn model_name(mut self, value: impl Into<String>) -> Self {
        self.model_name = value.into();
        self
    }
}

/// `AdminConfigPasswordLoginDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/AdminConfigPasswordLoginDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigPasswordLoginDTO {
    /// Enabled (required)
    pub enabled: bool,
}

impl AdminConfigPasswordLoginDTO {
    /// Creates a new [`AdminConfigPasswordLoginDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `AdminConfigReverseGeocodingDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/AdminConfigReverseGeocodingDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigReverseGeocodingDTO {
    /// Enabled (required)
    pub enabled: bool,
}

impl AdminConfigReverseGeocodingDTO {
    /// Creates a new [`AdminConfigReverseGeocodingDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `AdminConfigServerDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `external_domain` | [`String`] | ✓ | External domain |
/// | `login_page_message` | [`String`] | ✓ | Login page message |
/// | `public_users` | `bool` | ✓ | Public users |
///
/// See: <https://api.immich.app/models/AdminConfigServerDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigServerDTO {
    /// External domain (required)
    #[serde(rename = "externalDomain")]
    pub external_domain: String,
    /// Login page message (required)
    #[serde(rename = "loginPageMessage")]
    pub login_page_message: String,
    /// Public users (required)
    #[serde(rename = "publicUsers")]
    pub public_users: bool,
}

impl AdminConfigServerDTO {
    /// Creates a new [`AdminConfigServerDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(external_domain: impl Into<String>, login_page_message: impl Into<String>, public_users: bool) -> Self {
        Self {
            external_domain: external_domain.into(),
            login_page_message: login_page_message.into(),
            public_users,
        }
    }
    /// Sets `external_domain`.
    #[must_use]
    pub fn external_domain(mut self, value: impl Into<String>) -> Self {
        self.external_domain = value.into();
        self
    }
    /// Sets `login_page_message`.
    #[must_use]
    pub fn login_page_message(mut self, value: impl Into<String>) -> Self {
        self.login_page_message = value.into();
        self
    }
    /// Sets `public_users`.
    #[must_use]
    pub fn public_users(mut self, value: bool) -> Self {
        self.public_users = value;
        self
    }
}

/// `AdminConfigSmtpDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether SMTP email notifications are enabled |
/// | `from` | [`String`] | ✓ | Email address to send from |
/// | `reply_to` | [`String`] | ✓ | Email address for replies |
/// | `transport` | [`AdminConfigSmtpTransportDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigSmtpDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigSmtpDTO {
    /// Whether SMTP email notifications are enabled (required)
    pub enabled: bool,
    /// Email address to send from (required)
    pub from: String,
    /// Email address for replies (required)
    #[serde(rename = "replyTo")]
    pub reply_to: String,
    /// (required)
    pub transport: AdminConfigSmtpTransportDTO,
}

impl AdminConfigSmtpDTO {
    /// Creates a new [`AdminConfigSmtpDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        enabled: bool,
        from: impl Into<String>,
        reply_to: impl Into<String>,
        transport: AdminConfigSmtpTransportDTO,
    ) -> Self {
        Self {
            enabled,
            from: from.into(),
            reply_to: reply_to.into(),
            transport,
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `from`.
    #[must_use]
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = value.into();
        self
    }
    /// Sets `reply_to`.
    #[must_use]
    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = value.into();
        self
    }
    /// Sets `transport`.
    #[must_use]
    pub fn transport(mut self, value: AdminConfigSmtpTransportDTO) -> Self {
        self.transport = value;
        self
    }
}

/// `AdminConfigSmtpTransportDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `host` | [`String`] | ✓ | SMTP server hostname |
/// | `ignore_cert` | `bool` | ✓ | Whether to ignore SSL certificate errors |
/// | `password` | [`String`] | ✓ | SMTP password |
/// | `port` | `i64` | ✓ | SMTP server port |
/// | `secure` | `bool` | ✓ | Whether to use secure connection (TLS/SSL) |
/// | `username` | [`String`] | ✓ | SMTP username |
///
/// See: <https://api.immich.app/models/AdminConfigSmtpTransportDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigSmtpTransportDTO {
    /// SMTP server hostname (required)
    pub host: String,
    /// Whether to ignore SSL certificate errors (required)
    #[serde(rename = "ignoreCert")]
    pub ignore_cert: bool,
    /// SMTP password (required)
    pub password: String,
    /// SMTP server port (required)
    pub port: u16,
    /// Whether to use secure connection (TLS/SSL) (required)
    pub secure: bool,
    /// SMTP username (required)
    pub username: String,
}

impl AdminConfigSmtpTransportDTO {
    /// Creates a new [`AdminConfigSmtpTransportDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        host: impl Into<String>,
        ignore_cert: bool,
        password: impl Into<String>,
        port: u16,
        secure: bool,
        username: impl Into<String>,
    ) -> Self {
        Self {
            host: host.into(),
            ignore_cert,
            password: password.into(),
            port,
            secure,
            username: username.into(),
        }
    }
    /// Sets `host`.
    #[must_use]
    pub fn host(mut self, value: impl Into<String>) -> Self {
        self.host = value.into();
        self
    }
    /// Sets `ignore_cert`.
    #[must_use]
    pub fn ignore_cert(mut self, value: bool) -> Self {
        self.ignore_cert = value;
        self
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = value.into();
        self
    }
    /// Sets `port`.
    #[must_use]
    pub fn port(mut self, value: u16) -> Self {
        self.port = value;
        self
    }
    /// Sets `secure`.
    #[must_use]
    pub fn secure(mut self, value: bool) -> Self {
        self.secure = value;
        self
    }
    /// Sets `username`.
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = value.into();
        self
    }
}

/// `AdminConfigStorageTemplateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `hash_verification_enabled` | `bool` | ✓ | Hash verification enabled |
/// | `template` | [`String`] | ✓ | Template |
///
/// See: <https://api.immich.app/models/AdminConfigStorageTemplateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigStorageTemplateDTO {
    /// Enabled (required)
    pub enabled: bool,
    /// Hash verification enabled (required)
    #[serde(rename = "hashVerificationEnabled")]
    pub hash_verification_enabled: bool,
    /// Template (required)
    pub template: String,
}

impl AdminConfigStorageTemplateDTO {
    /// Creates a new [`AdminConfigStorageTemplateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, hash_verification_enabled: bool, template: impl Into<String>) -> Self {
        Self {
            enabled,
            hash_verification_enabled,
            template: template.into(),
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `hash_verification_enabled`.
    #[must_use]
    pub fn hash_verification_enabled(mut self, value: bool) -> Self {
        self.hash_verification_enabled = value;
        self
    }
    /// Sets `template`.
    #[must_use]
    pub fn template(mut self, value: impl Into<String>) -> Self {
        self.template = value.into();
        self
    }
}

/// `AdminConfigTemplateEmailsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_invite_template` | [`String`] | ✓ | Album invite template |
/// | `album_update_template` | [`String`] | ✓ | Album update template |
/// | `welcome_template` | [`String`] | ✓ | Welcome template |
///
/// See: <https://api.immich.app/models/AdminConfigTemplateEmailsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigTemplateEmailsDTO {
    /// Album invite template (required)
    #[serde(rename = "albumInviteTemplate")]
    pub album_invite_template: String,
    /// Album update template (required)
    #[serde(rename = "albumUpdateTemplate")]
    pub album_update_template: String,
    /// Welcome template (required)
    #[serde(rename = "welcomeTemplate")]
    pub welcome_template: String,
}

impl AdminConfigTemplateEmailsDTO {
    /// Creates a new [`AdminConfigTemplateEmailsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        album_invite_template: impl Into<String>,
        album_update_template: impl Into<String>,
        welcome_template: impl Into<String>,
    ) -> Self {
        Self {
            album_invite_template: album_invite_template.into(),
            album_update_template: album_update_template.into(),
            welcome_template: welcome_template.into(),
        }
    }
    /// Sets `album_invite_template`.
    #[must_use]
    pub fn album_invite_template(mut self, value: impl Into<String>) -> Self {
        self.album_invite_template = value.into();
        self
    }
    /// Sets `album_update_template`.
    #[must_use]
    pub fn album_update_template(mut self, value: impl Into<String>) -> Self {
        self.album_update_template = value.into();
        self
    }
    /// Sets `welcome_template`.
    #[must_use]
    pub fn welcome_template(mut self, value: impl Into<String>) -> Self {
        self.welcome_template = value.into();
        self
    }
}

/// `AdminConfigTemplatesDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `email` | [`AdminConfigTemplateEmailsDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AdminConfigTemplatesDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigTemplatesDTO {
    /// (required)
    pub email: AdminConfigTemplateEmailsDTO,
}

impl AdminConfigTemplatesDTO {
    /// Creates a new [`AdminConfigTemplatesDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(email: AdminConfigTemplateEmailsDTO) -> Self {
        Self { email }
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: AdminConfigTemplateEmailsDTO) -> Self {
        self.email = value;
        self
    }
}

/// `AdminConfigThemeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `custom_css` | [`String`] | ✓ | Custom CSS for theming |
///
/// See: <https://api.immich.app/models/AdminConfigThemeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigThemeDTO {
    /// Custom CSS for theming (required)
    #[serde(rename = "customCss")]
    pub custom_css: String,
}

impl AdminConfigThemeDTO {
    /// Creates a new [`AdminConfigThemeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(custom_css: impl Into<String>) -> Self {
        Self {
            custom_css: custom_css.into(),
        }
    }
    /// Sets `custom_css`.
    #[must_use]
    pub fn custom_css(mut self, value: impl Into<String>) -> Self {
        self.custom_css = value.into();
        self
    }
}

/// `AdminConfigTrashDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `days` | `i64` | ✓ | Days |
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/AdminConfigTrashDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigTrashDTO {
    /// Days (required)
    pub days: i64,
    /// Enabled (required)
    pub enabled: bool,
}

impl AdminConfigTrashDTO {
    /// Creates a new [`AdminConfigTrashDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(days: i64, enabled: bool) -> Self {
        Self { days, enabled }
    }
    /// Sets `days`.
    #[must_use]
    pub fn days(mut self, value: i64) -> Self {
        self.days = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `AdminConfigUserDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `delete_delay` | `i64` | ✓ | Delete delay |
///
/// See: <https://api.immich.app/models/AdminConfigUserDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AdminConfigUserDTO {
    /// Delete delay (required)
    #[serde(rename = "deleteDelay")]
    pub delete_delay: ::std::num::NonZeroU64,
}

impl AdminConfigUserDTO {
    /// Creates a new [`AdminConfigUserDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(delete_delay: ::std::num::NonZeroU64) -> Self {
        Self { delete_delay }
    }
    /// Sets `delete_delay`.
    #[must_use]
    pub fn delete_delay(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.delete_delay = value;
        self
    }
}

/// Target audio codec
///
/// See: <https://api.immich.app/models/AudioCodec>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AudioCodec {
    Mp3,
    Aac,
    Opus,
    PcmS16le,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AudioCodec {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mp3 => "mp3",
            Self::Aac => "aac",
            Self::Opus => "opus",
            Self::PcmS16le => "pcm_s16le",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AudioCodec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AudioCodec {
    fn from(value: &str) -> Self {
        match value {
            "mp3" => Self::Mp3,
            "aac" => Self::Aac,
            "opus" => Self::Opus,
            "pcm_s16le" => Self::PcmS16le,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AudioCodec {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mp3" => Self::Mp3,
            "aac" => Self::Aac,
            "opus" => Self::Opus,
            "pcm_s16le" => Self::PcmS16le,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AudioCodec {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AudioCodec {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AudioCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AudioCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AudioCodec;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AudioCodec::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AudioCodec::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Colorspace
///
/// See: <https://api.immich.app/models/Colorspace>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Colorspace {
    Srgb,
    P3,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl Colorspace {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Srgb => "srgb",
            Self::P3 => "p3",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for Colorspace {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for Colorspace {
    fn from(value: &str) -> Self {
        match value {
            "srgb" => Self::Srgb,
            "p3" => Self::P3,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for Colorspace {
    fn from(value: String) -> Self {
        match value.as_str() {
            "srgb" => Self::Srgb,
            "p3" => Self::P3,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for Colorspace {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for Colorspace {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for Colorspace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for Colorspace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = Colorspace;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(Colorspace::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(Colorspace::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// HLS video resolution
///
/// See: <https://api.immich.app/models/HlsVideoResolution>
#[derive(::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct HlsVideoResolution(pub i64);

impl ::std::ops::Deref for HlsVideoResolution {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl From<HlsVideoResolution> for i64 {
    fn from(value: HlsVideoResolution) -> Self {
        value.0
    }
}

impl TryFrom<i64> for HlsVideoResolution {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: i64) -> Result<Self, crate::common::error::ConversionError> {
        if [480_i64, 720_i64, 1080_i64, 1440_i64, 2160_i64].contains(&value) {
            Ok(Self(value))
        } else {
            Err(format!("invalid value for {}: {}", "HlsVideoResolution", value).into())
        }
    }
}

impl<'de> ::serde::Deserialize<'de> for HlsVideoResolution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// Image format
///
/// See: <https://api.immich.app/models/ImageFormat>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ImageFormat {
    Jpeg,
    Webp,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl ImageFormat {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Jpeg => "jpeg",
            Self::Webp => "webp",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for ImageFormat {
    fn from(value: &str) -> Self {
        match value {
            "jpeg" => Self::Jpeg,
            "webp" => Self::Webp,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for ImageFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "jpeg" => Self::Jpeg,
            "webp" => Self::Webp,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for ImageFormat {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for ImageFormat {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for ImageFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for ImageFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = ImageFormat;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ImageFormat::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ImageFormat::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Log level
///
/// See: <https://api.immich.app/models/LogLevel>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum LogLevel {
    Verbose,
    Debug,
    Log,
    Warn,
    Error,
    Fatal,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl LogLevel {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Verbose => "verbose",
            Self::Debug => "debug",
            Self::Log => "log",
            Self::Warn => "warn",
            Self::Error => "error",
            Self::Fatal => "fatal",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        match value {
            "verbose" => Self::Verbose,
            "debug" => Self::Debug,
            "log" => Self::Log,
            "warn" => Self::Warn,
            "error" => Self::Error,
            "fatal" => Self::Fatal,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for LogLevel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "verbose" => Self::Verbose,
            "debug" => Self::Debug,
            "log" => Self::Log,
            "warn" => Self::Warn,
            "error" => Self::Error,
            "fatal" => Self::Fatal,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for LogLevel {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for LogLevel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = LogLevel;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(LogLevel::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(LogLevel::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `OAuth` token endpoint auth method
///
/// See: <https://api.immich.app/models/OAuthTokenEndpointAuthMethod>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum OAuthTokenEndpointAuthMethod {
    ClientSecretPost,
    ClientSecretBasic,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl OAuthTokenEndpointAuthMethod {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientSecretPost => "client_secret_post",
            Self::ClientSecretBasic => "client_secret_basic",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for OAuthTokenEndpointAuthMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for OAuthTokenEndpointAuthMethod {
    fn from(value: &str) -> Self {
        match value {
            "client_secret_post" => Self::ClientSecretPost,
            "client_secret_basic" => Self::ClientSecretBasic,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for OAuthTokenEndpointAuthMethod {
    fn from(value: String) -> Self {
        match value.as_str() {
            "client_secret_post" => Self::ClientSecretPost,
            "client_secret_basic" => Self::ClientSecretBasic,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for OAuthTokenEndpointAuthMethod {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for OAuthTokenEndpointAuthMethod {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for OAuthTokenEndpointAuthMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for OAuthTokenEndpointAuthMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = OAuthTokenEndpointAuthMethod;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(OAuthTokenEndpointAuthMethod::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(OAuthTokenEndpointAuthMethod::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Configuration properties that are visible to everyone
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `oauth` | [`PublicConfigOAuthDTO`] | ✓ |  |
/// | `password_login` | [`PublicConfigPasswordLoginDTO`] | ✓ |  |
/// | `server` | [`PublicConfigServerDTO`] | ✓ |  |
/// | `theme` | [`PublicConfigThemeDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/PublicConfigDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PublicConfigDTO {
    /// (required)
    pub oauth: PublicConfigOAuthDTO,
    /// (required)
    #[serde(rename = "passwordLogin")]
    pub password_login: PublicConfigPasswordLoginDTO,
    /// (required)
    pub server: PublicConfigServerDTO,
    /// (required)
    pub theme: PublicConfigThemeDTO,
}

impl PublicConfigDTO {
    /// Creates a new [`PublicConfigDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        oauth: PublicConfigOAuthDTO,
        password_login: PublicConfigPasswordLoginDTO,
        server: PublicConfigServerDTO,
        theme: PublicConfigThemeDTO,
    ) -> Self {
        Self {
            oauth,
            password_login,
            server,
            theme,
        }
    }
    /// Sets `oauth`.
    #[must_use]
    pub fn oauth(mut self, value: PublicConfigOAuthDTO) -> Self {
        self.oauth = value;
        self
    }
    /// Sets `password_login`.
    #[must_use]
    pub fn password_login(mut self, value: PublicConfigPasswordLoginDTO) -> Self {
        self.password_login = value;
        self
    }
    /// Sets `server`.
    #[must_use]
    pub fn server(mut self, value: PublicConfigServerDTO) -> Self {
        self.server = value;
        self
    }
    /// Sets `theme`.
    #[must_use]
    pub fn theme(mut self, value: PublicConfigThemeDTO) -> Self {
        self.theme = value;
        self
    }
}

/// `PublicConfigOAuthDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `auto_launch` | `bool` | ✓ | Auto launch |
/// | `button_text` | [`String`] | ✓ | Button text |
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/PublicConfigOAuthDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PublicConfigOAuthDTO {
    /// Auto launch (required)
    #[serde(rename = "autoLaunch")]
    pub auto_launch: bool,
    /// Button text (required)
    #[serde(rename = "buttonText")]
    pub button_text: String,
    /// Enabled (required)
    pub enabled: bool,
}

impl PublicConfigOAuthDTO {
    /// Creates a new [`PublicConfigOAuthDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(auto_launch: bool, button_text: impl Into<String>, enabled: bool) -> Self {
        Self {
            auto_launch,
            button_text: button_text.into(),
            enabled,
        }
    }
    /// Sets `auto_launch`.
    #[must_use]
    pub fn auto_launch(mut self, value: bool) -> Self {
        self.auto_launch = value;
        self
    }
    /// Sets `button_text`.
    #[must_use]
    pub fn button_text(mut self, value: impl Into<String>) -> Self {
        self.button_text = value.into();
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `PublicConfigPasswordLoginDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/PublicConfigPasswordLoginDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PublicConfigPasswordLoginDTO {
    /// Enabled (required)
    pub enabled: bool,
}

impl PublicConfigPasswordLoginDTO {
    /// Creates a new [`PublicConfigPasswordLoginDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `PublicConfigServerDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `login_page_message` | [`String`] | ✓ | Login page message |
///
/// See: <https://api.immich.app/models/PublicConfigServerDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PublicConfigServerDTO {
    /// Login page message (required)
    #[serde(rename = "loginPageMessage")]
    pub login_page_message: String,
}

impl PublicConfigServerDTO {
    /// Creates a new [`PublicConfigServerDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(login_page_message: impl Into<String>) -> Self {
        Self {
            login_page_message: login_page_message.into(),
        }
    }
    /// Sets `login_page_message`.
    #[must_use]
    pub fn login_page_message(mut self, value: impl Into<String>) -> Self {
        self.login_page_message = value.into();
        self
    }
}

/// `PublicConfigThemeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `custom_css` | [`String`] | ✓ | Custom CSS for theming |
///
/// See: <https://api.immich.app/models/PublicConfigThemeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PublicConfigThemeDTO {
    /// Custom CSS for theming (required)
    #[serde(rename = "customCss")]
    pub custom_css: String,
}

impl PublicConfigThemeDTO {
    /// Creates a new [`PublicConfigThemeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(custom_css: impl Into<String>) -> Self {
        Self {
            custom_css: custom_css.into(),
        }
    }
    /// Sets `custom_css`.
    #[must_use]
    pub fn custom_css(mut self, value: impl Into<String>) -> Self {
        self.custom_css = value.into();
        self
    }
}

/// Release channel
///
/// See: <https://api.immich.app/models/ReleaseChannel>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ReleaseChannel {
    Stable,
    ReleaseCandidate,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl ReleaseChannel {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Stable => "stable",
            Self::ReleaseCandidate => "releaseCandidate",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for ReleaseChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for ReleaseChannel {
    fn from(value: &str) -> Self {
        match value {
            "stable" => Self::Stable,
            "releaseCandidate" => Self::ReleaseCandidate,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for ReleaseChannel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "stable" => Self::Stable,
            "releaseCandidate" => Self::ReleaseCandidate,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for ReleaseChannel {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for ReleaseChannel {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for ReleaseChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for ReleaseChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = ReleaseChannel;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ReleaseChannel::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ReleaseChannel::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Tone mapping
///
/// See: <https://api.immich.app/models/ToneMapping>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ToneMapping {
    Hable,
    Mobius,
    Reinhard,
    Disabled,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl ToneMapping {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Hable => "hable",
            Self::Mobius => "mobius",
            Self::Reinhard => "reinhard",
            Self::Disabled => "disabled",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for ToneMapping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for ToneMapping {
    fn from(value: &str) -> Self {
        match value {
            "hable" => Self::Hable,
            "mobius" => Self::Mobius,
            "reinhard" => Self::Reinhard,
            "disabled" => Self::Disabled,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for ToneMapping {
    fn from(value: String) -> Self {
        match value.as_str() {
            "hable" => Self::Hable,
            "mobius" => Self::Mobius,
            "reinhard" => Self::Reinhard,
            "disabled" => Self::Disabled,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for ToneMapping {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for ToneMapping {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for ToneMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for ToneMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = ToneMapping;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ToneMapping::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(ToneMapping::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Transcode policy
///
/// See: <https://api.immich.app/models/TranscodePolicy>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum TranscodePolicy {
    All,
    Optimal,
    Bitrate,
    Required,
    Disabled,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl TranscodePolicy {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "all",
            Self::Optimal => "optimal",
            Self::Bitrate => "bitrate",
            Self::Required => "required",
            Self::Disabled => "disabled",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for TranscodePolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for TranscodePolicy {
    fn from(value: &str) -> Self {
        match value {
            "all" => Self::All,
            "optimal" => Self::Optimal,
            "bitrate" => Self::Bitrate,
            "required" => Self::Required,
            "disabled" => Self::Disabled,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for TranscodePolicy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "all" => Self::All,
            "optimal" => Self::Optimal,
            "bitrate" => Self::Bitrate,
            "required" => Self::Required,
            "disabled" => Self::Disabled,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for TranscodePolicy {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for TranscodePolicy {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for TranscodePolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for TranscodePolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = TranscodePolicy;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(TranscodePolicy::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(TranscodePolicy::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `UserConfigClipDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether the task is enabled |
///
/// See: <https://api.immich.app/models/UserConfigClipDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigClipDTO {
    /// Whether the task is enabled (required)
    pub enabled: bool,
}

impl UserConfigClipDTO {
    /// Creates a new [`UserConfigClipDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// Configuration properties that are visible to a logged user
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ffmpeg` | [`UserConfigFFmpegDTO`] | ✓ |  |
/// | `image` | [`UserConfigImageDTO`] | ✓ |  |
/// | `machine_learning` | [`UserConfigMachineLearningDTO`] | ✓ |  |
/// | `map` | [`UserConfigMapDTO`] | ✓ |  |
/// | `oauth` | [`UserConfigOAuthDTO`] | ✓ |  |
/// | `password_login` | [`UserConfigPasswordLoginDTO`] | ✓ |  |
/// | `reverse_geocoding` | [`UserConfigReverseGeocodingDTO`] | ✓ |  |
/// | `server` | [`UserConfigServerDTO`] | ✓ |  |
/// | `theme` | [`UserConfigThemeDTO`] | ✓ |  |
/// | `trash` | [`UserConfigTrashDTO`] | ✓ |  |
/// | `user` | [`UserConfigUserDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/UserConfigDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigDTO {
    /// (required)
    pub ffmpeg: UserConfigFFmpegDTO,
    /// (required)
    pub image: UserConfigImageDTO,
    /// (required)
    #[serde(rename = "machineLearning")]
    pub machine_learning: UserConfigMachineLearningDTO,
    /// (required)
    pub map: UserConfigMapDTO,
    /// (required)
    pub oauth: UserConfigOAuthDTO,
    /// (required)
    #[serde(rename = "passwordLogin")]
    pub password_login: UserConfigPasswordLoginDTO,
    /// (required)
    #[serde(rename = "reverseGeocoding")]
    pub reverse_geocoding: UserConfigReverseGeocodingDTO,
    /// (required)
    pub server: UserConfigServerDTO,
    /// (required)
    pub theme: UserConfigThemeDTO,
    /// (required)
    pub trash: UserConfigTrashDTO,
    /// (required)
    pub user: UserConfigUserDTO,
}

impl UserConfigDTO {
    /// Creates a new [`UserConfigDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        ffmpeg: UserConfigFFmpegDTO,
        image: UserConfigImageDTO,
        machine_learning: UserConfigMachineLearningDTO,
        map: UserConfigMapDTO,
        oauth: UserConfigOAuthDTO,
        password_login: UserConfigPasswordLoginDTO,
        reverse_geocoding: UserConfigReverseGeocodingDTO,
        server: UserConfigServerDTO,
        theme: UserConfigThemeDTO,
        trash: UserConfigTrashDTO,
        user: UserConfigUserDTO,
    ) -> Self {
        Self {
            ffmpeg,
            image,
            machine_learning,
            map,
            oauth,
            password_login,
            reverse_geocoding,
            server,
            theme,
            trash,
            user,
        }
    }
    /// Sets `ffmpeg`.
    #[must_use]
    pub fn ffmpeg(mut self, value: UserConfigFFmpegDTO) -> Self {
        self.ffmpeg = value;
        self
    }
    /// Sets `image`.
    #[must_use]
    pub fn image(mut self, value: UserConfigImageDTO) -> Self {
        self.image = value;
        self
    }
    /// Sets `machine_learning`.
    #[must_use]
    pub fn machine_learning(mut self, value: UserConfigMachineLearningDTO) -> Self {
        self.machine_learning = value;
        self
    }
    /// Sets `map`.
    #[must_use]
    pub fn map(mut self, value: UserConfigMapDTO) -> Self {
        self.map = value;
        self
    }
    /// Sets `oauth`.
    #[must_use]
    pub fn oauth(mut self, value: UserConfigOAuthDTO) -> Self {
        self.oauth = value;
        self
    }
    /// Sets `password_login`.
    #[must_use]
    pub fn password_login(mut self, value: UserConfigPasswordLoginDTO) -> Self {
        self.password_login = value;
        self
    }
    /// Sets `reverse_geocoding`.
    #[must_use]
    pub fn reverse_geocoding(mut self, value: UserConfigReverseGeocodingDTO) -> Self {
        self.reverse_geocoding = value;
        self
    }
    /// Sets `server`.
    #[must_use]
    pub fn server(mut self, value: UserConfigServerDTO) -> Self {
        self.server = value;
        self
    }
    /// Sets `theme`.
    #[must_use]
    pub fn theme(mut self, value: UserConfigThemeDTO) -> Self {
        self.theme = value;
        self
    }
    /// Sets `trash`.
    #[must_use]
    pub fn trash(mut self, value: UserConfigTrashDTO) -> Self {
        self.trash = value;
        self
    }
    /// Sets `user`.
    #[must_use]
    pub fn user(mut self, value: UserConfigUserDTO) -> Self {
        self.user = value;
        self
    }
}

/// `UserConfigDuplicateDetectionDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether the task is enabled |
///
/// See: <https://api.immich.app/models/UserConfigDuplicateDetectionDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigDuplicateDetectionDTO {
    /// Whether the task is enabled (required)
    pub enabled: bool,
}

impl UserConfigDuplicateDetectionDTO {
    /// Creates a new [`UserConfigDuplicateDetectionDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `UserConfigFFmpegDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `realtime` | [`UserConfigFFmpegRealtimeDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/UserConfigFFmpegDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigFFmpegDTO {
    /// (required)
    pub realtime: UserConfigFFmpegRealtimeDTO,
}

impl UserConfigFFmpegDTO {
    /// Creates a new [`UserConfigFFmpegDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(realtime: UserConfigFFmpegRealtimeDTO) -> Self {
        Self { realtime }
    }
    /// Sets `realtime`.
    #[must_use]
    pub fn realtime(mut self, value: UserConfigFFmpegRealtimeDTO) -> Self {
        self.realtime = value;
        self
    }
}

/// `UserConfigFFmpegRealtimeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enable real-time HLS transcoding (alpha) |
/// | `resolutions` | [`Vec<HlsVideoResolution>`](crate::system::config::HlsVideoResolution) | ✓ | Resolutions to use for real-time HLS transcoding |
/// | `video_codecs` | [`Vec<VideoCodec>`](crate::system::config::VideoCodec) | ✓ | Video codecs to use for real-time HLS transcoding |
///
/// See: <https://api.immich.app/models/UserConfigFFmpegRealtimeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigFFmpegRealtimeDTO {
    /// Enable real-time HLS transcoding (alpha) (required)
    pub enabled: bool,
    /// Resolutions to use for real-time HLS transcoding (required)
    pub resolutions: Vec<HlsVideoResolution>,
    /// Video codecs to use for real-time HLS transcoding (required)
    #[serde(rename = "videoCodecs")]
    pub video_codecs: Vec<VideoCodec>,
}

impl UserConfigFFmpegRealtimeDTO {
    /// Creates a new [`UserConfigFFmpegRealtimeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        enabled: bool,
        resolutions: impl IntoIterator<Item = HlsVideoResolution>,
        video_codecs: impl IntoIterator<Item = VideoCodec>,
    ) -> Self {
        Self {
            enabled,
            resolutions: resolutions.into_iter().collect(),
            video_codecs: video_codecs.into_iter().collect(),
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `resolutions`.
    #[must_use]
    pub fn resolutions(mut self, value: impl IntoIterator<Item = HlsVideoResolution>) -> Self {
        self.resolutions = value.into_iter().collect();
        self
    }
    /// Sets `video_codecs`.
    #[must_use]
    pub fn video_codecs(mut self, value: impl IntoIterator<Item = VideoCodec>) -> Self {
        self.video_codecs = value.into_iter().collect();
        self
    }
}

/// `UserConfigFacialRecognitionDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether the task is enabled |
/// | `min_faces` | `i64` | ✓ | Minimum number of faces required for recognition |
///
/// See: <https://api.immich.app/models/UserConfigFacialRecognitionDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigFacialRecognitionDTO {
    /// Whether the task is enabled (required)
    pub enabled: bool,
    /// Minimum number of faces required for recognition (required)
    #[serde(rename = "minFaces")]
    pub min_faces: ::std::num::NonZeroU64,
}

impl UserConfigFacialRecognitionDTO {
    /// Creates a new [`UserConfigFacialRecognitionDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, min_faces: ::std::num::NonZeroU64) -> Self {
        Self { enabled, min_faces }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `min_faces`.
    #[must_use]
    pub fn min_faces(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.min_faces = value;
        self
    }
}

/// `UserConfigGeneratedFullsizeImageDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/UserConfigGeneratedFullsizeImageDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigGeneratedFullsizeImageDTO {
    /// Enabled (required)
    pub enabled: bool,
}

impl UserConfigGeneratedFullsizeImageDTO {
    /// Creates a new [`UserConfigGeneratedFullsizeImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `UserConfigGeneratedImageDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `size` | `i64` | ✓ | Size |
///
/// See: <https://api.immich.app/models/UserConfigGeneratedImageDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigGeneratedImageDTO {
    /// Size (required)
    pub size: ::std::num::NonZeroU64,
}

impl UserConfigGeneratedImageDTO {
    /// Creates a new [`UserConfigGeneratedImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(size: ::std::num::NonZeroU64) -> Self {
        Self { size }
    }
    /// Sets `size`.
    #[must_use]
    pub fn size(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.size = value;
        self
    }
}

/// `UserConfigImageDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `fullsize` | [`UserConfigGeneratedFullsizeImageDTO`] | ✓ |  |
/// | `preview` | [`UserConfigGeneratedImageDTO`] | ✓ |  |
/// | `thumbnail` | [`UserConfigGeneratedImageDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/UserConfigImageDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigImageDTO {
    /// (required)
    pub fullsize: UserConfigGeneratedFullsizeImageDTO,
    /// (required)
    pub preview: UserConfigGeneratedImageDTO,
    /// (required)
    pub thumbnail: UserConfigGeneratedImageDTO,
}

impl UserConfigImageDTO {
    /// Creates a new [`UserConfigImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        fullsize: UserConfigGeneratedFullsizeImageDTO,
        preview: UserConfigGeneratedImageDTO,
        thumbnail: UserConfigGeneratedImageDTO,
    ) -> Self {
        Self {
            fullsize,
            preview,
            thumbnail,
        }
    }
    /// Sets `fullsize`.
    #[must_use]
    pub fn fullsize(mut self, value: UserConfigGeneratedFullsizeImageDTO) -> Self {
        self.fullsize = value;
        self
    }
    /// Sets `preview`.
    #[must_use]
    pub fn preview(mut self, value: UserConfigGeneratedImageDTO) -> Self {
        self.preview = value;
        self
    }
    /// Sets `thumbnail`.
    #[must_use]
    pub fn thumbnail(mut self, value: UserConfigGeneratedImageDTO) -> Self {
        self.thumbnail = value;
        self
    }
}

/// `UserConfigMachineLearningDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `clip` | [`UserConfigClipDTO`] | ✓ |  |
/// | `duplicate_detection` | [`UserConfigDuplicateDetectionDTO`] | ✓ |  |
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `facial_recognition` | [`UserConfigFacialRecognitionDTO`] | ✓ |  |
/// | `ocr` | [`UserConfigOcrDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/UserConfigMachineLearningDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigMachineLearningDTO {
    /// (required)
    pub clip: UserConfigClipDTO,
    /// (required)
    #[serde(rename = "duplicateDetection")]
    pub duplicate_detection: UserConfigDuplicateDetectionDTO,
    /// Enabled (required)
    pub enabled: bool,
    /// (required)
    #[serde(rename = "facialRecognition")]
    pub facial_recognition: UserConfigFacialRecognitionDTO,
    /// (required)
    pub ocr: UserConfigOcrDTO,
}

impl UserConfigMachineLearningDTO {
    /// Creates a new [`UserConfigMachineLearningDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        clip: UserConfigClipDTO,
        duplicate_detection: UserConfigDuplicateDetectionDTO,
        enabled: bool,
        facial_recognition: UserConfigFacialRecognitionDTO,
        ocr: UserConfigOcrDTO,
    ) -> Self {
        Self {
            clip,
            duplicate_detection,
            enabled,
            facial_recognition,
            ocr,
        }
    }
    /// Sets `clip`.
    #[must_use]
    pub fn clip(mut self, value: UserConfigClipDTO) -> Self {
        self.clip = value;
        self
    }
    /// Sets `duplicate_detection`.
    #[must_use]
    pub fn duplicate_detection(mut self, value: UserConfigDuplicateDetectionDTO) -> Self {
        self.duplicate_detection = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `facial_recognition`.
    #[must_use]
    pub fn facial_recognition(mut self, value: UserConfigFacialRecognitionDTO) -> Self {
        self.facial_recognition = value;
        self
    }
    /// Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: UserConfigOcrDTO) -> Self {
        self.ocr = value;
        self
    }
}

/// `UserConfigMapDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `dark_style` | [`String`] | ✓ | Dark map style URL |
/// | `enabled` | `bool` | ✓ | Enabled |
/// | `light_style` | [`String`] | ✓ | Light map style URL |
///
/// See: <https://api.immich.app/models/UserConfigMapDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigMapDTO {
    /// Dark map style URL (required)
    #[serde(rename = "darkStyle")]
    pub dark_style: String,
    /// Enabled (required)
    pub enabled: bool,
    /// Light map style URL (required)
    #[serde(rename = "lightStyle")]
    pub light_style: String,
}

impl UserConfigMapDTO {
    /// Creates a new [`UserConfigMapDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(dark_style: impl Into<String>, enabled: bool, light_style: impl Into<String>) -> Self {
        Self {
            dark_style: dark_style.into(),
            enabled,
            light_style: light_style.into(),
        }
    }
    /// Sets `dark_style`.
    #[must_use]
    pub fn dark_style(mut self, value: impl Into<String>) -> Self {
        self.dark_style = value.into();
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `light_style`.
    #[must_use]
    pub fn light_style(mut self, value: impl Into<String>) -> Self {
        self.light_style = value.into();
        self
    }
}

/// `UserConfigOAuthDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `auto_launch` | `bool` | ✓ | Auto launch |
/// | `button_text` | [`String`] | ✓ | Button text |
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/UserConfigOAuthDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigOAuthDTO {
    /// Auto launch (required)
    #[serde(rename = "autoLaunch")]
    pub auto_launch: bool,
    /// Button text (required)
    #[serde(rename = "buttonText")]
    pub button_text: String,
    /// Enabled (required)
    pub enabled: bool,
}

impl UserConfigOAuthDTO {
    /// Creates a new [`UserConfigOAuthDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(auto_launch: bool, button_text: impl Into<String>, enabled: bool) -> Self {
        Self {
            auto_launch,
            button_text: button_text.into(),
            enabled,
        }
    }
    /// Sets `auto_launch`.
    #[must_use]
    pub fn auto_launch(mut self, value: bool) -> Self {
        self.auto_launch = value;
        self
    }
    /// Sets `button_text`.
    #[must_use]
    pub fn button_text(mut self, value: impl Into<String>) -> Self {
        self.button_text = value.into();
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `UserConfigOcrDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether the task is enabled |
///
/// See: <https://api.immich.app/models/UserConfigOcrDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigOcrDTO {
    /// Whether the task is enabled (required)
    pub enabled: bool,
}

impl UserConfigOcrDTO {
    /// Creates a new [`UserConfigOcrDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `UserConfigPasswordLoginDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/UserConfigPasswordLoginDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigPasswordLoginDTO {
    /// Enabled (required)
    pub enabled: bool,
}

impl UserConfigPasswordLoginDTO {
    /// Creates a new [`UserConfigPasswordLoginDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `UserConfigReverseGeocodingDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/UserConfigReverseGeocodingDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigReverseGeocodingDTO {
    /// Enabled (required)
    pub enabled: bool,
}

impl UserConfigReverseGeocodingDTO {
    /// Creates a new [`UserConfigReverseGeocodingDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `UserConfigServerDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `external_domain` | [`String`] | ✓ | External domain |
/// | `login_page_message` | [`String`] | ✓ | Login page message |
/// | `public_users` | `bool` | ✓ | Public users |
///
/// See: <https://api.immich.app/models/UserConfigServerDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigServerDTO {
    /// External domain (required)
    #[serde(rename = "externalDomain")]
    pub external_domain: String,
    /// Login page message (required)
    #[serde(rename = "loginPageMessage")]
    pub login_page_message: String,
    /// Public users (required)
    #[serde(rename = "publicUsers")]
    pub public_users: bool,
}

impl UserConfigServerDTO {
    /// Creates a new [`UserConfigServerDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(external_domain: impl Into<String>, login_page_message: impl Into<String>, public_users: bool) -> Self {
        Self {
            external_domain: external_domain.into(),
            login_page_message: login_page_message.into(),
            public_users,
        }
    }
    /// Sets `external_domain`.
    #[must_use]
    pub fn external_domain(mut self, value: impl Into<String>) -> Self {
        self.external_domain = value.into();
        self
    }
    /// Sets `login_page_message`.
    #[must_use]
    pub fn login_page_message(mut self, value: impl Into<String>) -> Self {
        self.login_page_message = value.into();
        self
    }
    /// Sets `public_users`.
    #[must_use]
    pub fn public_users(mut self, value: bool) -> Self {
        self.public_users = value;
        self
    }
}

/// `UserConfigThemeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `custom_css` | [`String`] | ✓ | Custom CSS for theming |
///
/// See: <https://api.immich.app/models/UserConfigThemeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigThemeDTO {
    /// Custom CSS for theming (required)
    #[serde(rename = "customCss")]
    pub custom_css: String,
}

impl UserConfigThemeDTO {
    /// Creates a new [`UserConfigThemeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(custom_css: impl Into<String>) -> Self {
        Self {
            custom_css: custom_css.into(),
        }
    }
    /// Sets `custom_css`.
    #[must_use]
    pub fn custom_css(mut self, value: impl Into<String>) -> Self {
        self.custom_css = value.into();
        self
    }
}

/// `UserConfigTrashDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `days` | `i64` | ✓ | Days |
/// | `enabled` | `bool` | ✓ | Enabled |
///
/// See: <https://api.immich.app/models/UserConfigTrashDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigTrashDTO {
    /// Days (required)
    pub days: i64,
    /// Enabled (required)
    pub enabled: bool,
}

impl UserConfigTrashDTO {
    /// Creates a new [`UserConfigTrashDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(days: i64, enabled: bool) -> Self {
        Self { days, enabled }
    }
    /// Sets `days`.
    #[must_use]
    pub fn days(mut self, value: i64) -> Self {
        self.days = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `UserConfigUserDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `delete_delay` | `i64` | ✓ | Delete delay |
///
/// See: <https://api.immich.app/models/UserConfigUserDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserConfigUserDTO {
    /// Delete delay (required)
    #[serde(rename = "deleteDelay")]
    pub delete_delay: ::std::num::NonZeroU64,
}

impl UserConfigUserDTO {
    /// Creates a new [`UserConfigUserDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(delete_delay: ::std::num::NonZeroU64) -> Self {
        Self { delete_delay }
    }
    /// Sets `delete_delay`.
    #[must_use]
    pub fn delete_delay(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.delete_delay = value;
        self
    }
}

/// Target video codec
///
/// See: <https://api.immich.app/models/VideoCodec>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum VideoCodec {
    H264,
    Hevc,
    Vp9,
    Av1,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl VideoCodec {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::H264 => "h264",
            Self::Hevc => "hevc",
            Self::Vp9 => "vp9",
            Self::Av1 => "av1",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for VideoCodec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for VideoCodec {
    fn from(value: &str) -> Self {
        match value {
            "h264" => Self::H264,
            "hevc" => Self::Hevc,
            "vp9" => Self::Vp9,
            "av1" => Self::Av1,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for VideoCodec {
    fn from(value: String) -> Self {
        match value.as_str() {
            "h264" => Self::H264,
            "hevc" => Self::Hevc,
            "vp9" => Self::Vp9,
            "av1" => Self::Av1,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for VideoCodec {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for VideoCodec {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for VideoCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for VideoCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = VideoCodec;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(VideoCodec::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(VideoCodec::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Accepted video containers
///
/// See: <https://api.immich.app/models/VideoContainer>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum VideoContainer {
    Mov,
    Mp4,
    Ogg,
    Webm,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl VideoContainer {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mov => "mov",
            Self::Mp4 => "mp4",
            Self::Ogg => "ogg",
            Self::Webm => "webm",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for VideoContainer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for VideoContainer {
    fn from(value: &str) -> Self {
        match value {
            "mov" => Self::Mov,
            "mp4" => Self::Mp4,
            "ogg" => Self::Ogg,
            "webm" => Self::Webm,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for VideoContainer {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mov" => Self::Mov,
            "mp4" => Self::Mp4,
            "ogg" => Self::Ogg,
            "webm" => Self::Webm,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for VideoContainer {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for VideoContainer {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for VideoContainer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for VideoContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = VideoContainer;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(VideoContainer::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(VideoContainer::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}
