// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `assets`.

pub mod downloads;
pub mod faces;
pub mod files;
pub mod stacks;

pub use crate::activities::{UserAvatarColor, UserResponseDTO};
pub use crate::id::{AssetId, DuplicateId, LibraryId, PersonId, StackId, TagId, UserId};

/// `AssetBulkDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `force` | `bool` |  | Force delete even if in use |
/// | `ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | IDs to process |
///
/// See: <https://api.immich.app/models/AssetBulkDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetBulkDeleteDTO {
    /// Force delete even if in use
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub force: Option<bool>,
    /// IDs to process (required)
    pub ids: Vec<AssetId>,
}

impl AssetBulkDeleteDTO {
    /// Creates a new [`AssetBulkDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        Self {
            force: None,
            ids: ids.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `force`.
    #[must_use]
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }
    /// Sets or clears `force`.
    #[must_use]
    pub fn set_force(mut self, value: Option<bool>) -> Self {
        self.force = value;
        self
    }
    /// Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.ids = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `AssetBulkUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `date_time_original` | [`String`] |  | Original date and time |
/// | `date_time_relative` | `i64` |  | Relative time offset in minutes |
/// | `description` | [`String`] |  | Asset description |
/// | `duplicate_id` | [`DuplicateId`](crate::id::DuplicateId) |  | Duplicate ID |
/// | `ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs to update |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `latitude` | `f64` |  | Latitude coordinate |
/// | `longitude` | `f64` |  | Longitude coordinate |
/// | `rating` | `i64` |  | Rating in range \[1-5\] (starred), -1 (rejected), or null (unrated) |
/// | `time_zone` | [`String`] |  | Time zone (IANA timezone) |
/// | `visibility` | [`AssetVisibility`] |  |  |
///
/// See: <https://api.immich.app/models/AssetBulkUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetBulkUpdateDTO {
    /// Original date and time
    #[serde(
        rename = "dateTimeOriginal",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_time_original: Option<String>,
    /// Relative time offset in minutes
    #[serde(
        rename = "dateTimeRelative",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_time_relative: Option<i64>,
    /// Asset description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Duplicate ID
    #[serde(
        rename = "duplicateId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub duplicate_id: Option<String>,
    /// Asset IDs to update (required)
    pub ids: Vec<AssetId>,
    /// Mark as favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Latitude coordinate
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: Option<f64>,
    /// Longitude coordinate
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: Option<f64>,
    /// Rating in range \[1-5\] (starred), -1 (rejected), or null (unrated)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<i64>,
    /// Time zone (IANA timezone)
    #[serde(rename = "timeZone", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<AssetVisibility>,
}

impl AssetBulkUpdateDTO {
    /// Creates a new [`AssetBulkUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        Self {
            date_time_original: None,
            date_time_relative: None,
            description: None,
            duplicate_id: None,
            ids: ids.into_iter().map(Into::into).collect(),
            is_favorite: None,
            latitude: None,
            longitude: None,
            rating: None,
            time_zone: None,
            visibility: None,
        }
    }
    /// Sets `date_time_original`.
    #[must_use]
    pub fn date_time_original(mut self, value: impl Into<String>) -> Self {
        self.date_time_original = Some(value.into());
        self
    }
    /// Sets or clears `date_time_original`.
    #[must_use]
    pub fn set_date_time_original(mut self, value: Option<String>) -> Self {
        self.date_time_original = value;
        self
    }
    /// Sets `date_time_relative`.
    #[must_use]
    pub fn date_time_relative(mut self, value: i64) -> Self {
        self.date_time_relative = Some(value);
        self
    }
    /// Sets or clears `date_time_relative`.
    #[must_use]
    pub fn set_date_time_relative(mut self, value: Option<i64>) -> Self {
        self.date_time_relative = value;
        self
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Sets `duplicate_id`.
    #[must_use]
    pub fn duplicate_id(mut self, value: impl Into<String>) -> Self {
        self.duplicate_id = Some(value.into());
        self
    }
    /// Sets or clears `duplicate_id`.
    #[must_use]
    pub fn set_duplicate_id(mut self, value: Option<String>) -> Self {
        self.duplicate_id = value;
        self
    }
    /// Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }
    /// Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }
    /// Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }
    /// Sets or clears `latitude`.
    #[must_use]
    pub fn set_latitude(mut self, value: Option<f64>) -> Self {
        self.latitude = value;
        self
    }
    /// Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }
    /// Sets or clears `longitude`.
    #[must_use]
    pub fn set_longitude(mut self, value: Option<f64>) -> Self {
        self.longitude = value;
        self
    }
    /// Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: i64) -> Self {
        self.rating = Some(value);
        self
    }
    /// Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<i64>) -> Self {
        self.rating = value;
        self
    }
    /// Sets `time_zone`.
    #[must_use]
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }
    /// Sets or clears `time_zone`.
    #[must_use]
    pub fn set_time_zone(mut self, value: Option<String>) -> Self {
        self.time_zone = value;
        self
    }
    /// Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }
    /// Sets or clears `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }
}

/// `AssetBulkUploadCheckDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `assets` | [`Vec<AssetBulkUploadCheckItem>`](crate::assets::AssetBulkUploadCheckItem) | ✓ | Assets to check |
///
/// See: <https://api.immich.app/models/AssetBulkUploadCheckDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetBulkUploadCheckDTO {
    /// Assets to check (required)
    pub assets: Vec<AssetBulkUploadCheckItem>,
}

impl AssetBulkUploadCheckDTO {
    /// Creates a new [`AssetBulkUploadCheckDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(assets: impl IntoIterator<Item = AssetBulkUploadCheckItem>) -> Self {
        Self {
            assets: assets.into_iter().collect(),
        }
    }
    /// Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: impl IntoIterator<Item = AssetBulkUploadCheckItem>) -> Self {
        self.assets = value.into_iter().collect();
        self
    }
}

/// `AssetBulkUploadCheckItem`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `checksum` | [`String`] | ✓ | Base64 or hex encoded SHA1 hash |
/// | `id` | [`String`] | ✓ | Client-side identifier echoed in the response to match results to inputs (e.g. filename) |
///
/// See: <https://api.immich.app/models/AssetBulkUploadCheckItem>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetBulkUploadCheckItem {
    /// Base64 or hex encoded SHA1 hash (required)
    pub checksum: String,
    /// Client-side identifier echoed in the response to match results to inputs (e.g. filename) (required)
    pub id: String,
}

impl AssetBulkUploadCheckItem {
    /// Creates a new [`AssetBulkUploadCheckItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(checksum: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            checksum: checksum.into(),
            id: id.into(),
        }
    }
    /// Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = value.into();
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
}

/// `AssetBulkUploadCheckResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `results` | [`Vec<AssetBulkUploadCheckResult>`](crate::assets::AssetBulkUploadCheckResult) | ✓ | Upload check results |
///
/// See: <https://api.immich.app/models/AssetBulkUploadCheckResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetBulkUploadCheckResponseDTO {
    /// Upload check results (required)
    pub results: Vec<AssetBulkUploadCheckResult>,
}

impl AssetBulkUploadCheckResponseDTO {
    /// Creates a new [`AssetBulkUploadCheckResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(results: impl IntoIterator<Item = AssetBulkUploadCheckResult>) -> Self {
        Self {
            results: results.into_iter().collect(),
        }
    }
    /// Sets `results`.
    #[must_use]
    pub fn results(mut self, value: impl IntoIterator<Item = AssetBulkUploadCheckResult>) -> Self {
        self.results = value.into_iter().collect();
        self
    }
}

/// `AssetBulkUploadCheckResult`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `action` | [`AssetUploadAction`] | ✓ |  |
/// | `asset_id` | [`AssetId`](crate::id::AssetId) |  | Existing asset ID if duplicate (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `id` | [`String`] | ✓ | Client-side identifier echoed from the request to match results to inputs |
/// | `is_trashed` | `bool` |  | Whether existing asset is trashed |
/// | `reason` | [`AssetRejectReason`] |  |  |
///
/// See: <https://api.immich.app/models/AssetBulkUploadCheckResult>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetBulkUploadCheckResult {
    /// (required)
    pub action: AssetUploadAction,
    /// Existing asset ID if duplicate (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(rename = "assetId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub asset_id: Option<AssetId>,
    /// Client-side identifier echoed from the request to match results to inputs (required)
    pub id: String,
    /// Whether existing asset is trashed
    #[serde(
        rename = "isTrashed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_trashed: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: Option<AssetRejectReason>,
}

impl AssetBulkUploadCheckResult {
    /// Creates a new [`AssetBulkUploadCheckResult`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(action: AssetUploadAction, id: impl Into<String>) -> Self {
        Self {
            action,
            asset_id: None,
            id: id.into(),
            is_trashed: None,
            reason: None,
        }
    }
    /// Sets `action`.
    #[must_use]
    pub fn action(mut self, value: AssetUploadAction) -> Self {
        self.action = value;
        self
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = Some(value.into());
        self
    }
    /// Sets or clears `asset_id`.
    #[must_use]
    pub fn set_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.asset_id = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_trashed`.
    #[must_use]
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = Some(value);
        self
    }
    /// Sets or clears `is_trashed`.
    #[must_use]
    pub fn set_is_trashed(mut self, value: Option<bool>) -> Self {
        self.is_trashed = value;
        self
    }
    /// Sets `reason`.
    #[must_use]
    pub fn reason(mut self, value: AssetRejectReason) -> Self {
        self.reason = Some(value);
        self
    }
    /// Sets or clears `reason`.
    #[must_use]
    pub fn set_reason(mut self, value: Option<AssetRejectReason>) -> Self {
        self.reason = value;
        self
    }
}

/// `AssetCopyDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `albums` | `bool` |  | Copy album associations |
/// | `favorite` | `bool` |  | Copy favorite status |
/// | `shared_links` | `bool` |  | Copy shared links |
/// | `sidecar` | `bool` |  | Copy sidecar file |
/// | `source_id` | [`AssetId`](crate::id::AssetId) | ✓ | Source asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `stack` | `bool` |  | Copy stack association |
/// | `target_id` | [`AssetId`](crate::id::AssetId) | ✓ | Target asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/AssetCopyDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetCopyDTO {
    /// Copy album associations
    #[serde(default = "crate::common::defaults::default_bool::<true>")]
    pub albums: bool,
    /// Copy favorite status
    #[serde(default = "crate::common::defaults::default_bool::<true>")]
    pub favorite: bool,
    /// Copy shared links
    #[serde(rename = "sharedLinks", default = "crate::common::defaults::default_bool::<true>")]
    pub shared_links: bool,
    /// Copy sidecar file
    #[serde(default = "crate::common::defaults::default_bool::<true>")]
    pub sidecar: bool,
    /// Source asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "sourceId")]
    pub source_id: AssetId,
    /// Copy stack association
    #[serde(default = "crate::common::defaults::default_bool::<true>")]
    pub stack: bool,
    /// Target asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "targetId")]
    pub target_id: AssetId,
}

