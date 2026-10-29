// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `users`.

pub mod admin;

pub use crate::activities::UserAvatarColor;
pub use crate::albums::AssetOrder;
pub use crate::system::config::{HlsVideoResolution, VideoCodec};
pub use crate::timeline::CalendarHeatmapResponseSeriesItem;

/// `AlbumsResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `default_asset_order` | [`AssetOrder`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AlbumsResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AlbumsResponse {
    /// (required)
    #[serde(rename = "defaultAssetOrder")]
    pub default_asset_order: AssetOrder,
}

impl AlbumsResponse {
    ///Creates a new [`AlbumsResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(default_asset_order: AssetOrder) -> Self {
        Self { default_asset_order }
    }

    ///Sets `default_asset_order`.
    #[must_use]
    pub fn default_asset_order(mut self, value: AssetOrder) -> Self {
        self.default_asset_order = value;
        self
    }
}

/// Album preferences
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `default_asset_order` | [`AssetOrder`] |  |  |
///
/// See: <https://api.immich.app/models/AlbumsUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct AlbumsUpdate {
    #[serde(
        rename = "defaultAssetOrder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_asset_order: Option<AssetOrder>,
}

impl AlbumsUpdate {
    ///Creates a new [`AlbumsUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            default_asset_order: None,
        }
    }

    ///Sets `default_asset_order`.
    #[must_use]
    pub fn default_asset_order(mut self, value: AssetOrder) -> Self {
        self.default_asset_order = Some(value);
        self
    }

    ///Sets or clears `default_asset_order`.
    #[must_use]
    pub fn set_default_asset_order(mut self, value: Option<AssetOrder>) -> Self {
        self.default_asset_order = value;
        self
    }
}

/// `AvatarUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `color` | [`UserAvatarColor`] |  |  |
///
/// See: <https://api.immich.app/models/AvatarUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct AvatarUpdate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<UserAvatarColor>,
}

impl AvatarUpdate {
    ///Creates a new [`AvatarUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { color: None }
    }

    ///Sets `color`.
    #[must_use]
    pub fn color(mut self, value: UserAvatarColor) -> Self {
        self.color = Some(value);
        self
    }

    ///Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.color = value;
        self
    }
}

/// `CalendarHeatmapResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `from` | [`String`] | ✓ | Start date in UTC |
/// | `series` | `Vec<object>` | ✓ |  |
/// | `to` | [`String`] | ✓ | End date in UTC |
/// | `total_count` | `i64` | ✓ | Total activity count over the period |
///
/// See: <https://api.immich.app/models/CalendarHeatmapResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CalendarHeatmapResponseDTO {
    pub from: String,
    pub series: Vec<CalendarHeatmapResponseSeriesItem>,
    pub to: String,
    /// Total activity count over the period (required)
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}

impl CalendarHeatmapResponseDTO {
    ///Creates a new [`CalendarHeatmapResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        from: impl Into<String>,
        series: Vec<CalendarHeatmapResponseSeriesItem>,
        to: impl Into<String>,
        total_count: i64,
    ) -> Self {
        Self {
            from: from.into(),
            series,
            to: to.into(),
            total_count,
        }
    }

    ///Sets `from`.
    #[must_use]
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = value.into();
        self
    }

    ///Sets `series`.
    #[must_use]
    pub fn series(mut self, value: Vec<CalendarHeatmapResponseSeriesItem>) -> Self {
        self.series = value;
        self
    }

    ///Sets `to`.
    #[must_use]
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = value.into();
        self
    }

    ///Sets `total_count`.
    #[must_use]
    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = value;
        self
    }
}

/// Type of calendar heatmap
///
/// See: <https://api.immich.app/models/CalendarHeatmapType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum CalendarHeatmapType {
    Upload,
    Taken,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for CalendarHeatmapType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Upload => f.write_str("Upload"),
            Self::Taken => f.write_str("Taken"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for CalendarHeatmapType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "Upload" => Ok(Self::Upload),
            "Taken" => Ok(Self::Taken),
            _ => Err(format!("invalid value for {}: {}", "CalendarHeatmapType", value).into()),
        }
    }
}

impl TryFrom<&str> for CalendarHeatmapType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for CalendarHeatmapType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for CalendarHeatmapType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `CastResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `g_cast_enabled` | `bool` | ✓ | Whether Google Cast is enabled |
///
/// See: <https://api.immich.app/models/CastResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CastResponse {
    /// Whether Google Cast is enabled (required)
    #[serde(rename = "gCastEnabled")]
    pub g_cast_enabled: bool,
}

impl CastResponse {
    ///Creates a new [`CastResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(g_cast_enabled: bool) -> Self {
        Self { g_cast_enabled }
    }

    ///Sets `g_cast_enabled`.
    #[must_use]
    pub fn g_cast_enabled(mut self, value: bool) -> Self {
        self.g_cast_enabled = value;
        self
    }
}

/// `CastUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `g_cast_enabled` | `bool` |  | Whether Google Cast is enabled |
///
/// See: <https://api.immich.app/models/CastUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct CastUpdate {
    /// Whether Google Cast is enabled
    #[serde(
        rename = "gCastEnabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub g_cast_enabled: Option<bool>,
}

impl CastUpdate {
    ///Creates a new [`CastUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { g_cast_enabled: None }
    }

    ///Sets `g_cast_enabled`.
    #[must_use]
    pub fn g_cast_enabled(mut self, value: bool) -> Self {
        self.g_cast_enabled = Some(value);
        self
    }

    ///Sets or clears `g_cast_enabled`.
    #[must_use]
    pub fn set_g_cast_enabled(mut self, value: Option<bool>) -> Self {
        self.g_cast_enabled = value;
        self
    }
}

/// `CreateProfileImageDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `file` | `String` | ✓ | Profile image file |
///
/// See: <https://api.immich.app/models/CreateProfileImageDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CreateProfileImageDTO {
    pub file: String,
}

impl CreateProfileImageDTO {
    ///Creates a new [`CreateProfileImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into() }
    }

    ///Sets `file`.
    #[must_use]
    pub fn file(mut self, value: impl Into<String>) -> Self {
        self.file = value.into();
        self
    }
}

/// `CreateProfileImageResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile image change date |
/// | `profile_image_path` | [`String`] | ✓ | Profile image file path |
/// | `user_id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
///
/// See: <https://api.immich.app/models/CreateProfileImageResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CreateProfileImageResponseDTO {
    /// Profile image change date (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Profile image file path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    /// User ID (required)
    #[serde(rename = "userId")]
    pub user_id: ::uuid::Uuid,
}

