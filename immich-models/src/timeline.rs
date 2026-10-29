// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `timeline`.

pub use crate::assets::AssetVisibility;

/// Asset sorting property
///
/// See: <https://api.immich.app/models/AssetOrderBy>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetOrderBy {
    #[serde(rename = "takenAt")]
    TakenAt,
    #[serde(rename = "createdAt")]
    CreatedAt,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetOrderBy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TakenAt => f.write_str("takenAt"),
            Self::CreatedAt => f.write_str("createdAt"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetOrderBy {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "takenAt" => Ok(Self::TakenAt),
            "createdAt" => Ok(Self::CreatedAt),
            _ => Err(format!("invalid value for {}: {}", "AssetOrderBy", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetOrderBy {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetOrderBy {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetOrderBy {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CalendarHeatmapResponseSeriesItem {
    ///Activity count
    pub count: i64,
    ///Date in UTC
    pub date: String,
}

impl CalendarHeatmapResponseSeriesItem {
    ///Creates a new [`CalendarHeatmapResponseSeriesItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(count: i64, date: impl Into<String>) -> Self {
        Self {
            count,
            date: date.into(),
        }
    }

    ///Sets `count`.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = value;
        self
    }

    ///Sets `date`.
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
/// | `live_photo_video_id` | [`Vec<String>`](::std::vec::Vec) | ✓ | Array of live photo video asset IDs (null for non-live photos) |
/// | `local_offset_hours` | `Vec<f64>` | ✓ | Array of UTC offset hours at the time each photo was taken. Positive values are east of UTC, negative values are west of UTC. Values may be fractional (e.g., 5.5 for +05:30, -9.75 for -09:45). Applying this offset to 'fileCreatedAt' will give you the time the photo was taken from the photographer's perspective. |
/// | `longitude` | `Vec<f64>` |  | Array of longitude coordinates extracted from EXIF GPS data |
/// | `owner_id` | [`Vec<String>`](::std::vec::Vec) | ✓ | Array of owner IDs for each asset |
/// | `projection_type` | [`Vec<String>`](::std::vec::Vec) | ✓ | Array of projection types for 360° content (e.g., "EQUIRECTANGULAR", "CUBEFACE", "CYLINDRICAL") |
/// | `ratio` | `Vec<f64>` | ✓ | Array of aspect ratios (width/height) for each asset |
/// | `stack` | `Vec<Vec<String>>` |  | Array of stack information as [stackId, assetCount] tuples (null for non-stacked assets) |
/// | `thumbhash` | [`Vec<String>`](::std::vec::Vec) | ✓ | Array of `BlurHash` strings for generating asset previews (base64 encoded) |
/// | `visibility` | [`Vec<AssetVisibility>`](crate::assets::AssetVisibility) | ✓ | Array of visibility statuses for each asset (e.g., ARCHIVE, TIMELINE, HIDDEN, LOCKED) |
///
/// See: <https://api.immich.app/models/TimeBucketAssetResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TimeBucketAssetResponseDTO {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub city: Vec<Option<String>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub country: Vec<Option<String>>,
    /// Array of UTC timestamps when each asset was originally uploaded to Immich (required)
    #[serde(rename = "createdAt")]
    pub created_at: Vec<String>,
    pub duration: Vec<Option<i64>>,
    /// Array of file creation timestamps in UTC (required)
    #[serde(rename = "fileCreatedAt")]
    pub file_created_at: Vec<String>,
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
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub latitude: Vec<Option<f64>>,
    /// Array of live photo video asset IDs (null for non-live photos) (required)
    #[serde(rename = "livePhotoVideoId")]
    pub live_photo_video_id: Vec<Option<String>>,
    /// Array of UTC offset hours at the time each photo was taken. Positive values are east of UTC, negative values are west of UTC. Values may be fractional (e.g., 5.5 for +05:30, -9.75 for -09:45). Applying this offset to 'fileCreatedAt' will give you the time the photo was taken from the photographer's perspective. (required)
    #[serde(rename = "localOffsetHours")]
    pub local_offset_hours: Vec<f64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub longitude: Vec<Option<f64>>,
    /// Array of owner IDs for each asset (required)
    #[serde(rename = "ownerId")]
    pub owner_id: Vec<String>,
    /// Array of projection types for 360° content (e.g., "EQUIRECTANGULAR", "CUBEFACE", "CYLINDRICAL") (required)
    #[serde(rename = "projectionType")]
    pub projection_type: Vec<Option<String>>,
    pub ratio: Vec<f64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub stack: Vec<Option<[String; 2usize]>>,
    pub thumbhash: Vec<Option<String>>,
    pub visibility: Vec<AssetVisibility>,
}

impl TimeBucketAssetResponseDTO {
    ///Creates a new [`TimeBucketAssetResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: Vec<String>,
        duration: Vec<Option<i64>>,
        file_created_at: Vec<String>,
        id: Vec<String>,
        is_favorite: Vec<bool>,
        is_image: Vec<bool>,
        is_trashed: Vec<bool>,
        live_photo_video_id: Vec<Option<String>>,
        local_offset_hours: Vec<f64>,
        owner_id: Vec<String>,
        projection_type: Vec<Option<String>>,
        ratio: Vec<f64>,
        thumbhash: Vec<Option<String>>,
        visibility: Vec<AssetVisibility>,
    ) -> Self {
        Self {
            city: Vec::new(),
            country: Vec::new(),
            created_at,
            duration,
            file_created_at,
            id,
            is_favorite,
            is_image,
            is_trashed,
            latitude: Vec::new(),
            live_photo_video_id,
            local_offset_hours,
            longitude: Vec::new(),
            owner_id,
            projection_type,
            ratio,
            stack: Vec::new(),
            thumbhash,
            visibility,
        }
    }

    ///Sets `city`.
    #[must_use]
    pub fn city(mut self, value: Vec<Option<String>>) -> Self {
        self.city = value;
        self
    }

    ///Sets `country`.
    #[must_use]
    pub fn country(mut self, value: Vec<Option<String>>) -> Self {
        self.country = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: Vec<String>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: Vec<Option<i64>>) -> Self {
        self.duration = value;
        self
    }

    ///Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: Vec<String>) -> Self {
        self.file_created_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: Vec<String>) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: Vec<bool>) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `is_image`.
    #[must_use]
    pub fn is_image(mut self, value: Vec<bool>) -> Self {
        self.is_image = value;
        self
    }

    ///Sets `is_trashed`.
    #[must_use]
    pub fn is_trashed(mut self, value: Vec<bool>) -> Self {
        self.is_trashed = value;
        self
    }

    ///Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: Vec<Option<f64>>) -> Self {
        self.latitude = value;
        self
    }

    ///Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: Vec<Option<String>>) -> Self {
        self.live_photo_video_id = value;
        self
    }

    ///Sets `local_offset_hours`.
    #[must_use]
    pub fn local_offset_hours(mut self, value: Vec<f64>) -> Self {
        self.local_offset_hours = value;
        self
    }

    ///Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: Vec<Option<f64>>) -> Self {
        self.longitude = value;
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: Vec<String>) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `projection_type`.
    #[must_use]
    pub fn projection_type(mut self, value: Vec<Option<String>>) -> Self {
        self.projection_type = value;
        self
    }

    ///Sets `ratio`.
    #[must_use]
    pub fn ratio(mut self, value: Vec<f64>) -> Self {
        self.ratio = value;
        self
    }

    ///Sets `stack`.
    #[must_use]
    pub fn stack(mut self, value: Vec<Option<[String; 2usize]>>) -> Self {
        self.stack = value;
        self
    }

    ///Sets `thumbhash`.
    #[must_use]
    pub fn thumbhash(mut self, value: Vec<Option<String>>) -> Self {
        self.thumbhash = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: Vec<AssetVisibility>) -> Self {
        self.visibility = value;
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TimeBucketsResponseDTO {
    pub count: i64,
    /// Time bucket identifier in YYYY-MM-DD format representing the start of the time period (required)
    #[serde(rename = "timeBucket")]
    pub time_bucket: String,
}

impl TimeBucketsResponseDTO {
    ///Creates a new [`TimeBucketsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(count: i64, time_bucket: impl Into<String>) -> Self {
        Self {
            count,
            time_bucket: time_bucket.into(),
        }
    }

    ///Sets `count`.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = value;
        self
    }

    ///Sets `time_bucket`.
    #[must_use]
    pub fn time_bucket(mut self, value: impl Into<String>) -> Self {
        self.time_bucket = value.into();
        self
    }
}