impl AssetCopyDTO {
    /// Creates a new [`AssetCopyDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(source_id: impl Into<AssetId>, target_id: impl Into<AssetId>) -> Self {
        Self {
            albums: crate::common::defaults::default_bool::<true>(),
            favorite: crate::common::defaults::default_bool::<true>(),
            shared_links: crate::common::defaults::default_bool::<true>(),
            sidecar: crate::common::defaults::default_bool::<true>(),
            source_id: source_id.into(),
            stack: crate::common::defaults::default_bool::<true>(),
            target_id: target_id.into(),
        }
    }
    /// Sets `albums`.
    #[must_use]
    pub fn albums(mut self, value: bool) -> Self {
        self.albums = value;
        self
    }
    /// Sets `favorite`.
    #[must_use]
    pub fn favorite(mut self, value: bool) -> Self {
        self.favorite = value;
        self
    }
    /// Sets `shared_links`.
    #[must_use]
    pub fn shared_links(mut self, value: bool) -> Self {
        self.shared_links = value;
        self
    }
    /// Sets `sidecar`.
    #[must_use]
    pub fn sidecar(mut self, value: bool) -> Self {
        self.sidecar = value;
        self
    }
    /// Sets `source_id`.
    #[must_use]
    pub fn source_id(mut self, value: impl Into<AssetId>) -> Self {
        self.source_id = value.into();
        self
    }
    /// Sets `stack`.
    #[must_use]
    pub fn stack(mut self, value: bool) -> Self {
        self.stack = value;
        self
    }
    /// Sets `target_id`.
    #[must_use]
    pub fn target_id(mut self, value: impl Into<AssetId>) -> Self {
        self.target_id = value.into();
        self
    }
}

/// Type of edit action to perform
///
/// See: <https://api.immich.app/models/AssetEditAction>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetEditAction {
    Crop,
    Rotate,
    Mirror,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetEditAction {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Crop => "crop",
            Self::Rotate => "rotate",
            Self::Mirror => "mirror",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetEditAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetEditAction {
    fn from(value: &str) -> Self {
        match value {
            "crop" => Self::Crop,
            "rotate" => Self::Rotate,
            "mirror" => Self::Mirror,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetEditAction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "crop" => Self::Crop,
            "rotate" => Self::Rotate,
            "mirror" => Self::Mirror,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetEditAction {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetEditAction {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetEditAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetEditAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetEditAction;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetEditAction::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetEditAction::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `AssetEditActionItemDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `action` | [`AssetEditAction`] | ✓ |  |
/// | `parameters` | [`CropParameters`] | ✓ | List of edit actions to apply (crop, rotate, or mirror) |
///
/// See: <https://api.immich.app/models/AssetEditActionItemDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetEditActionItemDTO {
    /// (required)
    pub action: AssetEditAction,
    /// List of edit actions to apply (crop, rotate, or mirror) (required)
    pub parameters: AssetEditActionItemParameters,
}

impl AssetEditActionItemDTO {
    /// Creates a new [`AssetEditActionItemDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(action: AssetEditAction, parameters: AssetEditActionItemParameters) -> Self {
        Self { action, parameters }
    }
    /// Sets `action`.
    #[must_use]
    pub fn action(mut self, value: AssetEditAction) -> Self {
        self.action = value;
        self
    }
    /// Sets `parameters`.
    #[must_use]
    pub fn parameters(mut self, value: AssetEditActionItemParameters) -> Self {
        self.parameters = value;
        self
    }
}

/// List of edit actions to apply (crop, rotate, or mirror)
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum AssetEditActionItemParameters {
    CropParameters(CropParameters),
    RotateParameters(RotateParameters),
    MirrorParameters(MirrorParameters),
    /// Unknown variant returned by a newer version of the Immich server.
    Unknown(::serde_json::Value),
}

impl From<CropParameters> for AssetEditActionItemParameters {
    fn from(value: CropParameters) -> Self {
        Self::CropParameters(value)
    }
}

impl From<RotateParameters> for AssetEditActionItemParameters {
    fn from(value: RotateParameters) -> Self {
        Self::RotateParameters(value)
    }
}

impl From<MirrorParameters> for AssetEditActionItemParameters {
    fn from(value: MirrorParameters) -> Self {
        Self::MirrorParameters(value)
    }
}

/// `AssetEditActionItemResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `action` | [`AssetEditAction`] | ✓ |  |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset edit ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `parameters` | [`CropParameters`] | ✓ | List of edit actions to apply (crop, rotate, or mirror) |
///
/// See: <https://api.immich.app/models/AssetEditActionItemResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetEditActionItemResponseDTO {
    /// (required)
    pub action: AssetEditAction,
    /// Asset edit ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: ::uuid::Uuid,
    /// List of edit actions to apply (crop, rotate, or mirror) (required)
    pub parameters: AssetEditActionItemResponseParameters,
}

impl AssetEditActionItemResponseDTO {
    /// Creates a new [`AssetEditActionItemResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(action: AssetEditAction, id: ::uuid::Uuid, parameters: AssetEditActionItemResponseParameters) -> Self {
        Self { action, id, parameters }
    }
    /// Sets `action`.
    #[must_use]
    pub fn action(mut self, value: AssetEditAction) -> Self {
        self.action = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }
    /// Sets `parameters`.
    #[must_use]
    pub fn parameters(mut self, value: AssetEditActionItemResponseParameters) -> Self {
        self.parameters = value;
        self
    }
}

/// List of edit actions to apply (crop, rotate, or mirror)
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum AssetEditActionItemResponseParameters {
    CropParameters(CropParameters),
    RotateParameters(RotateParameters),
    MirrorParameters(MirrorParameters),
    /// Unknown variant returned by a newer version of the Immich server.
    Unknown(::serde_json::Value),
}

impl From<CropParameters> for AssetEditActionItemResponseParameters {
    fn from(value: CropParameters) -> Self {
        Self::CropParameters(value)
    }
}

impl From<RotateParameters> for AssetEditActionItemResponseParameters {
    fn from(value: RotateParameters) -> Self {
        Self::RotateParameters(value)
    }
}

impl From<MirrorParameters> for AssetEditActionItemResponseParameters {
    fn from(value: MirrorParameters) -> Self {
        Self::MirrorParameters(value)
    }
}

/// `AssetEditsCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `edits` | [`Vec<AssetEditActionItemDTO>`](crate::assets::AssetEditActionItemDTO) | ✓ | List of edit actions to apply (crop, rotate, or mirror) |
///
/// See: <https://api.immich.app/models/AssetEditsCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetEditsCreateDTO {
    /// List of edit actions to apply (crop, rotate, or mirror) (required)
    pub edits: Vec<AssetEditActionItemDTO>,
}

impl AssetEditsCreateDTO {
    /// Creates a new [`AssetEditsCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(edits: impl IntoIterator<Item = AssetEditActionItemDTO>) -> Self {
        Self {
            edits: edits.into_iter().collect(),
        }
    }
    /// Sets `edits`.
    #[must_use]
    pub fn edits(mut self, value: impl IntoIterator<Item = AssetEditActionItemDTO>) -> Self {
        self.edits = value.into_iter().collect();
        self
    }
}

/// `AssetEditsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID these edits belong to (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `edits` | [`Vec<AssetEditActionItemResponseDTO>`](crate::assets::AssetEditActionItemResponseDTO) | ✓ | List of edit actions applied to the asset |
///
/// See: <https://api.immich.app/models/AssetEditsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetEditsResponseDTO {
    /// Asset ID these edits belong to (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// List of edit actions applied to the asset (required)
    pub edits: Vec<AssetEditActionItemResponseDTO>,
}

