// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `users`.

pub mod admin;

pub use crate::activities::UserAvatarColor;
pub use crate::albums::AssetOrder;
pub use crate::id::UserId;
pub use crate::timeline::CalendarHeatmapResponseSeriesItem;

/// `AlbumsResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `default_asset_order` | [`AssetOrder`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AlbumsResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AlbumsResponse {
    /// (required)
    #[serde(rename = "defaultAssetOrder")]
    pub default_asset_order: AssetOrder,
}

impl AlbumsResponse {
    /// Creates a new [`AlbumsResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(default_asset_order: AssetOrder) -> Self {
        Self { default_asset_order }
    }
    /// Sets `default_asset_order`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct AlbumsUpdate {
    #[serde(
        rename = "defaultAssetOrder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_asset_order: Option<AssetOrder>,
}

impl AlbumsUpdate {
    /// Creates a new [`AlbumsUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            default_asset_order: None,
        }
    }
    /// Sets `default_asset_order`.
    #[must_use]
    pub fn default_asset_order(mut self, value: AssetOrder) -> Self {
        self.default_asset_order = Some(value);
        self
    }
    /// Sets or clears `default_asset_order`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct AvatarUpdate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<UserAvatarColor>,
}

impl AvatarUpdate {
    /// Creates a new [`AvatarUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { color: None }
    }
    /// Sets `color`.
    #[must_use]
    pub fn color(mut self, value: UserAvatarColor) -> Self {
        self.color = Some(value);
        self
    }
    /// Sets or clears `color`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CalendarHeatmapResponseDTO {
    /// Start date in UTC (required)
    pub from: String,
    /// (required)
    pub series: Vec<CalendarHeatmapResponseSeriesItem>,
    /// End date in UTC (required)
    pub to: String,
    /// Total activity count over the period (required)
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}

impl CalendarHeatmapResponseDTO {
    /// Creates a new [`CalendarHeatmapResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        from: impl Into<String>,
        series: impl IntoIterator<Item = CalendarHeatmapResponseSeriesItem>,
        to: impl Into<String>,
        total_count: i64,
    ) -> Self {
        Self {
            from: from.into(),
            series: series.into_iter().collect(),
            to: to.into(),
            total_count,
        }
    }
    /// Sets `from`.
    #[must_use]
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = value.into();
        self
    }
    /// Sets `series`.
    #[must_use]
    pub fn series(mut self, value: impl IntoIterator<Item = CalendarHeatmapResponseSeriesItem>) -> Self {
        self.series = value.into_iter().collect();
        self
    }
    /// Sets `to`.
    #[must_use]
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = value.into();
        self
    }
    /// Sets `total_count`.
    #[must_use]
    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = value;
        self
    }
}