impl CreateProfileImageResponseDTO {
    ///Creates a new [`CreateProfileImageResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        profile_image_path: impl Into<String>,
        user_id: ::uuid::Uuid,
    ) -> Self {
        Self {
            profile_changed_at,
            profile_image_path: profile_image_path.into(),
            user_id,
        }
    }

    ///Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }

    ///Sets `profile_image_path`.
    #[must_use]
    pub fn profile_image_path(mut self, value: impl Into<String>) -> Self {
        self.profile_image_path = value.into();
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = value;
        self
    }
}

/// `DownloadResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `archive_size` | `i64` | ✓ | Maximum archive size in bytes |
/// | `include_embedded_videos` | `bool` | ✓ | Whether to include embedded videos in downloads |
///
/// See: <https://api.immich.app/models/DownloadResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct DownloadResponse {
    /// Maximum archive size in bytes (required)
    #[serde(rename = "archiveSize")]
    pub archive_size: i64,
    /// Whether to include embedded videos in downloads (required)
    #[serde(rename = "includeEmbeddedVideos")]
    pub include_embedded_videos: bool,
}

impl DownloadResponse {
    ///Creates a new [`DownloadResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(archive_size: i64, include_embedded_videos: bool) -> Self {
        Self {
            archive_size,
            include_embedded_videos,
        }
    }

    ///Sets `archive_size`.
    #[must_use]
    pub fn archive_size(mut self, value: i64) -> Self {
        self.archive_size = value;
        self
    }

    ///Sets `include_embedded_videos`.
    #[must_use]
    pub fn include_embedded_videos(mut self, value: bool) -> Self {
        self.include_embedded_videos = value;
        self
    }
}

/// `DownloadUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `archive_size` | `i64` |  | Maximum archive size in bytes |
/// | `include_embedded_videos` | `bool` |  | Whether to include embedded videos in downloads |
///
/// See: <https://api.immich.app/models/DownloadUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct DownloadUpdate {
    /// Maximum archive size in bytes
    #[serde(
        rename = "archiveSize",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub archive_size: Option<::std::num::NonZeroU64>,
    /// Whether to include embedded videos in downloads
    #[serde(
        rename = "includeEmbeddedVideos",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_embedded_videos: Option<bool>,
}

impl DownloadUpdate {
    ///Creates a new [`DownloadUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            archive_size: None,
            include_embedded_videos: None,
        }
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

    ///Sets `include_embedded_videos`.
    #[must_use]
    pub fn include_embedded_videos(mut self, value: bool) -> Self {
        self.include_embedded_videos = Some(value);
        self
    }

    ///Sets or clears `include_embedded_videos`.
    #[must_use]
    pub fn set_include_embedded_videos(mut self, value: Option<bool>) -> Self {
        self.include_embedded_videos = value;
        self
    }
}

/// `EmailNotificationsResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_invite` | `bool` | ✓ | Whether to receive email notifications for album invites |
/// | `album_update` | `bool` | ✓ | Whether to receive email notifications for album updates |
/// | `enabled` | `bool` | ✓ | Whether email notifications are enabled |
///
/// See: <https://api.immich.app/models/EmailNotificationsResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct EmailNotificationsResponse {
    /// Whether to receive email notifications for album invites (required)
    #[serde(rename = "albumInvite")]
    pub album_invite: bool,
    /// Whether to receive email notifications for album updates (required)
    #[serde(rename = "albumUpdate")]
    pub album_update: bool,
    pub enabled: bool,
}

impl EmailNotificationsResponse {
    ///Creates a new [`EmailNotificationsResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_invite: bool, album_update: bool, enabled: bool) -> Self {
        Self {
            album_invite,
            album_update,
            enabled,
        }
    }

    ///Sets `album_invite`.
    #[must_use]
    pub fn album_invite(mut self, value: bool) -> Self {
        self.album_invite = value;
        self
    }

    ///Sets `album_update`.
    #[must_use]
    pub fn album_update(mut self, value: bool) -> Self {
        self.album_update = value;
        self
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `EmailNotificationsUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_invite` | `bool` |  | Whether to receive email notifications for album invites |
/// | `album_update` | `bool` |  | Whether to receive email notifications for album updates |
/// | `enabled` | `bool` |  | Whether email notifications are enabled |
///
/// See: <https://api.immich.app/models/EmailNotificationsUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct EmailNotificationsUpdate {
    /// Whether to receive email notifications for album invites
    #[serde(
        rename = "albumInvite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub album_invite: Option<bool>,
    /// Whether to receive email notifications for album updates
    #[serde(
        rename = "albumUpdate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub album_update: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
}

impl EmailNotificationsUpdate {
    ///Creates a new [`EmailNotificationsUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_invite: None,
            album_update: None,
            enabled: None,
        }
    }

    ///Sets `album_invite`.
    #[must_use]
    pub fn album_invite(mut self, value: bool) -> Self {
        self.album_invite = Some(value);
        self
    }

    ///Sets or clears `album_invite`.
    #[must_use]
    pub fn set_album_invite(mut self, value: Option<bool>) -> Self {
        self.album_invite = value;
        self
    }

    ///Sets `album_update`.
    #[must_use]
    pub fn album_update(mut self, value: bool) -> Self {
        self.album_update = Some(value);
        self
    }

    ///Sets or clears `album_update`.
    #[must_use]
    pub fn set_album_update(mut self, value: Option<bool>) -> Self {
        self.album_update = value;
        self
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    ///Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }
}

/// `FoldersResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether folders are enabled |
/// | `sidebar_web` | `bool` | ✓ | Whether folders appear in web sidebar |
///
/// See: <https://api.immich.app/models/FoldersResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct FoldersResponse {
    pub enabled: bool,
    /// Whether folders appear in web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl FoldersResponse {
    ///Creates a new [`FoldersResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, sidebar_web: bool) -> Self {
        Self { enabled, sidebar_web }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `FoldersUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` |  | Whether folders are enabled |
/// | `sidebar_web` | `bool` |  | Whether folders appear in web sidebar |
///
/// See: <https://api.immich.app/models/FoldersUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct FoldersUpdate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether folders appear in web sidebar
    #[serde(
        rename = "sidebarWeb",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sidebar_web: Option<bool>,
}