impl AssetEditsResponseDTO {
    /// Creates a new [`AssetEditsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: impl Into<AssetId>, edits: impl IntoIterator<Item = AssetEditActionItemResponseDTO>) -> Self {
        Self {
            asset_id: asset_id.into(),
            edits: edits.into_iter().collect(),
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `edits`.
    #[must_use]
    pub fn edits(mut self, value: impl IntoIterator<Item = AssetEditActionItemResponseDTO>) -> Self {
        self.edits = value.into_iter().collect();
        self
    }
}

/// Job name
///
/// See: <https://api.immich.app/models/AssetJobName>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetJobName {
    RefreshFaces,
    RefreshMetadata,
    RegenerateThumbnail,
    TranscodeVideo,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetJobName {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::RefreshFaces => "refresh-faces",
            Self::RefreshMetadata => "refresh-metadata",
            Self::RegenerateThumbnail => "regenerate-thumbnail",
            Self::TranscodeVideo => "transcode-video",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetJobName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetJobName {
    fn from(value: &str) -> Self {
        match value {
            "refresh-faces" => Self::RefreshFaces,
            "refresh-metadata" => Self::RefreshMetadata,
            "regenerate-thumbnail" => Self::RegenerateThumbnail,
            "transcode-video" => Self::TranscodeVideo,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetJobName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "refresh-faces" => Self::RefreshFaces,
            "refresh-metadata" => Self::RefreshMetadata,
            "regenerate-thumbnail" => Self::RegenerateThumbnail,
            "transcode-video" => Self::TranscodeVideo,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetJobName {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetJobName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetJobName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetJobName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetJobName;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetJobName::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetJobName::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `AssetJobsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<AssetId>`](crate::id::AssetId) | ✓ | Asset IDs |
/// | `name` | [`AssetJobName`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AssetJobsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetJobsDTO {
    /// Asset IDs (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<AssetId>,
    /// (required)
    pub name: AssetJobName,
}

impl AssetJobsDTO {
    /// Creates a new [`AssetJobsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_ids: impl IntoIterator<Item = impl Into<AssetId>>, name: AssetJobName) -> Self {
        Self {
            asset_ids: asset_ids.into_iter().map(Into::into).collect(),
            name,
        }
    }
    /// Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self {
        self.asset_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: AssetJobName) -> Self {
        self.name = value;
        self
    }
}

/// `AssetMediaCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_data` | `Vec<u8>` | ✓ | Asset file data |
/// | `duration` | `i64` |  | Duration in milliseconds (for videos) |
/// | `file_created_at` | [`DateTime`](::chrono::DateTime) | ✓ | File creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `file_modified_at` | [`DateTime`](::chrono::DateTime) | ✓ | File modification date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `filename` | [`String`] |  | Filename |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `live_photo_video_id` | [`AssetId`](crate::id::AssetId) |  | Live photo video ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `metadata` | [`Vec<AssetMetadataUpsertItemDTO>`](crate::assets::AssetMetadataUpsertItemDTO) |  | Asset metadata items |
/// | `sidecar_data` | `Vec<u8>` |  | Sidecar file data |
/// | `visibility` | [`AssetVisibility`] |  |  |
///
/// See: <https://api.immich.app/models/AssetMediaCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetMediaCreateDTO {
    /// Asset file data (required)
    #[serde(rename = "assetData")]
    pub asset_data: Vec<u8>,
    /// Duration in milliseconds (for videos)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: Option<i64>,
    /// File creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "fileCreatedAt")]
    pub file_created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// File modification date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "fileModifiedAt")]
    pub file_modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Filename
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filename: Option<String>,
    /// Mark as favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Live photo video ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(
        rename = "livePhotoVideoId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub live_photo_video_id: Option<AssetId>,
    /// Asset metadata items
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub metadata: Vec<AssetMetadataUpsertItemDTO>,
    /// Sidecar file data
    #[serde(
        rename = "sidecarData",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sidecar_data: Option<Vec<u8>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<AssetVisibility>,
}

impl AssetMediaCreateDTO {
    /// Creates a new [`AssetMediaCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_data: impl Into<Vec<u8>>,
        file_created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        file_modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            asset_data: asset_data.into(),
            duration: None,
            file_created_at,
            file_modified_at,
            filename: None,
            is_favorite: None,
            live_photo_video_id: None,
            metadata: Vec::new(),
            sidecar_data: None,
            visibility: None,
        }
    }
    /// Sets `asset_data`.
    #[must_use]
    pub fn asset_data(mut self, value: impl Into<Vec<u8>>) -> Self {
        self.asset_data = value.into();
        self
    }
    /// Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }
    /// Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<i64>) -> Self {
        self.duration = value;
        self
    }
    /// Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_created_at = value;
        self
    }
    /// Sets `file_modified_at`.
    #[must_use]
    pub fn file_modified_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_modified_at = value;
        self
    }
    /// Sets `filename`.
    #[must_use]
    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }
    /// Sets or clears `filename`.
    #[must_use]
    pub fn set_filename(mut self, value: Option<String>) -> Self {
        self.filename = value;
        self
    }
    /// Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }
    /// Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }
    /// Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: impl Into<AssetId>) -> Self {
        self.live_photo_video_id = Some(value.into());
        self
    }
    /// Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<AssetId>) -> Self {
        self.live_photo_video_id = value;
        self
    }
    /// Sets `metadata`.
    #[must_use]
    pub fn metadata(mut self, value: impl IntoIterator<Item = AssetMetadataUpsertItemDTO>) -> Self {
        self.metadata = value.into_iter().collect();
        self
    }
    /// Sets `sidecar_data`.
    #[must_use]
    pub fn sidecar_data(mut self, value: impl Into<Vec<u8>>) -> Self {
        self.sidecar_data = Some(value.into());
        self
    }
    /// Sets or clears `sidecar_data`.
    #[must_use]
    pub fn set_sidecar_data(mut self, value: Option<Vec<u8>>) -> Self {
        self.sidecar_data = value;
        self
    }
    /// Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }
    /// Sets or clears `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }
}

/// `AssetMediaResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset media ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `status` | [`AssetMediaStatus`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AssetMediaResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetMediaResponseDTO {
    /// Asset media ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: AssetId,
    /// (required)
    pub status: AssetMediaStatus,
}

impl AssetMediaResponseDTO {
    /// Creates a new [`AssetMediaResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: impl Into<AssetId>, status: AssetMediaStatus) -> Self {
        Self { id: id.into(), status }
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<AssetId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `status`.
    #[must_use]
    pub fn status(mut self, value: AssetMediaStatus) -> Self {
        self.status = value;
        self
    }
}

