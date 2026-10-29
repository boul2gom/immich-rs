// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `search`.

pub use crate::albums::{AlbumResponseDTO, AssetOrder};
pub use crate::assets::{AssetResponseDTO, AssetTypeEnum, AssetVisibility};
pub use crate::common::{
    StringPatternFilterEndsWith, StringPatternFilterLike, StringPatternFilterNotLike, StringPatternFilterStartsWith,
    StringSimilarityFilterMatches,
};

/// `BoolFilter`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | `bool` | ✓ |  |
///
/// See: <https://api.immich.app/models/BoolFilter>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct BoolFilter {
    pub eq: bool,
}

impl BoolFilter {
    ///Creates a new [`BoolFilter`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(eq: bool) -> Self {
        Self { eq }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: bool) -> Self {
        self.eq = value;
        self
    }
}

/// `DateFilter`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `gt` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `gte` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `lt` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `lte` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `ne` | [`DateTime`](::chrono::DateTime) |  |  |
///
/// See: <https://api.immich.app/models/DateFilter>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct DateFilter {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gt: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gte: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lt: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lte: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl DateFilter {
    ///Creates a new [`DateFilter`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            eq: None,
            gt: None,
            gte: None,
            lt: None,
            lte: None,
            ne: None,
        }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.eq = Some(value);
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `gt`.
    #[must_use]
    pub fn gt(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.gt = Some(value);
        self
    }

    ///Sets or clears `gt`.
    #[must_use]
    pub fn set_gt(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.gt = value;
        self
    }

    ///Sets `gte`.
    #[must_use]
    pub fn gte(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.gte = Some(value);
        self
    }

    ///Sets or clears `gte`.
    #[must_use]
    pub fn set_gte(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.gte = value;
        self
    }

    ///Sets `lt`.
    #[must_use]
    pub fn lt(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.lt = Some(value);
        self
    }

    ///Sets or clears `lt`.
    #[must_use]
    pub fn set_lt(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.lt = value;
        self
    }

    ///Sets `lte`.
    #[must_use]
    pub fn lte(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.lte = Some(value);
        self
    }

    ///Sets or clears `lte`.
    #[must_use]
    pub fn set_lte(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.lte = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.ne = Some(value);
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.ne = value;
        self
    }
}

/// `DateFilterNullable`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `gt` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `gte` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `lt` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `lte` | [`DateTime`](::chrono::DateTime) |  |  |
/// | `ne` | [`DateTime`](::chrono::DateTime) |  |  |
///
/// See: <https://api.immich.app/models/DateFilterNullable>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct DateFilterNullable {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gt: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gte: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lt: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lte: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl DateFilterNullable {
    ///Creates a new [`DateFilterNullable`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            eq: None,
            gt: None,
            gte: None,
            lt: None,
            lte: None,
            ne: None,
        }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.eq = Some(value);
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `gt`.
    #[must_use]
    pub fn gt(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.gt = Some(value);
        self
    }

    ///Sets or clears `gt`.
    #[must_use]
    pub fn set_gt(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.gt = value;
        self
    }

    ///Sets `gte`.
    #[must_use]
    pub fn gte(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.gte = Some(value);
        self
    }

    ///Sets or clears `gte`.
    #[must_use]
    pub fn set_gte(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.gte = value;
        self
    }

    ///Sets `lt`.
    #[must_use]
    pub fn lt(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.lt = Some(value);
        self
    }

    ///Sets or clears `lt`.
    #[must_use]
    pub fn set_lt(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.lt = value;
        self
    }

    ///Sets `lte`.
    #[must_use]
    pub fn lte(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.lte = Some(value);
        self
    }

    ///Sets or clears `lte`.
    #[must_use]
    pub fn set_lte(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.lte = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.ne = Some(value);
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.ne = value;
        self
    }
}

/// `EnumFilterAssetType`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | [`AssetTypeEnum`] |  |  |
/// | `in` | [`Vec<AssetTypeEnum>`](crate::assets::AssetTypeEnum) |  |  |
/// | `ne` | [`AssetTypeEnum`] |  |  |
/// | `not_in` | [`Vec<AssetTypeEnum>`](crate::assets::AssetTypeEnum) |  |  |
///
/// See: <https://api.immich.app/models/EnumFilterAssetType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct EnumFilterAssetType {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<AssetTypeEnum>,
    #[serde(rename = "in", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub in_: Vec<AssetTypeEnum>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<AssetTypeEnum>,
    #[serde(rename = "notIn", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub not_in: Vec<AssetTypeEnum>,
}

impl EnumFilterAssetType {
    ///Creates a new [`EnumFilterAssetType`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            eq: None,
            in_: Vec::new(),
            ne: None,
            not_in: Vec::new(),
        }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: AssetTypeEnum) -> Self {
        self.eq = Some(value);
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<AssetTypeEnum>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `in_`.
    #[must_use]
    pub fn in_(mut self, value: Vec<AssetTypeEnum>) -> Self {
        self.in_ = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: AssetTypeEnum) -> Self {
        self.ne = Some(value);
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<AssetTypeEnum>) -> Self {
        self.ne = value;
        self
    }

    ///Sets `not_in`.
    #[must_use]
    pub fn not_in(mut self, value: Vec<AssetTypeEnum>) -> Self {
        self.not_in = value;
        self
    }
}

/// `EnumFilterAssetVisibility`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | [`AssetVisibility`] |  |  |
/// | `in` | [`Vec<AssetVisibility>`](crate::assets::AssetVisibility) |  |  |
/// | `ne` | [`AssetVisibility`] |  |  |
/// | `not_in` | [`Vec<AssetVisibility>`](crate::assets::AssetVisibility) |  |  |
///
/// See: <https://api.immich.app/models/EnumFilterAssetVisibility>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct EnumFilterAssetVisibility {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<AssetVisibility>,
    #[serde(rename = "in", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub in_: Vec<AssetVisibility>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<AssetVisibility>,
    #[serde(rename = "notIn", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub not_in: Vec<AssetVisibility>,
}

impl EnumFilterAssetVisibility {
    ///Creates a new [`EnumFilterAssetVisibility`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            eq: None,
            in_: Vec::new(),
            ne: None,
            not_in: Vec::new(),
        }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: AssetVisibility) -> Self {
        self.eq = Some(value);
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<AssetVisibility>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `in_`.
    #[must_use]
    pub fn in_(mut self, value: Vec<AssetVisibility>) -> Self {
        self.in_ = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: AssetVisibility) -> Self {
        self.ne = Some(value);
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<AssetVisibility>) -> Self {
        self.ne = value;
        self
    }

    ///Sets `not_in`.
    #[must_use]
    pub fn not_in(mut self, value: Vec<AssetVisibility>) -> Self {
        self.not_in = value;
        self
    }
}

/// `IdFilter`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | [`Uuid`](::uuid::Uuid) |  |  |
/// | `ne` | [`Uuid`](::uuid::Uuid) |  |  |
///
/// See: <https://api.immich.app/models/IdFilter>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct IdFilter {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<::uuid::Uuid>,
}

impl IdFilter {
    ///Creates a new [`IdFilter`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { eq: None, ne: None }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: ::uuid::Uuid) -> Self {
        self.eq = Some(value);
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: ::uuid::Uuid) -> Self {
        self.ne = Some(value);
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.ne = value;
        self
    }
}

/// `IdFilterNullable`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | [`Uuid`](::uuid::Uuid) |  |  |
/// | `ne` | [`Uuid`](::uuid::Uuid) |  |  |
///
/// See: <https://api.immich.app/models/IdFilterNullable>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct IdFilterNullable {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<::uuid::Uuid>,
}

impl IdFilterNullable {
    ///Creates a new [`IdFilterNullable`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { eq: None, ne: None }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: ::uuid::Uuid) -> Self {
        self.eq = Some(value);
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: ::uuid::Uuid) -> Self {
        self.ne = Some(value);
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.ne = value;
        self
    }
}

/// `IdsFilter`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `all` | [`Vec<Uuid>`](::uuid::Uuid) |  |  |
/// | `any` | [`Vec<Uuid>`](::uuid::Uuid) |  |  |
/// | `none` | [`Vec<Uuid>`](::uuid::Uuid) |  |  |
///
/// See: <https://api.immich.app/models/IdsFilter>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct IdsFilter {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub all: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub any: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub none: Vec<::uuid::Uuid>,
}

impl IdsFilter {
    ///Creates a new [`IdsFilter`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            all: Vec::new(),
            any: Vec::new(),
            none: Vec::new(),
        }
    }

    ///Sets `all`.
    #[must_use]
    pub fn all(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.all = value;
        self
    }

    ///Sets `any`.
    #[must_use]
    pub fn any(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.any = value;
        self
    }

    ///Sets `none`.
    #[must_use]
    pub fn none(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.none = value;
        self
    }
}

/// `MetadataSearchDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by album IDs |
/// | `checksum` | [`String`] |  | Filter by file checksum |
/// | `city` | [`String`] |  | Filter by city name |
/// | `country` | [`String`] |  | Filter by country name |
/// | `created_after` | [`DateTime`](::chrono::DateTime) |  | Filter by creation date (after) |
/// | `created_before` | [`DateTime`](::chrono::DateTime) |  | Filter by creation date (before) |
/// | `cursor` | [`String`] |  | Cursor for the next page of results |
/// | `description` | [`String`] |  | Filter by description text |
/// | `encoded_video_path` | [`String`] |  | Filter by encoded video file path |
/// | `filter` | [`SearchFilter`] |  |  |
/// | `id` | [`Uuid`](::uuid::Uuid) |  | Filter by asset ID |
/// | `is_encoded` | `bool` |  | Filter by encoded status |
/// | `is_favorite` | `bool` |  | Filter by favorite status |
/// | `is_motion` | `bool` |  | Filter by motion photo status |
/// | `is_not_in_album` | `bool` |  | Filter assets not in any album |
/// | `is_offline` | `bool` |  | Filter by offline status |
/// | `lens_model` | [`String`] |  | Filter by lens model |
/// | `library_id` | [`Uuid`](::uuid::Uuid) |  | Library ID to filter by |
/// | `make` | [`String`] |  | Filter by camera make |
/// | `model` | [`String`] |  | Filter by camera model |
/// | `ocr` | [`String`] |  | Filter by OCR text content |
/// | `order` | [`AssetOrder`] |  | Sort order |
/// | `order_by` | [`SearchOrder`] |  |  |
/// | `original_file_name` | [`String`] |  | Filter by original file name |
/// | `original_path` | [`String`] |  | Filter by original file path |
/// | `page` | `i64` |  | Page number |
/// | `person_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by person IDs |
/// | `preview_path` | [`String`] |  | Filter by preview file path |
/// | `rating` | `i64` |  | Filter by rating [1-5], or null for unrated |
/// | `size` | `i64` |  | Number of results to return |
/// | `state` | [`String`] |  | Filter by state/province name |
/// | `tag_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by tag IDs |
/// | `taken_after` | [`DateTime`](::chrono::DateTime) |  | Filter by taken date (after) |
/// | `taken_before` | [`DateTime`](::chrono::DateTime) |  | Filter by taken date (before) |
/// | `thumbnail_path` | [`String`] |  | Filter by thumbnail file path |
/// | `trashed_after` | [`DateTime`](::chrono::DateTime) |  | Filter by trash date (after) |
/// | `trashed_before` | [`DateTime`](::chrono::DateTime) |  | Filter by trash date (before) |
/// | `type` | [`AssetTypeEnum`] |  |  |
/// | `updated_after` | [`DateTime`](::chrono::DateTime) |  | Filter by update date (after) |
/// | `updated_before` | [`DateTime`](::chrono::DateTime) |  | Filter by update date (before) |
/// | `visibility` | [`AssetVisibility`] |  |  |
/// | `with_deleted` | `bool` |  | Include deleted assets |
/// | `with_exif` | `bool` |  | Include EXIF data in response |
/// | `with_people` | `bool` |  | Include people data in response |
/// | `with_stacked` | `bool` |  | Include stacked assets |
///
/// See: <https://api.immich.app/models/MetadataSearchDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MetadataSearchDTO {
    /// Filter by album IDs
    #[serde(rename = "albumIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub album_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: Option<String>,
    /// Filter by creation date (after)
    #[serde(
        rename = "createdAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by creation date (before)
    #[serde(
        rename = "createdBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: Option<MetadataSearchCursor>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Filter by encoded video file path
    #[serde(
        rename = "encodedVideoPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub encoded_video_path: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filter: Option<SearchFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: Option<::uuid::Uuid>,
    /// Filter by encoded status
    #[serde(
        rename = "isEncoded",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_encoded: Option<bool>,
    /// Filter by favorite status
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Filter by motion photo status
    #[serde(rename = "isMotion", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_motion: Option<bool>,
    /// Filter assets not in any album
    #[serde(
        rename = "isNotInAlbum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_not_in_album: Option<bool>,
    /// Filter by offline status
    #[serde(
        rename = "isOffline",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_offline: Option<bool>,
    /// Filter by lens model
    #[serde(
        rename = "lensModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lens_model: Option<String>,
    /// Library ID to filter by
    #[serde(
        rename = "libraryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub library_id: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub make: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ocr: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub order: Option<AssetOrder>,
    #[serde(rename = "orderBy", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub order_by: Option<SearchOrder>,
    /// Filter by original file name
    #[serde(
        rename = "originalFileName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_file_name: Option<String>,
    /// Filter by original file path
    #[serde(
        rename = "originalPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_path: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: Option<::std::num::NonZeroU64>,
    /// Filter by person IDs
    #[serde(rename = "personIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub person_ids: Vec<::uuid::Uuid>,
    /// Filter by preview file path
    #[serde(
        rename = "previewPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preview_path: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<::std::num::NonZeroU64>,
    #[serde(default = "crate::common::defaults::default_nzu64::<::std::num::NonZeroU64, 250>")]
    pub size: ::std::num::NonZeroU64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
    /// Filter by tag IDs
    #[serde(rename = "tagIds", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag_ids: Option<Vec<::uuid::Uuid>>,
    /// Filter by taken date (after)
    #[serde(
        rename = "takenAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taken_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by taken date (before)
    #[serde(
        rename = "takenBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taken_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by thumbnail file path
    #[serde(
        rename = "thumbnailPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub thumbnail_path: Option<String>,
    /// Filter by trash date (after)
    #[serde(
        rename = "trashedAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by trash date (before)
    #[serde(
        rename = "trashedBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<AssetTypeEnum>,
    /// Filter by update date (after)
    #[serde(
        rename = "updatedAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by update date (before)
    #[serde(
        rename = "updatedBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<AssetVisibility>,
    /// Include deleted assets
    #[serde(
        rename = "withDeleted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub with_deleted: Option<bool>,
    /// Include EXIF data in response
    #[serde(rename = "withExif", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub with_exif: Option<bool>,
    /// Include people data in response
    #[serde(
        rename = "withPeople",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub with_people: Option<bool>,
    /// Include stacked assets
    #[serde(
        rename = "withStacked",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub with_stacked: Option<bool>,
}