impl FoldersUpdate {
    ///Creates a new [`FoldersUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            enabled: None,
            sidebar_web: None,
        }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    ///Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }

    ///Sets or clears `sidebar_web`.
    #[must_use]
    pub fn set_sidebar_web(mut self, value: Option<bool>) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `MemoriesResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `duration` | `i64` | ✓ | Memory duration in seconds |
/// | `enabled` | `bool` | ✓ | Whether memories are enabled |
/// | `sidebar_web` | `bool` | ✓ | Whether memories appear in web sidebar |
///
/// See: <https://api.immich.app/models/MemoriesResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MemoriesResponse {
    pub duration: i64,
    pub enabled: bool,
    /// Whether memories appear in web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl MemoriesResponse {
    ///Creates a new [`MemoriesResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(duration: i64, enabled: bool, sidebar_web: bool) -> Self {
        Self {
            duration,
            enabled,
            sidebar_web,
        }
    }

    ///Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = value;
        self
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `MemoriesUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `duration` | `i64` |  | Memory duration in seconds |
/// | `enabled` | `bool` |  | Whether memories are enabled |
/// | `sidebar_web` | `bool` |  | Whether memories appear in web sidebar |
///
/// See: <https://api.immich.app/models/MemoriesUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct MemoriesUpdate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: Option<::std::num::NonZeroU64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether memories appear in web sidebar
    #[serde(
        rename = "sidebarWeb",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sidebar_web: Option<bool>,
}

impl MemoriesUpdate {
    ///Creates a new [`MemoriesUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            duration: None,
            enabled: None,
            sidebar_web: None,
        }
    }

    ///Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.duration = Some(value);
        self
    }

    ///Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.duration = value;
        self
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    ///Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }

    ///Sets or clears `sidebar_web`.
    #[must_use]
    pub fn set_sidebar_web(mut self, value: Option<bool>) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `OnboardingDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `is_onboarded` | `bool` | ✓ | Is user onboarded |
///
/// See: <https://api.immich.app/models/OnboardingDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct OnboardingDTO {
    /// Is user onboarded (required)
    #[serde(rename = "isOnboarded")]
    pub is_onboarded: bool,
}

impl OnboardingDTO {
    ///Creates a new [`OnboardingDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_onboarded: bool) -> Self {
        Self { is_onboarded }
    }

    ///Sets `is_onboarded`.
    #[must_use]
    pub fn is_onboarded(mut self, value: bool) -> Self {
        self.is_onboarded = value;
        self
    }
}

/// `OnboardingResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `is_onboarded` | `bool` | ✓ | Is user onboarded |
///
/// See: <https://api.immich.app/models/OnboardingResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct OnboardingResponseDTO {
    /// Is user onboarded (required)
    #[serde(rename = "isOnboarded")]
    pub is_onboarded: bool,
}

impl OnboardingResponseDTO {
    ///Creates a new [`OnboardingResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_onboarded: bool) -> Self {
        Self { is_onboarded }
    }

    ///Sets `is_onboarded`.
    #[must_use]
    pub fn is_onboarded(mut self, value: bool) -> Self {
        self.is_onboarded = value;
        self
    }
}

/// `PeopleResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether people are enabled |
/// | `minimum_faces` | `i64` |  | People face threshold |
/// | `sidebar_web` | `bool` | ✓ | Whether people appear in web sidebar |
/// | `update_strategy` | [`PersonUpdateStrategy`] | ✓ |  |
///
/// See: <https://api.immich.app/models/PeopleResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PeopleResponse {
    pub enabled: bool,
    /// People face threshold
    #[serde(
        rename = "minimumFaces",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub minimum_faces: Option<::std::num::NonZeroU64>,
    /// Whether people appear in web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
    /// (required)
    #[serde(rename = "updateStrategy")]
    pub update_strategy: PersonUpdateStrategy,
}

impl PeopleResponse {
    ///Creates a new [`PeopleResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, sidebar_web: bool, update_strategy: PersonUpdateStrategy) -> Self {
        Self {
            enabled,
            minimum_faces: None,
            sidebar_web,
            update_strategy,
        }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `minimum_faces`.
    #[must_use]
    pub fn minimum_faces(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.minimum_faces = Some(value);
        self
    }

    ///Sets or clears `minimum_faces`.
    #[must_use]
    pub fn set_minimum_faces(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.minimum_faces = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = value;
        self
    }

    ///Sets `update_strategy`.
    #[must_use]
    pub fn update_strategy(mut self, value: PersonUpdateStrategy) -> Self {
        self.update_strategy = value;
        self
    }
}

/// `PeopleUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` |  | Whether people are enabled |
/// | `minimum_faces` | `i64` |  | People face threshold |
/// | `sidebar_web` | `bool` |  | Whether people appear in web sidebar |
/// | `update_strategy` | [`PersonUpdateStrategy`] |  |  |
///
/// See: <https://api.immich.app/models/PeopleUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct PeopleUpdate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// People face threshold
    #[serde(
        rename = "minimumFaces",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub minimum_faces: Option<::std::num::NonZeroU64>,
    /// Whether people appear in web sidebar
    #[serde(
        rename = "sidebarWeb",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sidebar_web: Option<bool>,
    #[serde(
        rename = "updateStrategy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub update_strategy: Option<PersonUpdateStrategy>,
}

impl PeopleUpdate {
    ///Creates a new [`PeopleUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            enabled: None,
            minimum_faces: None,
            sidebar_web: None,
            update_strategy: None,
        }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    ///Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `minimum_faces`.
    #[must_use]
    pub fn minimum_faces(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.minimum_faces = Some(value);
        self
    }

    ///Sets or clears `minimum_faces`.
    #[must_use]
    pub fn set_minimum_faces(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.minimum_faces = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }

    ///Sets or clears `sidebar_web`.
    #[must_use]
    pub fn set_sidebar_web(mut self, value: Option<bool>) -> Self {
        self.sidebar_web = value;
        self
    }

    ///Sets `update_strategy`.
    #[must_use]
    pub fn update_strategy(mut self, value: PersonUpdateStrategy) -> Self {
        self.update_strategy = Some(value);
        self
    }

    ///Sets or clears `update_strategy`.
    #[must_use]
    pub fn set_update_strategy(mut self, value: Option<PersonUpdateStrategy>) -> Self {
        self.update_strategy = value;
        self
    }
}