/// Type of calendar heatmap
///
/// See: <https://api.immich.app/models/CalendarHeatmapType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum CalendarHeatmapType {
    Upload,
    Taken,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl CalendarHeatmapType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Upload => "Upload",
            Self::Taken => "Taken",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for CalendarHeatmapType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for CalendarHeatmapType {
    fn from(value: &str) -> Self {
        match value {
            "Upload" => Self::Upload,
            "Taken" => Self::Taken,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for CalendarHeatmapType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Upload" => Self::Upload,
            "Taken" => Self::Taken,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for CalendarHeatmapType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for CalendarHeatmapType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for CalendarHeatmapType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for CalendarHeatmapType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = CalendarHeatmapType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(CalendarHeatmapType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(CalendarHeatmapType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `CastResponse`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `g_cast_enabled` | `bool` | ✓ | Whether Google Cast is enabled |
///
/// See: <https://api.immich.app/models/CastResponse>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CastResponse {
    /// Whether Google Cast is enabled (required)
    #[serde(rename = "gCastEnabled")]
    pub g_cast_enabled: bool,
}

impl CastResponse {
    /// Creates a new [`CastResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(g_cast_enabled: bool) -> Self {
        Self { g_cast_enabled }
    }
    /// Sets `g_cast_enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Creates a new [`CastUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { g_cast_enabled: None }
    }
    /// Sets `g_cast_enabled`.
    #[must_use]
    pub fn g_cast_enabled(mut self, value: bool) -> Self {
        self.g_cast_enabled = Some(value);
        self
    }
    /// Sets or clears `g_cast_enabled`.
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
/// | `file` | `Vec<u8>` | ✓ | Profile image file |
///
/// See: <https://api.immich.app/models/CreateProfileImageDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CreateProfileImageDTO {
    /// Profile image file (required)
    pub file: String,
}

impl CreateProfileImageDTO {
    /// Creates a new [`CreateProfileImageDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(file: impl Into<String>) -> Self {
        Self { file: file.into() }
    }
    /// Sets `file`.
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
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile image change date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `profile_image_path` | [`String`] | ✓ | Profile image file path |
/// | `user_id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/CreateProfileImageResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CreateProfileImageResponseDTO {
    /// Profile image change date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Profile image file path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "userId")]
    pub user_id: UserId,
}

impl CreateProfileImageResponseDTO {
    /// Creates a new [`CreateProfileImageResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        profile_image_path: impl Into<String>,
        user_id: impl Into<UserId>,
    ) -> Self {
        Self {
            profile_changed_at,
            profile_image_path: profile_image_path.into(),
            user_id: user_id.into(),
        }
    }
    /// Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }
    /// Sets `profile_image_path`.
    #[must_use]
    pub fn profile_image_path(mut self, value: impl Into<String>) -> Self {
        self.profile_image_path = value.into();
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = value.into();
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DownloadResponse {
    /// Maximum archive size in bytes (required)
    #[serde(rename = "archiveSize")]
    pub archive_size: i64,
    /// Whether to include embedded videos in downloads (required)
    #[serde(rename = "includeEmbeddedVideos")]
    pub include_embedded_videos: bool,
}

impl DownloadResponse {
    /// Creates a new [`DownloadResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(archive_size: i64, include_embedded_videos: bool) -> Self {
        Self {
            archive_size,
            include_embedded_videos,
        }
    }
    /// Sets `archive_size`.
    #[must_use]
    pub fn archive_size(mut self, value: i64) -> Self {
        self.archive_size = value;
        self
    }
    /// Sets `include_embedded_videos`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Creates a new [`DownloadUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            archive_size: None,
            include_embedded_videos: None,
        }
    }
    /// Sets `archive_size`.
    #[must_use]
    pub fn archive_size(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.archive_size = Some(value);
        self
    }
    /// Sets or clears `archive_size`.
    #[must_use]
    pub fn set_archive_size(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.archive_size = value;
        self
    }
    /// Sets `include_embedded_videos`.
    #[must_use]
    pub fn include_embedded_videos(mut self, value: bool) -> Self {
        self.include_embedded_videos = Some(value);
        self
    }
    /// Sets or clears `include_embedded_videos`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct EmailNotificationsResponse {
    /// Whether to receive email notifications for album invites (required)
    #[serde(rename = "albumInvite")]
    pub album_invite: bool,
    /// Whether to receive email notifications for album updates (required)
    #[serde(rename = "albumUpdate")]
    pub album_update: bool,
    /// Whether email notifications are enabled (required)
    pub enabled: bool,
}

impl EmailNotificationsResponse {
    /// Creates a new [`EmailNotificationsResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(album_invite: bool, album_update: bool, enabled: bool) -> Self {
        Self {
            album_invite,
            album_update,
            enabled,
        }
    }
    /// Sets `album_invite`.
    #[must_use]
    pub fn album_invite(mut self, value: bool) -> Self {
        self.album_invite = value;
        self
    }
    /// Sets `album_update`.
    #[must_use]
    pub fn album_update(mut self, value: bool) -> Self {
        self.album_update = value;
        self
    }
    /// Sets `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Whether email notifications are enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
}