impl MetadataSearchDTO {
    ///Creates a new [`MetadataSearchDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_ids: Vec::new(),
            checksum: None,
            city: None,
            country: None,
            created_after: None,
            created_before: None,
            cursor: None,
            description: None,
            encoded_video_path: None,
            filter: None,
            id: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_not_in_album: None,
            is_offline: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            ocr: None,
            order: None,
            order_by: None,
            original_file_name: None,
            original_path: None,
            page: None,
            person_ids: Vec::new(),
            preview_path: None,
            rating: None,
            size: crate::common::defaults::default_nzu64::<::std::num::NonZeroU64, 250>(),
            state: None,
            tag_ids: None,
            taken_after: None,
            taken_before: None,
            thumbnail_path: None,
            trashed_after: None,
            trashed_before: None,
            type_: None,
            updated_after: None,
            updated_before: None,
            visibility: None,
            with_deleted: None,
            with_exif: None,
            with_people: None,
            with_stacked: None,
        }
    }

    ///Sets `album_ids`.
    #[must_use]
    pub fn album_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.album_ids = value;
        self
    }

    ///Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = Some(value.into());
        self
    }

    ///Sets or clears `checksum`.
    #[must_use]
    pub fn set_checksum(mut self, value: Option<String>) -> Self {
        self.checksum = value;
        self
    }

    ///Sets `city`.
    #[must_use]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    ///Sets or clears `city`.
    #[must_use]
    pub fn set_city(mut self, value: Option<String>) -> Self {
        self.city = value;
        self
    }

    ///Sets `country`.
    #[must_use]
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    ///Sets or clears `country`.
    #[must_use]
    pub fn set_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }

    ///Sets `created_after`.
    #[must_use]
    pub fn created_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_after = Some(value);
        self
    }

    ///Sets or clears `created_after`.
    #[must_use]
    pub fn set_created_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_after = value;
        self
    }

    ///Sets `created_before`.
    #[must_use]
    pub fn created_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_before = Some(value);
        self
    }

    ///Sets or clears `created_before`.
    #[must_use]
    pub fn set_created_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_before = value;
        self
    }

    ///Sets `cursor`.
    #[must_use]
    pub fn cursor(mut self, value: MetadataSearchCursor) -> Self {
        self.cursor = Some(value);
        self
    }

    ///Sets or clears `cursor`.
    #[must_use]
    pub fn set_cursor(mut self, value: Option<MetadataSearchCursor>) -> Self {
        self.cursor = value;
        self
    }

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    ///Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }

    ///Sets `encoded_video_path`.
    #[must_use]
    pub fn encoded_video_path(mut self, value: impl Into<String>) -> Self {
        self.encoded_video_path = Some(value.into());
        self
    }

    ///Sets or clears `encoded_video_path`.
    #[must_use]
    pub fn set_encoded_video_path(mut self, value: Option<String>) -> Self {
        self.encoded_video_path = value;
        self
    }

    ///Sets `filter`.
    #[must_use]
    pub fn filter(mut self, value: SearchFilter) -> Self {
        self.filter = Some(value);
        self
    }

    ///Sets or clears `filter`.
    #[must_use]
    pub fn set_filter(mut self, value: Option<SearchFilter>) -> Self {
        self.filter = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    ///Sets or clears `id`.
    #[must_use]
    pub fn set_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_encoded`.
    #[must_use]
    pub fn is_encoded(mut self, value: bool) -> Self {
        self.is_encoded = Some(value);
        self
    }

    ///Sets or clears `is_encoded`.
    #[must_use]
    pub fn set_is_encoded(mut self, value: Option<bool>) -> Self {
        self.is_encoded = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    ///Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `is_motion`.
    #[must_use]
    pub fn is_motion(mut self, value: bool) -> Self {
        self.is_motion = Some(value);
        self
    }

    ///Sets or clears `is_motion`.
    #[must_use]
    pub fn set_is_motion(mut self, value: Option<bool>) -> Self {
        self.is_motion = value;
        self
    }

    ///Sets `is_not_in_album`.
    #[must_use]
    pub fn is_not_in_album(mut self, value: bool) -> Self {
        self.is_not_in_album = Some(value);
        self
    }

    ///Sets or clears `is_not_in_album`.
    #[must_use]
    pub fn set_is_not_in_album(mut self, value: Option<bool>) -> Self {
        self.is_not_in_album = value;
        self
    }

    ///Sets `is_offline`.
    #[must_use]
    pub fn is_offline(mut self, value: bool) -> Self {
        self.is_offline = Some(value);
        self
    }

    ///Sets or clears `is_offline`.
    #[must_use]
    pub fn set_is_offline(mut self, value: Option<bool>) -> Self {
        self.is_offline = value;
        self
    }

    ///Sets `lens_model`.
    #[must_use]
    pub fn lens_model(mut self, value: impl Into<String>) -> Self {
        self.lens_model = Some(value.into());
        self
    }

    ///Sets or clears `lens_model`.
    #[must_use]
    pub fn set_lens_model(mut self, value: Option<String>) -> Self {
        self.lens_model = value;
        self
    }

    ///Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: ::uuid::Uuid) -> Self {
        self.library_id = Some(value);
        self
    }

    ///Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.library_id = value;
        self
    }

    ///Sets `make`.
    #[must_use]
    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }

    ///Sets or clears `make`.
    #[must_use]
    pub fn set_make(mut self, value: Option<String>) -> Self {
        self.make = value;
        self
    }

    ///Sets `model`.
    #[must_use]
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    ///Sets or clears `model`.
    #[must_use]
    pub fn set_model(mut self, value: Option<String>) -> Self {
        self.model = value;
        self
    }

    ///Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: impl Into<String>) -> Self {
        self.ocr = Some(value.into());
        self
    }

    ///Sets or clears `ocr`.
    #[must_use]
    pub fn set_ocr(mut self, value: Option<String>) -> Self {
        self.ocr = value;
        self
    }

    ///Sets `order`.
    #[must_use]
    pub fn order(mut self, value: AssetOrder) -> Self {
        self.order = Some(value);
        self
    }

    ///Sets or clears `order`.
    #[must_use]
    pub fn set_order(mut self, value: Option<AssetOrder>) -> Self {
        self.order = value;
        self
    }

    ///Sets `order_by`.
    #[must_use]
    pub fn order_by(mut self, value: SearchOrder) -> Self {
        self.order_by = Some(value);
        self
    }

    ///Sets or clears `order_by`.
    #[must_use]
    pub fn set_order_by(mut self, value: Option<SearchOrder>) -> Self {
        self.order_by = value;
        self
    }

    ///Sets `original_file_name`.
    #[must_use]
    pub fn original_file_name(mut self, value: impl Into<String>) -> Self {
        self.original_file_name = Some(value.into());
        self
    }

    ///Sets or clears `original_file_name`.
    #[must_use]
    pub fn set_original_file_name(mut self, value: Option<String>) -> Self {
        self.original_file_name = value;
        self
    }

    ///Sets `original_path`.
    #[must_use]
    pub fn original_path(mut self, value: impl Into<String>) -> Self {
        self.original_path = Some(value.into());
        self
    }

    ///Sets or clears `original_path`.
    #[must_use]
    pub fn set_original_path(mut self, value: Option<String>) -> Self {
        self.original_path = value;
        self
    }

    ///Sets `page`.
    #[must_use]
    pub fn page(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.page = Some(value);
        self
    }

    ///Sets or clears `page`.
    #[must_use]
    pub fn set_page(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.page = value;
        self
    }

    ///Sets `person_ids`.
    #[must_use]
    pub fn person_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.person_ids = value;
        self
    }

    ///Sets `preview_path`.
    #[must_use]
    pub fn preview_path(mut self, value: impl Into<String>) -> Self {
        self.preview_path = Some(value.into());
        self
    }

    ///Sets or clears `preview_path`.
    #[must_use]
    pub fn set_preview_path(mut self, value: Option<String>) -> Self {
        self.preview_path = value;
        self
    }

    ///Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.rating = Some(value);
        self
    }

    ///Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.rating = value;
        self
    }

    ///Sets `size`.
    #[must_use]
    pub fn size(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.size = value;
        self
    }

    ///Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    ///Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }

    ///Sets `tag_ids`.
    #[must_use]
    pub fn tag_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    ///Sets or clears `tag_ids`.
    #[must_use]
    pub fn set_tag_ids(mut self, value: Option<Vec<::uuid::Uuid>>) -> Self {
        self.tag_ids = value;
        self
    }

    ///Sets `taken_after`.
    #[must_use]
    pub fn taken_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.taken_after = Some(value);
        self
    }

    ///Sets or clears `taken_after`.
    #[must_use]
    pub fn set_taken_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.taken_after = value;
        self
    }

    ///Sets `taken_before`.
    #[must_use]
    pub fn taken_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.taken_before = Some(value);
        self
    }

    ///Sets or clears `taken_before`.
    #[must_use]
    pub fn set_taken_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.taken_before = value;
        self
    }

    ///Sets `thumbnail_path`.
    #[must_use]
    pub fn thumbnail_path(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_path = Some(value.into());
        self
    }

    ///Sets or clears `thumbnail_path`.
    #[must_use]
    pub fn set_thumbnail_path(mut self, value: Option<String>) -> Self {
        self.thumbnail_path = value;
        self
    }

    ///Sets `trashed_after`.
    #[must_use]
    pub fn trashed_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.trashed_after = Some(value);
        self
    }

    ///Sets or clears `trashed_after`.
    #[must_use]
    pub fn set_trashed_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.trashed_after = value;
        self
    }

    ///Sets `trashed_before`.
    #[must_use]
    pub fn trashed_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.trashed_before = Some(value);
        self
    }

    ///Sets or clears `trashed_before`.
    #[must_use]
    pub fn set_trashed_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.trashed_before = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = Some(value);
        self
    }

    ///Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<AssetTypeEnum>) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_after`.
    #[must_use]
    pub fn updated_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_after = Some(value);
        self
    }

    ///Sets or clears `updated_after`.
    #[must_use]
    pub fn set_updated_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_after = value;
        self
    }

    ///Sets `updated_before`.
    #[must_use]
    pub fn updated_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_before = Some(value);
        self
    }

    ///Sets or clears `updated_before`.
    #[must_use]
    pub fn set_updated_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_before = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    ///Sets or clears `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }

    ///Sets `with_deleted`.
    #[must_use]
    pub fn with_deleted(mut self, value: bool) -> Self {
        self.with_deleted = Some(value);
        self
    }

    ///Sets or clears `with_deleted`.
    #[must_use]
    pub fn set_with_deleted(mut self, value: Option<bool>) -> Self {
        self.with_deleted = value;
        self
    }

    ///Sets `with_exif`.
    #[must_use]
    pub fn with_exif(mut self, value: bool) -> Self {
        self.with_exif = Some(value);
        self
    }

    ///Sets or clears `with_exif`.
    #[must_use]
    pub fn set_with_exif(mut self, value: Option<bool>) -> Self {
        self.with_exif = value;
        self
    }

    ///Sets `with_people`.
    #[must_use]
    pub fn with_people(mut self, value: bool) -> Self {
        self.with_people = Some(value);
        self
    }

    ///Sets or clears `with_people`.
    #[must_use]
    pub fn set_with_people(mut self, value: Option<bool>) -> Self {
        self.with_people = value;
        self
    }

    ///Sets `with_stacked`.
    #[must_use]
    pub fn with_stacked(mut self, value: bool) -> Self {
        self.with_stacked = Some(value);
        self
    }

    ///Sets or clears `with_stacked`.
    #[must_use]
    pub fn set_with_stacked(mut self, value: Option<bool>) -> Self {
        self.with_stacked = value;
        self
    }
}

