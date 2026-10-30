// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `faces`.

pub use crate::assets::PersonResponseDTO;
pub use crate::id::{AssetId, FaceId, PersonId, UserId};

/// `AssetFaceCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `height` | `i64` | ✓ | Face bounding box height |
/// | `image_height` | `i64` | ✓ | Image height in pixels |
/// | `image_width` | `i64` | ✓ | Image width in pixels |
/// | `person_id` | [`PersonId`](crate::id::PersonId) | ✓ | Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `user_id` | [`UserId`](crate::id::UserId) |  | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `width` | `i64` | ✓ | Face bounding box width |
/// | `x` | `i64` | ✓ | Face bounding box X coordinate |
/// | `y` | `i64` | ✓ | Face bounding box Y coordinate |
///
/// See: <https://api.immich.app/models/AssetFaceCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetFaceCreateDTO {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Face bounding box height (required)
    pub height: i64,
    /// Image height in pixels (required)
    #[serde(rename = "imageHeight")]
    pub image_height: i64,
    /// Image width in pixels (required)
    #[serde(rename = "imageWidth")]
    pub image_width: i64,
    /// Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "personId")]
    pub person_id: PersonId,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<UserId>,
    /// Face bounding box width (required)
    pub width: i64,
    /// Face bounding box X coordinate (required)
    pub x: i64,
    /// Face bounding box Y coordinate (required)
    pub y: i64,
}

impl AssetFaceCreateDTO {
    /// Creates a new [`AssetFaceCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: impl Into<AssetId>,
        height: i64,
        image_height: i64,
        image_width: i64,
        person_id: impl Into<PersonId>,
        width: i64,
        x: i64,
        y: i64,
    ) -> Self {
        Self {
            asset_id: asset_id.into(),
            height,
            image_height,
            image_width,
            person_id: person_id.into(),
            user_id: None,
            width,
            x,
            y,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `height`.
    #[must_use]
    pub fn height(mut self, value: i64) -> Self {
        self.height = value;
        self
    }
    /// Sets `image_height`.
    #[must_use]
    pub fn image_height(mut self, value: i64) -> Self {
        self.image_height = value;
        self
    }
    /// Sets `image_width`.
    #[must_use]
    pub fn image_width(mut self, value: i64) -> Self {
        self.image_width = value;
        self
    }
    /// Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: impl Into<PersonId>) -> Self {
        self.person_id = value.into();
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<UserId>) -> Self {
        self.user_id = Some(value.into());
        self
    }
    /// Sets or clears `user_id`.
    #[must_use]
    pub fn set_user_id(mut self, value: Option<UserId>) -> Self {
        self.user_id = value;
        self
    }
    /// Sets `width`.
    #[must_use]
    pub fn width(mut self, value: i64) -> Self {
        self.width = value;
        self
    }
    /// Sets `x`.
    #[must_use]
    pub fn x(mut self, value: i64) -> Self {
        self.x = value;
        self
    }
    /// Sets `y`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetFaceDeleteDTO {
    /// Force delete even if person has other faces (required)
    pub force: bool,
}

impl AssetFaceDeleteDTO {
    /// Creates a new [`AssetFaceDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(force: bool) -> Self {
        Self { force }
    }
    /// Sets `force`.
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
/// | `id` | [`FaceId`](crate::id::FaceId) | ✓ | Face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `image_height` | `i64` | ✓ | Image height in pixels |
/// | `image_width` | `i64` | ✓ | Image width in pixels |
/// | `person` | [`PersonResponseDTO`] | ✓ |  |
/// | `source_type` | [`SourceType`] |  |  |
///
/// See: <https://api.immich.app/models/AssetFaceResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
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
    /// Face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: FaceId,
    /// Image height in pixels (required)
    #[serde(rename = "imageHeight")]
    pub image_height: i64,
    /// Image width in pixels (required)
    #[serde(rename = "imageWidth")]
    pub image_width: i64,
    /// (required)
    pub person: Option<PersonResponseDTO>,
    #[serde(
        rename = "sourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub source_type: Option<SourceType>,
}

impl AssetFaceResponseDTO {
    /// Creates a new [`AssetFaceResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        bounding_box_x1: i64,
        bounding_box_x2: i64,
        bounding_box_y1: i64,
        bounding_box_y2: i64,
        id: impl Into<FaceId>,
        image_height: i64,
        image_width: i64,
    ) -> Self {
        Self {
            bounding_box_x1,
            bounding_box_x2,
            bounding_box_y1,
            bounding_box_y2,
            id: id.into(),
            image_height,
            image_width,
            person: None,
            source_type: None,
        }
    }
    /// Sets `bounding_box_x1`.
    #[must_use]
    pub fn bounding_box_x1(mut self, value: i64) -> Self {
        self.bounding_box_x1 = value;
        self
    }
    /// Sets `bounding_box_x2`.
    #[must_use]
    pub fn bounding_box_x2(mut self, value: i64) -> Self {
        self.bounding_box_x2 = value;
        self
    }
    /// Sets `bounding_box_y1`.
    #[must_use]
    pub fn bounding_box_y1(mut self, value: i64) -> Self {
        self.bounding_box_y1 = value;
        self
    }
    /// Sets `bounding_box_y2`.
    #[must_use]
    pub fn bounding_box_y2(mut self, value: i64) -> Self {
        self.bounding_box_y2 = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<FaceId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `image_height`.
    #[must_use]
    pub fn image_height(mut self, value: i64) -> Self {
        self.image_height = value;
        self
    }
    /// Sets `image_width`.
    #[must_use]
    pub fn image_width(mut self, value: i64) -> Self {
        self.image_width = value;
        self
    }
    /// Sets `person`.
    #[must_use]
    pub fn person(mut self, value: PersonResponseDTO) -> Self {
        self.person = Some(value);
        self
    }
    /// Sets or clears `person`.
    #[must_use]
    pub fn set_person(mut self, value: Option<PersonResponseDTO>) -> Self {
        self.person = value;
        self
    }
    /// Sets `source_type`.
    #[must_use]
    pub fn source_type(mut self, value: SourceType) -> Self {
        self.source_type = Some(value);
        self
    }
    /// Sets or clears `source_type`.
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
/// | `id` | [`FaceId`](crate::id::FaceId) | ✓ | Face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/FaceDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct FaceDTO {
    /// Face ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: FaceId,
}

impl FaceDTO {
    /// Creates a new [`FaceDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: impl Into<FaceId>) -> Self {
        Self { id: id.into() }
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<FaceId>) -> Self {
        self.id = value.into();
        self
    }
}

/// Face detection source type
///
/// See: <https://api.immich.app/models/SourceType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SourceType {
    MachineLearning,
    Exif,
    Manual,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl SourceType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::MachineLearning => "machine-learning",
            Self::Exif => "exif",
            Self::Manual => "manual",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for SourceType {
    fn from(value: &str) -> Self {
        match value {
            "machine-learning" => Self::MachineLearning,
            "exif" => Self::Exif,
            "manual" => Self::Manual,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for SourceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "machine-learning" => Self::MachineLearning,
            "exif" => Self::Exif,
            "manual" => Self::Manual,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for SourceType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for SourceType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for SourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for SourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = SourceType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SourceType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SourceType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}
