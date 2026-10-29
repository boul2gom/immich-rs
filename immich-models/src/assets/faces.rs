// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `faces`.

pub use crate::assets::PersonResponseDTO;

/// `AssetFaceCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `height` | `i64` | ✓ | Face bounding box height |
/// | `image_height` | `i64` | ✓ | Image height in pixels |
/// | `image_width` | `i64` | ✓ | Image width in pixels |
/// | `person_id` | [`Uuid`](::uuid::Uuid) | ✓ | Person ID |
/// | `user_id` | [`Uuid`](::uuid::Uuid) |  | User ID |
/// | `width` | `i64` | ✓ | Face bounding box width |
/// | `x` | `i64` | ✓ | Face bounding box X coordinate |
/// | `y` | `i64` | ✓ | Face bounding box Y coordinate |
///
/// See: <https://api.immich.app/models/AssetFaceCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetFaceCreateDTO {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    pub height: i64,
    /// Image height in pixels (required)
    #[serde(rename = "imageHeight")]
    pub image_height: i64,
    /// Image width in pixels (required)
    #[serde(rename = "imageWidth")]
    pub image_width: i64,
    /// Person ID (required)
    #[serde(rename = "personId")]
    pub person_id: ::uuid::Uuid,
    /// User ID
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<::uuid::Uuid>,
    pub width: i64,
    pub x: i64,
    pub y: i64,
}

impl AssetFaceCreateDTO {
    ///Creates a new [`AssetFaceCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: ::uuid::Uuid,
        height: i64,
        image_height: i64,
        image_width: i64,
        person_id: ::uuid::Uuid,
        width: i64,
        x: i64,
        y: i64,
    ) -> Self {
        Self {
            asset_id,
            height,
            image_height,
            image_width,
            person_id,
            user_id: None,
            width,
            x,
            y,
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `height`.
    #[must_use]
    pub fn height(mut self, value: i64) -> Self {
        self.height = value;
        self
    }

    ///Sets `image_height`.
    #[must_use]
    pub fn image_height(mut self, value: i64) -> Self {
        self.image_height = value;
        self
    }

    ///Sets `image_width`.
    #[must_use]
    pub fn image_width(mut self, value: i64) -> Self {
        self.image_width = value;
        self
    }

    ///Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: ::uuid::Uuid) -> Self {
        self.person_id = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: ::uuid::Uuid) -> Self {
        self.user_id = Some(value);
        self
    }

    ///Sets or clears `user_id`.
    #[must_use]
    pub fn set_user_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.user_id = value;
        self
    }

    ///Sets `width`.
    #[must_use]
    pub fn width(mut self, value: i64) -> Self {
        self.width = value;
        self
    }

    ///Sets `x`.
    #[must_use]
    pub fn x(mut self, value: i64) -> Self {
        self.x = value;
        self
    }

    ///Sets `y`.
    #[must_use]
    pub fn y(mut self, value: i64) -> Self {
        self.y = value;
        self
    }
}

/// `AssetFaceDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `force` | `bool` | ✓ | Force delete even if person has other faces |
///
/// See: <https://api.immich.app/models/AssetFaceDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetFaceDeleteDTO {
    pub force: bool,
}

impl AssetFaceDeleteDTO {
    ///Creates a new [`AssetFaceDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(force: bool) -> Self {
        Self { force }
    }

    ///Sets `force`.
    #[must_use]
    pub fn force(mut self, value: bool) -> Self {
        self.force = value;
        self
    }
}