impl ::std::default::Default for MetadataSearchDTO {
    fn default() -> Self {
        Self {
            album_ids: Vec::new(),
            checksum: None,
            city: None,
            country: None,
            created_after: None,
            created_before: None,
            cursor: None,
            description: None,
            encoded_video_path: None,
            filter: None,
            id: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_not_in_album: None,
            is_offline: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            ocr: None,
            order: None,
            order_by: None,
            original_file_name: None,
            original_path: None,
            page: None,
            person_ids: Vec::new(),
            preview_path: None,
            rating: None,
            size: crate::common::defaults::default_nzu64::<::std::num::NonZeroU64, 250>(),
            state: None,
            tag_ids: None,
            taken_after: None,
            taken_before: None,
            thumbnail_path: None,
            trashed_after: None,
            trashed_before: None,
            type_: None,
            updated_after: None,
            updated_before: None,
            visibility: None,
            with_deleted: None,
            with_exif: None,
            with_people: None,
            with_stacked: None,
        }
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MetadataSearchCursor(pub String);

impl ::std::ops::Deref for MetadataSearchCursor {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for MetadataSearchCursor {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.is_empty() {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for MetadataSearchCursor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MetadataSearchCursor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MetadataSearchCursor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for MetadataSearchCursor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `NumberFilter`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | `f64` |  |  |
/// | `gt` | `f64` |  |  |
/// | `gte` | `f64` |  |  |
/// | `in` | `Vec<f64>` |  |  |
/// | `lt` | `f64` |  |  |
/// | `lte` | `f64` |  |  |
/// | `ne` | `f64` |  |  |
/// | `not_in` | `Vec<f64>` |  |  |
///
/// See: <https://api.immich.app/models/NumberFilter>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct NumberFilter {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gt: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gte: Option<f64>,
    #[serde(rename = "in", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub in_: Vec<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lt: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lte: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<f64>,
    #[serde(rename = "notIn", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub not_in: Vec<f64>,
}

impl NumberFilter {
    ///Creates a new [`NumberFilter`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            eq: None,
            gt: None,
            gte: None,
            in_: Vec::new(),
            lt: None,
            lte: None,
            ne: None,
            not_in: Vec::new(),
        }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: f64) -> Self {
        self.eq = Some(value);
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<f64>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `gt`.
    #[must_use]
    pub fn gt(mut self, value: f64) -> Self {
        self.gt = Some(value);
        self
    }

    ///Sets or clears `gt`.
    #[must_use]
    pub fn set_gt(mut self, value: Option<f64>) -> Self {
        self.gt = value;
        self
    }

    ///Sets `gte`.
    #[must_use]
    pub fn gte(mut self, value: f64) -> Self {
        self.gte = Some(value);
        self
    }

    ///Sets or clears `gte`.
    #[must_use]
    pub fn set_gte(mut self, value: Option<f64>) -> Self {
        self.gte = value;
        self
    }

    ///Sets `in_`.
    #[must_use]
    pub fn in_(mut self, value: Vec<f64>) -> Self {
        self.in_ = value;
        self
    }

    ///Sets `lt`.
    #[must_use]
    pub fn lt(mut self, value: f64) -> Self {
        self.lt = Some(value);
        self
    }

    ///Sets or clears `lt`.
    #[must_use]
    pub fn set_lt(mut self, value: Option<f64>) -> Self {
        self.lt = value;
        self
    }

    ///Sets `lte`.
    #[must_use]
    pub fn lte(mut self, value: f64) -> Self {
        self.lte = Some(value);
        self
    }

    ///Sets or clears `lte`.
    #[must_use]
    pub fn set_lte(mut self, value: Option<f64>) -> Self {
        self.lte = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: f64) -> Self {
        self.ne = Some(value);
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<f64>) -> Self {
        self.ne = value;
        self
    }

    ///Sets `not_in`.
    #[must_use]
    pub fn not_in(mut self, value: Vec<f64>) -> Self {
        self.not_in = value;
        self
    }
}

/// `NumberFilterNullable`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | `f64` |  |  |
/// | `gt` | `f64` |  |  |
/// | `gte` | `f64` |  |  |
/// | `in` | `Vec<f64>` |  |  |
/// | `lt` | `f64` |  |  |
/// | `lte` | `f64` |  |  |
/// | `ne` | `f64` |  |  |
/// | `not_in` | `Vec<f64>` |  |  |
///
/// See: <https://api.immich.app/models/NumberFilterNullable>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct NumberFilterNullable {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gt: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gte: Option<f64>,
    #[serde(rename = "in", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub in_: Vec<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lt: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lte: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<f64>,
    #[serde(rename = "notIn", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub not_in: Vec<f64>,
}

impl NumberFilterNullable {
    ///Creates a new [`NumberFilterNullable`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            eq: None,
            gt: None,
            gte: None,
            in_: Vec::new(),
            lt: None,
            lte: None,
            ne: None,
            not_in: Vec::new(),
        }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: f64) -> Self {
        self.eq = Some(value);
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<f64>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `gt`.
    #[must_use]
    pub fn gt(mut self, value: f64) -> Self {
        self.gt = Some(value);
        self
    }

    ///Sets or clears `gt`.
    #[must_use]
    pub fn set_gt(mut self, value: Option<f64>) -> Self {
        self.gt = value;
        self
    }

    ///Sets `gte`.
    #[must_use]
    pub fn gte(mut self, value: f64) -> Self {
        self.gte = Some(value);
        self
    }

    ///Sets or clears `gte`.
    #[must_use]
    pub fn set_gte(mut self, value: Option<f64>) -> Self {
        self.gte = value;
        self
    }

    ///Sets `in_`.
    #[must_use]
    pub fn in_(mut self, value: Vec<f64>) -> Self {
        self.in_ = value;
        self
    }

    ///Sets `lt`.
    #[must_use]
    pub fn lt(mut self, value: f64) -> Self {
        self.lt = Some(value);
        self
    }

    ///Sets or clears `lt`.
    #[must_use]
    pub fn set_lt(mut self, value: Option<f64>) -> Self {
        self.lt = value;
        self
    }

    ///Sets `lte`.
    #[must_use]
    pub fn lte(mut self, value: f64) -> Self {
        self.lte = Some(value);
        self
    }

    ///Sets or clears `lte`.
    #[must_use]
    pub fn set_lte(mut self, value: Option<f64>) -> Self {
        self.lte = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: f64) -> Self {
        self.ne = Some(value);
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<f64>) -> Self {
        self.ne = value;
        self
    }

    ///Sets `not_in`.
    #[must_use]
    pub fn not_in(mut self, value: Vec<f64>) -> Self {
        self.not_in = value;
        self
    }
}

/// `PlacesResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `admin1name` | [`String`] |  | Administrative level 1 name (state/province) |
/// | `admin2name` | [`String`] |  | Administrative level 2 name (county/district) |
/// | `latitude` | `f64` | ✓ | Latitude coordinate |
/// | `longitude` | `f64` | ✓ | Longitude coordinate |
/// | `name` | [`String`] | ✓ | Place name |
///
/// See: <https://api.immich.app/models/PlacesResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PlacesResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub admin1name: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub admin2name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
}

impl PlacesResponseDTO {
    ///Creates a new [`PlacesResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(latitude: f64, longitude: f64, name: impl Into<String>) -> Self {
        Self {
            admin1name: None,
            admin2name: None,
            latitude,
            longitude,
            name: name.into(),
        }
    }

    ///Sets `admin1name`.
    #[must_use]
    pub fn admin1name(mut self, value: impl Into<String>) -> Self {
        self.admin1name = Some(value.into());
        self
    }

    ///Sets or clears `admin1name`.
    #[must_use]
    pub fn set_admin1name(mut self, value: Option<String>) -> Self {
        self.admin1name = value;
        self
    }

    ///Sets `admin2name`.
    #[must_use]
    pub fn admin2name(mut self, value: impl Into<String>) -> Self {
        self.admin2name = Some(value.into());
        self
    }

    ///Sets or clears `admin2name`.
    #[must_use]
    pub fn set_admin2name(mut self, value: Option<String>) -> Self {
        self.admin2name = value;
        self
    }

    ///Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = value;
        self
    }

    ///Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
}

/// `RandomSearchDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by album IDs |
/// | `city` | [`String`] |  | Filter by city name |
/// | `country` | [`String`] |  | Filter by country name |
/// | `created_after` | [`DateTime`](::chrono::DateTime) |  | Filter by creation date (after) |
/// | `created_before` | [`DateTime`](::chrono::DateTime) |  | Filter by creation date (before) |
/// | `filter` | [`SearchFilter`] |  |  |
/// | `is_encoded` | `bool` |  | Filter by encoded status |
/// | `is_favorite` | `bool` |  | Filter by favorite status |
/// | `is_motion` | `bool` |  | Filter by motion photo status |
/// | `is_not_in_album` | `bool` |  | Filter assets not in any album |
/// | `is_offline` | `bool` |  | Filter by offline status |
/// | `lens_model` | [`String`] |  | Filter by lens model |
/// | `library_id` | [`Uuid`](::uuid::Uuid) |  | Library ID to filter by |
/// | `make` | [`String`] |  | Filter by camera make |
/// | `model` | [`String`] |  | Filter by camera model |
/// | `ocr` | [`String`] |  | Filter by OCR text content |
/// | `person_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by person IDs |
/// | `rating` | `i64` |  | Filter by rating [1-5], or null for unrated |
/// | `size` | `i64` |  | Number of results to return |
/// | `state` | [`String`] |  | Filter by state/province name |
/// | `tag_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by tag IDs |
/// | `taken_after` | [`DateTime`](::chrono::DateTime) |  | Filter by taken date (after) |
/// | `taken_before` | [`DateTime`](::chrono::DateTime) |  | Filter by taken date (before) |
/// | `trashed_after` | [`DateTime`](::chrono::DateTime) |  | Filter by trash date (after) |
/// | `trashed_before` | [`DateTime`](::chrono::DateTime) |  | Filter by trash date (before) |
/// | `type` | [`AssetTypeEnum`] |  |  |
/// | `updated_after` | [`DateTime`](::chrono::DateTime) |  | Filter by update date (after) |
/// | `updated_before` | [`DateTime`](::chrono::DateTime) |  | Filter by update date (before) |
/// | `visibility` | [`AssetVisibility`] |  |  |
/// | `with_deleted` | `bool` |  | Include deleted assets |
/// | `with_exif` | `bool` |  | Include EXIF data in response |
/// | `with_people` | `bool` |  | Include people data in response |
/// | `with_stacked` | `bool` |  | Include stacked assets |
///
/// See: <https://api.immich.app/models/RandomSearchDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct RandomSearchDTO {
    /// Filter by album IDs
    #[serde(rename = "albumIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub album_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: Option<String>,
    /// Filter by creation date (after)
    #[serde(
        rename = "createdAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by creation date (before)
    #[serde(
        rename = "createdBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filter: Option<SearchFilter>,
    /// Filter by encoded status
    #[serde(
        rename = "isEncoded",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_encoded: Option<bool>,
    /// Filter by favorite status
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Filter by motion photo status
    #[serde(rename = "isMotion", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_motion: Option<bool>,
    /// Filter assets not in any album
    #[serde(
        rename = "isNotInAlbum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_not_in_album: Option<bool>,
    /// Filter by offline status
    #[serde(
        rename = "isOffline",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_offline: Option<bool>,
    /// Filter by lens model
    #[serde(
        rename = "lensModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lens_model: Option<String>,
    /// Library ID to filter by
    #[serde(
        rename = "libraryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub library_id: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub make: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ocr: Option<String>,
    /// Filter by person IDs
    #[serde(rename = "personIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub person_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<::std::num::NonZeroU64>,
    #[serde(default = "crate::common::defaults::default_nzu64::<::std::num::NonZeroU64, 250>")]
    pub size: ::std::num::NonZeroU64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
    /// Filter by tag IDs
    #[serde(rename = "tagIds", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag_ids: Option<Vec<::uuid::Uuid>>,
    /// Filter by taken date (after)
    #[serde(
        rename = "takenAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taken_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by taken date (before)
    #[serde(
        rename = "takenBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taken_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by trash date (after)
    #[serde(
        rename = "trashedAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by trash date (before)
    #[serde(
        rename = "trashedBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<AssetTypeEnum>,
    /// Filter by update date (after)
    #[serde(
        rename = "updatedAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by update date (before)
    #[serde(
        rename = "updatedBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<AssetVisibility>,
    /// Include deleted assets
    #[serde(
        rename = "withDeleted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub with_deleted: Option<bool>,
    /// Include EXIF data in response
    #[serde(rename = "withExif", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub with_exif: Option<bool>,
    /// Include people data in response
    #[serde(
        rename = "withPeople",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub with_people: Option<bool>,
    /// Include stacked assets
    #[serde(
        rename = "withStacked",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub with_stacked: Option<bool>,
}

impl RandomSearchDTO {
    ///Creates a new [`RandomSearchDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_ids: Vec::new(),
            city: None,
            country: None,
            created_after: None,
            created_before: None,
            filter: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_not_in_album: None,
            is_offline: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            ocr: None,
            person_ids: Vec::new(),
            rating: None,
            size: crate::common::defaults::default_nzu64::<::std::num::NonZeroU64, 250>(),
            state: None,
            tag_ids: None,
            taken_after: None,
            taken_before: None,
            trashed_after: None,
            trashed_before: None,
            type_: None,
            updated_after: None,
            updated_before: None,
            visibility: None,
            with_deleted: None,
            with_exif: None,
            with_people: None,
            with_stacked: None,
        }
    }

    ///Sets `album_ids`.
    #[must_use]
    pub fn album_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.album_ids = value;
        self
    }

    ///Sets `city`.
    #[must_use]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    ///Sets or clears `city`.
    #[must_use]
    pub fn set_city(mut self, value: Option<String>) -> Self {
        self.city = value;
        self
    }

    ///Sets `country`.
    #[must_use]
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    ///Sets or clears `country`.
    #[must_use]
    pub fn set_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }

    ///Sets `created_after`.
    #[must_use]
    pub fn created_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_after = Some(value);
        self
    }

    ///Sets or clears `created_after`.
    #[must_use]
    pub fn set_created_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_after = value;
        self
    }

    ///Sets `created_before`.
    #[must_use]
    pub fn created_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_before = Some(value);
        self
    }

    ///Sets or clears `created_before`.
    #[must_use]
    pub fn set_created_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_before = value;
        self
    }

    ///Sets `filter`.
    #[must_use]
    pub fn filter(mut self, value: SearchFilter) -> Self {
        self.filter = Some(value);
        self
    }

    ///Sets or clears `filter`.
    #[must_use]
    pub fn set_filter(mut self, value: Option<SearchFilter>) -> Self {
        self.filter = value;
        self
    }

    ///Sets `is_encoded`.
    #[must_use]
    pub fn is_encoded(mut self, value: bool) -> Self {
        self.is_encoded = Some(value);
        self
    }

    ///Sets or clears `is_encoded`.
    #[must_use]
    pub fn set_is_encoded(mut self, value: Option<bool>) -> Self {
        self.is_encoded = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    ///Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `is_motion`.
    #[must_use]
    pub fn is_motion(mut self, value: bool) -> Self {
        self.is_motion = Some(value);
        self
    }

    ///Sets or clears `is_motion`.
    #[must_use]
    pub fn set_is_motion(mut self, value: Option<bool>) -> Self {
        self.is_motion = value;
        self
    }

    ///Sets `is_not_in_album`.
    #[must_use]
    pub fn is_not_in_album(mut self, value: bool) -> Self {
        self.is_not_in_album = Some(value);
        self
    }

    ///Sets or clears `is_not_in_album`.
    #[must_use]
    pub fn set_is_not_in_album(mut self, value: Option<bool>) -> Self {
        self.is_not_in_album = value;
        self
    }

    ///Sets `is_offline`.
    #[must_use]
    pub fn is_offline(mut self, value: bool) -> Self {
        self.is_offline = Some(value);
        self
    }

    ///Sets or clears `is_offline`.
    #[must_use]
    pub fn set_is_offline(mut self, value: Option<bool>) -> Self {
        self.is_offline = value;
        self
    }

    ///Sets `lens_model`.
    #[must_use]
    pub fn lens_model(mut self, value: impl Into<String>) -> Self {
        self.lens_model = Some(value.into());
        self
    }

    ///Sets or clears `lens_model`.
    #[must_use]
    pub fn set_lens_model(mut self, value: Option<String>) -> Self {
        self.lens_model = value;
        self
    }

    ///Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: ::uuid::Uuid) -> Self {
        self.library_id = Some(value);
        self
    }

    ///Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.library_id = value;
        self
    }

    ///Sets `make`.
    #[must_use]
    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }

    ///Sets or clears `make`.
    #[must_use]
    pub fn set_make(mut self, value: Option<String>) -> Self {
        self.make = value;
        self
    }

    ///Sets `model`.
    #[must_use]
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    ///Sets or clears `model`.
    #[must_use]
    pub fn set_model(mut self, value: Option<String>) -> Self {
        self.model = value;
        self
    }

    ///Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: impl Into<String>) -> Self {
        self.ocr = Some(value.into());
        self
    }

    ///Sets or clears `ocr`.
    #[must_use]
    pub fn set_ocr(mut self, value: Option<String>) -> Self {
        self.ocr = value;
        self
    }

    ///Sets `person_ids`.
    #[must_use]
    pub fn person_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.person_ids = value;
        self
    }

    ///Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.rating = Some(value);
        self
    }

    ///Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.rating = value;
        self
    }

    ///Sets `size`.
    #[must_use]
    pub fn size(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.size = value;
        self
    }

    ///Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    ///Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }

    ///Sets `tag_ids`.
    #[must_use]
    pub fn tag_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    ///Sets or clears `tag_ids`.
    #[must_use]
    pub fn set_tag_ids(mut self, value: Option<Vec<::uuid::Uuid>>) -> Self {
        self.tag_ids = value;
        self
    }

    ///Sets `taken_after`.
    #[must_use]
    pub fn taken_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.taken_after = Some(value);
        self
    }

    ///Sets or clears `taken_after`.
    #[must_use]
    pub fn set_taken_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.taken_after = value;
        self
    }

    ///Sets `taken_before`.
    #[must_use]
    pub fn taken_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.taken_before = Some(value);
        self
    }

    ///Sets or clears `taken_before`.
    #[must_use]
    pub fn set_taken_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.taken_before = value;
        self
    }

    ///Sets `trashed_after`.
    #[must_use]
    pub fn trashed_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.trashed_after = Some(value);
        self
    }

    ///Sets or clears `trashed_after`.
    #[must_use]
    pub fn set_trashed_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.trashed_after = value;
        self
    }

    ///Sets `trashed_before`.
    #[must_use]
    pub fn trashed_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.trashed_before = Some(value);
        self
    }

    ///Sets or clears `trashed_before`.
    #[must_use]
    pub fn set_trashed_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.trashed_before = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = Some(value);
        self
    }

    ///Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<AssetTypeEnum>) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_after`.
    #[must_use]
    pub fn updated_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_after = Some(value);
        self
    }

    ///Sets or clears `updated_after`.
    #[must_use]
    pub fn set_updated_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_after = value;
        self
    }

    ///Sets `updated_before`.
    #[must_use]
    pub fn updated_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_before = Some(value);
        self
    }

    ///Sets or clears `updated_before`.
    #[must_use]
    pub fn set_updated_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_before = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    ///Sets or clears `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }

    ///Sets `with_deleted`.
    #[must_use]
    pub fn with_deleted(mut self, value: bool) -> Self {
        self.with_deleted = Some(value);
        self
    }

    ///Sets or clears `with_deleted`.
    #[must_use]
    pub fn set_with_deleted(mut self, value: Option<bool>) -> Self {
        self.with_deleted = value;
        self
    }

    ///Sets `with_exif`.
    #[must_use]
    pub fn with_exif(mut self, value: bool) -> Self {
        self.with_exif = Some(value);
        self
    }

    ///Sets or clears `with_exif`.
    #[must_use]
    pub fn set_with_exif(mut self, value: Option<bool>) -> Self {
        self.with_exif = value;
        self
    }

    ///Sets `with_people`.
    #[must_use]
    pub fn with_people(mut self, value: bool) -> Self {
        self.with_people = Some(value);
        self
    }

    ///Sets or clears `with_people`.
    #[must_use]
    pub fn set_with_people(mut self, value: Option<bool>) -> Self {
        self.with_people = value;
        self
    }

    ///Sets `with_stacked`.
    #[must_use]
    pub fn with_stacked(mut self, value: bool) -> Self {
        self.with_stacked = Some(value);
        self
    }

    ///Sets or clears `with_stacked`.
    #[must_use]
    pub fn set_with_stacked(mut self, value: Option<bool>) -> Self {
        self.with_stacked = value;
        self
    }
}