/// Which person records to update when editing a person
///
/// See: <https://api.immich.app/models/PersonUpdateStrategy>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PersonUpdateStrategy {
    #[serde(rename = "self")]
    Self_,
    #[serde(rename = "everyone")]
    Everyone,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for PersonUpdateStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Self_ => f.write_str("self"),
            Self::Everyone => f.write_str("everyone"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for PersonUpdateStrategy {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "self" => Ok(Self::Self_),
            "everyone" => Ok(Self::Everyone),
            _ => Err(format!("invalid value for {}: {}", "PersonUpdateStrategy", value).into()),
        }
    }
}

impl TryFrom<&str> for PersonUpdateStrategy {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PersonUpdateStrategy {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PersonUpdateStrategy {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `PurchaseResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `hide_buy_button_until` | [`String`] | ✓ | Date until which to hide buy button |
/// | `show_support_badge` | `bool` | ✓ | Whether to show support badge |
///
/// See: <https://api.immich.app/models/PurchaseResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PurchaseResponse {
    /// Date until which to hide buy button (required)
    #[serde(rename = "hideBuyButtonUntil")]
    pub hide_buy_button_until: String,
    /// Whether to show support badge (required)
    #[serde(rename = "showSupportBadge")]
    pub show_support_badge: bool,
}

impl PurchaseResponse {
    ///Creates a new [`PurchaseResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(hide_buy_button_until: impl Into<String>, show_support_badge: bool) -> Self {
        Self {
            hide_buy_button_until: hide_buy_button_until.into(),
            show_support_badge,
        }
    }

    ///Sets `hide_buy_button_until`.
    #[must_use]
    pub fn hide_buy_button_until(mut self, value: impl Into<String>) -> Self {
        self.hide_buy_button_until = value.into();
        self
    }

    ///Sets `show_support_badge`.
    #[must_use]
    pub fn show_support_badge(mut self, value: bool) -> Self {
        self.show_support_badge = value;
        self
    }
}

/// `PurchaseUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `hide_buy_button_until` | [`String`] |  | Date until which to hide buy button |
/// | `show_support_badge` | `bool` |  | Whether to show support badge |
///
/// See: <https://api.immich.app/models/PurchaseUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct PurchaseUpdate {
    /// Date until which to hide buy button
    #[serde(
        rename = "hideBuyButtonUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hide_buy_button_until: Option<String>,
    /// Whether to show support badge
    #[serde(
        rename = "showSupportBadge",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub show_support_badge: Option<bool>,
}

impl PurchaseUpdate {
    ///Creates a new [`PurchaseUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            hide_buy_button_until: None,
            show_support_badge: None,
        }
    }

    ///Sets `hide_buy_button_until`.
    #[must_use]
    pub fn hide_buy_button_until(mut self, value: impl Into<String>) -> Self {
        self.hide_buy_button_until = Some(value.into());
        self
    }

    ///Sets or clears `hide_buy_button_until`.
    #[must_use]
    pub fn set_hide_buy_button_until(mut self, value: Option<String>) -> Self {
        self.hide_buy_button_until = value;
        self
    }

    ///Sets `show_support_badge`.
    #[must_use]
    pub fn show_support_badge(mut self, value: bool) -> Self {
        self.show_support_badge = Some(value);
        self
    }

    ///Sets or clears `show_support_badge`.
    #[must_use]
    pub fn set_show_support_badge(mut self, value: Option<bool>) -> Self {
        self.show_support_badge = value;
        self
    }
}

/// `RatingsResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether ratings are enabled |
///
/// See: <https://api.immich.app/models/RatingsResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct RatingsResponse {
    pub enabled: bool,
}

impl RatingsResponse {
    ///Creates a new [`RatingsResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
}

/// `RatingsUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` |  | Whether ratings are enabled |
///
/// See: <https://api.immich.app/models/RatingsUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct RatingsUpdate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
}

impl RatingsUpdate {
    ///Creates a new [`RatingsUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { enabled: None }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    ///Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }
}

/// `RecentlyAddedResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `sidebar_web` | `bool` | ✓ | Whether the recently added page appears in the web sidebar |
///
/// See: <https://api.immich.app/models/RecentlyAddedResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct RecentlyAddedResponse {
    /// Whether the recently added page appears in the web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl RecentlyAddedResponse {
    ///Creates a new [`RecentlyAddedResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(sidebar_web: bool) -> Self {
        Self { sidebar_web }
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `RecentlyAddedUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `sidebar_web` | `bool` |  | Whether the recently added page appears in the web sidebar |
///
/// See: <https://api.immich.app/models/RecentlyAddedUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct RecentlyAddedUpdate {
    /// Whether the recently added page appears in the web sidebar
    #[serde(
        rename = "sidebarWeb",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sidebar_web: Option<bool>,
}

impl RecentlyAddedUpdate {
    ///Creates a new [`RecentlyAddedUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { sidebar_web: None }
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }

    ///Sets or clears `sidebar_web`.
    #[must_use]
    pub fn set_sidebar_web(mut self, value: Option<bool>) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `SharedLinksResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether shared links are enabled |
/// | `sidebar_web` | `bool` | ✓ | Whether shared links appear in web sidebar |
///
/// See: <https://api.immich.app/models/SharedLinksResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SharedLinksResponse {
    pub enabled: bool,
    /// Whether shared links appear in web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl SharedLinksResponse {
    ///Creates a new [`SharedLinksResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, sidebar_web: bool) -> Self {
        Self { enabled, sidebar_web }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `SharedLinksUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` |  | Whether shared links are enabled |
/// | `sidebar_web` | `bool` |  | Whether shared links appear in web sidebar |
///
/// See: <https://api.immich.app/models/SharedLinksUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct SharedLinksUpdate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether shared links appear in web sidebar
    #[serde(
        rename = "sidebarWeb",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sidebar_web: Option<bool>,
}

impl SharedLinksUpdate {
    ///Creates a new [`SharedLinksUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            enabled: None,
            sidebar_web: None,
        }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    ///Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }

    ///Sets or clears `sidebar_web`.
    #[must_use]
    pub fn set_sidebar_web(mut self, value: Option<bool>) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `TagsResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether tags are enabled |
/// | `sidebar_web` | `bool` | ✓ | Whether tags appear in web sidebar |
///
/// See: <https://api.immich.app/models/TagsResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TagsResponse {
    pub enabled: bool,
    /// Whether tags appear in web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl TagsResponse {
    ///Creates a new [`TagsResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, sidebar_web: bool) -> Self {
        Self { enabled, sidebar_web }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `TagsUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` |  | Whether tags are enabled |
/// | `sidebar_web` | `bool` |  | Whether tags appear in web sidebar |
///
/// See: <https://api.immich.app/models/TagsUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct TagsUpdate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether tags appear in web sidebar
    #[serde(
        rename = "sidebarWeb",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sidebar_web: Option<bool>,
}