/// Asset media size
///
/// See: <https://api.immich.app/models/AssetMediaSize>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetMediaSize {
    Original,
    Fullsize,
    Preview,
    Thumbnail,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetMediaSize {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Original => "original",
            Self::Fullsize => "fullsize",
            Self::Preview => "preview",
            Self::Thumbnail => "thumbnail",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetMediaSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetMediaSize {
    fn from(value: &str) -> Self {
        match value {
            "original" => Self::Original,
            "fullsize" => Self::Fullsize,
            "preview" => Self::Preview,
            "thumbnail" => Self::Thumbnail,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetMediaSize {
    fn from(value: String) -> Self {
        match value.as_str() {
            "original" => Self::Original,
            "fullsize" => Self::Fullsize,
            "preview" => Self::Preview,
            "thumbnail" => Self::Thumbnail,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetMediaSize {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetMediaSize {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetMediaSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetMediaSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetMediaSize;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetMediaSize::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetMediaSize::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Upload status
///
/// See: <https://api.immich.app/models/AssetMediaStatus>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetMediaStatus {
    Created,
    Duplicate,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetMediaStatus {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Created => "created",
            Self::Duplicate => "duplicate",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetMediaStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetMediaStatus {
    fn from(value: &str) -> Self {
        match value {
            "created" => Self::Created,
            "duplicate" => Self::Duplicate,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetMediaStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "created" => Self::Created,
            "duplicate" => Self::Duplicate,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetMediaStatus {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetMediaStatus {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetMediaStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetMediaStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetMediaStatus;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetMediaStatus::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetMediaStatus::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `AssetMetadataBulkDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `items` | [`Vec<AssetMetadataBulkDeleteItemDTO>`](crate::assets::AssetMetadataBulkDeleteItemDTO) | ✓ | Metadata items to delete |
///
/// See: <https://api.immich.app/models/AssetMetadataBulkDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetMetadataBulkDeleteDTO {
    /// Metadata items to delete (required)
    pub items: Vec<AssetMetadataBulkDeleteItemDTO>,
}

impl AssetMetadataBulkDeleteDTO {
    /// Creates a new [`AssetMetadataBulkDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(items: impl IntoIterator<Item = AssetMetadataBulkDeleteItemDTO>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
    /// Sets `items`.
    #[must_use]
    pub fn items(mut self, value: impl IntoIterator<Item = AssetMetadataBulkDeleteItemDTO>) -> Self {
        self.items = value.into_iter().collect();
        self
    }
}

/// `AssetMetadataBulkDeleteItemDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `key` | [`String`] | ✓ | Metadata key |
///
/// See: <https://api.immich.app/models/AssetMetadataBulkDeleteItemDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetMetadataBulkDeleteItemDTO {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Metadata key (required)
    pub key: String,
}

impl AssetMetadataBulkDeleteItemDTO {
    /// Creates a new [`AssetMetadataBulkDeleteItemDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: impl Into<AssetId>, key: impl Into<String>) -> Self {
        Self {
            asset_id: asset_id.into(),
            key: key.into(),
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }
}

/// `AssetMetadataBulkResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `key` | [`String`] | ✓ | Metadata key |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `value` | `object` | ✓ | Metadata value (object) |
///
/// See: <https://api.immich.app/models/AssetMetadataBulkResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetMetadataBulkResponseDTO {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Metadata key (required)
    pub key: String,
    /// Last update date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Metadata value (object) (required)
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl AssetMetadataBulkResponseDTO {
    /// Creates a new [`AssetMetadataBulkResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: impl Into<AssetId>,
        key: impl Into<String>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        value: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self {
            asset_id: asset_id.into(),
            key: key.into(),
            updated_at,
            value,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
    /// Sets `value`.
    #[must_use]
    pub fn value(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.value = value;
        self
    }
}

/// `AssetMetadataBulkUpsertDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `items` | [`Vec<AssetMetadataBulkUpsertItemDTO>`](crate::assets::AssetMetadataBulkUpsertItemDTO) | ✓ | Metadata items to upsert |
///
/// See: <https://api.immich.app/models/AssetMetadataBulkUpsertDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetMetadataBulkUpsertDTO {
    /// Metadata items to upsert (required)
    pub items: Vec<AssetMetadataBulkUpsertItemDTO>,
}

impl AssetMetadataBulkUpsertDTO {
    /// Creates a new [`AssetMetadataBulkUpsertDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(items: impl IntoIterator<Item = AssetMetadataBulkUpsertItemDTO>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
    /// Sets `items`.
    #[must_use]
    pub fn items(mut self, value: impl IntoIterator<Item = AssetMetadataBulkUpsertItemDTO>) -> Self {
        self.items = value.into_iter().collect();
        self
    }
}

/// `AssetMetadataBulkUpsertItemDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `key` | [`String`] | ✓ | Metadata key |
/// | `value` | `object` | ✓ | Metadata value (object) |
///
/// See: <https://api.immich.app/models/AssetMetadataBulkUpsertItemDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetMetadataBulkUpsertItemDTO {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Metadata key (required)
    pub key: String,
    /// Metadata value (object) (required)
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl AssetMetadataBulkUpsertItemDTO {
    /// Creates a new [`AssetMetadataBulkUpsertItemDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: impl Into<AssetId>,
        key: impl Into<String>,
        value: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self {
            asset_id: asset_id.into(),
            key: key.into(),
            value,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }
    /// Sets `value`.
    #[must_use]
    pub fn value(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.value = value;
        self
    }
}

/// `AssetMetadataResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `key` | [`String`] | ✓ | Metadata key |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `value` | `object` | ✓ | Metadata value (object) |
///
/// See: <https://api.immich.app/models/AssetMetadataResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetMetadataResponseDTO {
    /// Metadata key (required)
    pub key: String,
    /// Last update date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Metadata value (object) (required)
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl AssetMetadataResponseDTO {
    /// Creates a new [`AssetMetadataResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        key: impl Into<String>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        value: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self {
            key: key.into(),
            updated_at,
            value,
        }
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
    /// Sets `value`.
    #[must_use]
    pub fn value(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.value = value;
        self
    }
}

/// `AssetMetadataUpsertDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `items` | [`Vec<AssetMetadataUpsertItemDTO>`](crate::assets::AssetMetadataUpsertItemDTO) | ✓ | Metadata items to upsert |
///
/// See: <https://api.immich.app/models/AssetMetadataUpsertDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetMetadataUpsertDTO {
    /// Metadata items to upsert (required)
    pub items: Vec<AssetMetadataUpsertItemDTO>,
}

impl AssetMetadataUpsertDTO {
    /// Creates a new [`AssetMetadataUpsertDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(items: impl IntoIterator<Item = AssetMetadataUpsertItemDTO>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
    /// Sets `items`.
    #[must_use]
    pub fn items(mut self, value: impl IntoIterator<Item = AssetMetadataUpsertItemDTO>) -> Self {
        self.items = value.into_iter().collect();
        self
    }
}

/// `AssetMetadataUpsertItemDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `key` | [`String`] | ✓ | Metadata key |
/// | `value` | `object` | ✓ | Metadata value (object) |
///
/// See: <https://api.immich.app/models/AssetMetadataUpsertItemDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetMetadataUpsertItemDTO {
    /// Metadata key (required)
    pub key: String,
    /// Metadata value (object) (required)
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl AssetMetadataUpsertItemDTO {
    /// Creates a new [`AssetMetadataUpsertItemDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(key: impl Into<String>, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self { key: key.into(), value }
    }
    /// Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }
    /// Sets `value`.
    #[must_use]
    pub fn value(mut self, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        self.value = value;
        self
    }
}

/// `AssetOcrResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$` |
/// | `box_score` | `f64` | ✓ | Confidence score for text detection box |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$` |
/// | `text` | [`String`] | ✓ | Recognized text |
/// | `text_score` | `f64` | ✓ | Confidence score for text recognition |
/// | `x1` | `f64` | ✓ | Normalized x coordinate of box corner 1 (0-1) |
/// | `x2` | `f64` | ✓ | Normalized x coordinate of box corner 2 (0-1) |
/// | `x3` | `f64` | ✓ | Normalized x coordinate of box corner 3 (0-1) |
/// | `x4` | `f64` | ✓ | Normalized x coordinate of box corner 4 (0-1) |
/// | `y1` | `f64` | ✓ | Normalized y coordinate of box corner 1 (0-1) |
/// | `y2` | `f64` | ✓ | Normalized y coordinate of box corner 2 (0-1) |
/// | `y3` | `f64` | ✓ | Normalized y coordinate of box corner 3 (0-1) |
/// | `y4` | `f64` | ✓ | Normalized y coordinate of box corner 4 (0-1) |
///
/// See: <https://api.immich.app/models/AssetOcrResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetOcrResponseDTO {
    /// Pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$` (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Confidence score for text detection box (required)
    #[serde(rename = "boxScore")]
    pub box_score: f64,
    /// Pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$` (required)
    pub id: ::uuid::Uuid,
    /// Recognized text (required)
    pub text: String,
    /// Confidence score for text recognition (required)
    #[serde(rename = "textScore")]
    pub text_score: f64,
    /// Normalized x coordinate of box corner 1 (0-1) (required)
    pub x1: f64,
    /// Normalized x coordinate of box corner 2 (0-1) (required)
    pub x2: f64,
    /// Normalized x coordinate of box corner 3 (0-1) (required)
    pub x3: f64,
    /// Normalized x coordinate of box corner 4 (0-1) (required)
    pub x4: f64,
    /// Normalized y coordinate of box corner 1 (0-1) (required)
    pub y1: f64,
    /// Normalized y coordinate of box corner 2 (0-1) (required)
    pub y2: f64,
    /// Normalized y coordinate of box corner 3 (0-1) (required)
    pub y3: f64,
    /// Normalized y coordinate of box corner 4 (0-1) (required)
    pub y4: f64,
}

impl AssetOcrResponseDTO {
    /// Creates a new [`AssetOcrResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: impl Into<AssetId>,
        box_score: f64,
        id: ::uuid::Uuid,
        text: impl Into<String>,
        text_score: f64,
        x1: f64,
        x2: f64,
        x3: f64,
        x4: f64,
        y1: f64,
        y2: f64,
        y3: f64,
        y4: f64,
    ) -> Self {
        Self {
            asset_id: asset_id.into(),
            box_score,
            id,
            text: text.into(),
            text_score,
            x1,
            x2,
            x3,
            x4,
            y1,
            y2,
            y3,
            y4,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
        self
    }
    /// Sets `box_score`.
    #[must_use]
    pub fn box_score(mut self, value: f64) -> Self {
        self.box_score = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }
    /// Sets `text`.
    #[must_use]
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = value.into();
        self
    }
    /// Sets `text_score`.
    #[must_use]
    pub fn text_score(mut self, value: f64) -> Self {
        self.text_score = value;
        self
    }
    /// Sets `x1`.
    #[must_use]
    pub fn x1(mut self, value: f64) -> Self {
        self.x1 = value;
        self
    }
    /// Sets `x2`.
    #[must_use]
    pub fn x2(mut self, value: f64) -> Self {
        self.x2 = value;
        self
    }
    /// Sets `x3`.
    #[must_use]
    pub fn x3(mut self, value: f64) -> Self {
        self.x3 = value;
        self
    }
    /// Sets `x4`.
    #[must_use]
    pub fn x4(mut self, value: f64) -> Self {
        self.x4 = value;
        self
    }
    /// Sets `y1`.
    #[must_use]
    pub fn y1(mut self, value: f64) -> Self {
        self.y1 = value;
        self
    }
    /// Sets `y2`.
    #[must_use]
    pub fn y2(mut self, value: f64) -> Self {
        self.y2 = value;
        self
    }
    /// Sets `y3`.
    #[must_use]
    pub fn y3(mut self, value: f64) -> Self {
        self.y3 = value;
        self
    }
    /// Sets `y4`.
    #[must_use]
    pub fn y4(mut self, value: f64) -> Self {
        self.y4 = value;
        self
    }
}

/// Rejection reason if rejected
///
/// See: <https://api.immich.app/models/AssetRejectReason>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetRejectReason {
    Duplicate,
    UnsupportedFormat,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetRejectReason {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Duplicate => "duplicate",
            Self::UnsupportedFormat => "unsupported-format",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetRejectReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetRejectReason {
    fn from(value: &str) -> Self {
        match value {
            "duplicate" => Self::Duplicate,
            "unsupported-format" => Self::UnsupportedFormat,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetRejectReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "duplicate" => Self::Duplicate,
            "unsupported-format" => Self::UnsupportedFormat,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetRejectReason {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetRejectReason {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetRejectReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetRejectReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetRejectReason;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetRejectReason::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetRejectReason::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `AssetResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `checksum` | [`String`] | ✓ | Base64 encoded SHA1 hash |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | The UTC timestamp when the asset was originally uploaded to Immich. |
/// | `duplicate_id` | [`DuplicateId`](crate::id::DuplicateId) |  | Duplicate group ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `duration` | `i64` | ✓ | Video/gif duration in milliseconds (null for static images) |
/// | `exif_info` | [`ExifResponseDTO`] |  |  |
/// | `file_created_at` | [`DateTime`](::chrono::DateTime) | ✓ | The actual UTC timestamp when the file was created/captured, preserving timezone information. This is the authoritative timestamp for chronological sorting within timeline groups. Combined with timezone data, this can be used to determine the exact moment the photo was taken. |
/// | `file_modified_at` | [`DateTime`](::chrono::DateTime) | ✓ | The UTC timestamp when the file was last modified on the filesystem. This reflects the last time the physical file was changed, which may be different from when the photo was originally taken. |
/// | `has_metadata` | `bool` | ✓ | Whether asset has metadata |
/// | `height` | `i64` | ✓ | Asset height |
/// | `id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_archived` | `bool` | ✓ | Is archived |
/// | `is_edited` | `bool` | ✓ | Is edited |
/// | `is_favorite` | `bool` | ✓ | Is favorite |
/// | `is_offline` | `bool` | ✓ | Is offline |
/// | `is_trashed` | `bool` | ✓ | Is trashed |
/// | `library_id` | [`LibraryId`](crate::id::LibraryId) |  | Library ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `live_photo_video_id` | [`AssetId`](crate::id::AssetId) |  | Live photo video ID |
/// | `local_date_time` | [`DateTime`](::chrono::DateTime) | ✓ | The local date and time when the photo/video was taken, derived from EXIF metadata. This represents the photographer's local time regardless of timezone, stored as a timezone-agnostic timestamp. Used for timeline grouping by "local" days and months. |
/// | `original_file_name` | [`String`] | ✓ | Original file name |
/// | `original_mime_type` | [`String`] |  | Original MIME type |
/// | `original_path` | [`String`] | ✓ | Original file path |
/// | `owner` | [`UserResponseDTO`] |  |  |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `people` | [`Vec<PersonResponseDTO>`](crate::assets::PersonResponseDTO) |  |  |
/// | `resized` | `bool` |  | Is resized |
/// | `stack` | [`AssetStackResponseDTO`] |  |  |
/// | `tags` | [`Vec<TagResponseDTO>`](crate::assets::TagResponseDTO) |  |  |
/// | `thumbhash` | [`String`] | ✓ | Thumbhash for thumbnail generation (base64) also used as the c query param for thumbnail cache busting. |
/// | `type_` | [`AssetTypeEnum`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | The UTC timestamp when the asset record was last updated in the database. This is automatically maintained by the database and reflects when any field in the asset was last modified. |
/// | `visibility` | [`AssetVisibility`] | ✓ |  |
/// | `width` | `i64` | ✓ | Asset width |
///
/// See: <https://api.immich.app/models/AssetResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AssetResponseDTO {
    /// Base64 encoded SHA1 hash (required)
    pub checksum: String,
    /// The UTC timestamp when the asset was originally uploaded to Immich. (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Duplicate group ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(
        rename = "duplicateId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub duplicate_id: Option<DuplicateId>,
    /// Video/gif duration in milliseconds (null for static images) (required)
    pub duration: Option<i64>,
    #[serde(rename = "exifInfo", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exif_info: Option<ExifResponseDTO>,
    /// The actual UTC timestamp when the file was created/captured, preserving timezone information. This is the authoritative timestamp for chronological sorting within timeline groups. Combined with timezone data, this can be used to determine the exact moment the photo was taken. (required)
    #[serde(rename = "fileCreatedAt")]
    pub file_created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// The UTC timestamp when the file was last modified on the filesystem. This reflects the last time the physical file was changed, which may be different from when the photo was originally taken. (required)
    #[serde(rename = "fileModifiedAt")]
    pub file_modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Whether asset has metadata (required)
    #[serde(rename = "hasMetadata")]
    pub has_metadata: bool,
    /// Asset height (required)
    pub height: Option<i64>,
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: AssetId,
    /// Is archived (required)
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    /// Is edited (required)
    #[serde(rename = "isEdited")]
    pub is_edited: bool,
    /// Is favorite (required)
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    /// Is offline (required)
    #[serde(rename = "isOffline")]
    pub is_offline: bool,
    /// Is trashed (required)
    #[serde(rename = "isTrashed")]
    pub is_trashed: bool,
    /// Library ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(
        rename = "libraryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub library_id: Option<LibraryId>,
    /// Live photo video ID
    #[serde(
        rename = "livePhotoVideoId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub live_photo_video_id: Option<String>,
    /// The local date and time when the photo/video was taken, derived from EXIF metadata. This represents the photographer's local time regardless of timezone, stored as a timezone-agnostic timestamp. Used for timeline grouping by "local" days and months. (required)
    #[serde(rename = "localDateTime")]
    pub local_date_time: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Original file name (required)
    #[serde(rename = "originalFileName")]
    pub original_file_name: String,
    /// Original MIME type
    #[serde(
        rename = "originalMimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_mime_type: Option<String>,
    /// Original file path (required)
    #[serde(rename = "originalPath")]
    pub original_path: String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub owner: Option<UserResponseDTO>,
    /// Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub people: Vec<PersonResponseDTO>,
    /// Is resized
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resized: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stack: Option<AssetStackResponseDTO>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: Vec<TagResponseDTO>,
    /// Thumbhash for thumbnail generation (base64) also used as the c query param for thumbnail cache busting. (required)
    pub thumbhash: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: AssetTypeEnum,
    /// The UTC timestamp when the asset record was last updated in the database. This is automatically maintained by the database and reflects when any field in the asset was last modified. (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// (required)
    pub visibility: AssetVisibility,
    /// Asset width (required)
    pub width: Option<i64>,
}

impl AssetResponseDTO {
    /// Creates a new [`AssetResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        checksum: impl Into<String>,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        file_created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        file_modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
        has_metadata: bool,
        id: impl Into<AssetId>,
        is_archived: bool,
        is_edited: bool,
        is_favorite: bool,
        is_offline: bool,
        is_trashed: bool,
        local_date_time: ::chrono::DateTime<::chrono::offset::Utc>,
        original_file_name: impl Into<String>,
        original_path: impl Into<String>,
        owner_id: impl Into<UserId>,
        type_: AssetTypeEnum,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        visibility: AssetVisibility,
    ) -> Self {
        Self {
            checksum: checksum.into(),
            created_at,
            duplicate_id: None,
            duration: None,
            exif_info: None,
            file_created_at,
            file_modified_at,
            has_metadata,
            height: None,
            id: id.into(),
            is_archived,
            is_edited,
            is_favorite,
            is_offline,
            is_trashed,
            library_id: None,
            live_photo_video_id: None,
            local_date_time,
            original_file_name: original_file_name.into(),
            original_mime_type: None,
            original_path: original_path.into(),
            owner: None,
            owner_id: owner_id.into(),
            people: Vec::new(),
            resized: None,
            stack: None,
            tags: Vec::new(),
            thumbhash: None,
            type_,
            updated_at,
            visibility,
            width: None,
        }
    }
    /// Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = value.into();
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `duplicate_id`.
    #[must_use]
    pub fn duplicate_id(mut self, value: impl Into<DuplicateId>) -> Self {
        self.duplicate_id = Some(value.into());
        self
    }
    /// Sets or clears `duplicate_id`.
    #[must_use]
    pub fn set_duplicate_id(mut self, value: Option<DuplicateId>) -> Self {
        self.duplicate_id = value;
        self
    }
    /// Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }
    /// Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<i64>) -> Self {
        self.duration = value;
        self
    }
    /// Sets `exif_info`.
    #[must_use]
    pub fn exif_info(mut self, value: ExifResponseDTO) -> Self {
        self.exif_info = Some(value);
        self
    }
    /// Sets or clears `exif_info`.
    #[must_use]
    pub fn set_exif_info(mut self, value: Option<ExifResponseDTO>) -> Self {
        self.exif_info = value;
        self
    }
    /// Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_created_at = value;
        self
    }
    /// Sets `file_modified_at`.
    #[must_use]
    pub fn file_modified_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_modified_at = value;
        self
    }
    /// Sets `has_metadata`.
    #[must_use]
    pub fn has_metadata(mut self, value: bool) -> Self {
        self.has_metadata = value;
        self
    }
    /// Sets `height`.
    #[must_use]
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }
    /// Sets or clears `height`.
    #[must_use]
    pub fn set_height(mut self, value: Option<i64>) -> Self {
        self.height = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<AssetId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_archived`.
    #[must_use]
    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = value;
        self
    }
    /// Sets `is_edited`.
    #[must_use]
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = value;
        self
    }
    /// Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = value;
        self
    }
    /// Sets `is_offline`.
    #[must_use]
    pub fn is_offline(mut self, value: bool) -> Self {
        self.is_offline = value;
        self
    }
    /// Sets `is_trashed`.
    #[must_use]
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = value;
        self
    }
    /// Sets `library_id`.
    #[must_use]
    pub fn library_id(mut self, value: impl Into<LibraryId>) -> Self {
        self.library_id = Some(value.into());
        self
    }
    /// Sets or clears `library_id`.
    #[must_use]
    pub fn set_library_id(mut self, value: Option<LibraryId>) -> Self {
        self.library_id = value;
        self
    }
    /// Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: impl Into<String>) -> Self {
        self.live_photo_video_id = Some(value.into());
        self
    }
    /// Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<String>) -> Self {
        self.live_photo_video_id = value;
        self
    }
    /// Sets `local_date_time`.
    #[must_use]
    pub fn local_date_time(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.local_date_time = value;
        self
    }
    /// Sets `original_file_name`.
    #[must_use]
    pub fn original_file_name(mut self, value: impl Into<String>) -> Self {
        self.original_file_name = value.into();
        self
    }
    /// Sets `original_mime_type`.
    #[must_use]
    pub fn original_mime_type(mut self, value: impl Into<String>) -> Self {
        self.original_mime_type = Some(value.into());
        self
    }
    /// Sets or clears `original_mime_type`.
    #[must_use]
    pub fn set_original_mime_type(mut self, value: Option<String>) -> Self {
        self.original_mime_type = value;
        self
    }
    /// Sets `original_path`.
    #[must_use]
    pub fn original_path(mut self, value: impl Into<String>) -> Self {
        self.original_path = value.into();
        self
    }
    /// Sets `owner`.
    #[must_use]
    pub fn owner(mut self, value: UserResponseDTO) -> Self {
        self.owner = Some(value);
        self
    }
    /// Sets or clears `owner`.
    #[must_use]
    pub fn set_owner(mut self, value: Option<UserResponseDTO>) -> Self {
        self.owner = value;
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `people`.
    #[must_use]
    pub fn people(mut self, value: impl IntoIterator<Item = PersonResponseDTO>) -> Self {
        self.people = value.into_iter().collect();
        self
    }
    /// Sets `resized`.
    #[must_use]
    pub fn resized(mut self, value: bool) -> Self {
        self.resized = Some(value);
        self
    }
    /// Sets or clears `resized`.
    #[must_use]
    pub fn set_resized(mut self, value: Option<bool>) -> Self {
        self.resized = value;
        self
    }
    /// Sets `stack`.
    #[must_use]
    pub fn stack(mut self, value: AssetStackResponseDTO) -> Self {
        self.stack = Some(value);
        self
    }
    /// Sets or clears `stack`.
    #[must_use]
    pub fn set_stack(mut self, value: Option<AssetStackResponseDTO>) -> Self {
        self.stack = value;
        self
    }
    /// Sets `tags`.
    #[must_use]
    pub fn tags(mut self, value: impl IntoIterator<Item = TagResponseDTO>) -> Self {
        self.tags = value.into_iter().collect();
        self
    }
    /// Sets `thumbhash`.
    #[must_use]
    pub fn thumbhash(mut self, value: impl Into<String>) -> Self {
        self.thumbhash = Some(value.into());
        self
    }
    /// Sets or clears `thumbhash`.
    #[must_use]
    pub fn set_thumbhash(mut self, value: Option<String>) -> Self {
        self.thumbhash = value;
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = value;
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
    /// Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = value;
        self
    }
    /// Sets `width`.
    #[must_use]
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }
    /// Sets or clears `width`.
    #[must_use]
    pub fn set_width(mut self, value: Option<i64>) -> Self {
        self.width = value;
        self
    }
}

/// `AssetStackResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_count` | `i64` | ✓ | Number of assets in stack |
/// | `id` | [`StackId`](crate::id::StackId) | ✓ | Stack ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `primary_asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Primary asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/AssetStackResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetStackResponseDTO {
    /// Number of assets in stack (required)
    #[serde(rename = "assetCount")]
    pub asset_count: i64,
    /// Stack ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: StackId,
    /// Primary asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "primaryAssetId")]
    pub primary_asset_id: AssetId,
}

impl AssetStackResponseDTO {
    /// Creates a new [`AssetStackResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_count: i64, id: impl Into<StackId>, primary_asset_id: impl Into<AssetId>) -> Self {
        Self {
            asset_count,
            id: id.into(),
            primary_asset_id: primary_asset_id.into(),
        }
    }
    /// Sets `asset_count`.
    #[must_use]
    pub fn asset_count(mut self, value: i64) -> Self {
        self.asset_count = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<StackId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `primary_asset_id`.
    #[must_use]
    pub fn primary_asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.primary_asset_id = value.into();
        self
    }
}

/// `AssetStatsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `images` | `i64` | ✓ | Number of images |
/// | `total` | `i64` | ✓ | Total number of assets |
/// | `videos` | `i64` | ✓ | Number of videos |
///
/// See: <https://api.immich.app/models/AssetStatsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetStatsResponseDTO {
    /// Number of images (required)
    pub images: i64,
    /// Total number of assets (required)
    pub total: i64,
    /// Number of videos (required)
    pub videos: i64,
}

impl AssetStatsResponseDTO {
    /// Creates a new [`AssetStatsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(images: i64, total: i64, videos: i64) -> Self {
        Self { images, total, videos }
    }
    /// Sets `images`.
    #[must_use]
    pub fn images(mut self, value: i64) -> Self {
        self.images = value;
        self
    }
    /// Sets `total`.
    #[must_use]
    pub fn total(mut self, value: i64) -> Self {
        self.total = value;
        self
    }
    /// Sets `videos`.
    #[must_use]
    pub fn videos(mut self, value: i64) -> Self {
        self.videos = value;
        self
    }
}

/// Asset type
///
/// See: <https://api.immich.app/models/AssetTypeEnum>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetTypeEnum {
    Image,
    Video,
    Audio,
    Other,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetTypeEnum {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Image => "IMAGE",
            Self::Video => "VIDEO",
            Self::Audio => "AUDIO",
            Self::Other => "OTHER",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetTypeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetTypeEnum {
    fn from(value: &str) -> Self {
        match value {
            "IMAGE" => Self::Image,
            "VIDEO" => Self::Video,
            "AUDIO" => Self::Audio,
            "OTHER" => Self::Other,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetTypeEnum {
    fn from(value: String) -> Self {
        match value.as_str() {
            "IMAGE" => Self::Image,
            "VIDEO" => Self::Video,
            "AUDIO" => Self::Audio,
            "OTHER" => Self::Other,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetTypeEnum {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetTypeEnum {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetTypeEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetTypeEnum;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetTypeEnum::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetTypeEnum::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Upload action
///
/// See: <https://api.immich.app/models/AssetUploadAction>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetUploadAction {
    Accept,
    Reject,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetUploadAction {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Accept => "accept",
            Self::Reject => "reject",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetUploadAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetUploadAction {
    fn from(value: &str) -> Self {
        match value {
            "accept" => Self::Accept,
            "reject" => Self::Reject,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetUploadAction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "accept" => Self::Accept,
            "reject" => Self::Reject,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetUploadAction {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetUploadAction {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetUploadAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetUploadAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetUploadAction;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetUploadAction::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetUploadAction::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// Asset visibility
///
/// See: <https://api.immich.app/models/AssetVisibility>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetVisibility {
    Archive,
    Timeline,
    Hidden,
    Locked,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl AssetVisibility {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Archive => "archive",
            Self::Timeline => "timeline",
            Self::Hidden => "hidden",
            Self::Locked => "locked",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for AssetVisibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for AssetVisibility {
    fn from(value: &str) -> Self {
        match value {
            "archive" => Self::Archive,
            "timeline" => Self::Timeline,
            "hidden" => Self::Hidden,
            "locked" => Self::Locked,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for AssetVisibility {
    fn from(value: String) -> Self {
        match value.as_str() {
            "archive" => Self::Archive,
            "timeline" => Self::Timeline,
            "hidden" => Self::Hidden,
            "locked" => Self::Locked,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for AssetVisibility {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for AssetVisibility {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for AssetVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = AssetVisibility;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetVisibility::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(AssetVisibility::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `CropParameters`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `height` | `i64` | ✓ | Height of the crop |
/// | `width` | `i64` | ✓ | Width of the crop |
/// | `x` | `i64` | ✓ | Top-Left X coordinate of crop |
/// | `y` | `i64` | ✓ | Top-Left Y coordinate of crop |
///
/// See: <https://api.immich.app/models/CropParameters>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CropParameters {
    /// Height of the crop (required)
    pub height: ::std::num::NonZeroU64,
    /// Width of the crop (required)
    pub width: ::std::num::NonZeroU64,
    /// Top-Left X coordinate of crop (required)
    pub x: i64,
    /// Top-Left Y coordinate of crop (required)
    pub y: i64,
}

impl CropParameters {
    /// Creates a new [`CropParameters`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(height: ::std::num::NonZeroU64, width: ::std::num::NonZeroU64, x: i64, y: i64) -> Self {
        Self { height, width, x, y }
    }
    /// Sets `height`.
    #[must_use]
    pub fn height(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.height = value;
        self
    }
    /// Sets `width`.
    #[must_use]
    pub fn width(mut self, value: ::std::num::NonZeroU64) -> Self {
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

/// EXIF response
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `city` | [`String`] |  | City name |
/// | `country` | [`String`] |  | Country name |
/// | `date_time_original` | [`DateTime`](::chrono::DateTime) |  | Original date/time |
/// | `description` | [`String`] |  | Image description |
/// | `exif_image_height` | `i64` |  | Image height in pixels |
/// | `exif_image_width` | `i64` |  | Image width in pixels |
/// | `exposure_time` | [`String`] |  | Exposure time |
/// | `f_number` | `f64` |  | F-number (aperture) |
/// | `file_size_in_byte` | `i64` |  | File size in bytes |
/// | `focal_length` | `f64` |  | Focal length in mm |
/// | `iso` | `i64` |  | ISO sensitivity |
/// | `latitude` | `f64` |  | GPS latitude |
/// | `lens_model` | [`String`] |  | Lens model |
/// | `longitude` | `f64` |  | GPS longitude |
/// | `make` | [`String`] |  | Camera make |
/// | `model` | [`String`] |  | Camera model |
/// | `modify_date` | [`DateTime`](::chrono::DateTime) |  | Modification date/time |
/// | `orientation` | [`String`] |  | Image orientation |
/// | `projection_type` | [`String`] |  | Projection type |
/// | `rating` | `i64` |  | Rating |
/// | `state` | [`String`] |  | State/province name |
/// | `time_zone` | [`String`] |  | Time zone |
///
/// See: <https://api.immich.app/models/ExifResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub struct ExifResponseDTO {
    /// City name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: Option<String>,
    /// Country name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: Option<String>,
    /// Original date/time
    #[serde(
        rename = "dateTimeOriginal",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_time_original: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Image description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Image height in pixels
    #[serde(
        rename = "exifImageHeight",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exif_image_height: Option<i64>,
    /// Image width in pixels
    #[serde(
        rename = "exifImageWidth",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exif_image_width: Option<i64>,
    /// Exposure time
    #[serde(
        rename = "exposureTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exposure_time: Option<String>,
    /// F-number (aperture)
    #[serde(rename = "fNumber", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub f_number: Option<f64>,
    /// File size in bytes
    #[serde(
        rename = "fileSizeInByte",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_size_in_byte: Option<i64>,
    /// Focal length in mm
    #[serde(
        rename = "focalLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub focal_length: Option<f64>,
    /// ISO sensitivity
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub iso: Option<i64>,
    /// GPS latitude
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: Option<f64>,
    /// Lens model
    #[serde(
        rename = "lensModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lens_model: Option<String>,
    /// GPS longitude
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: Option<f64>,
    /// Camera make
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub make: Option<String>,
    /// Camera model
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: Option<String>,
    /// Modification date/time
    #[serde(
        rename = "modifyDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modify_date: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Image orientation
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orientation: Option<String>,
    /// Projection type
    #[serde(
        rename = "projectionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub projection_type: Option<String>,
    /// Rating
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<::std::num::NonZeroU64>,
    /// State/province name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
    /// Time zone
    #[serde(rename = "timeZone", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time_zone: Option<String>,
}

impl ExifResponseDTO {
    /// Creates a new [`ExifResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            date_time_original: None,
            description: None,
            exif_image_height: None,
            exif_image_width: None,
            exposure_time: None,
            f_number: None,
            file_size_in_byte: None,
            focal_length: None,
            iso: None,
            latitude: None,
            lens_model: None,
            longitude: None,
            make: None,
            model: None,
            modify_date: None,
            orientation: None,
            projection_type: None,
            rating: None,
            state: None,
            time_zone: None,
        }
    }
    /// Sets `city`.
    #[must_use]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }
    /// Sets or clears `city`.
    #[must_use]
    pub fn set_city(mut self, value: Option<String>) -> Self {
        self.city = value;
        self
    }
    /// Sets `country`.
    #[must_use]
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }
    /// Sets or clears `country`.
    #[must_use]
    pub fn set_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }
    /// Sets `date_time_original`.
    #[must_use]
    pub fn date_time_original(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.date_time_original = Some(value);
        self
    }
    /// Sets or clears `date_time_original`.
    #[must_use]
    pub fn set_date_time_original(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.date_time_original = value;
        self
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Sets `exif_image_height`.
    #[must_use]
    pub fn exif_image_height(mut self, value: i64) -> Self {
        self.exif_image_height = Some(value);
        self
    }
    /// Sets or clears `exif_image_height`.
    #[must_use]
    pub fn set_exif_image_height(mut self, value: Option<i64>) -> Self {
        self.exif_image_height = value;
        self
    }
    /// Sets `exif_image_width`.
    #[must_use]
    pub fn exif_image_width(mut self, value: i64) -> Self {
        self.exif_image_width = Some(value);
        self
    }
    /// Sets or clears `exif_image_width`.
    #[must_use]
    pub fn set_exif_image_width(mut self, value: Option<i64>) -> Self {
        self.exif_image_width = value;
        self
    }
    /// Sets `exposure_time`.
    #[must_use]
    pub fn exposure_time(mut self, value: impl Into<String>) -> Self {
        self.exposure_time = Some(value.into());
        self
    }
    /// Sets or clears `exposure_time`.
    #[must_use]
    pub fn set_exposure_time(mut self, value: Option<String>) -> Self {
        self.exposure_time = value;
        self
    }
    /// Sets `f_number`.
    #[must_use]
    pub fn f_number(mut self, value: f64) -> Self {
        self.f_number = Some(value);
        self
    }
    /// Sets or clears `f_number`.
    #[must_use]
    pub fn set_f_number(mut self, value: Option<f64>) -> Self {
        self.f_number = value;
        self
    }
    /// Sets `file_size_in_byte`.
    #[must_use]
    pub fn file_size_in_byte(mut self, value: i64) -> Self {
        self.file_size_in_byte = Some(value);
        self
    }
    /// Sets or clears `file_size_in_byte`.
    #[must_use]
    pub fn set_file_size_in_byte(mut self, value: Option<i64>) -> Self {
        self.file_size_in_byte = value;
        self
    }
    /// Sets `focal_length`.
    #[must_use]
    pub fn focal_length(mut self, value: f64) -> Self {
        self.focal_length = Some(value);
        self
    }
    /// Sets or clears `focal_length`.
    #[must_use]
    pub fn set_focal_length(mut self, value: Option<f64>) -> Self {
        self.focal_length = value;
        self
    }
    /// Sets `iso`.
    #[must_use]
    pub fn iso(mut self, value: i64) -> Self {
        self.iso = Some(value);
        self
    }
    /// Sets or clears `iso`.
    #[must_use]
    pub fn set_iso(mut self, value: Option<i64>) -> Self {
        self.iso = value;
        self
    }
    /// Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }
    /// Sets or clears `latitude`.
    #[must_use]
    pub fn set_latitude(mut self, value: Option<f64>) -> Self {
        self.latitude = value;
        self
    }
    /// Sets `lens_model`.
    #[must_use]
    pub fn lens_model(mut self, value: impl Into<String>) -> Self {
        self.lens_model = Some(value.into());
        self
    }
    /// Sets or clears `lens_model`.
    #[must_use]
    pub fn set_lens_model(mut self, value: Option<String>) -> Self {
        self.lens_model = value;
        self
    }
    /// Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }
    /// Sets or clears `longitude`.
    #[must_use]
    pub fn set_longitude(mut self, value: Option<f64>) -> Self {
        self.longitude = value;
        self
    }
    /// Sets `make`.
    #[must_use]
    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }
    /// Sets or clears `make`.
    #[must_use]
    pub fn set_make(mut self, value: Option<String>) -> Self {
        self.make = value;
        self
    }
    /// Sets `model`.
    #[must_use]
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }
    /// Sets or clears `model`.
    #[must_use]
    pub fn set_model(mut self, value: Option<String>) -> Self {
        self.model = value;
        self
    }
    /// Sets `modify_date`.
    #[must_use]
    pub fn modify_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.modify_date = Some(value);
        self
    }
    /// Sets or clears `modify_date`.
    #[must_use]
    pub fn set_modify_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.modify_date = value;
        self
    }
    /// Sets `orientation`.
    #[must_use]
    pub fn orientation(mut self, value: impl Into<String>) -> Self {
        self.orientation = Some(value.into());
        self
    }
    /// Sets or clears `orientation`.
    #[must_use]
    pub fn set_orientation(mut self, value: Option<String>) -> Self {
        self.orientation = value;
        self
    }
    /// Sets `projection_type`.
    #[must_use]
    pub fn projection_type(mut self, value: impl Into<String>) -> Self {
        self.projection_type = Some(value.into());
        self
    }
    /// Sets or clears `projection_type`.
    #[must_use]
    pub fn set_projection_type(mut self, value: Option<String>) -> Self {
        self.projection_type = value;
        self
    }
    /// Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.rating = Some(value);
        self
    }
    /// Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<::std::num::NonZeroU64>) -> Self {
        self.rating = value;
        self
    }
    /// Sets `state`.
    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }
    /// Sets or clears `state`.
    #[must_use]
    pub fn set_state(mut self, value: Option<String>) -> Self {
        self.state = value;
        self
    }
    /// Sets `time_zone`.
    #[must_use]
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }
    /// Sets or clears `time_zone`.
    #[must_use]
    pub fn set_time_zone(mut self, value: Option<String>) -> Self {
        self.time_zone = value;
        self
    }
}

/// Axis to mirror along
///
/// See: <https://api.immich.app/models/MirrorAxis>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MirrorAxis {
    Horizontal,
    Vertical,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl MirrorAxis {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for MirrorAxis {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for MirrorAxis {
    fn from(value: &str) -> Self {
        match value {
            "horizontal" => Self::Horizontal,
            "vertical" => Self::Vertical,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for MirrorAxis {
    fn from(value: String) -> Self {
        match value.as_str() {
            "horizontal" => Self::Horizontal,
            "vertical" => Self::Vertical,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for MirrorAxis {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for MirrorAxis {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for MirrorAxis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for MirrorAxis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = MirrorAxis;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MirrorAxis::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(MirrorAxis::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `MirrorParameters`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `axis` | [`MirrorAxis`] | ✓ |  |
///
/// See: <https://api.immich.app/models/MirrorParameters>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MirrorParameters {
    /// (required)
    pub axis: MirrorAxis,
}

impl MirrorParameters {
    /// Creates a new [`MirrorParameters`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(axis: MirrorAxis) -> Self {
        Self { axis }
    }
    /// Sets `axis`.
    #[must_use]
    pub fn axis(mut self, value: MirrorAxis) -> Self {
        self.axis = value;
        self
    }
}

/// `PeopleUserResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `avatar_color` | [`UserAvatarColor`] | ✓ |  |
/// | `email` | [`String`] | ✓ | User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{\|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) |
/// | `id` | [`UserId`](crate::id::UserId) | ✓ | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `name` | [`String`] | ✓ | User name |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile change date |
/// | `profile_image_path` | [`String`] | ✓ | Profile image path |
/// | `role` | [`PersonUserRole`] | ✓ | Access role |
///
/// See: <https://api.immich.app/models/PeopleUserResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PeopleUserResponseDTO {
    /// (required)
    #[serde(rename = "avatarColor")]
    pub avatar_color: UserAvatarColor,
    /// User email (pattern: `` ^[\p{L}\p{M}\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?(?:\.[\p{L}\p{N}](?:[\p{L}\p{M}\p{N}-]{0,61}[\p{L}\p{M}\p{N}])?)*$ ``) (required)
    pub email: String,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: UserId,
    /// User name (required)
    pub name: String,
    /// Profile change date (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Profile image path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    /// Access role (required)
    pub role: PersonUserRole,
}

impl PeopleUserResponseDTO {
    /// Creates a new [`PeopleUserResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        avatar_color: UserAvatarColor,
        email: impl Into<String>,
        id: impl Into<UserId>,
        name: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        profile_image_path: impl Into<String>,
        role: PersonUserRole,
    ) -> Self {
        Self {
            avatar_color,
            email: email.into(),
            id: id.into(),
            name: name.into(),
            profile_changed_at,
            profile_image_path: profile_image_path.into(),
            role,
        }
    }
    /// Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = value;
        self
    }
    /// Sets `email`.
    #[must_use]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = value.into();
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<UserId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
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
    /// Sets `role`.
    #[must_use]
    pub fn role(mut self, value: PersonUserRole) -> Self {
        self.role = value;
        self
    }
}

/// `PersonOtherResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`String`] | ✓ |  |
/// | `name` | [`String`] | ✓ |  |
/// | `role` | [`PersonUserRole`] | ✓ |  |
/// | `shared_by_id` | [`UserId`](crate::id::UserId) | ✓ | Pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\|00000000-0000-0000-0000-000000000000\|ffffffff-ffff-ffff-ffff-ffffffffffff)$` |
///
/// See: <https://api.immich.app/models/PersonOtherResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PersonOtherResponseDTO {
    /// (required)
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,
    /// (required)
    pub name: String,
    /// (required)
    pub role: PersonUserRole,
    /// Pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$` (required)
    #[serde(rename = "sharedById")]
    pub shared_by_id: UserId,
}

impl PersonOtherResponseDTO {
    /// Creates a new [`PersonOtherResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(name: impl Into<String>, role: PersonUserRole, shared_by_id: impl Into<UserId>) -> Self {
        Self {
            birth_date: None,
            name: name.into(),
            role,
            shared_by_id: shared_by_id.into(),
        }
    }
    /// Sets `birth_date`.
    #[must_use]
    pub fn birth_date(mut self, value: impl Into<String>) -> Self {
        self.birth_date = Some(value.into());
        self
    }
    /// Sets or clears `birth_date`.
    #[must_use]
    pub fn set_birth_date(mut self, value: Option<String>) -> Self {
        self.birth_date = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `role`.
    #[must_use]
    pub fn role(mut self, value: PersonUserRole) -> Self {
        self.role = value;
        self
    }
    /// Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_by_id = value.into();
        self
    }
}

/// `PersonResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`String`] | ✓ | Person date of birth |
/// | `color` | [`String`] |  | Person color (hex) |
/// | `id` | [`PersonId`](crate::id::PersonId) | ✓ | Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_favorite` | `bool` |  | Is favorite |
/// | `is_hidden` | `bool` | ✓ | Is hidden |
/// | `name` | [`String`] | ✓ | Person name |
/// | `other_people` | [`Vec<PersonOtherResponseDTO>`](crate::assets::PersonOtherResponseDTO) | ✓ |  |
/// | `shared_by` | [`Vec<PeopleUserResponseDTO>`](crate::assets::PeopleUserResponseDTO) | ✓ | Users that gave the current user access to this person |
/// | `shared_with` | [`Vec<PeopleUserResponseDTO>`](crate::assets::PeopleUserResponseDTO) | ✓ | Users the current user gave access to this person |
/// | `thumbnail_path` | [`String`] | ✓ | Thumbnail path |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) |  | Last update date |
///
/// See: <https://api.immich.app/models/PersonResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PersonResponseDTO {
    /// Person date of birth (required)
    #[serde(rename = "birthDate")]
    pub birth_date: Option<::chrono::naive::NaiveDate>,
    /// Person color (hex)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<String>,
    /// Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: PersonId,
    /// Is favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Is hidden (required)
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    /// Person name (required)
    pub name: String,
    /// (required)
    #[serde(rename = "otherPeople")]
    pub other_people: Vec<PersonOtherResponseDTO>,
    /// Users that gave the current user access to this person (required)
    #[serde(rename = "sharedBy")]
    pub shared_by: Vec<PeopleUserResponseDTO>,
    /// Users the current user gave access to this person (required)
    #[serde(rename = "sharedWith")]
    pub shared_with: Vec<PeopleUserResponseDTO>,
    /// Thumbnail path (required)
    #[serde(rename = "thumbnailPath")]
    pub thumbnail_path: String,
    /// Last update date
    #[serde(
        rename = "updatedAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub updated_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}

impl PersonResponseDTO {
    /// Creates a new [`PersonResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        id: impl Into<PersonId>,
        is_hidden: bool,
        name: impl Into<String>,
        other_people: impl IntoIterator<Item = PersonOtherResponseDTO>,
        shared_by: impl IntoIterator<Item = PeopleUserResponseDTO>,
        shared_with: impl IntoIterator<Item = PeopleUserResponseDTO>,
        thumbnail_path: impl Into<String>,
    ) -> Self {
        Self {
            birth_date: None,
            color: None,
            id: id.into(),
            is_favorite: None,
            is_hidden,
            name: name.into(),
            other_people: other_people.into_iter().collect(),
            shared_by: shared_by.into_iter().collect(),
            shared_with: shared_with.into_iter().collect(),
            thumbnail_path: thumbnail_path.into(),
            updated_at: None,
        }
    }
    /// Sets `birth_date`.
    #[must_use]
    pub fn birth_date(mut self, value: ::chrono::naive::NaiveDate) -> Self {
        self.birth_date = Some(value);
        self
    }
    /// Sets or clears `birth_date`.
    #[must_use]
    pub fn set_birth_date(mut self, value: Option<::chrono::naive::NaiveDate>) -> Self {
        self.birth_date = value;
        self
    }
    /// Sets `color`.
    #[must_use]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }
    /// Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<String>) -> Self {
        self.color = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<PersonId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }
    /// Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }
    /// Sets `is_hidden`.
    #[must_use]
    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `other_people`.
    #[must_use]
    pub fn other_people(mut self, value: impl IntoIterator<Item = PersonOtherResponseDTO>) -> Self {
        self.other_people = value.into_iter().collect();
        self
    }
    /// Sets `shared_by`.
    #[must_use]
    pub fn shared_by(mut self, value: impl IntoIterator<Item = PeopleUserResponseDTO>) -> Self {
        self.shared_by = value.into_iter().collect();
        self
    }
    /// Sets `shared_with`.
    #[must_use]
    pub fn shared_with(mut self, value: impl IntoIterator<Item = PeopleUserResponseDTO>) -> Self {
        self.shared_with = value.into_iter().collect();
        self
    }
    /// Sets `thumbnail_path`.
    #[must_use]
    pub fn thumbnail_path(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_path = value.into();
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }
    /// Sets or clears `updated_at`.
    #[must_use]
    pub fn set_updated_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_at = value;
        self
    }
}

/// Levels of access for managing people resources on behalf of another user.
///
/// See: <https://api.immich.app/models/PersonUserRole>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PersonUserRole {
    Read,
    Write,
    Admin,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl PersonUserRole {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Admin => "admin",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for PersonUserRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for PersonUserRole {
    fn from(value: &str) -> Self {
        match value {
            "read" => Self::Read,
            "write" => Self::Write,
            "admin" => Self::Admin,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for PersonUserRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "read" => Self::Read,
            "write" => Self::Write,
            "admin" => Self::Admin,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for PersonUserRole {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for PersonUserRole {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for PersonUserRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for PersonUserRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = PersonUserRole;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(PersonUserRole::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(PersonUserRole::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `RotateParameters`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `angle` | `i64` | ✓ | Rotation angle in degrees |
///
/// See: <https://api.immich.app/models/RotateParameters>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct RotateParameters {
    /// Rotation angle in degrees (required)
    pub angle: i64,
}

impl RotateParameters {
    /// Creates a new [`RotateParameters`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(angle: i64) -> Self {
        Self { angle }
    }
    /// Sets `angle`.
    #[must_use]
    pub fn angle(mut self, value: i64) -> Self {
        self.angle = value;
        self
    }
}

/// `TagResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `color` | [`String`] |  | Tag color (hex) |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `id` | [`TagId`](crate::id::TagId) | ✓ | Tag ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `name` | [`String`] | ✓ | Tag name |
/// | `parent_id` | [`String`] |  | Parent tag ID |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
/// | `value` | [`String`] | ✓ | Tag value (full path) |
///
/// See: <https://api.immich.app/models/TagResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TagResponseDTO {
    /// Tag color (hex)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<String>,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Tag ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: TagId,
    /// Tag name (required)
    pub name: String,
    /// Parent tag ID
    #[serde(rename = "parentId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_id: Option<String>,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Tag value (full path) (required)
    pub value: String,
}

impl TagResponseDTO {
    /// Creates a new [`TagResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: impl Into<TagId>,
        name: impl Into<String>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            color: None,
            created_at,
            id: id.into(),
            name: name.into(),
            parent_id: None,
            updated_at,
            value: value.into(),
        }
    }
    /// Sets `color`.
    #[must_use]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }
    /// Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<String>) -> Self {
        self.color = value;
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<TagId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `parent_id`.
    #[must_use]
    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }
    /// Sets or clears `parent_id`.
    #[must_use]
    pub fn set_parent_id(mut self, value: Option<String>) -> Self {
        self.parent_id = value;
        self
    }
    /// Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
    /// Sets `value`.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
}

/// `UpdateAssetDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `date_time_original` | [`String`] |  | Original date and time |
/// | `description` | [`String`] |  | Asset description |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `latitude` | `f64` |  | Latitude coordinate |
/// | `live_photo_video_id` | [`AssetId`](crate::id::AssetId) |  | Live photo video ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `longitude` | `f64` |  | Longitude coordinate |
/// | `rating` | `i64` |  | Rating in range \[1-5\] (starred), -1 (rejected), or null (unrated) |
/// | `visibility` | [`AssetVisibility`] |  |  |
///
/// See: <https://api.immich.app/models/UpdateAssetDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub struct UpdateAssetDTO {
    /// Original date and time
    #[serde(
        rename = "dateTimeOriginal",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_time_original: Option<String>,
    /// Asset description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Mark as favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Latitude coordinate
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: Option<f64>,
    /// Live photo video ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(
        rename = "livePhotoVideoId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub live_photo_video_id: Option<AssetId>,
    /// Longitude coordinate
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: Option<f64>,
    /// Rating in range \[1-5\] (starred), -1 (rejected), or null (unrated)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<AssetVisibility>,
}

impl UpdateAssetDTO {
    /// Creates a new [`UpdateAssetDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            date_time_original: None,
            description: None,
            is_favorite: None,
            latitude: None,
            live_photo_video_id: None,
            longitude: None,
            rating: None,
            visibility: None,
        }
    }
    /// Sets `date_time_original`.
    #[must_use]
    pub fn date_time_original(mut self, value: impl Into<String>) -> Self {
        self.date_time_original = Some(value.into());
        self
    }
    /// Sets or clears `date_time_original`.
    #[must_use]
    pub fn set_date_time_original(mut self, value: Option<String>) -> Self {
        self.date_time_original = value;
        self
    }
    /// Sets `description`.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Sets or clears `description`.
    #[must_use]
    pub fn set_description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    /// Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }
    /// Sets or clears `is_favorite`.
    #[must_use]
    pub fn set_is_favorite(mut self, value: Option<bool>) -> Self {
        self.is_favorite = value;
        self
    }
    /// Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }
    /// Sets or clears `latitude`.
    #[must_use]
    pub fn set_latitude(mut self, value: Option<f64>) -> Self {
        self.latitude = value;
        self
    }
    /// Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: impl Into<AssetId>) -> Self {
        self.live_photo_video_id = Some(value.into());
        self
    }
    /// Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<AssetId>) -> Self {
        self.live_photo_video_id = value;
        self
    }
    /// Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }
    /// Sets or clears `longitude`.
    #[must_use]
    pub fn set_longitude(mut self, value: Option<f64>) -> Self {
        self.longitude = value;
        self
    }
    /// Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: i64) -> Self {
        self.rating = Some(value);
        self
    }
    /// Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<i64>) -> Self {
        self.rating = value;
        self
    }
    /// Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = Some(value);
        self
    }
    /// Sets or clears `visibility`.
    #[must_use]
    pub fn set_visibility(mut self, value: Option<AssetVisibility>) -> Self {
        self.visibility = value;
        self
    }
}