impl ::std::default::Default for RandomSearchDTO {
    fn default() -> Self {
        Self {
            album_ids: Vec::new(),
            city: None,
            country: None,
            created_after: None,
            created_before: None,
            filter: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_not_in_album: None,
            is_offline: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            ocr: None,
            person_ids: Vec::new(),
            rating: None,
            size: crate::common::defaults::default_nzu64::<::std::num::NonZeroU64, 250>(),
            state: None,
            tag_ids: None,
            taken_after: None,
            taken_before: None,
            trashed_after: None,
            trashed_before: None,
            type_: None,
            updated_after: None,
            updated_before: None,
            visibility: None,
            with_deleted: None,
            with_exif: None,
            with_people: None,
            with_stacked: None,
        }
    }
}

/// `SearchAlbumResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `count` | `i64` | ✓ | Number of albums in this page |
/// | `facets` | [`Vec<SearchFacetResponseDTO>`](crate::search::SearchFacetResponseDTO) | ✓ |  |
/// | `items` | [`Vec<AlbumResponseDTO>`](crate::albums::AlbumResponseDTO) | ✓ |  |
/// | `total` | `i64` | ✓ | Total number of matching albums |
///
/// See: <https://api.immich.app/models/SearchAlbumResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SearchAlbumResponseDTO {
    pub count: i64,
    pub facets: Vec<SearchFacetResponseDTO>,
    pub items: Vec<AlbumResponseDTO>,
    pub total: i64,
}

impl SearchAlbumResponseDTO {
    ///Creates a new [`SearchAlbumResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(count: i64, facets: Vec<SearchFacetResponseDTO>, items: Vec<AlbumResponseDTO>, total: i64) -> Self {
        Self {
            count,
            facets,
            items,
            total,
        }
    }

    ///Sets `count`.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = value;
        self
    }

    ///Sets `facets`.
    #[must_use]
    pub fn facets(mut self, value: Vec<SearchFacetResponseDTO>) -> Self {
        self.facets = value;
        self
    }

    ///Sets `items`.
    #[must_use]
    pub fn items(mut self, value: Vec<AlbumResponseDTO>) -> Self {
        self.items = value;
        self
    }

    ///Sets `total`.
    #[must_use]
    pub fn total(mut self, value: i64) -> Self {
        self.total = value;
        self
    }
}

/// `SearchAssetResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `count` | `i64` | ✓ | Number of assets in this page |
/// | `facets` | [`Vec<SearchFacetResponseDTO>`](crate::search::SearchFacetResponseDTO) | ✓ |  |
/// | `items` | [`Vec<AssetResponseDTO>`](crate::assets::AssetResponseDTO) | ✓ |  |
/// | `next_cursor` | [`String`] | ✓ | Cursor for the next page of results |
/// | `next_page` | [`String`] | ✓ | Next page token |
/// | `total` | `i64` | ✓ | Total number of matching assets |
///
/// See: <https://api.immich.app/models/SearchAssetResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SearchAssetResponseDTO {
    pub count: i64,
    pub facets: Vec<SearchFacetResponseDTO>,
    pub items: Vec<AssetResponseDTO>,
    /// Cursor for the next page of results (required)
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Next page token (required)
    #[serde(rename = "nextPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    pub total: i64,
}

impl SearchAssetResponseDTO {
    ///Creates a new [`SearchAssetResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(count: i64, facets: Vec<SearchFacetResponseDTO>, items: Vec<AssetResponseDTO>, total: i64) -> Self {
        Self {
            count,
            facets,
            items,
            next_cursor: None,
            next_page: None,
            total,
        }
    }

    ///Sets `count`.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = value;
        self
    }

    ///Sets `facets`.
    #[must_use]
    pub fn facets(mut self, value: Vec<SearchFacetResponseDTO>) -> Self {
        self.facets = value;
        self
    }

    ///Sets `items`.
    #[must_use]
    pub fn items(mut self, value: Vec<AssetResponseDTO>) -> Self {
        self.items = value;
        self
    }

    ///Sets `next_cursor`.
    #[must_use]
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    ///Sets or clears `next_cursor`.
    #[must_use]
    pub fn set_next_cursor(mut self, value: Option<String>) -> Self {
        self.next_cursor = value;
        self
    }

    ///Sets `next_page`.
    #[must_use]
    pub fn next_page(mut self, value: impl Into<String>) -> Self {
        self.next_page = Some(value.into());
        self
    }

    ///Sets or clears `next_page`.
    #[must_use]
    pub fn set_next_page(mut self, value: Option<String>) -> Self {
        self.next_page = value;
        self
    }

    ///Sets `total`.
    #[must_use]
    pub fn total(mut self, value: i64) -> Self {
        self.total = value;
        self
    }
}

/// `SearchExploreItem`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `data` | [`AssetResponseDTO`] | ✓ |  |
/// | `value` | [`String`] | ✓ | Explore value |
///
/// See: <https://api.immich.app/models/SearchExploreItem>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SearchExploreItem {
    pub data: AssetResponseDTO,
    pub value: String,
}

impl SearchExploreItem {
    ///Creates a new [`SearchExploreItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(data: AssetResponseDTO, value: impl Into<String>) -> Self {
        Self {
            data,
            value: value.into(),
        }
    }

    ///Sets `data`.
    #[must_use]
    pub fn data(mut self, value: AssetResponseDTO) -> Self {
        self.data = value;
        self
    }

    ///Sets `value`.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
}

/// `SearchExploreResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `field_name` | [`String`] | ✓ | Explore field name |
/// | `items` | [`Vec<SearchExploreItem>`](crate::search::SearchExploreItem) | ✓ |  |
///
/// See: <https://api.immich.app/models/SearchExploreResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SearchExploreResponseDTO {
    /// Explore field name (required)
    #[serde(rename = "fieldName")]
    pub field_name: String,
    pub items: Vec<SearchExploreItem>,
}

impl SearchExploreResponseDTO {
    ///Creates a new [`SearchExploreResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(field_name: impl Into<String>, items: Vec<SearchExploreItem>) -> Self {
        Self {
            field_name: field_name.into(),
            items,
        }
    }

    ///Sets `field_name`.
    #[must_use]
    pub fn field_name(mut self, value: impl Into<String>) -> Self {
        self.field_name = value.into();
        self
    }

    ///Sets `items`.
    #[must_use]
    pub fn items(mut self, value: Vec<SearchExploreItem>) -> Self {
        self.items = value;
        self
    }
}

/// `SearchFacetCountResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `count` | `i64` | ✓ | Number of assets with this facet value |
/// | `value` | [`String`] | ✓ | Facet value |
///
/// See: <https://api.immich.app/models/SearchFacetCountResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SearchFacetCountResponseDTO {
    pub count: i64,
    pub value: String,
}

impl SearchFacetCountResponseDTO {
    ///Creates a new [`SearchFacetCountResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(count: i64, value: impl Into<String>) -> Self {
        Self {
            count,
            value: value.into(),
        }
    }

    ///Sets `count`.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = value;
        self
    }

    ///Sets `value`.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
}

/// `SearchFacetResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `counts` | [`Vec<SearchFacetCountResponseDTO>`](crate::search::SearchFacetCountResponseDTO) | ✓ |  |
/// | `field_name` | [`String`] | ✓ | Facet field name |
///
/// See: <https://api.immich.app/models/SearchFacetResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SearchFacetResponseDTO {
    pub counts: Vec<SearchFacetCountResponseDTO>,
    /// Facet field name (required)
    #[serde(rename = "fieldName")]
    pub field_name: String,
}

impl SearchFacetResponseDTO {
    ///Creates a new [`SearchFacetResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(counts: Vec<SearchFacetCountResponseDTO>, field_name: impl Into<String>) -> Self {
        Self {
            counts,
            field_name: field_name.into(),
        }
    }

    ///Sets `counts`.
    #[must_use]
    pub fn counts(mut self, value: Vec<SearchFacetCountResponseDTO>) -> Self {
        self.counts = value;
        self
    }

    ///Sets `field_name`.
    #[must_use]
    pub fn field_name(mut self, value: impl Into<String>) -> Self {
        self.field_name = value.into();
        self
    }
}

/// `SearchFilter`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_ids` | [`IdsFilter`] |  |  |
/// | `checksum` | [`StringFilter`] |  |  |
/// | `city` | [`StringFilterNullable`] |  |  |
/// | `country` | [`StringFilterNullable`] |  |  |
/// | `created_at` | [`DateFilter`] |  |  |
/// | `description` | [`StringPatternFilter`] |  |  |
/// | `encoded_video_path` | [`StringFilter`] |  |  |
/// | `file_size_in_bytes` | [`NumberFilter`] |  |  |
/// | `has_albums` | [`BoolFilter`] |  |  |
/// | `has_people` | [`BoolFilter`] |  |  |
/// | `has_tags` | [`BoolFilter`] |  |  |
/// | `id` | [`IdFilter`] |  |  |
/// | `is_encoded` | [`BoolFilter`] |  |  |
/// | `is_favorite` | [`BoolFilter`] |  |  |
/// | `is_motion` | [`BoolFilter`] |  |  |
/// | `is_offline` | [`BoolFilter`] |  |  |
/// | `lens_model` | [`StringFilterNullable`] |  |  |
/// | `library_id` | [`IdFilterNullable`] |  |  |
/// | `make` | [`StringFilterNullable`] |  |  |
/// | `model` | [`StringFilterNullable`] |  |  |
/// | `ocr` | [`StringSimilarityFilter`] |  |  |
/// | `or` | [`Vec<SearchFilterBranch>`](crate::search::SearchFilterBranch) |  |  |
/// | `original_file_name` | [`StringPatternFilter`] |  |  |
/// | `original_path` | [`StringPatternFilter`] |  |  |
/// | `person_ids` | [`IdsFilter`] |  |  |
/// | `rating` | [`NumberFilterNullable`] |  |  |
/// | `state` | [`StringFilterNullable`] |  |  |
/// | `tag_ids` | [`IdsFilter`] |  |  |
/// | `taken_at` | [`DateFilter`] |  |  |
/// | `trashed_at` | [`DateFilterNullable`] |  |  |
/// | `type` | [`EnumFilterAssetType`] |  |  |
/// | `updated_at` | [`DateFilter`] |  |  |
/// | `visibility` | [`EnumFilterAssetVisibility`] |  |  |
///
/// See: <https://api.immich.app/models/SearchFilter>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct SearchFilter {
    #[serde(rename = "albumIds", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub album_ids: Option<IdsFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub checksum: Option<StringFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: Option<StringFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: Option<StringFilterNullable>,
    #[serde(
        rename = "createdAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_at: Option<DateFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<StringPatternFilter>,
    #[serde(
        rename = "encodedVideoPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub encoded_video_path: Option<StringFilter>,
    #[serde(
        rename = "fileSizeInBytes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_size_in_bytes: Option<NumberFilter>,
    #[serde(
        rename = "hasAlbums",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_albums: Option<BoolFilter>,
    #[serde(
        rename = "hasPeople",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_people: Option<BoolFilter>,
    #[serde(rename = "hasTags", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub has_tags: Option<BoolFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: Option<IdFilter>,
    #[serde(
        rename = "isEncoded",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_encoded: Option<BoolFilter>,
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<BoolFilter>,
    #[serde(rename = "isMotion", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_motion: Option<BoolFilter>,
    #[serde(
        rename = "isOffline",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_offline: Option<BoolFilter>,
    #[serde(
        rename = "lensModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lens_model: Option<StringFilterNullable>,
    #[serde(
        rename = "libraryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub library_id: Option<IdFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub make: Option<StringFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: Option<StringFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ocr: Option<StringSimilarityFilter>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub or: Vec<SearchFilterBranch>,
    #[serde(
        rename = "originalFileName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_file_name: Option<StringPatternFilter>,
    #[serde(
        rename = "originalPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_path: Option<StringPatternFilter>,
    #[serde(
        rename = "personIds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub person_ids: Option<IdsFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<NumberFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<StringFilterNullable>,
    #[serde(rename = "tagIds", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag_ids: Option<IdsFilter>,
    #[serde(rename = "takenAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub taken_at: Option<DateFilter>,
    #[serde(
        rename = "trashedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_at: Option<DateFilterNullable>,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<EnumFilterAssetType>,
    #[serde(
        rename = "updatedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_at: Option<DateFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<EnumFilterAssetVisibility>,
}

