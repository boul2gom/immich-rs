// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `memories`.

pub use crate::assets::AssetResponseDTO;
pub use crate::id::{AssetId, MemoryId, PersonId, UserId};

/// `MemoryCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) |  | Asset IDs to associate with memory |
/// | `data` | [`MemoryDataDTO`] | ✓ |  |
/// | `hide_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory should be hidden (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `is_saved` | `bool` |  | Is memory saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) | ✓ | Memory date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory was seen (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `show_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory should be shown (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `type_` | [`MemoryType`] | ✓ |  |
///
/// See: <https://api.immich.app/models/MemoryCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MemoryCreateDTO {
    /// Asset IDs to associate with memory
    #[serde(rename = "assetIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub asset_ids: Vec<AssetId>,
    /// (required)
    pub data: MemoryDataDTO,
    /// Date when memory should be hidden (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "hideAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hide_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Is memory saved
    #[serde(rename = "isSaved", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_saved: Option<bool>,
    /// Memory date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "memoryAt")]
    pub memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Date when memory was seen (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "seenAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Date when memory should be shown (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "showAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub show_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: MemoryType,
}

impl MemoryCreateDTO {
    /// Creates a new [`MemoryCreateDTO`].
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
    /// Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `data`.
    #[must_use]
    pub fn data(mut self, value: MemoryDataDTO) -> Self {
        self.data = value;
        self
    }
    /// Sets `hide_at`.
    #[must_use]
    pub fn hide_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.hide_at = Some(value);
        self
    }
    /// Sets or clears `hide_at`.
    #[must_use]
    pub fn set_hide_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.hide_at = value;
        self
    }
    /// Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = Some(value);
        self
    }
    /// Sets or clears `is_saved`.
    #[must_use]
    pub fn set_is_saved(mut self, value: Option<bool>) -> Self {
        self.is_saved = value;
        self
    }
    /// Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = value;
        self
    }
    /// Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }
    /// Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }
    /// Sets `show_at`.
    #[must_use]
    pub fn show_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.show_at = Some(value);
        self
    }
    /// Sets or clears `show_at`.
    #[must_use]
    pub fn set_show_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.show_at = value;
        self
    }
    /// Sets `type_`.
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
/// | `person_id` | [`PersonId`](crate::id::PersonId) |  | Person ID (birthday memories) (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `person_name` | [`String`] |  | Name of the person when the memory was created (birthday memories) |
/// | `year` | `i64` | ✓ | Year of the memory |
///
/// See: <https://api.immich.app/models/MemoryDataDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MemoryDataDTO {
    /// Person ID (birthday memories) (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(rename = "personId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub person_id: Option<PersonId>,
    /// Name of the person when the memory was created (birthday memories)
    #[serde(
        rename = "personName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub person_name: Option<String>,
    /// Year of the memory (required)
    pub year: i64,
}