impl TagsUpdate {
    ///Creates a new [`TagsUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            enabled: None,
            sidebar_web: None,
        }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    ///Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }

    ///Sets or clears `sidebar_web`.
    #[must_use]
    pub fn set_sidebar_web(mut self, value: Option<bool>) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// `UserConfigClipDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` | ✓ | Whether the task is enabled |
///
/// See: <https://api.immich.app/models/UserConfigClipDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigClipDTO {
    pub enabled: bool,
}

impl UserConfigClipDTO {
    ///Creates a new [`UserConfigClipDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    ///Sets `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigDTO {
    pub ffmpeg: UserConfigFFmpegDTO,
    pub image: UserConfigImageDTO,
    /// (required)
    #[serde(rename = "machineLearning")]
    pub machine_learning: UserConfigMachineLearningDTO,
    pub map: UserConfigMapDTO,
    pub oauth: UserConfigOAuthDTO,
    /// (required)
    #[serde(rename = "passwordLogin")]
    pub password_login: UserConfigPasswordLoginDTO,
    /// (required)
    #[serde(rename = "reverseGeocoding")]
    pub reverse_geocoding: UserConfigReverseGeocodingDTO,
    pub server: UserConfigServerDTO,
    pub theme: UserConfigThemeDTO,
    pub trash: UserConfigTrashDTO,
    pub user: UserConfigUserDTO,
}

impl UserConfigDTO {
    ///Creates a new [`UserConfigDTO`].
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

    ///Sets `ffmpeg`.
    #[must_use]
    pub fn ffmpeg(mut self, value: UserConfigFFmpegDTO) -> Self {
        self.ffmpeg = value;
        self
    }

    ///Sets `image`.
    #[must_use]
    pub fn image(mut self, value: UserConfigImageDTO) -> Self {
        self.image = value;
        self
    }

    ///Sets `machine_learning`.
    #[must_use]
    pub fn machine_learning(mut self, value: UserConfigMachineLearningDTO) -> Self {
        self.machine_learning = value;
        self
    }

    ///Sets `map`.
    #[must_use]
    pub fn map(mut self, value: UserConfigMapDTO) -> Self {
        self.map = value;
        self
    }

    ///Sets `oauth`.
    #[must_use]
    pub fn oauth(mut self, value: UserConfigOAuthDTO) -> Self {
        self.oauth = value;
        self
    }

    ///Sets `password_login`.
    #[must_use]
    pub fn password_login(mut self, value: UserConfigPasswordLoginDTO) -> Self {
        self.password_login = value;
        self
    }

    ///Sets `reverse_geocoding`.
    #[must_use]
    pub fn reverse_geocoding(mut self, value: UserConfigReverseGeocodingDTO) -> Self {
        self.reverse_geocoding = value;
        self
    }

    ///Sets `server`.
    #[must_use]
    pub fn server(mut self, value: UserConfigServerDTO) -> Self {
        self.server = value;
        self
    }

    ///Sets `theme`.
    #[must_use]
    pub fn theme(mut self, value: UserConfigThemeDTO) -> Self {
        self.theme = value;
        self
    }

    ///Sets `trash`.
    #[must_use]
    pub fn trash(mut self, value: UserConfigTrashDTO) -> Self {
        self.trash = value;
        self
    }

    ///Sets `user`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigDuplicateDetectionDTO {
    pub enabled: bool,
}

impl UserConfigDuplicateDetectionDTO {
    ///Creates a new [`UserConfigDuplicateDetectionDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    ///Sets `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigFFmpegDTO {
    pub realtime: UserConfigFFmpegRealtimeDTO,
}

impl UserConfigFFmpegDTO {
    ///Creates a new [`UserConfigFFmpegDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(realtime: UserConfigFFmpegRealtimeDTO) -> Self {
        Self { realtime }
    }

    ///Sets `realtime`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigFFmpegRealtimeDTO {
    pub enabled: bool,
    pub resolutions: Vec<HlsVideoResolution>,
    /// Video codecs to use for real-time HLS transcoding (required)
    #[serde(rename = "videoCodecs")]
    pub video_codecs: Vec<VideoCodec>,
}

impl UserConfigFFmpegRealtimeDTO {
    ///Creates a new [`UserConfigFFmpegRealtimeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, resolutions: Vec<HlsVideoResolution>, video_codecs: Vec<VideoCodec>) -> Self {
        Self {
            enabled,
            resolutions,
            video_codecs,
        }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `resolutions`.
    #[must_use]
    pub fn resolutions(mut self, value: Vec<HlsVideoResolution>) -> Self {
        self.resolutions = value;
        self
    }