impl SearchFilter {
    ///Creates a new [`SearchFilter`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_ids: None,
            checksum: None,
            city: None,
            country: None,
            created_at: None,
            description: None,
            encoded_video_path: None,
            file_size_in_bytes: None,
            has_albums: None,
            has_people: None,
            has_tags: None,
            id: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_offline: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            ocr: None,
            or: Vec::new(),
            original_file_name: None,
            original_path: None,
            person_ids: None,
            rating: None,
            state: None,
            tag_ids: None,
            taken_at: None,
            trashed_at: None,
            type_: None,
            updated_at: None,
            visibility: None,
        }
    }

    ///Sets `album_ids`.
    #[must_use]
    pub fn album_ids(mut self, value: IdsFilter) -> Self {
        self.album_ids = Some(value);
        self
    }

    ///Sets or clears `album_ids`.
    #[must_use]
    pub fn set_album_ids(mut self, value: Option<IdsFilter>) -> Self {
        self.album_ids = value;
        self
    }

    ///Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: StringFilter) -> Self {
        self.checksum = Some(value);
        self
    }

    ///Sets or clears `checksum`.
    #[must_use]
    pub fn set_checksum(mut self, value: Option<StringFilter>) -> Self {
        self.checksum = value;
        self
    }

    ///Sets `city`.
    #[must_use]
    pub fn city(mut self, value: StringFilterNullable) -> Self {
        self.city = Some(value);
        self
    }

    ///Sets or clears `city`.
    #[must_use]
    pub fn set_city(mut self, value: Option<StringFilterNullable>) -> Self {
        self.city = value;
        self
    }

    ///Sets `country`.
    #[must_use]
    pub fn country(mut self, value: StringFilterNullable) -> Self {
        self.country = Some(value);
        self
    }

    ///Sets or clears `country`.
    #[must_use]
    pub fn set_country(mut self, value: Option<StringFilterNullable>) -> Self {
        self.country = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: DateFilter) -> Self {
        self.created_at = Some(value);
        self
    }

    ///Sets or clears `created_at`.
    #[must_use]
    pub fn set_created_at(mut self, value: Option<DateFilter>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: StringPatternFilter) -> Self {
        self.description = Some(value);
        self
    }

    ///Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<StringPatternFilter>) -> Self {
        self.description = value;
        self
    }

    ///Sets `encoded_video_path`.
    #[must_use]
    pub fn encoded_video_path(mut self, value: StringFilter) -> Self {
        self.encoded_video_path = Some(value);
        self
    }

    ///Sets or clears `encoded_video_path`.
    #[must_use]
    pub fn set_encoded_video_path(mut self, value: Option<StringFilter>) -> Self {
        self.encoded_video_path = value;
        self
    }

    ///Sets `file_size_in_bytes`.
    #[must_use]
    pub fn file_size_in_bytes(mut self, value: NumberFilter) -> Self {
        self.file_size_in_bytes = Some(value);
        self
    }

    ///Sets or clears `file_size_in_bytes`.
    #[must_use]
    pub fn set_file_size_in_bytes(mut self, value: Option<NumberFilter>) -> Self {
        self.file_size_in_bytes = value;
        self
    }

    ///Sets `has_albums`.
    #[must_use]
    pub fn has_albums(mut self, value: BoolFilter) -> Self {
        self.has_albums = Some(value);
        self
    }

    ///Sets or clears `has_albums`.
    #[must_use]
    pub fn set_has_albums(mut self, value: Option<BoolFilter>) -> Self {
        self.has_albums = value;
        self
    }

    ///Sets `has_people`.
    #[must_use]
    pub fn has_people(mut self, value: BoolFilter) -> Self {
        self.has_people = Some(value);
        self
    }

    ///Sets or clears `has_people`.
    #[must_use]
    pub fn set_has_people(mut self, value: Option<BoolFilter>) -> Self {
        self.has_people = value;
        self
    }

    ///Sets `has_tags`.
    #[must_use]
    pub fn has_tags(mut self, value: BoolFilter) -> Self {
        self.has_tags = Some(value);
        self
    }

    ///Sets or clears `has_tags`.
    #[must_use]
    pub fn set_has_tags(mut self, value: Option<BoolFilter>) -> Self {
        self.has_tags = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: IdFilter) -> Self {
        self.id = Some(value);
        self
    }

    ///Sets or clears `id`.
    #[must_use]
    pub fn set_id(mut self, value: Option<IdFilter>) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_encoded`.
    #[must_use]
    pub fn is_encoded(mut self, value: BoolFilter) -> Self {
        self.is_encoded = Some(value);
        self
    }

    ///Sets or clears `is_encoded`.
    #[must_use]
    pub fn set_is_encoded(mut self, value: Option<BoolFilter>) -> Self {
        self.is_encoded = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: BoolFilter) -> Self {
        self.is_favorite = Some(value);
        self
    }

    ///Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<BoolFilter>) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `is_motion`.
    #[must_use]
    pub fn is_motion(mut self, value: BoolFilter) -> Self {
        self.is_motion = Some(value);
        self
    }

    ///Sets or clears `is_motion`.
    #[must_use]
    pub fn set_is_motion(mut self, value: Option<BoolFilter>) -> Self {
        self.is_motion = value;
        self
    }

    ///Sets `is_offline`.
    #[must_use]
    pub fn is_offline(mut self, value: BoolFilter) -> Self {
        self.is_offline = Some(value);
        self
    }

    ///Sets or clears `is_offline`.
    #[must_use]
    pub fn set_is_offline(mut self, value: Option<BoolFilter>) -> Self {
        self.is_offline = value;
        self
    }

    ///Sets `lens_model`.
    #[must_use]
    pub fn lens_model(mut self, value: StringFilterNullable) -> Self {
        self.lens_model = Some(value);
        self
    }

    ///Sets or clears `lens_model`.
    #[must_use]
    pub fn set_lens_model(mut self, value: Option<StringFilterNullable>) -> Self {
        self.lens_model = value;
        self
    }

    ///Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: IdFilterNullable) -> Self {
        self.library_id = Some(value);
        self
    }

    ///Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<IdFilterNullable>) -> Self {
        self.library_id = value;
        self
    }

    ///Sets `make`.
    #[must_use]
    pub fn make(mut self, value: StringFilterNullable) -> Self {
        self.make = Some(value);
        self
    }

    ///Sets or clears `make`.
    #[must_use]
    pub fn set_make(mut self, value: Option<StringFilterNullable>) -> Self {
        self.make = value;
        self
    }

    ///Sets `model`.
    #[must_use]
    pub fn model(mut self, value: StringFilterNullable) -> Self {
        self.model = Some(value);
        self
    }

    ///Sets or clears `model`.
    #[must_use]
    pub fn set_model(mut self, value: Option<StringFilterNullable>) -> Self {
        self.model = value;
        self
    }

    ///Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: StringSimilarityFilter) -> Self {
        self.ocr = Some(value);
        self
    }

    ///Sets or clears `ocr`.
    #[must_use]
    pub fn set_ocr(mut self, value: Option<StringSimilarityFilter>) -> Self {
        self.ocr = value;
        self
    }

    ///Sets `or`.
    #[must_use]
    pub fn or(mut self, value: Vec<SearchFilterBranch>) -> Self {
        self.or = value;
        self
    }

    ///Sets `original_file_name`.
    #[must_use]
    pub fn original_file_name(mut self, value: StringPatternFilter) -> Self {
        self.original_file_name = Some(value);
        self
    }

    ///Sets or clears `original_file_name`.
    #[must_use]
    pub fn set_original_file_name(mut self, value: Option<StringPatternFilter>) -> Self {
        self.original_file_name = value;
        self
    }

    ///Sets `original_path`.
    #[must_use]
    pub fn original_path(mut self, value: StringPatternFilter) -> Self {
        self.original_path = Some(value);
        self
    }

    ///Sets or clears `original_path`.
    #[must_use]
    pub fn set_original_path(mut self, value: Option<StringPatternFilter>) -> Self {
        self.original_path = value;
        self
    }

    ///Sets `person_ids`.
    #[must_use]
    pub fn person_ids(mut self, value: IdsFilter) -> Self {
        self.person_ids = Some(value);
        self
    }

    ///Sets or clears `person_ids`.
    #[must_use]
    pub fn set_person_ids(mut self, value: Option<IdsFilter>) -> Self {
        self.person_ids = value;
        self
    }

    ///Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: NumberFilterNullable) -> Self {
        self.rating = Some(value);
        self
    }

    ///Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<NumberFilterNullable>) -> Self {
        self.rating = value;
        self
    }

    ///Sets `state`.
    #[must_use]
    pub fn state(mut self, value: StringFilterNullable) -> Self {
        self.state = Some(value);
        self
    }

    ///Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<StringFilterNullable>) -> Self {
        self.state = value;
        self
    }

    ///Sets `tag_ids`.
    #[must_use]
    pub fn tag_ids(mut self, value: IdsFilter) -> Self {
        self.tag_ids = Some(value);
        self
    }

    ///Sets or clears `tag_ids`.
    #[must_use]
    pub fn set_tag_ids(mut self, value: Option<IdsFilter>) -> Self {
        self.tag_ids = value;
        self
    }

    ///Sets `taken_at`.
    #[must_use]
    pub fn taken_at(mut self, value: DateFilter) -> Self {
        self.taken_at = Some(value);
        self
    }

    ///Sets or clears `taken_at`.
    #[must_use]
    pub fn set_taken_at(mut self, value: Option<DateFilter>) -> Self {
        self.taken_at = value;
        self
    }

    ///Sets `trashed_at`.
    #[must_use]
    pub fn trashed_at(mut self, value: DateFilterNullable) -> Self {
        self.trashed_at = Some(value);
        self
    }

    ///Sets or clears `trashed_at`.
    #[must_use]
    pub fn set_trashed_at(mut self, value: Option<DateFilterNullable>) -> Self {
        self.trashed_at = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: EnumFilterAssetType) -> Self {
        self.type_ = Some(value);
        self
    }

    ///Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<EnumFilterAssetType>) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: DateFilter) -> Self {
        self.updated_at = Some(value);
        self
    }

    ///Sets or clears `updated_at`.
    #[must_use]
    pub fn set_updated_at(mut self, value: Option<DateFilter>) -> Self {
        self.updated_at = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: EnumFilterAssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    ///Sets or clears `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Option<EnumFilterAssetVisibility>) -> Self {
        self.visibility = value;
        self
    }
}

/// `SearchFilterBranch`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_ids` | [`IdsFilter`] |  |  |
/// | `checksum` | [`StringFilter`] |  |  |
/// | `city` | [`StringFilterNullable`] |  |  |
/// | `country` | [`StringFilterNullable`] |  |  |
/// | `created_at` | [`DateFilter`] |  |  |
/// | `description` | [`StringPatternFilter`] |  |  |
/// | `encoded_video_path` | [`StringFilter`] |  |  |
/// | `file_size_in_bytes` | [`NumberFilter`] |  |  |
/// | `has_albums` | [`BoolFilter`] |  |  |
/// | `has_people` | [`BoolFilter`] |  |  |
/// | `has_tags` | [`BoolFilter`] |  |  |
/// | `id` | [`IdFilter`] |  |  |
/// | `is_encoded` | [`BoolFilter`] |  |  |
/// | `is_favorite` | [`BoolFilter`] |  |  |
/// | `is_motion` | [`BoolFilter`] |  |  |
/// | `is_offline` | [`BoolFilter`] |  |  |
/// | `lens_model` | [`StringFilterNullable`] |  |  |
/// | `library_id` | [`IdFilterNullable`] |  |  |
/// | `make` | [`StringFilterNullable`] |  |  |
/// | `model` | [`StringFilterNullable`] |  |  |
/// | `ocr` | [`StringSimilarityFilter`] |  |  |
/// | `original_file_name` | [`StringPatternFilter`] |  |  |
/// | `original_path` | [`StringPatternFilter`] |  |  |
/// | `person_ids` | [`IdsFilter`] |  |  |
/// | `rating` | [`NumberFilterNullable`] |  |  |
/// | `state` | [`StringFilterNullable`] |  |  |
/// | `tag_ids` | [`IdsFilter`] |  |  |
/// | `taken_at` | [`DateFilter`] |  |  |
/// | `trashed_at` | [`DateFilterNullable`] |  |  |
/// | `type` | [`EnumFilterAssetType`] |  |  |
/// | `updated_at` | [`DateFilter`] |  |  |
/// | `visibility` | [`EnumFilterAssetVisibility`] |  |  |
///
/// See: <https://api.immich.app/models/SearchFilterBranch>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct SearchFilterBranch {
    #[serde(rename = "albumIds", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub album_ids: Option<IdsFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub checksum: Option<StringFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: Option<StringFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: Option<StringFilterNullable>,
    #[serde(
        rename = "createdAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_at: Option<DateFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<StringPatternFilter>,
    #[serde(
        rename = "encodedVideoPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub encoded_video_path: Option<StringFilter>,
    #[serde(
        rename = "fileSizeInBytes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_size_in_bytes: Option<NumberFilter>,
    #[serde(
        rename = "hasAlbums",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_albums: Option<BoolFilter>,
    #[serde(
        rename = "hasPeople",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_people: Option<BoolFilter>,
    #[serde(rename = "hasTags", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub has_tags: Option<BoolFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: Option<IdFilter>,
    #[serde(
        rename = "isEncoded",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_encoded: Option<BoolFilter>,
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<BoolFilter>,
    #[serde(rename = "isMotion", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_motion: Option<BoolFilter>,
    #[serde(
        rename = "isOffline",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_offline: Option<BoolFilter>,
    #[serde(
        rename = "lensModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lens_model: Option<StringFilterNullable>,
    #[serde(
        rename = "libraryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub library_id: Option<IdFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub make: Option<StringFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: Option<StringFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ocr: Option<StringSimilarityFilter>,
    #[serde(
        rename = "originalFileName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_file_name: Option<StringPatternFilter>,
    #[serde(
        rename = "originalPath",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_path: Option<StringPatternFilter>,
    #[serde(
        rename = "personIds",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub person_ids: Option<IdsFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<NumberFilterNullable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<StringFilterNullable>,
    #[serde(rename = "tagIds", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag_ids: Option<IdsFilter>,
    #[serde(rename = "takenAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub taken_at: Option<DateFilter>,
    #[serde(
        rename = "trashedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_at: Option<DateFilterNullable>,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<EnumFilterAssetType>,
    #[serde(
        rename = "updatedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_at: Option<DateFilter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<EnumFilterAssetVisibility>,
}