impl MemoryDataDTO {
    /// Creates a new [`MemoryDataDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(year: i64) -> Self {
        Self {
            person_id: None,
            person_name: None,
            year,
        }
    }
    /// Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: impl Into<PersonId>) -> Self {
        self.person_id = Some(value.into());
        self
    }
    /// Sets or clears `person_id`.
    #[must_use]
    pub fn set_person_id(mut self, value: Option<PersonId>) -> Self {
        self.person_id = value;
        self
    }
    /// Sets `person_name`.
    #[must_use]
    pub fn person_name(mut self, value: impl Into<String>) -> Self {
        self.person_name = Some(value.into());
        self
    }
    /// Sets or clears `person_name`.
    #[must_use]
    pub fn set_person_name(mut self, value: Option<String>) -> Self {
        self.person_name = value;
        self
    }
    /// Sets `year`.
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
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `data` | [`MemoryDataDTO`] | ✓ |  |
/// | `deleted_at` | [`DateTime`](::chrono::DateTime) |  | Deletion date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `hide_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory should be hidden (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `id` | [`MemoryId`](crate::id::MemoryId) | ✓ | Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_saved` | `bool` | ✓ | Is memory saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) | ✓ | Memory date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory was seen (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `show_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory should be shown (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `type_` | [`MemoryType`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/MemoryResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct MemoryResponseDTO {
    /// (required)
    pub assets: Vec<AssetResponseDTO>,
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// (required)
    pub data: MemoryDataDTO,
    /// Deletion date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(
        rename = "deletedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub deleted_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Date when memory should be hidden (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "hideAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hide_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Memory ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: MemoryId,
    /// Is memory saved (required)
    #[serde(rename = "isSaved")]
    pub is_saved: bool,
    /// Memory date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "memoryAt")]
    pub memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    /// Date when memory was seen (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "seenAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Date when memory should be shown (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "showAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub show_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: MemoryType,
    /// Last update date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl MemoryResponseDTO {
    /// Creates a new [`MemoryResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        assets: impl IntoIterator<Item = AssetResponseDTO>,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        data: MemoryDataDTO,
        id: impl Into<MemoryId>,
        is_saved: bool,
        memory_at: ::chrono::DateTime<::chrono::offset::Utc>,
        owner_id: impl Into<UserId>,
        type_: MemoryType,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            assets: assets.into_iter().collect(),
            created_at,
            data,
            deleted_at: None,
            hide_at: None,
            id: id.into(),
            is_saved,
            memory_at,
            owner_id: owner_id.into(),
            seen_at: None,
            show_at: None,
            type_,
            updated_at,
        }
    }
    /// Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: impl IntoIterator<Item = AssetResponseDTO>) -> Self {
        self.assets = value.into_iter().collect();
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `data`.
    #[must_use]
    pub fn data(mut self, value: MemoryDataDTO) -> Self {
        self.data = value;
        self
    }
    /// Sets `deleted_at`.
    #[must_use]
    pub fn deleted_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }
    /// Sets or clears `deleted_at`.
    #[must_use]
    pub fn set_deleted_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.deleted_at = value;
        self
    }
    /// Sets `hide_at`.
    #[must_use]
    pub fn hide_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.hide_at = Some(value);
        self
    }
    /// Sets or clears `hide_at`.
    #[must_use]
    pub fn set_hide_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.hide_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<MemoryId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = value;
        self
    }
    /// Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = value;
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }
    /// Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }
    /// Sets `show_at`.
    #[must_use]
    pub fn show_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.show_at = Some(value);
        self
    }
    /// Sets or clears `show_at`.
    #[must_use]
    pub fn set_show_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.show_at = value;
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: MemoryType) -> Self {
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

/// Sort order
///
/// See: <https://api.immich.app/models/MemorySearchOrder>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MemorySearchOrder {
    Asc,
    Desc,
    Random,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl MemorySearchOrder {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
            Self::Random => "random",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for MemorySearchOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for MemorySearchOrder {
    fn from(value: &str) -> Self {
        match value {
            "asc" => Self::Asc,
            "desc" => Self::Desc,
            "random" => Self::Random,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for MemorySearchOrder {
    fn from(value: String) -> Self {
        match value.as_str() {
            "asc" => Self::Asc,
            "desc" => Self::Desc,
            "random" => Self::Random,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for MemorySearchOrder {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for MemorySearchOrder {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for MemorySearchOrder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for MemorySearchOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = MemorySearchOrder;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MemorySearchOrder::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MemorySearchOrder::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `MemoryStatisticsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `total` | `i64` | ✓ | Total number of memories |
///
/// See: <https://api.immich.app/models/MemoryStatisticsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MemoryStatisticsResponseDTO {
    /// Total number of memories (required)
    pub total: i64,
}

impl MemoryStatisticsResponseDTO {
    /// Creates a new [`MemoryStatisticsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(total: i64) -> Self {
        Self { total }
    }
    /// Sets `total`.
    #[must_use]
    pub fn total(mut self, value: i64) -> Self {
        self.total = value;
        self
    }
}

/// Memory type
///
/// See: <https://api.immich.app/models/MemoryType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MemoryType {
    OnThisDay,
    Birthday,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl MemoryType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::OnThisDay => "on_this_day",
            Self::Birthday => "birthday",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for MemoryType {
    fn from(value: &str) -> Self {
        match value {
            "on_this_day" => Self::OnThisDay,
            "birthday" => Self::Birthday,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for MemoryType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "on_this_day" => Self::OnThisDay,
            "birthday" => Self::Birthday,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for MemoryType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for MemoryType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for MemoryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for MemoryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = MemoryType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MemoryType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MemoryType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `MemoryTypeV1`
///
/// See: <https://api.immich.app/models/MemoryTypeV1>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MemoryTypeV1 {
    OnThisDay,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl MemoryTypeV1 {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::OnThisDay => "on_this_day",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for MemoryTypeV1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for MemoryTypeV1 {
    fn from(value: &str) -> Self {
        match value {
            "on_this_day" => Self::OnThisDay,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for MemoryTypeV1 {
    fn from(value: String) -> Self {
        match value.as_str() {
            "on_this_day" => Self::OnThisDay,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for MemoryTypeV1 {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for MemoryTypeV1 {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for MemoryTypeV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for MemoryTypeV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = MemoryTypeV1;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MemoryTypeV1::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MemoryTypeV1::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `MemoryUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `is_saved` | `bool` |  | Is memory saved |
/// | `memory_at` | [`DateTime`](::chrono::DateTime) |  | Memory date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `seen_at` | [`DateTime`](::chrono::DateTime) |  | Date when memory was seen (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/MemoryUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct MemoryUpdateDTO {
    /// Is memory saved
    #[serde(rename = "isSaved", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_saved: Option<bool>,
    /// Memory date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "memoryAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memory_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Date when memory was seen (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`)
    #[serde(rename = "seenAt", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seen_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl MemoryUpdateDTO {
    /// Creates a new [`MemoryUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            is_saved: None,
            memory_at: None,
            seen_at: None,
        }
    }
    /// Sets `is_saved`.
    #[must_use]
    pub fn is_saved(mut self, value: bool) -> Self {
        self.is_saved = Some(value);
        self
    }
    /// Sets or clears `is_saved`.
    #[must_use]
    pub fn set_is_saved(mut self, value: Option<bool>) -> Self {
        self.is_saved = value;
        self
    }
    /// Sets `memory_at`.
    #[must_use]
    pub fn memory_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.memory_at = Some(value);
        self
    }
    /// Sets or clears `memory_at`.
    #[must_use]
    pub fn set_memory_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.memory_at = value;
        self
    }
    /// Sets `seen_at`.
    #[must_use]
    pub fn seen_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.seen_at = Some(value);
        self
    }
    /// Sets or clears `seen_at`.
    #[must_use]
    pub fn set_seen_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.seen_at = value;
        self
    }
}