/// Asset face with person
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `bounding_box_x1` | `i64` | ✓ | Bounding box X1 coordinate |
/// | `bounding_box_x2` | `i64` | ✓ | Bounding box X2 coordinate |
/// | `bounding_box_y1` | `i64` | ✓ | Bounding box Y1 coordinate |
/// | `bounding_box_y2` | `i64` | ✓ | Bounding box Y2 coordinate |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Face ID |
/// | `image_height` | `i64` | ✓ | Image height in pixels |
/// | `image_width` | `i64` | ✓ | Image width in pixels |
/// | `person` | [`PersonResponseDTO`] | ✓ |  |
/// | `source_type` | [`SourceType`] |  |  |
///
/// See: <https://api.immich.app/models/AssetFaceResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetFaceResponseDTO {
    /// Bounding box X1 coordinate (required)
    #[serde(rename = "boundingBoxX1")]
    pub bounding_box_x1: i64,
    /// Bounding box X2 coordinate (required)
    #[serde(rename = "boundingBoxX2")]
    pub bounding_box_x2: i64,
    /// Bounding box Y1 coordinate (required)
    #[serde(rename = "boundingBoxY1")]
    pub bounding_box_y1: i64,
    /// Bounding box Y2 coordinate (required)
    #[serde(rename = "boundingBoxY2")]
    pub bounding_box_y2: i64,
    pub id: ::uuid::Uuid,
    /// Image height in pixels (required)
    #[serde(rename = "imageHeight")]
    pub image_height: i64,
    /// Image width in pixels (required)
    #[serde(rename = "imageWidth")]
    pub image_width: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonResponseDTO>,
    #[serde(
        rename = "sourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_type: Option<SourceType>,
}

impl AssetFaceResponseDTO {
    ///Creates a new [`AssetFaceResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        bounding_box_x1: i64,
        bounding_box_x2: i64,
        bounding_box_y1: i64,
        bounding_box_y2: i64,
        id: ::uuid::Uuid,
        image_height: i64,
        image_width: i64,
    ) -> Self {
        Self {
            bounding_box_x1,
            bounding_box_x2,
            bounding_box_y1,
            bounding_box_y2,
            id,
            image_height,
            image_width,
            person: None,
            source_type: None,
        }
    }

    ///Sets `bounding_box_x1`.
    #[must_use]
    pub fn bounding_box_x1(mut self, value: i64) -> Self {
        self.bounding_box_x1 = value;
        self
    }

    ///Sets `bounding_box_x2`.
    #[must_use]
    pub fn bounding_box_x2(mut self, value: i64) -> Self {
        self.bounding_box_x2 = value;
        self
    }

    ///Sets `bounding_box_y1`.
    #[must_use]
    pub fn bounding_box_y1(mut self, value: i64) -> Self {
        self.bounding_box_y1 = value;
        self
    }

    ///Sets `bounding_box_y2`.
    #[must_use]
    pub fn bounding_box_y2(mut self, value: i64) -> Self {
        self.bounding_box_y2 = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `image_height`.
    #[must_use]
    pub fn image_height(mut self, value: i64) -> Self {
        self.image_height = value;
        self
    }

    ///Sets `image_width`.
    #[must_use]
    pub fn image_width(mut self, value: i64) -> Self {
        self.image_width = value;
        self
    }

    ///Sets `person`.
    #[must_use]
    pub fn person(mut self, value: PersonResponseDTO) -> Self {
        self.person = Some(value);
        self
    }

    ///Sets or clears `person`.
    #[must_use]
    pub fn set_person(mut self, value: Option<PersonResponseDTO>) -> Self {
        self.person = value;
        self
    }

    ///Sets `source_type`.
    #[must_use]
    pub fn source_type(mut self, value: SourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    ///Sets or clears `source_type`.
    #[must_use]
    pub fn set_source_type(mut self, value: Option<SourceType>) -> Self {
        self.source_type = value;
        self
    }
}

/// `FaceDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Face ID |
///
/// See: <https://api.immich.app/models/FaceDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct FaceDTO {
    pub id: ::uuid::Uuid,
}

impl FaceDTO {
    ///Creates a new [`FaceDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: ::uuid::Uuid) -> Self {
        Self { id }
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }
}

/// Face detection source type
///
/// See: <https://api.immich.app/models/SourceType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SourceType {
    #[serde(rename = "machine-learning")]
    MachineLearning,
    #[serde(rename = "exif")]
    Exif,
    #[serde(rename = "manual")]
    Manual,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MachineLearning => f.write_str("machine-learning"),
            Self::Exif => f.write_str("exif"),
            Self::Manual => f.write_str("manual"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for SourceType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "machine-learning" => Ok(Self::MachineLearning),
            "exif" => Ok(Self::Exif),
            "manual" => Ok(Self::Manual),
            _ => Err(format!("invalid value for {}: {}", "SourceType", value).into()),
        }
    }
}

impl TryFrom<&str> for SourceType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SourceType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SourceType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}