impl SearchFilterBranch {
    ///Creates a new [`SearchFilterBranch`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_ids: None,
            checksum: None,
            city: None,
            country: None,
            created_at: None,
            description: None,
            encoded_video_path: None,
            file_size_in_bytes: None,
            has_albums: None,
            has_people: None,
            has_tags: None,
            id: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_offline: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            ocr: None,
            original_file_name: None,
            original_path: None,
            person_ids: None,
            rating: None,
            state: None,
            tag_ids: None,
            taken_at: None,
            trashed_at: None,
            type_: None,
            updated_at: None,
            visibility: None,
        }
    }

    ///Sets `album_ids`.
    #[must_use]
    pub fn album_ids(mut self, value: IdsFilter) -> Self {
        self.album_ids = Some(value);
        self
    }

    ///Sets or clears `album_ids`.
    #[must_use]
    pub fn set_album_ids(mut self, value: Option<IdsFilter>) -> Self {
        self.album_ids = value;
        self
    }

    ///Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: StringFilter) -> Self {
        self.checksum = Some(value);
        self
    }

    ///Sets or clears `checksum`.
    #[must_use]
    pub fn set_checksum(mut self, value: Option<StringFilter>) -> Self {
        self.checksum = value;
        self
    }

    ///Sets `city`.
    #[must_use]
    pub fn city(mut self, value: StringFilterNullable) -> Self {
        self.city = Some(value);
        self
    }

    ///Sets or clears `city`.
    #[must_use]
    pub fn set_city(mut self, value: Option<StringFilterNullable>) -> Self {
        self.city = value;
        self
    }

    ///Sets `country`.
    #[must_use]
    pub fn country(mut self, value: StringFilterNullable) -> Self {
        self.country = Some(value);
        self
    }

    ///Sets or clears `country`.
    #[must_use]
    pub fn set_country(mut self, value: Option<StringFilterNullable>) -> Self {
        self.country = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: DateFilter) -> Self {
        self.created_at = Some(value);
        self
    }

    ///Sets or clears `created_at`.
    #[must_use]
    pub fn set_created_at(mut self, value: Option<DateFilter>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: StringPatternFilter) -> Self {
        self.description = Some(value);
        self
    }

    ///Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<StringPatternFilter>) -> Self {
        self.description = value;
        self
    }

    ///Sets `encoded_video_path`.
    #[must_use]
    pub fn encoded_video_path(mut self, value: StringFilter) -> Self {
        self.encoded_video_path = Some(value);
        self
    }

    ///Sets or clears `encoded_video_path`.
    #[must_use]
    pub fn set_encoded_video_path(mut self, value: Option<StringFilter>) -> Self {
        self.encoded_video_path = value;
        self
    }

    ///Sets `file_size_in_bytes`.
    #[must_use]
    pub fn file_size_in_bytes(mut self, value: NumberFilter) -> Self {
        self.file_size_in_bytes = Some(value);
        self
    }

    ///Sets or clears `file_size_in_bytes`.
    #[must_use]
    pub fn set_file_size_in_bytes(mut self, value: Option<NumberFilter>) -> Self {
        self.file_size_in_bytes = value;
        self
    }

    ///Sets `has_albums`.
    #[must_use]
    pub fn has_albums(mut self, value: BoolFilter) -> Self {
        self.has_albums = Some(value);
        self
    }

    ///Sets or clears `has_albums`.
    #[must_use]
    pub fn set_has_albums(mut self, value: Option<BoolFilter>) -> Self {
        self.has_albums = value;
        self
    }

    ///Sets `has_people`.
    #[must_use]
    pub fn has_people(mut self, value: BoolFilter) -> Self {
        self.has_people = Some(value);
        self
    }

    ///Sets or clears `has_people`.
    #[must_use]
    pub fn set_has_people(mut self, value: Option<BoolFilter>) -> Self {
        self.has_people = value;
        self
    }

    ///Sets `has_tags`.
    #[must_use]
    pub fn has_tags(mut self, value: BoolFilter) -> Self {
        self.has_tags = Some(value);
        self
    }

    ///Sets or clears `has_tags`.
    #[must_use]
    pub fn set_has_tags(mut self, value: Option<BoolFilter>) -> Self {
        self.has_tags = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: IdFilter) -> Self {
        self.id = Some(value);
        self
    }

    ///Sets or clears `id`.
    #[must_use]
    pub fn set_id(mut self, value: Option<IdFilter>) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_encoded`.
    #[must_use]
    pub fn is_encoded(mut self, value: BoolFilter) -> Self {
        self.is_encoded = Some(value);
        self
    }

    ///Sets or clears `is_encoded`.
    #[must_use]
    pub fn set_is_encoded(mut self, value: Option<BoolFilter>) -> Self {
        self.is_encoded = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: BoolFilter) -> Self {
        self.is_favorite = Some(value);
        self
    }

    ///Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<BoolFilter>) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `is_motion`.
    #[must_use]
    pub fn is_motion(mut self, value: BoolFilter) -> Self {
        self.is_motion = Some(value);
        self
    }

    ///Sets or clears `is_motion`.
    #[must_use]
    pub fn set_is_motion(mut self, value: Option<BoolFilter>) -> Self {
        self.is_motion = value;
        self
    }

    ///Sets `is_offline`.
    #[must_use]
    pub fn is_offline(mut self, value: BoolFilter) -> Self {
        self.is_offline = Some(value);
        self
    }

    ///Sets or clears `is_offline`.
    #[must_use]
    pub fn set_is_offline(mut self, value: Option<BoolFilter>) -> Self {
        self.is_offline = value;
        self
    }

    ///Sets `lens_model`.
    #[must_use]
    pub fn lens_model(mut self, value: StringFilterNullable) -> Self {
        self.lens_model = Some(value);
        self
    }

    ///Sets or clears `lens_model`.
    #[must_use]
    pub fn set_lens_model(mut self, value: Option<StringFilterNullable>) -> Self {
        self.lens_model = value;
        self
    }

    ///Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: IdFilterNullable) -> Self {
        self.library_id = Some(value);
        self
    }

    ///Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<IdFilterNullable>) -> Self {
        self.library_id = value;
        self
    }

    ///Sets `make`.
    #[must_use]
    pub fn make(mut self, value: StringFilterNullable) -> Self {
        self.make = Some(value);
        self
    }

    ///Sets or clears `make`.
    #[must_use]
    pub fn set_make(mut self, value: Option<StringFilterNullable>) -> Self {
        self.make = value;
        self
    }

    ///Sets `model`.
    #[must_use]
    pub fn model(mut self, value: StringFilterNullable) -> Self {
        self.model = Some(value);
        self
    }

    ///Sets or clears `model`.
    #[must_use]
    pub fn set_model(mut self, value: Option<StringFilterNullable>) -> Self {
        self.model = value;
        self
    }

    ///Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: StringSimilarityFilter) -> Self {
        self.ocr = Some(value);
        self
    }

    ///Sets or clears `ocr`.
    #[must_use]
    pub fn set_ocr(mut self, value: Option<StringSimilarityFilter>) -> Self {
        self.ocr = value;
        self
    }

    ///Sets `original_file_name`.
    #[must_use]
    pub fn original_file_name(mut self, value: StringPatternFilter) -> Self {
        self.original_file_name = Some(value);
        self
    }

    ///Sets or clears `original_file_name`.
    #[must_use]
    pub fn set_original_file_name(mut self, value: Option<StringPatternFilter>) -> Self {
        self.original_file_name = value;
        self
    }

    ///Sets `original_path`.
    #[must_use]
    pub fn original_path(mut self, value: StringPatternFilter) -> Self {
        self.original_path = Some(value);
        self
    }

    ///Sets or clears `original_path`.
    #[must_use]
    pub fn set_original_path(mut self, value: Option<StringPatternFilter>) -> Self {
        self.original_path = value;
        self
    }

    ///Sets `person_ids`.
    #[must_use]
    pub fn person_ids(mut self, value: IdsFilter) -> Self {
        self.person_ids = Some(value);
        self
    }

    ///Sets or clears `person_ids`.
    #[must_use]
    pub fn set_person_ids(mut self, value: Option<IdsFilter>) -> Self {
        self.person_ids = value;
        self
    }

    ///Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: NumberFilterNullable) -> Self {
        self.rating = Some(value);
        self
    }

    ///Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<NumberFilterNullable>) -> Self {
        self.rating = value;
        self
    }

    ///Sets `state`.
    #[must_use]
    pub fn state(mut self, value: StringFilterNullable) -> Self {
        self.state = Some(value);
        self
    }

    ///Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<StringFilterNullable>) -> Self {
        self.state = value;
        self
    }

    ///Sets `tag_ids`.
    #[must_use]
    pub fn tag_ids(mut self, value: IdsFilter) -> Self {
        self.tag_ids = Some(value);
        self
    }

    ///Sets or clears `tag_ids`.
    #[must_use]
    pub fn set_tag_ids(mut self, value: Option<IdsFilter>) -> Self {
        self.tag_ids = value;
        self
    }

    ///Sets `taken_at`.
    #[must_use]
    pub fn taken_at(mut self, value: DateFilter) -> Self {
        self.taken_at = Some(value);
        self
    }

    ///Sets or clears `taken_at`.
    #[must_use]
    pub fn set_taken_at(mut self, value: Option<DateFilter>) -> Self {
        self.taken_at = value;
        self
    }

    ///Sets `trashed_at`.
    #[must_use]
    pub fn trashed_at(mut self, value: DateFilterNullable) -> Self {
        self.trashed_at = Some(value);
        self
    }

    ///Sets or clears `trashed_at`.
    #[must_use]
    pub fn set_trashed_at(mut self, value: Option<DateFilterNullable>) -> Self {
        self.trashed_at = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: EnumFilterAssetType) -> Self {
        self.type_ = Some(value);
        self
    }

    ///Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<EnumFilterAssetType>) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: DateFilter) -> Self {
        self.updated_at = Some(value);
        self
    }

    ///Sets or clears `updated_at`.
    #[must_use]
    pub fn set_updated_at(mut self, value: Option<DateFilter>) -> Self {
        self.updated_at = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: EnumFilterAssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    ///Sets or clears `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Option<EnumFilterAssetVisibility>) -> Self {
        self.visibility = value;
        self
    }
}

/// `SearchOrder`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `direction` | [`AssetOrder`] |  |  |
/// | `field` | [`SearchOrderField`] |  |  |
///
/// See: <https://api.immich.app/models/SearchOrder>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SearchOrder {
    #[serde(default = "crate::common::defaults::search_order_direction")]
    pub direction: AssetOrder,
    #[serde(default = "crate::common::defaults::search_order_field")]
    pub field: SearchOrderField,
}

impl SearchOrder {
    ///Creates a new [`SearchOrder`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            direction: crate::common::defaults::search_order_direction(),
            field: crate::common::defaults::search_order_field(),
        }
    }

    ///Sets `direction`.
    #[must_use]
    pub fn direction(mut self, value: AssetOrder) -> Self {
        self.direction = value;
        self
    }

    ///Sets `field`.
    #[must_use]
    pub fn field(mut self, value: SearchOrderField) -> Self {
        self.field = value;
        self
    }
}

impl ::std::default::Default for SearchOrder {
    fn default() -> Self {
        Self {
            direction: crate::common::defaults::search_order_direction(),
            field: crate::common::defaults::search_order_field(),
        }
    }
}

///
/// See: <https://api.immich.app/models/SearchOrderField>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SearchOrderField {
    #[serde(rename = "fileCreatedAt")]
    FileCreatedAt,
    #[serde(rename = "localDateTime")]
    LocalDateTime,
    #[serde(rename = "fileSizeInBytes")]
    FileSizeInBytes,
    #[serde(rename = "rating")]
    Rating,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for SearchOrderField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FileCreatedAt => f.write_str("fileCreatedAt"),
            Self::LocalDateTime => f.write_str("localDateTime"),
            Self::FileSizeInBytes => f.write_str("fileSizeInBytes"),
            Self::Rating => f.write_str("rating"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for SearchOrderField {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "fileCreatedAt" => Ok(Self::FileCreatedAt),
            "localDateTime" => Ok(Self::LocalDateTime),
            "fileSizeInBytes" => Ok(Self::FileSizeInBytes),
            "rating" => Ok(Self::Rating),
            _ => Err(format!("invalid value for {}: {}", "SearchOrderField", value).into()),
        }
    }
}

impl TryFrom<&str> for SearchOrderField {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SearchOrderField {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SearchOrderField {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `SearchResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `albums` | [`SearchAlbumResponseDTO`] | ✓ |  |
/// | `assets` | [`SearchAssetResponseDTO`] | ✓ |  |
///
/// See: <https://api.immich.app/models/SearchResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SearchResponseDTO {
    pub albums: SearchAlbumResponseDTO,
    pub assets: SearchAssetResponseDTO,
}

impl SearchResponseDTO {
    ///Creates a new [`SearchResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(albums: SearchAlbumResponseDTO, assets: SearchAssetResponseDTO) -> Self {
        Self { albums, assets }
    }

    ///Sets `albums`.
    #[must_use]
    pub fn albums(mut self, value: SearchAlbumResponseDTO) -> Self {
        self.albums = value;
        self
    }

    ///Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: SearchAssetResponseDTO) -> Self {
        self.assets = value;
        self
    }
}

/// `SearchStatisticsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `total` | `i64` | ✓ | Total number of matching assets |
///
/// See: <https://api.immich.app/models/SearchStatisticsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SearchStatisticsResponseDTO {
    pub total: i64,
}

impl SearchStatisticsResponseDTO {
    ///Creates a new [`SearchStatisticsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(total: i64) -> Self {
        Self { total }
    }

    ///Sets `total`.
    #[must_use]
    pub fn total(mut self, value: i64) -> Self {
        self.total = value;
        self
    }
}

/// Suggestion type
///
/// See: <https://api.immich.app/models/SearchSuggestionType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SearchSuggestionType {
    #[serde(rename = "country")]
    Country,
    #[serde(rename = "state")]
    State,
    #[serde(rename = "city")]
    City,
    #[serde(rename = "camera-make")]
    CameraMake,
    #[serde(rename = "camera-model")]
    CameraModel,
    #[serde(rename = "camera-lens-model")]
    CameraLensModel,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for SearchSuggestionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Country => f.write_str("country"),
            Self::State => f.write_str("state"),
            Self::City => f.write_str("city"),
            Self::CameraMake => f.write_str("camera-make"),
            Self::CameraModel => f.write_str("camera-model"),
            Self::CameraLensModel => f.write_str("camera-lens-model"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for SearchSuggestionType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "country" => Ok(Self::Country),
            "state" => Ok(Self::State),
            "city" => Ok(Self::City),
            "camera-make" => Ok(Self::CameraMake),
            "camera-model" => Ok(Self::CameraModel),
            "camera-lens-model" => Ok(Self::CameraLensModel),
            _ => Err(format!("invalid value for {}: {}", "SearchSuggestionType", value).into()),
        }
    }
}