    ///Sets `video_codecs`.
    #[must_use]
    pub fn video_codecs(mut self, value: Vec<VideoCodec>) -> Self {
        self.video_codecs = value;
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigFacialRecognitionDTO {
    pub enabled: bool,
    /// Minimum number of faces required for recognition (required)
    #[serde(rename = "minFaces")]
    pub min_faces: ::std::num::NonZeroU64,
}

impl UserConfigFacialRecognitionDTO {
    ///Creates a new [`UserConfigFacialRecognitionDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, min_faces: ::std::num::NonZeroU64) -> Self {
        Self { enabled, min_faces }
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `min_faces`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigGeneratedFullsizeImageDTO {
    pub enabled: bool,
}

impl UserConfigGeneratedFullsizeImageDTO {
    ///Creates a new [`UserConfigGeneratedFullsizeImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    ///Sets `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigGeneratedImageDTO {
    pub size: ::std::num::NonZeroU64,
}

impl UserConfigGeneratedImageDTO {
    ///Creates a new [`UserConfigGeneratedImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(size: ::std::num::NonZeroU64) -> Self {
        Self { size }
    }

    ///Sets `size`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigImageDTO {
    pub fullsize: UserConfigGeneratedFullsizeImageDTO,
    pub preview: UserConfigGeneratedImageDTO,
    pub thumbnail: UserConfigGeneratedImageDTO,
}

impl UserConfigImageDTO {
    ///Creates a new [`UserConfigImageDTO`].
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

    ///Sets `fullsize`.
    #[must_use]
    pub fn fullsize(mut self, value: UserConfigGeneratedFullsizeImageDTO) -> Self {
        self.fullsize = value;
        self
    }

    ///Sets `preview`.
    #[must_use]
    pub fn preview(mut self, value: UserConfigGeneratedImageDTO) -> Self {
        self.preview = value;
        self
    }

    ///Sets `thumbnail`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigMachineLearningDTO {
    pub clip: UserConfigClipDTO,
    /// (required)
    #[serde(rename = "duplicateDetection")]
    pub duplicate_detection: UserConfigDuplicateDetectionDTO,
    pub enabled: bool,
    /// (required)
    #[serde(rename = "facialRecognition")]
    pub facial_recognition: UserConfigFacialRecognitionDTO,
    pub ocr: UserConfigOcrDTO,
}

impl UserConfigMachineLearningDTO {
    ///Creates a new [`UserConfigMachineLearningDTO`].
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

    ///Sets `clip`.
    #[must_use]
    pub fn clip(mut self, value: UserConfigClipDTO) -> Self {
        self.clip = value;
        self
    }

    ///Sets `duplicate_detection`.
    #[must_use]
    pub fn duplicate_detection(mut self, value: UserConfigDuplicateDetectionDTO) -> Self {
        self.duplicate_detection = value;
        self
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `facial_recognition`.
    #[must_use]
    pub fn facial_recognition(mut self, value: UserConfigFacialRecognitionDTO) -> Self {
        self.facial_recognition = value;
        self
    }

    ///Sets `ocr`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigMapDTO {
    /// Dark map style URL (required)
    #[serde(rename = "darkStyle")]
    pub dark_style: String,
    pub enabled: bool,
    /// Light map style URL (required)
    #[serde(rename = "lightStyle")]
    pub light_style: String,
}

impl UserConfigMapDTO {
    ///Creates a new [`UserConfigMapDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(dark_style: impl Into<String>, enabled: bool, light_style: impl Into<String>) -> Self {
        Self {
            dark_style: dark_style.into(),
            enabled,
            light_style: light_style.into(),
        }
    }

    ///Sets `dark_style`.
    #[must_use]
    pub fn dark_style(mut self, value: impl Into<String>) -> Self {
        self.dark_style = value.into();
        self
    }

    ///Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    ///Sets `light_style`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigOAuthDTO {
    /// Auto launch (required)
    #[serde(rename = "autoLaunch")]
    pub auto_launch: bool,
    /// Button text (required)
    #[serde(rename = "buttonText")]
    pub button_text: String,
    pub enabled: bool,
}

impl UserConfigOAuthDTO {
    ///Creates a new [`UserConfigOAuthDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(auto_launch: bool, button_text: impl Into<String>, enabled: bool) -> Self {
        Self {
            auto_launch,
            button_text: button_text.into(),
            enabled,
        }
    }

    ///Sets `auto_launch`.
    #[must_use]
    pub fn auto_launch(mut self, value: bool) -> Self {
        self.auto_launch = value;
        self
    }

    ///Sets `button_text`.
    #[must_use]
    pub fn button_text(mut self, value: impl Into<String>) -> Self {
        self.button_text = value.into();
        self
    }

    ///Sets `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigOcrDTO {
    pub enabled: bool,
}

impl UserConfigOcrDTO {
    ///Creates a new [`UserConfigOcrDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    ///Sets `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigPasswordLoginDTO {
    pub enabled: bool,
}

impl UserConfigPasswordLoginDTO {
    ///Creates a new [`UserConfigPasswordLoginDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    ///Sets `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigReverseGeocodingDTO {
    pub enabled: bool,
}

impl UserConfigReverseGeocodingDTO {
    ///Creates a new [`UserConfigReverseGeocodingDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    ///Sets `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
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
    ///Creates a new [`UserConfigServerDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(external_domain: impl Into<String>, login_page_message: impl Into<String>, public_users: bool) -> Self {
        Self {
            external_domain: external_domain.into(),
            login_page_message: login_page_message.into(),
            public_users,
        }
    }

    ///Sets `external_domain`.
    #[must_use]
    pub fn external_domain(mut self, value: impl Into<String>) -> Self {
        self.external_domain = value.into();
        self
    }

    ///Sets `login_page_message`.
    #[must_use]
    pub fn login_page_message(mut self, value: impl Into<String>) -> Self {
        self.login_page_message = value.into();
        self
    }

    ///Sets `public_users`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigThemeDTO {
    /// Custom CSS for theming (required)
    #[serde(rename = "customCss")]
    pub custom_css: String,
}

impl UserConfigThemeDTO {
    ///Creates a new [`UserConfigThemeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(custom_css: impl Into<String>) -> Self {
        Self {
            custom_css: custom_css.into(),
        }
    }

    ///Sets `custom_css`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigTrashDTO {
    pub days: i64,
    pub enabled: bool,
}

impl UserConfigTrashDTO {
    ///Creates a new [`UserConfigTrashDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(days: i64, enabled: bool) -> Self {
        Self { days, enabled }
    }

    ///Sets `days`.
    #[must_use]
    pub fn days(mut self, value: i64) -> Self {
        self.days = value;
        self
    }

    ///Sets `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserConfigUserDTO {
    /// Delete delay (required)
    #[serde(rename = "deleteDelay")]
    pub delete_delay: ::std::num::NonZeroU64,
}

