// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `timeline`.

pub use crate::assets::AssetVisibility;

/// Asset sorting property
///
/// See: <https://api.immich.app/models/AssetOrderBy>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetOrderBy {
    TakenAt,
    CreatedAt,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetOrderBy {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::TakenAt => "takenAt",
            Self::CreatedAt => "createdAt",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetOrderBy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetOrderBy {
    fn from(value: &str) -> Self {
        match value {
            "takenAt" => Self::TakenAt,
            "createdAt" => Self::CreatedAt,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetOrderBy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "takenAt" => Self::TakenAt,
            "createdAt" => Self::CreatedAt,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetOrderBy {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetOrderBy {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetOrderBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetOrderBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetOrderBy;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetOrderBy::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetOrderBy::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `CalendarHeatmapResponseDtoSeriesItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CalendarHeatmapResponseSeriesItem {
    /// Activity count
    pub count: i64,
    /// Date in UTC
    pub date: String,
}

impl CalendarHeatmapResponseSeriesItem {
    /// Creates a new [`CalendarHeatmapResponseSeriesItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(count: i64, date: impl Into<String>) -> Self {
        Self {
            count,
            date: date.into(),
        }
    }
    /// Sets `count`.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = value;
        self
    }
    /// Sets `date`.
    #[must_use]
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = value.into();
        self
    }
}

/// `TimeBucketAssetResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `city` | [`Vec<String>`](::std::vec::Vec) |  | Array of city names extracted from EXIF GPS data |
/// | `country` | [`Vec<String>`](::std::vec::Vec) |  | Array of country names extracted from EXIF GPS data |
/// | `created_at` | [`Vec<String>`](::std::vec::Vec) | ✓ | Array of UTC timestamps when each asset was originally uploaded to Immich |
/// | `duration` | `Vec<i64>` | ✓ | Array of video/gif durations in milliseconds (null for static images) |
/// | `file_created_at` | [`Vec<String>`](::std::vec::Vec) | ✓ | Array of file creation timestamps in UTC |
/// | `id` | [`Vec<String>`](::std::vec::Vec) | ✓ | Array of asset IDs in the time bucket |
/// | `is_favorite` | `Vec<bool>` | ✓ | Array indicating whether each asset is favorited |
/// | `is_image` | `Vec<bool>` | ✓ | Array indicating whether each asset is an image (false for videos) |
/// | `is_trashed` | `Vec<bool>` | ✓ | Array indicating whether each asset is in the trash |
/// | `latitude` | `Vec<f64>` |  | Array of latitude coordinates extracted from EXIF GPS data |
/// | `live_photo_video_id` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Array of live photo video asset IDs (null for non-live photos) |
/// | `local_offset_hours` | `Vec<f64>` | ✓ | Array of UTC offset hours at the time each photo was taken. Positive values are east of UTC, negative values are west of UTC. Values may be fractional (e.g., 5.5 for +05:30, -9.75 for -09:45). Applying this offset to '`fileCreatedAt`' will give you the time the photo was taken from the photographer's perspective. |
/// | `longitude` | `Vec<f64>` |  | Array of longitude coordinates extracted from EXIF GPS data |
/// | `owner_id` | [`Vec<UserId>`](crate::id::UserId) | ✓ | Array of owner IDs for each asset |
/// | `projection_type` | [`Vec<String>`](::std::vec::Vec) | ✓ | Array of projection types for 360° content (e.g., "EQUIRECTANGULAR", "CUBEFACE", "CYLINDRICAL") |
/// | `ratio` | `Vec<f64>` | ✓ | Array of aspect ratios (width/height) for each asset |
/// | `stack` | `Vec<Vec<String>>` |  | Array of stack information as \[stackId, assetCount\] tuples (null for non-stacked assets) |
/// | `thumbhash` | [`Vec<String>`](::std::vec::Vec) | ✓ | Array of `BlurHash` strings for generating asset previews (base64 encoded) |
/// | `visibility` | [`Vec<AssetVisibility>`](crate::assets::AssetVisibility) | ✓ | Array of visibility statuses for each asset (e.g., ARCHIVE, TIMELINE, HIDDEN, LOCKED) |
///
/// See: <https://api.immich.app/models/TimeBucketAssetResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct TimeBucketAssetResponseDTO {
    /// Array of city names extracted from EXIF GPS data
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub city: Vec<Option<String>>,
    /// Array of country names extracted from EXIF GPS data
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub country: Vec<Option<String>>,
    /// Array of UTC timestamps when each asset was originally uploaded to Immich (required)
    #[serde(rename = "createdAt")]
    pub created_at: Vec<String>,
    /// Array of video/gif durations in milliseconds (null for static images) (required)
    pub duration: Vec<Option<i64>>,
    /// Array of file creation timestamps in UTC (required)
    #[serde(rename = "fileCreatedAt")]
    pub file_created_at: Vec<String>,
    /// Array of asset IDs in the time bucket (required)
    pub id: Vec<String>,
    /// Array indicating whether each asset is favorited (required)
    #[serde(rename = "isFavorite")]
    pub is_favorite: Vec<bool>,
    /// Array indicating whether each asset is an image (false for videos) (required)
    #[serde(rename = "isImage")]
    pub is_image: Vec<bool>,
    /// Array indicating whether each asset is in the trash (required)
    #[serde(rename = "isTrashed")]
    pub is_trashed: Vec<bool>,
    /// Array of latitude coordinates extracted from EXIF GPS data
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub latitude: Vec<Option<f64>>,
    /// Array of live photo video asset IDs (null for non-live photos) (required)
    #[serde(rename = "livePhotoVideoId")]
    pub live_photo_video_id: Vec<Option<String>>,
    /// Array of UTC offset hours at the time each photo was taken. Positive values are east of UTC, negative values are west of UTC. Values may be fractional (e.g., 5.5 for +05:30, -9.75 for -09:45). Applying this offset to '`fileCreatedAt`' will give you the time the photo was taken from the photographer's perspective. (required)
    #[serde(rename = "localOffsetHours")]
    pub local_offset_hours: Vec<f64>,
    /// Array of longitude coordinates extracted from EXIF GPS data
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub longitude: Vec<Option<f64>>,
    /// Array of owner IDs for each asset (required)
    #[serde(rename = "ownerId")]
    pub owner_id: Vec<String>,
    /// Array of projection types for 360° content (e.g., "EQUIRECTANGULAR", "CUBEFACE", "CYLINDRICAL") (required)
    #[serde(rename = "projectionType")]
    pub projection_type: Vec<Option<String>>,
    /// Array of aspect ratios (width/height) for each asset (required)
    pub ratio: Vec<f64>,
    /// Array of stack information as \[stackId, assetCount\] tuples (null for non-stacked assets)
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub stack: Vec<Option<[String; 2usize]>>,
    /// Array of `BlurHash` strings for generating asset previews (base64 encoded) (required)
    pub thumbhash: Vec<Option<String>>,
    /// Array of visibility statuses for each asset (e.g., ARCHIVE, TIMELINE, HIDDEN, LOCKED) (required)
    pub visibility: Vec<AssetVisibility>,
}

impl TimeBucketAssetResponseDTO {
    /// Creates a new [`TimeBucketAssetResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: impl IntoIterator<Item = impl Into<String>>,
        duration: impl IntoIterator<Item = Option<i64>>,
        file_created_at: impl IntoIterator<Item = impl Into<String>>,
        id: impl IntoIterator<Item = impl Into<String>>,
        is_favorite: impl IntoIterator<Item = bool>,
        is_image: impl IntoIterator<Item = bool>,
        is_trashed: impl IntoIterator<Item = bool>,
        live_photo_video_id: impl IntoIterator<Item = Option<String>>,
        local_offset_hours: impl IntoIterator<Item = f64>,
        owner_id: impl IntoIterator<Item = impl Into<String>>,
        projection_type: impl IntoIterator<Item = Option<String>>,
        ratio: impl IntoIterator<Item = f64>,
        thumbhash: impl IntoIterator<Item = Option<String>>,
        visibility: impl IntoIterator<Item = AssetVisibility>,
    ) -> Self {
        Self {
            city: Vec::new(),
            country: Vec::new(),
            created_at: created_at.into_iter().map(Into::into).collect(),
            duration: duration.into_iter().collect(),
            file_created_at: file_created_at.into_iter().map(Into::into).collect(),
            id: id.into_iter().map(Into::into).collect(),
            is_favorite: is_favorite.into_iter().collect(),
            is_image: is_image.into_iter().collect(),
            is_trashed: is_trashed.into_iter().collect(),
            latitude: Vec::new(),
            live_photo_video_id: live_photo_video_id.into_iter().collect(),
            local_offset_hours: local_offset_hours.into_iter().collect(),
            longitude: Vec::new(),
            owner_id: owner_id.into_iter().map(Into::into).collect(),
            projection_type: projection_type.into_iter().collect(),
            ratio: ratio.into_iter().collect(),
            stack: Vec::new(),
            thumbhash: thumbhash.into_iter().collect(),
            visibility: visibility.into_iter().collect(),
        }
    }
    /// Sets `city`.
    #[must_use]
    pub fn city(mut self, value: impl IntoIterator<Item = Option<String>>) -> Self {
        self.city = value.into_iter().collect();
        self
    }
    /// Sets `country`.
    #[must_use]
    pub fn country(mut self, value: impl IntoIterator<Item = Option<String>>) -> Self {
        self.country = value.into_iter().collect();
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.created_at = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: impl IntoIterator<Item = Option<i64>>) -> Self {
        self.duration = value.into_iter().collect();
        self
    }
    /// Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.file_created_at = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.id = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: impl IntoIterator<Item = bool>) -> Self {
        self.is_favorite = value.into_iter().collect();
        self
    }
    /// Sets `is_image`.
    #[must_use]
    pub fn is_image(mut self, value: impl IntoIterator<Item = bool>) -> Self {
        self.is_image = value.into_iter().collect();
        self
    }
    /// Sets `is_trashed`.
    #[must_use]
    pub fn is_trashed(mut self, value: impl IntoIterator<Item = bool>) -> Self {
        self.is_trashed = value.into_iter().collect();
        self
    }
    /// Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: impl IntoIterator<Item = Option<f64>>) -> Self {
        self.latitude = value.into_iter().collect();
        self
    }
    /// Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: impl IntoIterator<Item = Option<String>>) -> Self {
        self.live_photo_video_id = value.into_iter().collect();
        self
    }
    /// Sets `local_offset_hours`.
    #[must_use]
    pub fn local_offset_hours(mut self, value: impl IntoIterator<Item = f64>) -> Self {
        self.local_offset_hours = value.into_iter().collect();
        self
    }
    /// Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: impl IntoIterator<Item = Option<f64>>) -> Self {
        self.longitude = value.into_iter().collect();
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.owner_id = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `projection_type`.
    #[must_use]
    pub fn projection_type(mut self, value: impl IntoIterator<Item = Option<String>>) -> Self {
        self.projection_type = value.into_iter().collect();
        self
    }
    /// Sets `ratio`.
    #[must_use]
    pub fn ratio(mut self, value: impl IntoIterator<Item = f64>) -> Self {
        self.ratio = value.into_iter().collect();
        self
    }
    /// Sets `stack`.
    #[must_use]
    pub fn stack(mut self, value: impl IntoIterator<Item = Option<[String; 2usize]>>) -> Self {
        self.stack = value.into_iter().collect();
        self
    }
    /// Sets `thumbhash`.
    #[must_use]
    pub fn thumbhash(mut self, value: impl IntoIterator<Item = Option<String>>) -> Self {
        self.thumbhash = value.into_iter().collect();
        self
    }
    /// Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: impl IntoIterator<Item = AssetVisibility>) -> Self {
        self.visibility = value.into_iter().collect();
        self
    }
}

/// `TimeBucketsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `count` | `i64` | ✓ | Number of assets in this time bucket |
/// | `time_bucket` | [`String`] | ✓ | Time bucket identifier in YYYY-MM-DD format representing the start of the time period |
///
/// See: <https://api.immich.app/models/TimeBucketsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TimeBucketsResponseDTO {
    /// Number of assets in this time bucket (required)
    pub count: i64,
    /// Time bucket identifier in YYYY-MM-DD format representing the start of the time period (required)
    #[serde(rename = "timeBucket")]
    pub time_bucket: String,
}

impl TimeBucketsResponseDTO {
    /// Creates a new [`TimeBucketsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(count: i64, time_bucket: impl Into<String>) -> Self {
        Self {
            count,
            time_bucket: time_bucket.into(),
        }
    }
    /// Sets `count`.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = value;
        self
    }
    /// Sets `time_bucket`.
    #[must_use]
    pub fn time_bucket(mut self, value: impl Into<String>) -> Self {
        self.time_bucket = value.into();
        self
    }
}