impl TryFrom<&str> for SearchSuggestionType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SearchSuggestionType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SearchSuggestionType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `SmartSearchDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by album IDs |
/// | `city` | [`String`] |  | Filter by city name |
/// | `country` | [`String`] |  | Filter by country name |
/// | `created_after` | [`DateTime`](::chrono::DateTime) |  | Filter by creation date (after) |
/// | `created_before` | [`DateTime`](::chrono::DateTime) |  | Filter by creation date (before) |
/// | `filter` | [`SearchFilter`] |  |  |
/// | `is_encoded` | `bool` |  | Filter by encoded status |
/// | `is_favorite` | `bool` |  | Filter by favorite status |
/// | `is_motion` | `bool` |  | Filter by motion photo status |
/// | `is_not_in_album` | `bool` |  | Filter assets not in any album |
/// | `is_offline` | `bool` |  | Filter by offline status |
/// | `language` | [`String`] |  | Search language code |
/// | `lens_model` | [`String`] |  | Filter by lens model |
/// | `library_id` | [`Uuid`](::uuid::Uuid) |  | Library ID to filter by |
/// | `make` | [`String`] |  | Filter by camera make |
/// | `model` | [`String`] |  | Filter by camera model |
/// | `ocr` | [`String`] |  | Filter by OCR text content |
/// | `page` | `i64` |  | Page number |
/// | `person_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by person IDs |
/// | `query` | [`String`] |  | Natural language search query |
/// | `query_asset_id` | [`Uuid`](::uuid::Uuid) |  | Asset ID to use as search reference |
/// | `rating` | `i64` |  | Filter by rating [1-5], or null for unrated |
/// | `size` | `i64` |  | Number of results to return |
/// | `state` | [`String`] |  | Filter by state/province name |
/// | `tag_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by tag IDs |
/// | `taken_after` | [`DateTime`](::chrono::DateTime) |  | Filter by taken date (after) |
/// | `taken_before` | [`DateTime`](::chrono::DateTime) |  | Filter by taken date (before) |
/// | `trashed_after` | [`DateTime`](::chrono::DateTime) |  | Filter by trash date (after) |
/// | `trashed_before` | [`DateTime`](::chrono::DateTime) |  | Filter by trash date (before) |
/// | `type` | [`AssetTypeEnum`] |  |  |
/// | `updated_after` | [`DateTime`](::chrono::DateTime) |  | Filter by update date (after) |
/// | `updated_before` | [`DateTime`](::chrono::DateTime) |  | Filter by update date (before) |
/// | `visibility` | [`AssetVisibility`] |  |  |
/// | `with_deleted` | `bool` |  | Include deleted assets |
/// | `with_exif` | `bool` |  | Include EXIF data in response |
///
/// See: <https://api.immich.app/models/SmartSearchDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct SmartSearchDTO {
    /// Filter by album IDs
    #[serde(rename = "albumIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub album_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: Option<String>,
    /// Filter by creation date (after)
    #[serde(
        rename = "createdAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by creation date (before)
    #[serde(
        rename = "createdBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filter: Option<SearchFilter>,
    /// Filter by encoded status
    #[serde(
        rename = "isEncoded",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_encoded: Option<bool>,
    /// Filter by favorite status
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Filter by motion photo status
    #[serde(rename = "isMotion", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_motion: Option<bool>,
    /// Filter assets not in any album
    #[serde(
        rename = "isNotInAlbum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_not_in_album: Option<bool>,
    /// Filter by offline status
    #[serde(
        rename = "isOffline",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_offline: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language: Option<String>,
    /// Filter by lens model
    #[serde(
        rename = "lensModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lens_model: Option<String>,
    /// Library ID to filter by
    #[serde(
        rename = "libraryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub library_id: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub make: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ocr: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: Option<::std::num::NonZeroU64>,
    /// Filter by person IDs
    #[serde(rename = "personIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub person_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub query: Option<String>,
    /// Asset ID to use as search reference
    #[serde(
        rename = "queryAssetId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub query_asset_id: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<::std::num::NonZeroU64>,
    #[serde(default = "crate::common::defaults::default_nzu64::<::std::num::NonZeroU64, 100>")]
    pub size: ::std::num::NonZeroU64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
    /// Filter by tag IDs
    #[serde(rename = "tagIds", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag_ids: Option<Vec<::uuid::Uuid>>,
    /// Filter by taken date (after)
    #[serde(
        rename = "takenAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taken_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by taken date (before)
    #[serde(
        rename = "takenBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taken_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by trash date (after)
    #[serde(
        rename = "trashedAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by trash date (before)
    #[serde(
        rename = "trashedBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<AssetTypeEnum>,
    /// Filter by update date (after)
    #[serde(
        rename = "updatedAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by update date (before)
    #[serde(
        rename = "updatedBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<AssetVisibility>,
    /// Include deleted assets
    #[serde(
        rename = "withDeleted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub with_deleted: Option<bool>,
    /// Include EXIF data in response
    #[serde(rename = "withExif", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub with_exif: Option<bool>,
}

impl SmartSearchDTO {
    ///Creates a new [`SmartSearchDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_ids: Vec::new(),
            city: None,
            country: None,
            created_after: None,
            created_before: None,
            filter: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_not_in_album: None,
            is_offline: None,
            language: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            ocr: None,
            page: None,
            person_ids: Vec::new(),
            query: None,
            query_asset_id: None,
            rating: None,
            size: crate::common::defaults::default_nzu64::<::std::num::NonZeroU64, 100>(),
            state: None,
            tag_ids: None,
            taken_after: None,
            taken_before: None,
            trashed_after: None,
            trashed_before: None,
            type_: None,
            updated_after: None,
            updated_before: None,
            visibility: None,
            with_deleted: None,
            with_exif: None,
        }
    }

    ///Sets `album_ids`.
    #[must_use]
    pub fn album_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.album_ids = value;
        self
    }

    ///Sets `city`.
    #[must_use]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    ///Sets or clears `city`.
    #[must_use]
    pub fn set_city(mut self, value: Option<String>) -> Self {
        self.city = value;
        self
    }

    ///Sets `country`.
    #[must_use]
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    ///Sets or clears `country`.
    #[must_use]
    pub fn set_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }

    ///Sets `created_after`.
    #[must_use]
    pub fn created_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_after = Some(value);
        self
    }

    ///Sets or clears `created_after`.
    #[must_use]
    pub fn set_created_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_after = value;
        self
    }

    ///Sets `created_before`.
    #[must_use]
    pub fn created_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_before = Some(value);
        self
    }

    ///Sets or clears `created_before`.
    #[must_use]
    pub fn set_created_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_before = value;
        self
    }

    ///Sets `filter`.
    #[must_use]
    pub fn filter(mut self, value: SearchFilter) -> Self {
        self.filter = Some(value);
        self
    }

    ///Sets or clears `filter`.
    #[must_use]
    pub fn set_filter(mut self, value: Option<SearchFilter>) -> Self {
        self.filter = value;
        self
    }

    ///Sets `is_encoded`.
    #[must_use]
    pub fn is_encoded(mut self, value: bool) -> Self {
        self.is_encoded = Some(value);
        self
    }

    ///Sets or clears `is_encoded`.
    #[must_use]
    pub fn set_is_encoded(mut self, value: Option<bool>) -> Self {
        self.is_encoded = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    ///Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `is_motion`.
    #[must_use]
    pub fn is_motion(mut self, value: bool) -> Self {
        self.is_motion = Some(value);
        self
    }

    ///Sets or clears `is_motion`.
    #[must_use]
    pub fn set_is_motion(mut self, value: Option<bool>) -> Self {
        self.is_motion = value;
        self
    }

    ///Sets `is_not_in_album`.
    #[must_use]
    pub fn is_not_in_album(mut self, value: bool) -> Self {
        self.is_not_in_album = Some(value);
        self
    }

    ///Sets or clears `is_not_in_album`.
    #[must_use]
    pub fn set_is_not_in_album(mut self, value: Option<bool>) -> Self {
        self.is_not_in_album = value;
        self
    }

    ///Sets `is_offline`.
    #[must_use]
    pub fn is_offline(mut self, value: bool) -> Self {
        self.is_offline = Some(value);
        self
    }

    ///Sets or clears `is_offline`.
    #[must_use]
    pub fn set_is_offline(mut self, value: Option<bool>) -> Self {
        self.is_offline = value;
        self
    }

    ///Sets `language`.
    #[must_use]
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    ///Sets or clears `language`.
    #[must_use]
    pub fn set_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }

    ///Sets `lens_model`.
    #[must_use]
    pub fn lens_model(mut self, value: impl Into<String>) -> Self {
        self.lens_model = Some(value.into());
        self
    }

    ///Sets or clears `lens_model`.
    #[must_use]
    pub fn set_lens_model(mut self, value: Option<String>) -> Self {
        self.lens_model = value;
        self
    }

    ///Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: ::uuid::Uuid) -> Self {
        self.library_id = Some(value);
        self
    }

    ///Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.library_id = value;
        self
    }

    ///Sets `make`.
    #[must_use]
    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }

    ///Sets or clears `make`.
    #[must_use]
    pub fn set_make(mut self, value: Option<String>) -> Self {
        self.make = value;
        self
    }

    ///Sets `model`.
    #[must_use]
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    ///Sets or clears `model`.
    #[must_use]
    pub fn set_model(mut self, value: Option<String>) -> Self {
        self.model = value;
        self
    }

    ///Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: impl Into<String>) -> Self {
        self.ocr = Some(value.into());
        self
    }

    ///Sets or clears `ocr`.
    #[must_use]
    pub fn set_ocr(mut self, value: Option<String>) -> Self {
        self.ocr = value;
        self
    }

    ///Sets `page`.
    #[must_use]
    pub fn page(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.page = Some(value);
        self
    }

    ///Sets or clears `page`.
    #[must_use]
    pub fn set_page(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.page = value;
        self
    }

    ///Sets `person_ids`.
    #[must_use]
    pub fn person_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.person_ids = value;
        self
    }

    ///Sets `query`.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    ///Sets or clears `query`.
    #[must_use]
    pub fn set_query(mut self, value: Option<String>) -> Self {
        self.query = value;
        self
    }

    ///Sets `query_asset_id`.
    #[must_use]
    pub fn query_asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.query_asset_id = Some(value);
        self
    }

    ///Sets or clears `query_asset_id`.
    #[must_use]
    pub fn set_query_asset_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.query_asset_id = value;
        self
    }

    ///Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.rating = Some(value);
        self
    }

    ///Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.rating = value;
        self
    }

    ///Sets `size`.
    #[must_use]
    pub fn size(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.size = value;
        self
    }

    ///Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    ///Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }

    ///Sets `tag_ids`.
    #[must_use]
    pub fn tag_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    ///Sets or clears `tag_ids`.
    #[must_use]
    pub fn set_tag_ids(mut self, value: Option<Vec<::uuid::Uuid>>) -> Self {
        self.tag_ids = value;
        self
    }

    ///Sets `taken_after`.
    #[must_use]
    pub fn taken_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.taken_after = Some(value);
        self
    }

    ///Sets or clears `taken_after`.
    #[must_use]
    pub fn set_taken_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.taken_after = value;
        self
    }

    ///Sets `taken_before`.
    #[must_use]
    pub fn taken_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.taken_before = Some(value);
        self
    }

    ///Sets or clears `taken_before`.
    #[must_use]
    pub fn set_taken_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.taken_before = value;
        self
    }

    ///Sets `trashed_after`.
    #[must_use]
    pub fn trashed_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.trashed_after = Some(value);
        self
    }

    ///Sets or clears `trashed_after`.
    #[must_use]
    pub fn set_trashed_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.trashed_after = value;
        self
    }

    ///Sets `trashed_before`.
    #[must_use]
    pub fn trashed_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.trashed_before = Some(value);
        self
    }

    ///Sets or clears `trashed_before`.
    #[must_use]
    pub fn set_trashed_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.trashed_before = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = Some(value);
        self
    }

    ///Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<AssetTypeEnum>) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_after`.
    #[must_use]
    pub fn updated_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_after = Some(value);
        self
    }

    ///Sets or clears `updated_after`.
    #[must_use]
    pub fn set_updated_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_after = value;
        self
    }

    ///Sets `updated_before`.
    #[must_use]
    pub fn updated_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_before = Some(value);
        self
    }

    ///Sets or clears `updated_before`.
    #[must_use]
    pub fn set_updated_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_before = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    ///Sets or clears `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }

    ///Sets `with_deleted`.
    #[must_use]
    pub fn with_deleted(mut self, value: bool) -> Self {
        self.with_deleted = Some(value);
        self
    }

    ///Sets or clears `with_deleted`.
    #[must_use]
    pub fn set_with_deleted(mut self, value: Option<bool>) -> Self {
        self.with_deleted = value;
        self
    }

    ///Sets `with_exif`.
    #[must_use]
    pub fn with_exif(mut self, value: bool) -> Self {
        self.with_exif = Some(value);
        self
    }

    ///Sets or clears `with_exif`.
    #[must_use]
    pub fn set_with_exif(mut self, value: Option<bool>) -> Self {
        self.with_exif = value;
        self
    }
}

impl ::std::default::Default for SmartSearchDTO {
    fn default() -> Self {
        Self {
            album_ids: Vec::new(),
            city: None,
            country: None,
            created_after: None,
            created_before: None,
            filter: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_not_in_album: None,
            is_offline: None,
            language: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            ocr: None,
            page: None,
            person_ids: Vec::new(),
            query: None,
            query_asset_id: None,
            rating: None,
            size: crate::common::defaults::default_nzu64::<::std::num::NonZeroU64, 100>(),
            state: None,
            tag_ids: None,
            taken_after: None,
            taken_before: None,
            trashed_after: None,
            trashed_before: None,
            type_: None,
            updated_after: None,
            updated_before: None,
            visibility: None,
            with_deleted: None,
            with_exif: None,
        }
    }
}

/// `StatisticsSearchDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `album_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by album IDs |
/// | `city` | [`String`] |  | Filter by city name |
/// | `country` | [`String`] |  | Filter by country name |
/// | `created_after` | [`DateTime`](::chrono::DateTime) |  | Filter by creation date (after) |
/// | `created_before` | [`DateTime`](::chrono::DateTime) |  | Filter by creation date (before) |
/// | `description` | [`String`] |  | Filter by description text |
/// | `filter` | [`SearchFilter`] |  |  |
/// | `is_encoded` | `bool` |  | Filter by encoded status |
/// | `is_favorite` | `bool` |  | Filter by favorite status |
/// | `is_motion` | `bool` |  | Filter by motion photo status |
/// | `is_not_in_album` | `bool` |  | Filter assets not in any album |
/// | `is_offline` | `bool` |  | Filter by offline status |
/// | `lens_model` | [`String`] |  | Filter by lens model |
/// | `library_id` | [`Uuid`](::uuid::Uuid) |  | Library ID to filter by |
/// | `make` | [`String`] |  | Filter by camera make |
/// | `model` | [`String`] |  | Filter by camera model |
/// | `ocr` | [`String`] |  | Filter by OCR text content |
/// | `person_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by person IDs |
/// | `rating` | `i64` |  | Filter by rating [1-5], or null for unrated |
/// | `state` | [`String`] |  | Filter by state/province name |
/// | `tag_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Filter by tag IDs |
/// | `taken_after` | [`DateTime`](::chrono::DateTime) |  | Filter by taken date (after) |
/// | `taken_before` | [`DateTime`](::chrono::DateTime) |  | Filter by taken date (before) |
/// | `trashed_after` | [`DateTime`](::chrono::DateTime) |  | Filter by trash date (after) |
/// | `trashed_before` | [`DateTime`](::chrono::DateTime) |  | Filter by trash date (before) |
/// | `type` | [`AssetTypeEnum`] |  |  |
/// | `updated_after` | [`DateTime`](::chrono::DateTime) |  | Filter by update date (after) |
/// | `updated_before` | [`DateTime`](::chrono::DateTime) |  | Filter by update date (before) |
/// | `visibility` | [`AssetVisibility`] |  |  |
///
/// See: <https://api.immich.app/models/StatisticsSearchDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct StatisticsSearchDTO {
    /// Filter by album IDs
    #[serde(rename = "albumIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub album_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: Option<String>,
    /// Filter by creation date (after)
    #[serde(
        rename = "createdAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by creation date (before)
    #[serde(
        rename = "createdBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filter: Option<SearchFilter>,
    /// Filter by encoded status
    #[serde(
        rename = "isEncoded",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_encoded: Option<bool>,
    /// Filter by favorite status
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Filter by motion photo status
    #[serde(rename = "isMotion", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_motion: Option<bool>,
    /// Filter assets not in any album
    #[serde(
        rename = "isNotInAlbum",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_not_in_album: Option<bool>,
    /// Filter by offline status
    #[serde(
        rename = "isOffline",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_offline: Option<bool>,
    /// Filter by lens model
    #[serde(
        rename = "lensModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lens_model: Option<String>,
    /// Library ID to filter by
    #[serde(
        rename = "libraryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub library_id: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub make: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ocr: Option<String>,
    /// Filter by person IDs
    #[serde(rename = "personIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub person_ids: Vec<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<::std::num::NonZeroU64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
    /// Filter by tag IDs
    #[serde(rename = "tagIds", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag_ids: Option<Vec<::uuid::Uuid>>,
    /// Filter by taken date (after)
    #[serde(
        rename = "takenAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taken_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by taken date (before)
    #[serde(
        rename = "takenBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub taken_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by trash date (after)
    #[serde(
        rename = "trashedAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by trash date (before)
    #[serde(
        rename = "trashedBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trashed_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<AssetTypeEnum>,
    /// Filter by update date (after)
    #[serde(
        rename = "updatedAfter",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_after: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Filter by update date (before)
    #[serde(
        rename = "updatedBefore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_before: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<AssetVisibility>,
}

