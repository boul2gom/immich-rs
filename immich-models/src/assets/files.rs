// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `files`.

pub use crate::id::AssetFileId;

/// `AssetFileResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`AssetFileId`](crate::id::AssetFileId) | ✓ | Asset file ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_edited` | `bool` | ✓ | The file was generated from an edit |
/// | `is_progressive` | `bool` | ✓ | The file is a progressively encoded JPEG |
/// | `is_transparent` | `bool` | ✓ | The file is transparent |
/// | `path` | [`String`] | ✓ | File path |
/// | `type_` | [`AssetFileType`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Update date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/AssetFileResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetFileResponseDTO {
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Asset file ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: AssetFileId,
    /// The file was generated from an edit (required)
    #[serde(rename = "isEdited")]
    pub is_edited: bool,
    /// The file is a progressively encoded JPEG (required)
    #[serde(rename = "isProgressive")]
    pub is_progressive: bool,
    /// The file is transparent (required)
    #[serde(rename = "isTransparent")]
    pub is_transparent: bool,
    /// File path (required)
    pub path: String,
    /// (required)
    #[serde(rename = "type")]
    pub type_: AssetFileType,
    /// Update date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl AssetFileResponseDTO {
    /// Creates a new [`AssetFileResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<AssetFileId>,
        is_edited: bool,
        is_progressive: bool,
        is_transparent: bool,
        path: impl Into<String>,
        type_: AssetFileType,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            id: id.into(),
            is_edited,
            is_progressive,
            is_transparent,
            path: path.into(),
            type_,
            updated_at,
        }
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<AssetFileId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_edited`.
    #[must_use]
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = value;
        self
    }
    /// Sets `is_progressive`.
    #[must_use]
    pub fn is_progressive(mut self, value: bool) -> Self {
        self.is_progressive = value;
        self
    }
    /// Sets `is_transparent`.
    #[must_use]
    pub fn is_transparent(mut self, value: bool) -> Self {
        self.is_transparent = value;
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
    pub fn type_(mut self, value: AssetFileType) -> Self {
        self.type_ = value;
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// Type of file
///
/// See: <https://api.immich.app/models/AssetFileType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetFileType {
    Fullsize,
    Preview,
    Thumbnail,
    Sidecar,
    EncodedVideo,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetFileType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Fullsize => "fullsize",
            Self::Preview => "preview",
            Self::Thumbnail => "thumbnail",
            Self::Sidecar => "sidecar",
            Self::EncodedVideo => "encoded_video",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetFileType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetFileType {
    fn from(value: &str) -> Self {
        match value {
            "fullsize" => Self::Fullsize,
            "preview" => Self::Preview,
            "thumbnail" => Self::Thumbnail,
            "sidecar" => Self::Sidecar,
            "encoded_video" => Self::EncodedVideo,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetFileType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "fullsize" => Self::Fullsize,
            "preview" => Self::Preview,
            "thumbnail" => Self::Thumbnail,
            "sidecar" => Self::Sidecar,
            "encoded_video" => Self::EncodedVideo,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetFileType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetFileType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetFileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetFileType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetFileType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetFileType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetFileType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}