impl UserConfigUserDTO {
    ///Creates a new [`UserConfigUserDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(delete_delay: ::std::num::NonZeroU64) -> Self {
        Self { delete_delay }
    }

    ///Sets `delete_delay`.
    #[must_use]
    pub fn delete_delay(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.delete_delay = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UserLicenseLicenseKey(pub String);

impl ::std::ops::Deref for UserLicenseLicenseKey {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for UserLicenseLicenseKey {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> =
            ::std::sync::LazyLock::new(|| ::regex::Regex::new("^IM(SV|CL)(-[\\dA-Za-z]{4}){8}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^IM(SV|CL)(-[\\dA-Za-z]{4}){8}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for UserLicenseLicenseKey {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for UserLicenseLicenseKey {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for UserLicenseLicenseKey {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for UserLicenseLicenseKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// User metadata key
///
/// See: <https://api.immich.app/models/UserMetadataKey>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum UserMetadataKey {
    #[serde(rename = "preferences")]
    Preferences,
    #[serde(rename = "license")]
    License,
    #[serde(rename = "onboarding")]
    Onboarding,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for UserMetadataKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Preferences => f.write_str("preferences"),
            Self::License => f.write_str("license"),
            Self::Onboarding => f.write_str("onboarding"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for UserMetadataKey {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "preferences" => Ok(Self::Preferences),
            "license" => Ok(Self::License),
            "onboarding" => Ok(Self::Onboarding),
            _ => Err(format!("invalid value for {}: {}", "UserMetadataKey", value).into()),
        }
    }
}

impl TryFrom<&str> for UserMetadataKey {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for UserMetadataKey {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for UserMetadataKey {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `UserPreferencesResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `albums` | [`AlbumsResponse`] | ✓ |  |
/// | `cast` | [`CastResponse`] | ✓ |  |
/// | `download` | [`DownloadResponse`] | ✓ |  |
/// | `email_notifications` | [`EmailNotificationsResponse`] | ✓ |  |
/// | `folders` | [`FoldersResponse`] | ✓ |  |
/// | `memories` | [`MemoriesResponse`] | ✓ |  |
/// | `people` | [`PeopleResponse`] | ✓ |  |
/// | `purchase` | [`PurchaseResponse`] | ✓ |  |
/// | `ratings` | [`RatingsResponse`] | ✓ |  |
/// | `recently_added` | [`RecentlyAddedResponse`] | ✓ |  |
/// | `shared_links` | [`SharedLinksResponse`] | ✓ |  |
/// | `tags` | [`TagsResponse`] | ✓ |  |
///
/// See: <https://api.immich.app/models/UserPreferencesResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct UserPreferencesResponseDTO {
    pub albums: AlbumsResponse,
    pub cast: CastResponse,
    pub download: DownloadResponse,
    /// (required)
    #[serde(rename = "emailNotifications")]
    pub email_notifications: EmailNotificationsResponse,
    pub folders: FoldersResponse,
    pub memories: MemoriesResponse,
    pub people: PeopleResponse,
    pub purchase: PurchaseResponse,
    pub ratings: RatingsResponse,
    /// (required)
    #[serde(rename = "recentlyAdded")]
    pub recently_added: RecentlyAddedResponse,
    /// (required)
    #[serde(rename = "sharedLinks")]
    pub shared_links: SharedLinksResponse,
    pub tags: TagsResponse,
}

impl UserPreferencesResponseDTO {
    ///Creates a new [`UserPreferencesResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        albums: AlbumsResponse,
        cast: CastResponse,
        download: DownloadResponse,
        email_notifications: EmailNotificationsResponse,
        folders: FoldersResponse,
        memories: MemoriesResponse,
        people: PeopleResponse,
        purchase: PurchaseResponse,
        ratings: RatingsResponse,
        recently_added: RecentlyAddedResponse,
        shared_links: SharedLinksResponse,
        tags: TagsResponse,
    ) -> Self {
        Self {
            albums,
            cast,
            download,
            email_notifications,
            folders,
            memories,
            people,
            purchase,
            ratings,
            recently_added,
            shared_links,
            tags,
        }
    }

    ///Sets `albums`.
    #[must_use]
    pub fn albums(mut self, value: AlbumsResponse) -> Self {
        self.albums = value;
        self
    }

    ///Sets `cast`.
    #[must_use]
    pub fn cast(mut self, value: CastResponse) -> Self {
        self.cast = value;
        self
    }

    ///Sets `download`.
    #[must_use]
    pub fn download(mut self, value: DownloadResponse) -> Self {
        self.download = value;
        self
    }

    ///Sets `email_notifications`.
    #[must_use]
    pub fn email_notifications(mut self, value: EmailNotificationsResponse) -> Self {
        self.email_notifications = value;
        self
    }

    ///Sets `folders`.
    #[must_use]
    pub fn folders(mut self, value: FoldersResponse) -> Self {
        self.folders = value;
        self
    }

    ///Sets `memories`.
    #[must_use]
    pub fn memories(mut self, value: MemoriesResponse) -> Self {
        self.memories = value;
        self
    }

    ///Sets `people`.
    #[must_use]
    pub fn people(mut self, value: PeopleResponse) -> Self {
        self.people = value;
        self
    }

    ///Sets `purchase`.
    #[must_use]
    pub fn purchase(mut self, value: PurchaseResponse) -> Self {
        self.purchase = value;
        self
    }

    ///Sets `ratings`.
    #[must_use]
    pub fn ratings(mut self, value: RatingsResponse) -> Self {
        self.ratings = value;
        self
    }

    ///Sets `recently_added`.
    #[must_use]
    pub fn recently_added(mut self, value: RecentlyAddedResponse) -> Self {
        self.recently_added = value;
        self
    }

    ///Sets `shared_links`.
    #[must_use]
    pub fn shared_links(mut self, value: SharedLinksResponse) -> Self {
        self.shared_links = value;
        self
    }

    ///Sets `tags`.
    #[must_use]
    pub fn tags(mut self, value: TagsResponse) -> Self {
        self.tags = value;
        self
    }
}

/// `UserPreferencesUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `albums` | [`AlbumsUpdate`] |  |  |
/// | `avatar` | [`AvatarUpdate`] |  |  |
/// | `cast` | [`CastUpdate`] |  |  |
/// | `download` | [`DownloadUpdate`] |  |  |
/// | `email_notifications` | [`EmailNotificationsUpdate`] |  |  |
/// | `folders` | [`FoldersUpdate`] |  |  |
/// | `memories` | [`MemoriesUpdate`] |  |  |
/// | `people` | [`PeopleUpdate`] |  |  |
/// | `purchase` | [`PurchaseUpdate`] |  |  |
/// | `ratings` | [`RatingsUpdate`] |  |  |
/// | `recently_added` | [`RecentlyAddedUpdate`] |  |  |
/// | `shared_links` | [`SharedLinksUpdate`] |  |  |
/// | `tags` | [`TagsUpdate`] |  |  |
///
/// See: <https://api.immich.app/models/UserPreferencesUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct UserPreferencesUpdateDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub albums: Option<AlbumsUpdate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: Option<AvatarUpdate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cast: Option<CastUpdate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub download: Option<DownloadUpdate>,
    #[serde(
        rename = "emailNotifications",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub email_notifications: Option<EmailNotificationsUpdate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub folders: Option<FoldersUpdate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memories: Option<MemoriesUpdate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub people: Option<PeopleUpdate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub purchase: Option<PurchaseUpdate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ratings: Option<RatingsUpdate>,
    #[serde(
        rename = "recentlyAdded",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub recently_added: Option<RecentlyAddedUpdate>,
    #[serde(
        rename = "sharedLinks",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub shared_links: Option<SharedLinksUpdate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: Option<TagsUpdate>,
}

impl UserPreferencesUpdateDTO {
    ///Creates a new [`UserPreferencesUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            albums: None,
            avatar: None,
            cast: None,
            download: None,
            email_notifications: None,
            folders: None,
            memories: None,
            people: None,
            purchase: None,
            ratings: None,
            recently_added: None,
            shared_links: None,
            tags: None,
        }
    }

    ///Sets `albums`.
    #[must_use]
    pub fn albums(mut self, value: AlbumsUpdate) -> Self {
        self.albums = Some(value);
        self
    }

    ///Sets or clears `albums`.
    #[must_use]
    pub fn set_albums(mut self, value: Option<AlbumsUpdate>) -> Self {
        self.albums = value;
        self
    }

    ///Sets `avatar`.
    #[must_use]
    pub fn avatar(mut self, value: AvatarUpdate) -> Self {
        self.avatar = Some(value);
        self
    }

    ///Sets or clears `avatar`.
    #[must_use]
    pub fn set_avatar(mut self, value: Option<AvatarUpdate>) -> Self {
        self.avatar = value;
        self
    }

    ///Sets `cast`.
    #[must_use]
    pub fn cast(mut self, value: CastUpdate) -> Self {
        self.cast = Some(value);
        self
    }

    ///Sets or clears `cast`.
    #[must_use]
    pub fn set_cast(mut self, value: Option<CastUpdate>) -> Self {
        self.cast = value;
        self
    }

    ///Sets `download`.
    #[must_use]
    pub fn download(mut self, value: DownloadUpdate) -> Self {
        self.download = Some(value);
        self
    }

    ///Sets or clears `download`.
    #[must_use]
    pub fn set_download(mut self, value: Option<DownloadUpdate>) -> Self {
        self.download = value;
        self
    }

    ///Sets `email_notifications`.
    #[must_use]
    pub fn email_notifications(mut self, value: EmailNotificationsUpdate) -> Self {
        self.email_notifications = Some(value);
        self
    }

    ///Sets or clears `email_notifications`.
    #[must_use]
    pub fn set_email_notifications(mut self, value: Option<EmailNotificationsUpdate>) -> Self {
        self.email_notifications = value;
        self
    }

    ///Sets `folders`.
    #[must_use]
    pub fn folders(mut self, value: FoldersUpdate) -> Self {
        self.folders = Some(value);
        self
    }

    ///Sets or clears `folders`.
    #[must_use]
    pub fn set_folders(mut self, value: Option<FoldersUpdate>) -> Self {
        self.folders = value;
        self
    }

    ///Sets `memories`.
    #[must_use]
    pub fn memories(mut self, value: MemoriesUpdate) -> Self {
        self.memories = Some(value);
        self
    }

    ///Sets or clears `memories`.
    #[must_use]
    pub fn set_memories(mut self, value: Option<MemoriesUpdate>) -> Self {
        self.memories = value;
        self
    }

    ///Sets `people`.
    #[must_use]
    pub fn people(mut self, value: PeopleUpdate) -> Self {
        self.people = Some(value);
        self
    }

    ///Sets or clears `people`.
    #[must_use]
    pub fn set_people(mut self, value: Option<PeopleUpdate>) -> Self {
        self.people = value;
        self
    }

    ///Sets `purchase`.
    #[must_use]
    pub fn purchase(mut self, value: PurchaseUpdate) -> Self {
        self.purchase = Some(value);
        self
    }

    ///Sets or clears `purchase`.
    #[must_use]
    pub fn set_purchase(mut self, value: Option<PurchaseUpdate>) -> Self {
        self.purchase = value;
        self
    }

    ///Sets `ratings`.
    #[must_use]
    pub fn ratings(mut self, value: RatingsUpdate) -> Self {
        self.ratings = Some(value);
        self
    }

    ///Sets or clears `ratings`.
    #[must_use]
    pub fn set_ratings(mut self, value: Option<RatingsUpdate>) -> Self {
        self.ratings = value;
        self
    }

    ///Sets `recently_added`.
    #[must_use]
    pub fn recently_added(mut self, value: RecentlyAddedUpdate) -> Self {
        self.recently_added = Some(value);
        self
    }

    ///Sets or clears `recently_added`.
    #[must_use]
    pub fn set_recently_added(mut self, value: Option<RecentlyAddedUpdate>) -> Self {
        self.recently_added = value;
        self
    }

    ///Sets `shared_links`.
    #[must_use]
    pub fn shared_links(mut self, value: SharedLinksUpdate) -> Self {
        self.shared_links = Some(value);
        self
    }

    ///Sets or clears `shared_links`.
    #[must_use]
    pub fn set_shared_links(mut self, value: Option<SharedLinksUpdate>) -> Self {
        self.shared_links = value;
        self
    }

    ///Sets `tags`.
    #[must_use]
    pub fn tags(mut self, value: TagsUpdate) -> Self {
        self.tags = Some(value);
        self
    }

    ///Sets or clears `tags`.
    #[must_use]
    pub fn set_tags(mut self, value: Option<TagsUpdate>) -> Self {
        self.tags = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UserResponseEmail(pub String);

impl ::std::ops::Deref for UserResponseEmail {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for UserResponseEmail {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new(
                    "^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for UserResponseEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for UserResponseEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for UserResponseEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for UserResponseEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `UserUpdateMeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] |  |  |
/// | `email` | [`String`] |  | User email |
/// | `name` | [`String`] |  | User name |
/// | `password` | [`String`] |  | User password (deprecated, use change password endpoint) |
///
/// See: <https://api.immich.app/models/UserUpdateMeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct UserUpdateMeDTO {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: Option<UserUpdateMeEmail>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
}

impl UserUpdateMeDTO {
    ///Creates a new [`UserUpdateMeDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            avatar_color: None,
            email: None,
            name: None,
            password: None,
        }
    }

    ///Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }

    ///Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: UserUpdateMeEmail) -> Self {
        self.email = Some(value);
        self
    }

    ///Sets or clears `email`.
    #[must_use]
    pub fn set_email(mut self, value: Option<UserUpdateMeEmail>) -> Self {
        self.email = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }

    ///Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    ///Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UserUpdateMeEmail(pub String);

impl ::std::ops::Deref for UserUpdateMeEmail {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for UserUpdateMeEmail {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new(
                    "^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for UserUpdateMeEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for UserUpdateMeEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for UserUpdateMeEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for UserUpdateMeEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