impl StatisticsSearchDTO {
    ///Creates a new [`StatisticsSearchDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            album_ids: Vec::new(),
            city: None,
            country: None,
            created_after: None,
            created_before: None,
            description: None,
            filter: None,
            is_encoded: None,
            is_favorite: None,
            is_motion: None,
            is_not_in_album: None,
            is_offline: None,
            lens_model: None,
            library_id: None,
            make: None,
            model: None,
            ocr: None,
            person_ids: Vec::new(),
            rating: None,
            state: None,
            tag_ids: None,
            taken_after: None,
            taken_before: None,
            trashed_after: None,
            trashed_before: None,
            type_: None,
            updated_after: None,
            updated_before: None,
            visibility: None,
        }
    }

    ///Sets `album_ids`.
    #[must_use]
    pub fn album_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.album_ids = value;
        self
    }

    ///Sets `city`.
    #[must_use]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    ///Sets or clears `city`.
    #[must_use]
    pub fn set_city(mut self, value: Option<String>) -> Self {
        self.city = value;
        self
    }

    ///Sets `country`.
    #[must_use]
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    ///Sets or clears `country`.
    #[must_use]
    pub fn set_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }

    ///Sets `created_after`.
    #[must_use]
    pub fn created_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_after = Some(value);
        self
    }

    ///Sets or clears `created_after`.
    #[must_use]
    pub fn set_created_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_after = value;
        self
    }

    ///Sets `created_before`.
    #[must_use]
    pub fn created_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_before = Some(value);
        self
    }

    ///Sets or clears `created_before`.
    #[must_use]
    pub fn set_created_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.created_before = value;
        self
    }

    ///Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    ///Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }

    ///Sets `filter`.
    #[must_use]
    pub fn filter(mut self, value: SearchFilter) -> Self {
        self.filter = Some(value);
        self
    }

    ///Sets or clears `filter`.
    #[must_use]
    pub fn set_filter(mut self, value: Option<SearchFilter>) -> Self {
        self.filter = value;
        self
    }

    ///Sets `is_encoded`.
    #[must_use]
    pub fn is_encoded(mut self, value: bool) -> Self {
        self.is_encoded = Some(value);
        self
    }

    ///Sets or clears `is_encoded`.
    #[must_use]
    pub fn set_is_encoded(mut self, value: Option<bool>) -> Self {
        self.is_encoded = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    ///Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `is_motion`.
    #[must_use]
    pub fn is_motion(mut self, value: bool) -> Self {
        self.is_motion = Some(value);
        self
    }

    ///Sets or clears `is_motion`.
    #[must_use]
    pub fn set_is_motion(mut self, value: Option<bool>) -> Self {
        self.is_motion = value;
        self
    }

    ///Sets `is_not_in_album`.
    #[must_use]
    pub fn is_not_in_album(mut self, value: bool) -> Self {
        self.is_not_in_album = Some(value);
        self
    }

    ///Sets or clears `is_not_in_album`.
    #[must_use]
    pub fn set_is_not_in_album(mut self, value: Option<bool>) -> Self {
        self.is_not_in_album = value;
        self
    }

    ///Sets `is_offline`.
    #[must_use]
    pub fn is_offline(mut self, value: bool) -> Self {
        self.is_offline = Some(value);
        self
    }

    ///Sets or clears `is_offline`.
    #[must_use]
    pub fn set_is_offline(mut self, value: Option<bool>) -> Self {
        self.is_offline = value;
        self
    }

    ///Sets `lens_model`.
    #[must_use]
    pub fn lens_model(mut self, value: impl Into<String>) -> Self {
        self.lens_model = Some(value.into());
        self
    }

    ///Sets or clears `lens_model`.
    #[must_use]
    pub fn set_lens_model(mut self, value: Option<String>) -> Self {
        self.lens_model = value;
        self
    }

    ///Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: ::uuid::Uuid) -> Self {
        self.library_id = Some(value);
        self
    }

    ///Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.library_id = value;
        self
    }

    ///Sets `make`.
    #[must_use]
    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }

    ///Sets or clears `make`.
    #[must_use]
    pub fn set_make(mut self, value: Option<String>) -> Self {
        self.make = value;
        self
    }

    ///Sets `model`.
    #[must_use]
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    ///Sets or clears `model`.
    #[must_use]
    pub fn set_model(mut self, value: Option<String>) -> Self {
        self.model = value;
        self
    }

    ///Sets `ocr`.
    #[must_use]
    pub fn ocr(mut self, value: impl Into<String>) -> Self {
        self.ocr = Some(value.into());
        self
    }

    ///Sets or clears `ocr`.
    #[must_use]
    pub fn set_ocr(mut self, value: Option<String>) -> Self {
        self.ocr = value;
        self
    }

    ///Sets `person_ids`.
    #[must_use]
    pub fn person_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.person_ids = value;
        self
    }

    ///Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.rating = Some(value);
        self
    }

    ///Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.rating = value;
        self
    }

    ///Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    ///Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }

    ///Sets `tag_ids`.
    #[must_use]
    pub fn tag_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    ///Sets or clears `tag_ids`.
    #[must_use]
    pub fn set_tag_ids(mut self, value: Option<Vec<::uuid::Uuid>>) -> Self {
        self.tag_ids = value;
        self
    }

    ///Sets `taken_after`.
    #[must_use]
    pub fn taken_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.taken_after = Some(value);
        self
    }

    ///Sets or clears `taken_after`.
    #[must_use]
    pub fn set_taken_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.taken_after = value;
        self
    }

    ///Sets `taken_before`.
    #[must_use]
    pub fn taken_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.taken_before = Some(value);
        self
    }

    ///Sets or clears `taken_before`.
    #[must_use]
    pub fn set_taken_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.taken_before = value;
        self
    }

    ///Sets `trashed_after`.
    #[must_use]
    pub fn trashed_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.trashed_after = Some(value);
        self
    }

    ///Sets or clears `trashed_after`.
    #[must_use]
    pub fn set_trashed_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.trashed_after = value;
        self
    }

    ///Sets `trashed_before`.
    #[must_use]
    pub fn trashed_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.trashed_before = Some(value);
        self
    }

    ///Sets or clears `trashed_before`.
    #[must_use]
    pub fn set_trashed_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.trashed_before = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = Some(value);
        self
    }

    ///Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<AssetTypeEnum>) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_after`.
    #[must_use]
    pub fn updated_after(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_after = Some(value);
        self
    }

    ///Sets or clears `updated_after`.
    #[must_use]
    pub fn set_updated_after(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_after = value;
        self
    }

    ///Sets `updated_before`.
    #[must_use]
    pub fn updated_before(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_before = Some(value);
        self
    }

    ///Sets or clears `updated_before`.
    #[must_use]
    pub fn set_updated_before(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_before = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    ///Sets or clears `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }
}

/// `StringFilter`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | [`String`] |  |  |
/// | `in` | [`Vec<String>`](::std::vec::Vec) |  |  |
/// | `ne` | [`String`] |  |  |
/// | `not_in` | [`Vec<String>`](::std::vec::Vec) |  |  |
///
/// See: <https://api.immich.app/models/StringFilter>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct StringFilter {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<String>,
    #[serde(rename = "in", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub in_: Vec<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<String>,
    #[serde(rename = "notIn", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub not_in: Vec<String>,
}

impl StringFilter {
    ///Creates a new [`StringFilter`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            eq: None,
            in_: Vec::new(),
            ne: None,
            not_in: Vec::new(),
        }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: impl Into<String>) -> Self {
        self.eq = Some(value.into());
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<String>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `in_`.
    #[must_use]
    pub fn in_(mut self, value: Vec<String>) -> Self {
        self.in_ = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: impl Into<String>) -> Self {
        self.ne = Some(value.into());
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<String>) -> Self {
        self.ne = value;
        self
    }

    ///Sets `not_in`.
    #[must_use]
    pub fn not_in(mut self, value: Vec<String>) -> Self {
        self.not_in = value;
        self
    }
}

/// `StringFilterNullable`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `eq` | [`String`] |  |  |
/// | `in` | [`Vec<String>`](::std::vec::Vec) |  |  |
/// | `ne` | [`String`] |  |  |
/// | `not_in` | [`Vec<String>`](::std::vec::Vec) |  |  |
///
/// See: <https://api.immich.app/models/StringFilterNullable>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct StringFilterNullable {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<String>,
    #[serde(rename = "in", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub in_: Vec<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<String>,
    #[serde(rename = "notIn", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub not_in: Vec<String>,
}

impl StringFilterNullable {
    ///Creates a new [`StringFilterNullable`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            eq: None,
            in_: Vec::new(),
            ne: None,
            not_in: Vec::new(),
        }
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: impl Into<String>) -> Self {
        self.eq = Some(value.into());
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<String>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `in_`.
    #[must_use]
    pub fn in_(mut self, value: Vec<String>) -> Self {
        self.in_ = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: impl Into<String>) -> Self {
        self.ne = Some(value.into());
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<String>) -> Self {
        self.ne = value;
        self
    }

    ///Sets `not_in`.
    #[must_use]
    pub fn not_in(mut self, value: Vec<String>) -> Self {
        self.not_in = value;
        self
    }
}

/// `StringPatternFilter`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ends_with` | [`String`] |  |  |
/// | `eq` | [`String`] |  |  |
/// | `in` | [`Vec<String>`](::std::vec::Vec) |  |  |
/// | `like` | [`String`] |  |  |
/// | `ne` | [`String`] |  |  |
/// | `not_in` | [`Vec<String>`](::std::vec::Vec) |  |  |
/// | `not_like` | [`String`] |  |  |
/// | `starts_with` | [`String`] |  |  |
///
/// See: <https://api.immich.app/models/StringPatternFilter>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct StringPatternFilter {
    #[serde(rename = "endsWith", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ends_with: Option<StringPatternFilterEndsWith>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eq: Option<String>,
    #[serde(rename = "in", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub in_: Vec<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub like: Option<StringPatternFilterLike>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ne: Option<String>,
    #[serde(rename = "notIn", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub not_in: Vec<String>,
    #[serde(rename = "notLike", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub not_like: Option<StringPatternFilterNotLike>,
    #[serde(
        rename = "startsWith",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub starts_with: Option<StringPatternFilterStartsWith>,
}

impl StringPatternFilter {
    ///Creates a new [`StringPatternFilter`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            ends_with: None,
            eq: None,
            in_: Vec::new(),
            like: None,
            ne: None,
            not_in: Vec::new(),
            not_like: None,
            starts_with: None,
        }
    }

    ///Sets `ends_with`.
    #[must_use]
    pub fn ends_with(mut self, value: StringPatternFilterEndsWith) -> Self {
        self.ends_with = Some(value);
        self
    }

    ///Sets or clears `ends_with`.
    #[must_use]
    pub fn set_ends_with(mut self, value: Option<StringPatternFilterEndsWith>) -> Self {
        self.ends_with = value;
        self
    }

    ///Sets `eq`.
    #[must_use]
    pub fn eq(mut self, value: impl Into<String>) -> Self {
        self.eq = Some(value.into());
        self
    }

    ///Sets or clears `eq`.
    #[must_use]
    pub fn set_eq(mut self, value: Option<String>) -> Self {
        self.eq = value;
        self
    }

    ///Sets `in_`.
    #[must_use]
    pub fn in_(mut self, value: Vec<String>) -> Self {
        self.in_ = value;
        self
    }

    ///Sets `like`.
    #[must_use]
    pub fn like(mut self, value: StringPatternFilterLike) -> Self {
        self.like = Some(value);
        self
    }

    ///Sets or clears `like`.
    #[must_use]
    pub fn set_like(mut self, value: Option<StringPatternFilterLike>) -> Self {
        self.like = value;
        self
    }

    ///Sets `ne`.
    #[must_use]
    pub fn ne(mut self, value: impl Into<String>) -> Self {
        self.ne = Some(value.into());
        self
    }

    ///Sets or clears `ne`.
    #[must_use]
    pub fn set_ne(mut self, value: Option<String>) -> Self {
        self.ne = value;
        self
    }

    ///Sets `not_in`.
    #[must_use]
    pub fn not_in(mut self, value: Vec<String>) -> Self {
        self.not_in = value;
        self
    }

    ///Sets `not_like`.
    #[must_use]
    pub fn not_like(mut self, value: StringPatternFilterNotLike) -> Self {
        self.not_like = Some(value);
        self
    }

    ///Sets or clears `not_like`.
    #[must_use]
    pub fn set_not_like(mut self, value: Option<StringPatternFilterNotLike>) -> Self {
        self.not_like = value;
        self
    }

    ///Sets `starts_with`.
    #[must_use]
    pub fn starts_with(mut self, value: StringPatternFilterStartsWith) -> Self {
        self.starts_with = Some(value);
        self
    }

    ///Sets or clears `starts_with`.
    #[must_use]
    pub fn set_starts_with(mut self, value: Option<StringPatternFilterStartsWith>) -> Self {
        self.starts_with = value;
        self
    }
}

/// `StringSimilarityFilter`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `matches` | [`String`] | ✓ |  |
///
/// See: <https://api.immich.app/models/StringSimilarityFilter>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct StringSimilarityFilter {
    pub matches: StringSimilarityFilterMatches,
}

impl StringSimilarityFilter {
    ///Creates a new [`StringSimilarityFilter`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(matches: StringSimilarityFilterMatches) -> Self {
        Self { matches }
    }

    ///Sets `matches`.
    #[must_use]
    pub fn matches(mut self, value: StringSimilarityFilterMatches) -> Self {
        self.matches = value;
        self
    }
}