impl EmailNotificationsUpdate {
    /// Creates a new [`EmailNotificationsUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_invite: None,
            album_update: None,
            enabled: None,
        }
    }
    /// Sets `album_invite`.
    #[must_use]
    pub fn album_invite(mut self, value: bool) -> Self {
        self.album_invite = Some(value);
        self
    }
    /// Sets or clears `album_invite`.
    #[must_use]
    pub fn set_album_invite(mut self, value: Option<bool>) -> Self {
        self.album_invite = value;
        self
    }
    /// Sets `album_update`.
    #[must_use]
    pub fn album_update(mut self, value: bool) -> Self {
        self.album_update = Some(value);
        self
    }
    /// Sets or clears `album_update`.
    #[must_use]
    pub fn set_album_update(mut self, value: Option<bool>) -> Self {
        self.album_update = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Sets or clears `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct FoldersResponse {
    /// Whether folders are enabled (required)
    pub enabled: bool,
    /// Whether folders appear in web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl FoldersResponse {
    /// Creates a new [`FoldersResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, sidebar_web: bool) -> Self {
        Self { enabled, sidebar_web }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `sidebar_web`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct FoldersUpdate {
    /// Whether folders are enabled
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
    /// Creates a new [`FoldersUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            enabled: None,
            sidebar_web: None,
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }
    /// Sets or clears `sidebar_web`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MemoriesResponse {
    /// Memory duration in seconds (required)
    pub duration: i64,
    /// Whether memories are enabled (required)
    pub enabled: bool,
    /// Whether memories appear in web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl MemoriesResponse {
    /// Creates a new [`MemoriesResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(duration: i64, enabled: bool, sidebar_web: bool) -> Self {
        Self {
            duration,
            enabled,
            sidebar_web,
        }
    }
    /// Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `sidebar_web`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct MemoriesUpdate {
    /// Memory duration in seconds
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: Option<::std::num::NonZeroU64>,
    /// Whether memories are enabled
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
    /// Creates a new [`MemoriesUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            duration: None,
            enabled: None,
            sidebar_web: None,
        }
    }
    /// Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.duration = Some(value);
        self
    }
    /// Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.duration = value;
        self
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }
    /// Sets or clears `sidebar_web`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OnboardingDTO {
    /// Is user onboarded (required)
    #[serde(rename = "isOnboarded")]
    pub is_onboarded: bool,
}

impl OnboardingDTO {
    /// Creates a new [`OnboardingDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_onboarded: bool) -> Self {
        Self { is_onboarded }
    }
    /// Sets `is_onboarded`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OnboardingResponseDTO {
    /// Is user onboarded (required)
    #[serde(rename = "isOnboarded")]
    pub is_onboarded: bool,
}

impl OnboardingResponseDTO {
    /// Creates a new [`OnboardingResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_onboarded: bool) -> Self {
        Self { is_onboarded }
    }
    /// Sets `is_onboarded`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PeopleResponse {
    /// Whether people are enabled (required)
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
    /// Creates a new [`PeopleResponse`].
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
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `minimum_faces`.
    #[must_use]
    pub fn minimum_faces(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.minimum_faces = Some(value);
        self
    }
    /// Sets or clears `minimum_faces`.
    #[must_use]
    pub fn set_minimum_faces(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.minimum_faces = value;
        self
    }
    /// Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = value;
        self
    }
    /// Sets `update_strategy`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct PeopleUpdate {
    /// Whether people are enabled
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
    /// Creates a new [`PeopleUpdate`].
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
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `minimum_faces`.
    #[must_use]
    pub fn minimum_faces(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.minimum_faces = Some(value);
        self
    }
    /// Sets or clears `minimum_faces`.
    #[must_use]
    pub fn set_minimum_faces(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.minimum_faces = value;
        self
    }
    /// Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }
    /// Sets or clears `sidebar_web`.
    #[must_use]
    pub fn set_sidebar_web(mut self, value: Option<bool>) -> Self {
        self.sidebar_web = value;
        self
    }
    /// Sets `update_strategy`.
    #[must_use]
    pub fn update_strategy(mut self, value: PersonUpdateStrategy) -> Self {
        self.update_strategy = Some(value);
        self
    }
    /// Sets or clears `update_strategy`.
    #[must_use]
    pub fn set_update_strategy(mut self, value: Option<PersonUpdateStrategy>) -> Self {
        self.update_strategy = value;
        self
    }
}

