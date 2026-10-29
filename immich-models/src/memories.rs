// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `memories`.

pub use crate::assets::AssetResponseDTO;

/// `MemoryCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Asset IDs to associate with memory |
/// | `data` | [`MemoryDataDTO`] | ✓ |  |
/// | `hide_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory should be hidden |
/// | `is_saved` | `bool` |  | Is memory saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) | ✓ | Memory date |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory was seen |
/// | `show_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory should be shown |
/// | `type` | [`MemoryType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/MemoryCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MemoryCreateDTO {
    /// Asset IDs to associate with memory
    #[serde(rename = "assetIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub asset_ids: Vec<::uuid::Uuid>,
    pub data: MemoryDataDTO,
    /// Date when memory should be hidden
    #[serde(rename = "hideAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hide_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Is memory saved
    #[serde(rename = "isSaved", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_saved: Option<bool>,
    /// Memory date (required)
    #[serde(rename = "memoryAt")]
    pub memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Date when memory was seen
    #[serde(rename = "seenAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Date when memory should be shown
    #[serde(rename = "showAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub show_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: MemoryType,
}

impl MemoryCreateDTO {
    ///Creates a new [`MemoryCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(data: MemoryDataDTO, memory_at: ::chrono::DateTime<::chrono::offset::Utc>, type_: MemoryType) -> Self {
        Self {
            asset_ids: Vec::new(),
            data,
            hide_at: None,
            is_saved: None,
            memory_at,
            seen_at: None,
            show_at: None,
            type_,
        }
    }

    ///Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.asset_ids = value;
        self
    }

    ///Sets `data`.
    #[must_use]
    pub fn data(mut self, value: MemoryDataDTO) -> Self {
        self.data = value;
        self
    }

    ///Sets `hide_at`.
    #[must_use]
    pub fn hide_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.hide_at = Some(value);
        self
    }

    ///Sets or clears `hide_at`.
    #[must_use]
    pub fn set_hide_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.hide_at = value;
        self
    }

    ///Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = Some(value);
        self
    }

    ///Sets or clears `is_saved`.
    #[must_use]
    pub fn set_is_saved(mut self, value: Option<bool>) -> Self {
        self.is_saved = value;
        self
    }

    ///Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = value;
        self
    }

    ///Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }

    ///Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }

    ///Sets `show_at`.
    #[must_use]
    pub fn show_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.show_at = Some(value);
        self
    }

    ///Sets or clears `show_at`.
    #[must_use]
    pub fn set_show_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.show_at = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: MemoryType) -> Self {
        self.type_ = value;
        self
    }
}

/// `MemoryDataDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `person_id` | [`Uuid`](::uuid::Uuid) |  | Person ID (birthday memories) |
/// | `person_name` | [`String`] |  | Name of the person when the memory was created (birthday memories) |
/// | `year` | `i64` | ✓ | Year of the memory |
///
/// See: <https://api.immich.app/models/MemoryDataDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MemoryDataDTO {
    /// Person ID (birthday memories)
    #[serde(rename = "personId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub person_id: Option<::uuid::Uuid>,
    /// Name of the person when the memory was created (birthday memories)
    #[serde(
        rename = "personName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub person_name: Option<String>,
    pub year: i64,
}

impl MemoryDataDTO {
    ///Creates a new [`MemoryDataDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(year: i64) -> Self {
        Self {
            person_id: None,
            person_name: None,
            year,
        }
    }

    ///Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: ::uuid::Uuid) -> Self {
        self.person_id = Some(value);
        self
    }

    ///Sets or clears `person_id`.
    #[must_use]
    pub fn set_person_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.person_id = value;
        self
    }

    ///Sets `person_name`.
    #[must_use]
    pub fn person_name(mut self, value: impl Into<String>) -> Self {
        self.person_name = Some(value.into());
        self
    }

    ///Sets or clears `person_name`.
    #[must_use]
    pub fn set_person_name(mut self, value: Option<String>) -> Self {
        self.person_name = value;
        self
    }

    ///Sets `year`.
    #[must_use]
    pub fn year(mut self, value: i64) -> Self {
        self.year = value;
        self
    }
}