/// Which person records to update when editing a person
///
/// See: <https://api.immich.app/models/PersonUpdateStrategy>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PersonUpdateStrategy {
    Self_,
    Everyone,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl PersonUpdateStrategy {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Self_ => "self",
            Self::Everyone => "everyone",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for PersonUpdateStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for PersonUpdateStrategy {
    fn from(value: &str) -> Self {
        match value {
            "self" => Self::Self_,
            "everyone" => Self::Everyone,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for PersonUpdateStrategy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "self" => Self::Self_,
            "everyone" => Self::Everyone,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for PersonUpdateStrategy {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for PersonUpdateStrategy {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for PersonUpdateStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for PersonUpdateStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = PersonUpdateStrategy;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(PersonUpdateStrategy::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(PersonUpdateStrategy::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PurchaseResponse {
    /// Date until which to hide buy button (required)
    #[serde(rename = "hideBuyButtonUntil")]
    pub hide_buy_button_until: String,
    /// Whether to show support badge (required)
    #[serde(rename = "showSupportBadge")]
    pub show_support_badge: bool,
}

impl PurchaseResponse {
    /// Creates a new [`PurchaseResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(hide_buy_button_until: impl Into<String>, show_support_badge: bool) -> Self {
        Self {
            hide_buy_button_until: hide_buy_button_until.into(),
            show_support_badge,
        }
    }
    /// Sets `hide_buy_button_until`.
    #[must_use]
    pub fn hide_buy_button_until(mut self, value: impl Into<String>) -> Self {
        self.hide_buy_button_until = value.into();
        self
    }
    /// Sets `show_support_badge`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Creates a new [`PurchaseUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            hide_buy_button_until: None,
            show_support_badge: None,
        }
    }
    /// Sets `hide_buy_button_until`.
    #[must_use]
    pub fn hide_buy_button_until(mut self, value: impl Into<String>) -> Self {
        self.hide_buy_button_until = Some(value.into());
        self
    }
    /// Sets or clears `hide_buy_button_until`.
    #[must_use]
    pub fn set_hide_buy_button_until(mut self, value: Option<String>) -> Self {
        self.hide_buy_button_until = value;
        self
    }
    /// Sets `show_support_badge`.
    #[must_use]
    pub fn show_support_badge(mut self, value: bool) -> Self {
        self.show_support_badge = Some(value);
        self
    }
    /// Sets or clears `show_support_badge`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct RatingsResponse {
    /// Whether ratings are enabled (required)
    pub enabled: bool,
}

impl RatingsResponse {
    /// Creates a new [`RatingsResponse`].
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

/// `RatingsUpdate`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `enabled` | `bool` |  | Whether ratings are enabled |
///
/// See: <https://api.immich.app/models/RatingsUpdate>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct RatingsUpdate {
    /// Whether ratings are enabled
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: Option<bool>,
}

impl RatingsUpdate {
    /// Creates a new [`RatingsUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { enabled: None }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Sets or clears `enabled`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct RecentlyAddedResponse {
    /// Whether the recently added page appears in the web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl RecentlyAddedResponse {
    /// Creates a new [`RecentlyAddedResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(sidebar_web: bool) -> Self {
        Self { sidebar_web }
    }
    /// Sets `sidebar_web`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Creates a new [`RecentlyAddedUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { sidebar_web: None }
    }
    /// Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }
    /// Sets or clears `sidebar_web`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct SharedLinksResponse {
    /// Whether shared links are enabled (required)
    pub enabled: bool,
    /// Whether shared links appear in web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl SharedLinksResponse {
    /// Creates a new [`SharedLinksResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, sidebar_web: bool) -> Self {
        Self { enabled, sidebar_web }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `sidebar_web`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct SharedLinksUpdate {
    /// Whether shared links are enabled
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
    /// Creates a new [`SharedLinksUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            enabled: None,
            sidebar_web: None,
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }
    /// Sets or clears `sidebar_web`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TagsResponse {
    /// Whether tags are enabled (required)
    pub enabled: bool,
    /// Whether tags appear in web sidebar (required)
    #[serde(rename = "sidebarWeb")]
    pub sidebar_web: bool,
}

impl TagsResponse {
    /// Creates a new [`TagsResponse`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(enabled: bool, sidebar_web: bool) -> Self {
        Self { enabled, sidebar_web }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `sidebar_web`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct TagsUpdate {
    /// Whether tags are enabled
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
    /// Creates a new [`TagsUpdate`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            enabled: None,
            sidebar_web: None,
        }
    }
    /// Sets `enabled`.
    #[must_use]
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    /// Sets or clears `enabled`.
    #[must_use]
    pub fn set_enabled(mut self, value: Option<bool>) -> Self {
        self.enabled = value;
        self
    }
    /// Sets `sidebar_web`.
    #[must_use]
    pub fn sidebar_web(mut self, value: bool) -> Self {
        self.sidebar_web = Some(value);
        self
    }
    /// Sets or clears `sidebar_web`.
    #[must_use]
    pub fn set_sidebar_web(mut self, value: Option<bool>) -> Self {
        self.sidebar_web = value;
        self
    }
}

/// User metadata key
///
/// See: <https://api.immich.app/models/UserMetadataKey>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum UserMetadataKey {
    Preferences,
    License,
    Onboarding,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl UserMetadataKey {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Preferences => "preferences",
            Self::License => "license",
            Self::Onboarding => "onboarding",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for UserMetadataKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for UserMetadataKey {
    fn from(value: &str) -> Self {
        match value {
            "preferences" => Self::Preferences,
            "license" => Self::License,
            "onboarding" => Self::Onboarding,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for UserMetadataKey {
    fn from(value: String) -> Self {
        match value.as_str() {
            "preferences" => Self::Preferences,
            "license" => Self::License,
            "onboarding" => Self::Onboarding,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for UserMetadataKey {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for UserMetadataKey {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for UserMetadataKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for UserMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = UserMetadataKey;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(UserMetadataKey::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(UserMetadataKey::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct UserPreferencesResponseDTO {
    /// (required)
    pub albums: AlbumsResponse,
    /// (required)
    pub cast: CastResponse,
    /// (required)
    pub download: DownloadResponse,
    /// (required)
    #[serde(rename = "emailNotifications")]
    pub email_notifications: EmailNotificationsResponse,
    /// (required)
    pub folders: FoldersResponse,
    /// (required)
    pub memories: MemoriesResponse,
    /// (required)
    pub people: PeopleResponse,
    /// (required)
    pub purchase: PurchaseResponse,
    /// (required)
    pub ratings: RatingsResponse,
    /// (required)
    #[serde(rename = "recentlyAdded")]
    pub recently_added: RecentlyAddedResponse,
    /// (required)
    #[serde(rename = "sharedLinks")]
    pub shared_links: SharedLinksResponse,
    /// (required)
    pub tags: TagsResponse,
}

impl UserPreferencesResponseDTO {
    /// Creates a new [`UserPreferencesResponseDTO`].
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
    /// Sets `albums`.
    #[must_use]
    pub fn albums(mut self, value: AlbumsResponse) -> Self {
        self.albums = value;
        self
    }
    /// Sets `cast`.
    #[must_use]
    pub fn cast(mut self, value: CastResponse) -> Self {
        self.cast = value;
        self
    }
    /// Sets `download`.
    #[must_use]
    pub fn download(mut self, value: DownloadResponse) -> Self {
        self.download = value;
        self
    }
    /// Sets `email_notifications`.
    #[must_use]
    pub fn email_notifications(mut self, value: EmailNotificationsResponse) -> Self {
        self.email_notifications = value;
        self
    }
    /// Sets `folders`.
    #[must_use]
    pub fn folders(mut self, value: FoldersResponse) -> Self {
        self.folders = value;
        self
    }
    /// Sets `memories`.
    #[must_use]
    pub fn memories(mut self, value: MemoriesResponse) -> Self {
        self.memories = value;
        self
    }
    /// Sets `people`.
    #[must_use]
    pub fn people(mut self, value: PeopleResponse) -> Self {
        self.people = value;
        self
    }
    /// Sets `purchase`.
    #[must_use]
    pub fn purchase(mut self, value: PurchaseResponse) -> Self {
        self.purchase = value;
        self
    }
    /// Sets `ratings`.
    #[must_use]
    pub fn ratings(mut self, value: RatingsResponse) -> Self {
        self.ratings = value;
        self
    }
    /// Sets `recently_added`.
    #[must_use]
    pub fn recently_added(mut self, value: RecentlyAddedResponse) -> Self {
        self.recently_added = value;
        self
    }
    /// Sets `shared_links`.
    #[must_use]
    pub fn shared_links(mut self, value: SharedLinksResponse) -> Self {
        self.shared_links = value;
        self
    }
    /// Sets `tags`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Creates a new [`UserPreferencesUpdateDTO`].
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
    /// Sets `albums`.
    #[must_use]
    pub fn albums(mut self, value: AlbumsUpdate) -> Self {
        self.albums = Some(value);
        self
    }
    /// Sets or clears `albums`.
    #[must_use]
    pub fn set_albums(mut self, value: Option<AlbumsUpdate>) -> Self {
        self.albums = value;
        self
    }
    /// Sets `avatar`.
    #[must_use]
    pub fn avatar(mut self, value: AvatarUpdate) -> Self {
        self.avatar = Some(value);
        self
    }
    /// Sets or clears `avatar`.
    #[must_use]
    pub fn set_avatar(mut self, value: Option<AvatarUpdate>) -> Self {
        self.avatar = value;
        self
    }
    /// Sets `cast`.
    #[must_use]
    pub fn cast(mut self, value: CastUpdate) -> Self {
        self.cast = Some(value);
        self
    }
    /// Sets or clears `cast`.
    #[must_use]
    pub fn set_cast(mut self, value: Option<CastUpdate>) -> Self {
        self.cast = value;
        self
    }
    /// Sets `download`.
    #[must_use]
    pub fn download(mut self, value: DownloadUpdate) -> Self {
        self.download = Some(value);
        self
    }
    /// Sets or clears `download`.
    #[must_use]
    pub fn set_download(mut self, value: Option<DownloadUpdate>) -> Self {
        self.download = value;
        self
    }
    /// Sets `email_notifications`.
    #[must_use]
    pub fn email_notifications(mut self, value: EmailNotificationsUpdate) -> Self {
        self.email_notifications = Some(value);
        self
    }
    /// Sets or clears `email_notifications`.
    #[must_use]
    pub fn set_email_notifications(mut self, value: Option<EmailNotificationsUpdate>) -> Self {
        self.email_notifications = value;
        self
    }
    /// Sets `folders`.
    #[must_use]
    pub fn folders(mut self, value: FoldersUpdate) -> Self {
        self.folders = Some(value);
        self
    }
    /// Sets or clears `folders`.
    #[must_use]
    pub fn set_folders(mut self, value: Option<FoldersUpdate>) -> Self {
        self.folders = value;
        self
    }
    /// Sets `memories`.
    #[must_use]
    pub fn memories(mut self, value: MemoriesUpdate) -> Self {
        self.memories = Some(value);
        self
    }
    /// Sets or clears `memories`.
    #[must_use]
    pub fn set_memories(mut self, value: Option<MemoriesUpdate>) -> Self {
        self.memories = value;
        self
    }
    /// Sets `people`.
    #[must_use]
    pub fn people(mut self, value: PeopleUpdate) -> Self {
        self.people = Some(value);
        self
    }
    /// Sets or clears `people`.
    #[must_use]
    pub fn set_people(mut self, value: Option<PeopleUpdate>) -> Self {
        self.people = value;
        self
    }
    /// Sets `purchase`.
    #[must_use]
    pub fn purchase(mut self, value: PurchaseUpdate) -> Self {
        self.purchase = Some(value);
        self
    }
    /// Sets or clears `purchase`.
    #[must_use]
    pub fn set_purchase(mut self, value: Option<PurchaseUpdate>) -> Self {
        self.purchase = value;
        self
    }
    /// Sets `ratings`.
    #[must_use]
    pub fn ratings(mut self, value: RatingsUpdate) -> Self {
        self.ratings = Some(value);
        self
    }
    /// Sets or clears `ratings`.
    #[must_use]
    pub fn set_ratings(mut self, value: Option<RatingsUpdate>) -> Self {
        self.ratings = value;
        self
    }
    /// Sets `recently_added`.
    #[must_use]
    pub fn recently_added(mut self, value: RecentlyAddedUpdate) -> Self {
        self.recently_added = Some(value);
        self
    }
    /// Sets or clears `recently_added`.
    #[must_use]
    pub fn set_recently_added(mut self, value: Option<RecentlyAddedUpdate>) -> Self {
        self.recently_added = value;
        self
    }
    /// Sets `shared_links`.
    #[must_use]
    pub fn shared_links(mut self, value: SharedLinksUpdate) -> Self {
        self.shared_links = Some(value);
        self
    }
    /// Sets or clears `shared_links`.
    #[must_use]
    pub fn set_shared_links(mut self, value: Option<SharedLinksUpdate>) -> Self {
        self.shared_links = value;
        self
    }
    /// Sets `tags`.
    #[must_use]
    pub fn tags(mut self, value: TagsUpdate) -> Self {
        self.tags = Some(value);
        self
    }
    /// Sets or clears `tags`.
    #[must_use]
    pub fn set_tags(mut self, value: Option<TagsUpdate>) -> Self {
        self.tags = value;
        self
    }
}

/// `UserUpdateMeDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] |  |  |
/// | `email` | [`String`] |  | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `name` | [`String`] |  | User name |
/// | `password` | [`String`] |  | User password (deprecated, use change password endpoint) |
///
/// See: <https://api.immich.app/models/UserUpdateMeDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct UserUpdateMeDTO {
    #[serde(
        rename = "avatarColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub avatar_color: Option<UserAvatarColor>,
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: Option<String>,
    /// User name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// User password (deprecated, use change password endpoint)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,
}

impl UserUpdateMeDTO {
    /// Creates a new [`UserUpdateMeDTO`].
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
    /// Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = Some(value);
        self
    }
    /// Sets or clears `avatar_color`.
    #[must_use]
    pub fn set_avatar_color(mut self, value: Option<UserAvatarColor>) -> Self {
        self.avatar_color = value;
        self
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }
    /// Sets or clears `email`.
    #[must_use]
    pub fn set_email(mut self, value: Option<String>) -> Self {
        self.email = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    /// Sets `password`.
    #[must_use]
    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }
    /// Sets or clears `password`.
    #[must_use]
    pub fn set_password(mut self, value: Option<String>) -> Self {
        self.password = value;
        self
    }
}