/// `MemoryResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `assets` | [`Vec<AssetResponseDTO>`](crate::assets::AssetResponseDTO) | ✓ |  |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `data` | [`MemoryDataDTO`] | ✓ |  |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) |  | Deletion date |
/// | `hide_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory should be hidden |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Memory ID |
/// | `is_saved` | `bool` | ✓ | Is memory saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) | ✓ | Memory date |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner user ID |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory was seen |
/// | `show_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory should be shown |
/// | `type` | [`MemoryType`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
///
/// See: <https://api.immich.app/models/MemoryResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MemoryResponseDTO {
    pub assets: Vec<AssetResponseDTO>,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub data: MemoryDataDTO,
    /// Deletion date
    #[serde(
        rename = "deletedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Date when memory should be hidden
    #[serde(rename = "hideAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hide_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub id: ::uuid::Uuid,
    /// Is memory saved (required)
    #[serde(rename = "isSaved")]
    pub is_saved: bool,
    /// Memory date (required)
    #[serde(rename = "memoryAt")]
    pub memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Owner user ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    /// Date when memory was seen
    #[serde(rename = "seenAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Date when memory should be shown
    #[serde(rename = "showAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub show_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: MemoryType,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl MemoryResponseDTO {
    ///Creates a new [`MemoryResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        assets: Vec<AssetResponseDTO>,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        data: MemoryDataDTO,
        id: ::uuid::Uuid,
        is_saved: bool,
        memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
        owner_id: ::uuid::Uuid,
        type_: MemoryType,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            assets,
            created_at,
            data,
            deleted_at: None,
            hide_at: None,
            id,
            is_saved,
            memory_at,
            owner_id,
            seen_at: None,
            show_at: None,
            type_,
            updated_at,
        }
    }

    ///Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: Vec<AssetResponseDTO>) -> Self {
        self.assets = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `data`.
    #[must_use]
    pub fn data(mut self, value: MemoryDataDTO) -> Self {
        self.data = value;
        self
    }

    ///Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    ///Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }

    ///Sets `hide_at`.
    #[must_use]
    pub fn hide_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.hide_at = Some(value);
        self
    }

    ///Sets or clears `hide_at`.
    #[must_use]
    pub fn set_hide_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.hide_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = value;
        self
    }

    ///Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = value;
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }

    ///Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }

    ///Sets `show_at`.
    #[must_use]
    pub fn show_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.show_at = Some(value);
        self
    }

    ///Sets or clears `show_at`.
    #[must_use]
    pub fn set_show_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.show_at = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: MemoryType) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// Sort order
///
/// See: <https://api.immich.app/models/MemorySearchOrder>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MemorySearchOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "random")]
    Random,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for MemorySearchOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
            Self::Random => f.write_str("random"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for MemorySearchOrder {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            "random" => Ok(Self::Random),
            _ => Err(format!("invalid value for {}: {}", "MemorySearchOrder", value).into()),
        }
    }
}

impl TryFrom<&str> for MemorySearchOrder {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MemorySearchOrder {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MemorySearchOrder {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `MemoryStatisticsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `total` | `i64` | ✓ | Total number of memories |
///
/// See: <https://api.immich.app/models/MemoryStatisticsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MemoryStatisticsResponseDTO {
    pub total: i64,
}

impl MemoryStatisticsResponseDTO {
    ///Creates a new [`MemoryStatisticsResponseDTO`].
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

/// Memory type
///
/// See: <https://api.immich.app/models/MemoryType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MemoryType {
    #[serde(rename = "on_this_day")]
    OnThisDay,
    #[serde(rename = "birthday")]
    Birthday,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OnThisDay => f.write_str("on_this_day"),
            Self::Birthday => f.write_str("birthday"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for MemoryType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "on_this_day" => Ok(Self::OnThisDay),
            "birthday" => Ok(Self::Birthday),
            _ => Err(format!("invalid value for {}: {}", "MemoryType", value).into()),
        }
    }
}

impl TryFrom<&str> for MemoryType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MemoryType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MemoryType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

///
/// See: <https://api.immich.app/models/MemoryTypeV1>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MemoryTypeV1 {
    #[serde(rename = "on_this_day")]
    OnThisDay,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for MemoryTypeV1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OnThisDay => f.write_str("on_this_day"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for MemoryTypeV1 {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "on_this_day" => Ok(Self::OnThisDay),
            _ => Err(format!("invalid value for {}: {}", "MemoryTypeV1", value).into()),
        }
    }
}

impl TryFrom<&str> for MemoryTypeV1 {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MemoryTypeV1 {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MemoryTypeV1 {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `MemoryUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `is_saved` | `bool` |  | Is memory saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) |  | Memory date |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory was seen |
///
/// See: <https://api.immich.app/models/MemoryUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct MemoryUpdateDTO {
    /// Is memory saved
    #[serde(rename = "isSaved", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_saved: Option<bool>,
    /// Memory date
    #[serde(rename = "memoryAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memory_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Date when memory was seen
    #[serde(rename = "seenAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl MemoryUpdateDTO {
    ///Creates a new [`MemoryUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            is_saved: None,
            memory_at: None,
            seen_at: None,
        }
    }

    ///Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = Some(value);
        self
    }

    ///Sets or clears `is_saved`.
    #[must_use]
    pub fn set_is_saved(mut self, value: Option<bool>) -> Self {
        self.is_saved = value;
        self
    }

    ///Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = Some(value);
        self
    }

    ///Sets or clears `memory_at`.
    #[must_use]
    pub fn set_memory_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.memory_at = value;
        self
    }

    ///Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }

    ///Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }
}
