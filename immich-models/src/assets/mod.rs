// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `assets`.

pub mod download;
pub mod faces;
pub mod stacks;

pub use crate::activities::{UserAvatarColor, UserResponseDTO};
pub use crate::people::PeopleUserResponseEmail;

/// `AssetBulkDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `force` | `bool` |  | Force delete even if in use |
/// | `ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | IDs to process |
///
/// See: <https://api.immich.app/models/AssetBulkDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetBulkDeleteDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub force: Option<bool>,
    pub ids: Vec<::uuid::Uuid>,
}

impl AssetBulkDeleteDTO {
    ///Creates a new [`AssetBulkDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: Vec<::uuid::Uuid>) -> Self {
        Self { force: None, ids }
    }

    ///Sets `force`.
    #[must_use]
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    ///Sets or clears `force`.
    #[must_use]
    pub fn set_force(mut self, value: Option<bool>) -> Self {
        self.force = value;
        self
    }

    ///Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.ids = value;
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
/// | `duplicate_id` | [`String`] |  | Duplicate ID |
/// | `ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs to update |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `latitude` | `f64` |  | Latitude coordinate |
/// | `longitude` | `f64` |  | Longitude coordinate |
/// | `rating` | `i64` |  | Rating in range [1-5] (starred), -1 (rejected), or null (unrated) |
/// | `time_zone` | [`String`] |  | Time zone (IANA timezone) |
/// | `visibility` | [`AssetVisibility`] |  |  |
///
/// See: <https://api.immich.app/models/AssetBulkUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
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
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Duplicate ID
    #[serde(
        rename = "duplicateId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub duplicate_id: Option<String>,
    pub ids: Vec<::uuid::Uuid>,
    /// Mark as favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<i64>,
    /// Time zone (IANA timezone)
    #[serde(rename = "timeZone", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<AssetVisibility>,
}

impl AssetBulkUpdateDTO {
    ///Creates a new [`AssetBulkUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: Vec<::uuid::Uuid>) -> Self {
        Self {
            date_time_original: None,
            date_time_relative: None,
            description: None,
            duplicate_id: None,
            ids,
            is_favorite: None,
            latitude: None,
            longitude: None,
            rating: None,
            time_zone: None,
            visibility: None,
        }
    }

    ///Sets `date_time_original`.
    #[must_use]
    pub fn date_time_original(mut self, value: impl Into<String>) -> Self {
        self.date_time_original = Some(value.into());
        self
    }

    ///Sets or clears `date_time_original`.
    #[must_use]
    pub fn set_date_time_original(mut self, value: Option<String>) -> Self {
        self.date_time_original = value;
        self
    }

    ///Sets `date_time_relative`.
    #[must_use]
    pub fn date_time_relative(mut self, value: i64) -> Self {
        self.date_time_relative = Some(value);
        self
    }

    ///Sets or clears `date_time_relative`.
    #[must_use]
    pub fn set_date_time_relative(mut self, value: Option<i64>) -> Self {
        self.date_time_relative = value;
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

    ///Sets `duplicate_id`.
    #[must_use]
    pub fn duplicate_id(mut self, value: impl Into<String>) -> Self {
        self.duplicate_id = Some(value.into());
        self
    }

    ///Sets or clears `duplicate_id`.
    #[must_use]
    pub fn set_duplicate_id(mut self, value: Option<String>) -> Self {
        self.duplicate_id = value;
        self
    }

    ///Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.ids = value;
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

    ///Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    ///Sets or clears `latitude`.
    #[must_use]
    pub fn set_latitude(mut self, value: Option<f64>) -> Self {
        self.latitude = value;
        self
    }

    ///Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    ///Sets or clears `longitude`.
    #[must_use]
    pub fn set_longitude(mut self, value: Option<f64>) -> Self {
        self.longitude = value;
        self
    }

    ///Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: i64) -> Self {
        self.rating = Some(value);
        self
    }

    ///Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<i64>) -> Self {
        self.rating = value;
        self
    }

    ///Sets `time_zone`.
    #[must_use]
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    ///Sets or clears `time_zone`.
    #[must_use]
    pub fn set_time_zone(mut self, value: Option<String>) -> Self {
        self.time_zone = value;
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

/// `AssetBulkUploadCheckDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `assets` | [`Vec<AssetBulkUploadCheckItem>`](crate::assets::AssetBulkUploadCheckItem) | ✓ | Assets to check |
///
/// See: <https://api.immich.app/models/AssetBulkUploadCheckDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetBulkUploadCheckDTO {
    pub assets: Vec<AssetBulkUploadCheckItem>,
}

impl AssetBulkUploadCheckDTO {
    ///Creates a new [`AssetBulkUploadCheckDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(assets: Vec<AssetBulkUploadCheckItem>) -> Self {
        Self { assets }
    }

    ///Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: Vec<AssetBulkUploadCheckItem>) -> Self {
        self.assets = value;
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetBulkUploadCheckItem {
    pub checksum: String,
    pub id: String,
}

impl AssetBulkUploadCheckItem {
    ///Creates a new [`AssetBulkUploadCheckItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(checksum: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            checksum: checksum.into(),
            id: id.into(),
        }
    }

    ///Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = value.into();
        self
    }

    ///Sets `id`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetBulkUploadCheckResponseDTO {
    pub results: Vec<AssetBulkUploadCheckResult>,
}

impl AssetBulkUploadCheckResponseDTO {
    ///Creates a new [`AssetBulkUploadCheckResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(results: Vec<AssetBulkUploadCheckResult>) -> Self {
        Self { results }
    }

    ///Sets `results`.
    #[must_use]
    pub fn results(mut self, value: Vec<AssetBulkUploadCheckResult>) -> Self {
        self.results = value;
        self
    }
}

/// `AssetBulkUploadCheckResult`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `action` | [`AssetUploadAction`] | ✓ |  |
/// | `asset_id` | [`Uuid`](::uuid::Uuid) |  | Existing asset ID if duplicate |
/// | `id` | [`String`] | ✓ | Client-side identifier echoed from the request to match results to inputs |
/// | `is_trashed` | `bool` |  | Whether existing asset is trashed |
/// | `reason` | [`AssetRejectReason`] |  |  |
///
/// See: <https://api.immich.app/models/AssetBulkUploadCheckResult>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetBulkUploadCheckResult {
    pub action: AssetUploadAction,
    /// Existing asset ID if duplicate
    #[serde(rename = "assetId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub asset_id: Option<::uuid::Uuid>,
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
    ///Creates a new [`AssetBulkUploadCheckResult`].
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

    ///Sets `action`.
    #[must_use]
    pub fn action(mut self, value: AssetUploadAction) -> Self {
        self.action = value;
        self
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = Some(value);
        self
    }

    ///Sets or clears `asset_id`.
    #[must_use]
    pub fn set_asset_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    ///Sets `is_trashed`.
    #[must_use]
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = Some(value);
        self
    }

    ///Sets or clears `is_trashed`.
    #[must_use]
    pub fn set_is_trashed(mut self, value: Option<bool>) -> Self {
        self.is_trashed = value;
        self
    }

    ///Sets `reason`.
    #[must_use]
    pub fn reason(mut self, value: AssetRejectReason) -> Self {
        self.reason = Some(value);
        self
    }

    ///Sets or clears `reason`.
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
/// | `source_id` | [`Uuid`](::uuid::Uuid) | ✓ | Source asset ID |
/// | `stack` | `bool` |  | Copy stack association |
/// | `target_id` | [`Uuid`](::uuid::Uuid) | ✓ | Target asset ID |
///
/// See: <https://api.immich.app/models/AssetCopyDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetCopyDTO {
    #[serde(default = "crate::common::defaults::default_bool::<true>")]
    pub albums: bool,
    #[serde(default = "crate::common::defaults::default_bool::<true>")]
    pub favorite: bool,
    /// Copy shared links
    #[serde(rename = "sharedLinks", default = "crate::common::defaults::default_bool::<true>")]
    pub shared_links: bool,
    #[serde(default = "crate::common::defaults::default_bool::<true>")]
    pub sidecar: bool,
    /// Source asset ID (required)
    #[serde(rename = "sourceId")]
    pub source_id: ::uuid::Uuid,
    #[serde(default = "crate::common::defaults::default_bool::<true>")]
    pub stack: bool,
    /// Target asset ID (required)
    #[serde(rename = "targetId")]
    pub target_id: ::uuid::Uuid,
}

impl AssetCopyDTO {
    ///Creates a new [`AssetCopyDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(source_id: ::uuid::Uuid, target_id: ::uuid::Uuid) -> Self {
        Self {
            albums: crate::common::defaults::default_bool::<true>(),
            favorite: crate::common::defaults::default_bool::<true>(),
            shared_links: crate::common::defaults::default_bool::<true>(),
            sidecar: crate::common::defaults::default_bool::<true>(),
            source_id,
            stack: crate::common::defaults::default_bool::<true>(),
            target_id,
        }
    }

    ///Sets `albums`.
    #[must_use]
    pub fn albums(mut self, value: bool) -> Self {
        self.albums = value;
        self
    }

    ///Sets `favorite`.
    #[must_use]
    pub fn favorite(mut self, value: bool) -> Self {
        self.favorite = value;
        self
    }

    ///Sets `shared_links`.
    #[must_use]
    pub fn shared_links(mut self, value: bool) -> Self {
        self.shared_links = value;
        self
    }

    ///Sets `sidecar`.
    #[must_use]
    pub fn sidecar(mut self, value: bool) -> Self {
        self.sidecar = value;
        self
    }

    ///Sets `source_id`.
    #[must_use]
    pub fn source_id(mut self, value: ::uuid::Uuid) -> Self {
        self.source_id = value;
        self
    }

    ///Sets `stack`.
    #[must_use]
    pub fn stack(mut self, value: bool) -> Self {
        self.stack = value;
        self
    }

    ///Sets `target_id`.
    #[must_use]
    pub fn target_id(mut self, value: ::uuid::Uuid) -> Self {
        self.target_id = value;
        self
    }
}

/// Type of edit action to perform
///
/// See: <https://api.immich.app/models/AssetEditAction>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetEditAction {
    #[serde(rename = "crop")]
    Crop,
    #[serde(rename = "rotate")]
    Rotate,
    #[serde(rename = "mirror")]
    Mirror,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetEditAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Crop => f.write_str("crop"),
            Self::Rotate => f.write_str("rotate"),
            Self::Mirror => f.write_str("mirror"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetEditAction {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "crop" => Ok(Self::Crop),
            "rotate" => Ok(Self::Rotate),
            "mirror" => Ok(Self::Mirror),
            _ => Err(format!("invalid value for {}: {}", "AssetEditAction", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetEditAction {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetEditAction {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetEditAction {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
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
pub struct AssetEditActionItemDTO {
    pub action: AssetEditAction,
    pub parameters: AssetEditActionItemParameters,
}

impl AssetEditActionItemDTO {
    ///Creates a new [`AssetEditActionItemDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(action: AssetEditAction, parameters: AssetEditActionItemParameters) -> Self {
        Self { action, parameters }
    }

    ///Sets `action`.
    #[must_use]
    pub fn action(mut self, value: AssetEditAction) -> Self {
        self.action = value;
        self
    }

    ///Sets `parameters`.
    #[must_use]
    pub fn parameters(mut self, value: AssetEditActionItemParameters) -> Self {
        self.parameters = value;
        self
    }
}

///List of edit actions to apply (crop, rotate, or mirror)
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AssetEditActionItemParameters {
    CropParameters(CropParameters),
    RotateParameters(RotateParameters),
    MirrorParameters(MirrorParameters),
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
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset edit ID |
/// | `parameters` | [`CropParameters`] | ✓ | List of edit actions to apply (crop, rotate, or mirror) |
///
/// See: <https://api.immich.app/models/AssetEditActionItemResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetEditActionItemResponseDTO {
    pub action: AssetEditAction,
    pub id: ::uuid::Uuid,
    pub parameters: AssetEditActionItemResponseParameters,
}

impl AssetEditActionItemResponseDTO {
    ///Creates a new [`AssetEditActionItemResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(action: AssetEditAction, id: ::uuid::Uuid, parameters: AssetEditActionItemResponseParameters) -> Self {
        Self { action, id, parameters }
    }

    ///Sets `action`.
    #[must_use]
    pub fn action(mut self, value: AssetEditAction) -> Self {
        self.action = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `parameters`.
    #[must_use]
    pub fn parameters(mut self, value: AssetEditActionItemResponseParameters) -> Self {
        self.parameters = value;
        self
    }
}

///List of edit actions to apply (crop, rotate, or mirror)
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AssetEditActionItemResponseParameters {
    CropParameters(CropParameters),
    RotateParameters(RotateParameters),
    MirrorParameters(MirrorParameters),
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
pub struct AssetEditsCreateDTO {
    pub edits: Vec<AssetEditActionItemDTO>,
}

impl AssetEditsCreateDTO {
    ///Creates a new [`AssetEditsCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(edits: Vec<AssetEditActionItemDTO>) -> Self {
        Self { edits }
    }

    ///Sets `edits`.
    #[must_use]
    pub fn edits(mut self, value: Vec<AssetEditActionItemDTO>) -> Self {
        self.edits = value;
        self
    }
}

/// `AssetEditsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID these edits belong to |
/// | `edits` | [`Vec<AssetEditActionItemResponseDTO>`](crate::assets::AssetEditActionItemResponseDTO) | ✓ | List of edit actions applied to the asset |
///
/// See: <https://api.immich.app/models/AssetEditsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetEditsResponseDTO {
    /// Asset ID these edits belong to (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    pub edits: Vec<AssetEditActionItemResponseDTO>,
}

impl AssetEditsResponseDTO {
    ///Creates a new [`AssetEditsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: ::uuid::Uuid, edits: Vec<AssetEditActionItemResponseDTO>) -> Self {
        Self { asset_id, edits }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `edits`.
    #[must_use]
    pub fn edits(mut self, value: Vec<AssetEditActionItemResponseDTO>) -> Self {
        self.edits = value;
        self
    }
}

/// `AssetFileResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset file ID |
/// | `is_edited` | `bool` | ✓ | The file was generated from an edit |
/// | `is_progressive` | `bool` | ✓ | The file is a progressively encoded JPEG |
/// | `is_transparent` | `bool` | ✓ | The file is transparent |
/// | `path` | [`String`] | ✓ | File path |
/// | `type` | [`AssetFileType`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Update date |
///
/// See: <https://api.immich.app/models/AssetFileResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetFileResponseDTO {
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::uuid::Uuid,
    /// The file was generated from an edit (required)
    #[serde(rename = "isEdited")]
    pub is_edited: bool,
    /// The file is a progressively encoded JPEG (required)
    #[serde(rename = "isProgressive")]
    pub is_progressive: bool,
    /// The file is transparent (required)
    #[serde(rename = "isTransparent")]
    pub is_transparent: bool,
    pub path: String,
    /// (required)
    #[serde(rename = "type")]
    pub type_: AssetFileType,
    /// Update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl AssetFileResponseDTO {
    ///Creates a new [`AssetFileResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        is_edited: bool,
        is_progressive: bool,
        is_transparent: bool,
        path: impl Into<String>,
        type_: AssetFileType,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            created_at,
            id,
            is_edited,
            is_progressive,
            is_transparent,
            path: path.into(),
            type_,
            updated_at,
        }
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_edited`.
    #[must_use]
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = value;
        self
    }

    ///Sets `is_progressive`.
    #[must_use]
    pub fn is_progressive(mut self, value: bool) -> Self {
        self.is_progressive = value;
        self
    }

    ///Sets `is_transparent`.
    #[must_use]
    pub fn is_transparent(mut self, value: bool) -> Self {
        self.is_transparent = value;
        self
    }

    ///Sets `path`.
    #[must_use]
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = value.into();
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetFileType) -> Self {
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

/// Type of file
///
/// See: <https://api.immich.app/models/AssetFileType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetFileType {
    #[serde(rename = "fullsize")]
    Fullsize,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "thumbnail")]
    Thumbnail,
    #[serde(rename = "sidecar")]
    Sidecar,
    #[serde(rename = "encoded_video")]
    EncodedVideo,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetFileType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Fullsize => f.write_str("fullsize"),
            Self::Preview => f.write_str("preview"),
            Self::Thumbnail => f.write_str("thumbnail"),
            Self::Sidecar => f.write_str("sidecar"),
            Self::EncodedVideo => f.write_str("encoded_video"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetFileType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "fullsize" => Ok(Self::Fullsize),
            "preview" => Ok(Self::Preview),
            "thumbnail" => Ok(Self::Thumbnail),
            "sidecar" => Ok(Self::Sidecar),
            "encoded_video" => Ok(Self::EncodedVideo),
            _ => Err(format!("invalid value for {}: {}", "AssetFileType", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetFileType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetFileType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetFileType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Job name
///
/// See: <https://api.immich.app/models/AssetJobName>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetJobName {
    #[serde(rename = "refresh-faces")]
    RefreshFaces,
    #[serde(rename = "refresh-metadata")]
    RefreshMetadata,
    #[serde(rename = "regenerate-thumbnail")]
    RegenerateThumbnail,
    #[serde(rename = "transcode-video")]
    TranscodeVideo,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetJobName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RefreshFaces => f.write_str("refresh-faces"),
            Self::RefreshMetadata => f.write_str("refresh-metadata"),
            Self::RegenerateThumbnail => f.write_str("regenerate-thumbnail"),
            Self::TranscodeVideo => f.write_str("transcode-video"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetJobName {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "refresh-faces" => Ok(Self::RefreshFaces),
            "refresh-metadata" => Ok(Self::RefreshMetadata),
            "regenerate-thumbnail" => Ok(Self::RegenerateThumbnail),
            "transcode-video" => Ok(Self::TranscodeVideo),
            _ => Err(format!("invalid value for {}: {}", "AssetJobName", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetJobName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetJobName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetJobName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `AssetJobsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs |
/// | `name` | [`AssetJobName`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AssetJobsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetJobsDTO {
    /// Asset IDs (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<::uuid::Uuid>,
    pub name: AssetJobName,
}

impl AssetJobsDTO {
    ///Creates a new [`AssetJobsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_ids: Vec<::uuid::Uuid>, name: AssetJobName) -> Self {
        Self { asset_ids, name }
    }

    ///Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.asset_ids = value;
        self
    }

    ///Sets `name`.
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
/// | `asset_data` | `String` | ✓ | Asset file data |
/// | `duration` | `i64` |  | Duration in milliseconds (for videos) |
/// | `file_created_at` | [`DateTime`](::chrono::DateTime) | ✓ | File creation date |
/// | `file_modified_at` | [`DateTime`](::chrono::DateTime) | ✓ | File modification date |
/// | `filename` | [`String`] |  | Filename |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `live_photo_video_id` | [`Uuid`](::uuid::Uuid) |  | Live photo video ID |
/// | `metadata` | [`Vec<AssetMetadataUpsertItemDTO>`](crate::assets::AssetMetadataUpsertItemDTO) |  | Asset metadata items |
/// | `sidecar_data` | `String` |  | Sidecar file data |
/// | `visibility` | [`AssetVisibility`] |  |  |
///
/// See: <https://api.immich.app/models/AssetMediaCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetMediaCreateDTO {
    /// Asset file data (required)
    #[serde(rename = "assetData")]
    pub asset_data: Vec<u8>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: Option<i64>,
    /// File creation date (required)
    #[serde(rename = "fileCreatedAt")]
    pub file_created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// File modification date (required)
    #[serde(rename = "fileModifiedAt")]
    pub file_modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filename: Option<String>,
    /// Mark as favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Live photo video ID
    #[serde(
        rename = "livePhotoVideoId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub live_photo_video_id: Option<::uuid::Uuid>,
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
    ///Creates a new [`AssetMediaCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_data: Vec<u8>,
        file_created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        file_modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            asset_data,
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

    ///Sets `asset_data`.
    #[must_use]
    pub fn asset_data(mut self, value: Vec<u8>) -> Self {
        self.asset_data = value;
        self
    }

    ///Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    ///Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<i64>) -> Self {
        self.duration = value;
        self
    }

    ///Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_created_at = value;
        self
    }

    ///Sets `file_modified_at`.
    #[must_use]
    pub fn file_modified_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_modified_at = value;
        self
    }

    ///Sets `filename`.
    #[must_use]
    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    ///Sets or clears `filename`.
    #[must_use]
    pub fn set_filename(mut self, value: Option<String>) -> Self {
        self.filename = value;
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

    ///Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: ::uuid::Uuid) -> Self {
        self.live_photo_video_id = Some(value);
        self
    }

    ///Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.live_photo_video_id = value;
        self
    }

    ///Sets `metadata`.
    #[must_use]
    pub fn metadata(mut self, value: Vec<AssetMetadataUpsertItemDTO>) -> Self {
        self.metadata = value;
        self
    }

    ///Sets `sidecar_data`.
    #[must_use]
    pub fn sidecar_data(mut self, value: Vec<u8>) -> Self {
        self.sidecar_data = Some(value);
        self
    }

    ///Sets or clears `sidecar_data`.
    #[must_use]
    pub fn set_sidecar_data(mut self, value: Option<Vec<u8>>) -> Self {
        self.sidecar_data = value;
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

/// `AssetMediaResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset media ID |
/// | `status` | [`AssetMediaStatus`] | ✓ |  |
///
/// See: <https://api.immich.app/models/AssetMediaResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetMediaResponseDTO {
    pub id: ::uuid::Uuid,
    pub status: AssetMediaStatus,
}

impl AssetMediaResponseDTO {
    ///Creates a new [`AssetMediaResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: ::uuid::Uuid, status: AssetMediaStatus) -> Self {
        Self { id, status }
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `status`.
    #[must_use]
    pub fn status(mut self, value: AssetMediaStatus) -> Self {
        self.status = value;
        self
    }
}

/// Asset media size
///
/// See: <https://api.immich.app/models/AssetMediaSize>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetMediaSize {
    #[serde(rename = "original")]
    Original,
    #[serde(rename = "fullsize")]
    Fullsize,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "thumbnail")]
    Thumbnail,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetMediaSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Original => f.write_str("original"),
            Self::Fullsize => f.write_str("fullsize"),
            Self::Preview => f.write_str("preview"),
            Self::Thumbnail => f.write_str("thumbnail"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetMediaSize {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "original" => Ok(Self::Original),
            "fullsize" => Ok(Self::Fullsize),
            "preview" => Ok(Self::Preview),
            "thumbnail" => Ok(Self::Thumbnail),
            _ => Err(format!("invalid value for {}: {}", "AssetMediaSize", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetMediaSize {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetMediaSize {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetMediaSize {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Upload status
///
/// See: <https://api.immich.app/models/AssetMediaStatus>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetMediaStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "duplicate")]
    Duplicate,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetMediaStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Created => f.write_str("created"),
            Self::Duplicate => f.write_str("duplicate"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetMediaStatus {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "created" => Ok(Self::Created),
            "duplicate" => Ok(Self::Duplicate),
            _ => Err(format!("invalid value for {}: {}", "AssetMediaStatus", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetMediaStatus {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetMediaStatus {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetMediaStatus {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `AssetMetadataBulkDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `items` | [`Vec<AssetMetadataBulkDeleteItemDTO>`](crate::assets::AssetMetadataBulkDeleteItemDTO) | ✓ | Metadata items to delete |
///
/// See: <https://api.immich.app/models/AssetMetadataBulkDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetMetadataBulkDeleteDTO {
    pub items: Vec<AssetMetadataBulkDeleteItemDTO>,
}

impl AssetMetadataBulkDeleteDTO {
    ///Creates a new [`AssetMetadataBulkDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(items: Vec<AssetMetadataBulkDeleteItemDTO>) -> Self {
        Self { items }
    }

    ///Sets `items`.
    #[must_use]
    pub fn items(mut self, value: Vec<AssetMetadataBulkDeleteItemDTO>) -> Self {
        self.items = value;
        self
    }
}

/// `AssetMetadataBulkDeleteItemDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `key` | [`String`] | ✓ | Metadata key |
///
/// See: <https://api.immich.app/models/AssetMetadataBulkDeleteItemDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetMetadataBulkDeleteItemDTO {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    pub key: String,
}

impl AssetMetadataBulkDeleteItemDTO {
    ///Creates a new [`AssetMetadataBulkDeleteItemDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: ::uuid::Uuid, key: impl Into<String>) -> Self {
        Self {
            asset_id,
            key: key.into(),
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `key`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `key` | [`String`] | ✓ | Metadata key |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
/// | `value` | `object` | ✓ | Metadata value (object) |
///
/// See: <https://api.immich.app/models/AssetMetadataBulkResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetMetadataBulkResponseDTO {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    pub key: String,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl AssetMetadataBulkResponseDTO {
    ///Creates a new [`AssetMetadataBulkResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: ::uuid::Uuid,
        key: impl Into<String>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        value: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self {
            asset_id,
            key: key.into(),
            updated_at,
            value,
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }

    ///Sets `value`.
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
pub struct AssetMetadataBulkUpsertDTO {
    pub items: Vec<AssetMetadataBulkUpsertItemDTO>,
}

impl AssetMetadataBulkUpsertDTO {
    ///Creates a new [`AssetMetadataBulkUpsertDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(items: Vec<AssetMetadataBulkUpsertItemDTO>) -> Self {
        Self { items }
    }

    ///Sets `items`.
    #[must_use]
    pub fn items(mut self, value: Vec<AssetMetadataBulkUpsertItemDTO>) -> Self {
        self.items = value;
        self
    }
}

/// `AssetMetadataBulkUpsertItemDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `key` | [`String`] | ✓ | Metadata key |
/// | `value` | `object` | ✓ | Metadata value (object) |
///
/// See: <https://api.immich.app/models/AssetMetadataBulkUpsertItemDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetMetadataBulkUpsertItemDTO {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    pub key: String,
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl AssetMetadataBulkUpsertItemDTO {
    ///Creates a new [`AssetMetadataBulkUpsertItemDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: ::uuid::Uuid,
        key: impl Into<String>,
        value: ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Self {
        Self {
            asset_id,
            key: key.into(),
            value,
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }

    ///Sets `value`.
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
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
/// | `value` | `object` | ✓ | Metadata value (object) |
///
/// See: <https://api.immich.app/models/AssetMetadataResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetMetadataResponseDTO {
    pub key: String,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl AssetMetadataResponseDTO {
    ///Creates a new [`AssetMetadataResponseDTO`].
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

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }

    ///Sets `value`.
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
pub struct AssetMetadataUpsertDTO {
    pub items: Vec<AssetMetadataUpsertItemDTO>,
}

impl AssetMetadataUpsertDTO {
    ///Creates a new [`AssetMetadataUpsertDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(items: Vec<AssetMetadataUpsertItemDTO>) -> Self {
        Self { items }
    }

    ///Sets `items`.
    #[must_use]
    pub fn items(mut self, value: Vec<AssetMetadataUpsertItemDTO>) -> Self {
        self.items = value;
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
pub struct AssetMetadataUpsertItemDTO {
    pub key: String,
    pub value: ::serde_json::Map<String, ::serde_json::Value>,
}

impl AssetMetadataUpsertItemDTO {
    ///Creates a new [`AssetMetadataUpsertItemDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(key: impl Into<String>, value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self { key: key.into(), value }
    }

    ///Sets `key`.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = value.into();
        self
    }

    ///Sets `value`.
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
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ |  |
/// | `box_score` | `f64` | ✓ | Confidence score for text detection box |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ |  |
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
pub struct AssetOcrResponseDTO {
    /// (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    /// Confidence score for text detection box (required)
    #[serde(rename = "boxScore")]
    pub box_score: f64,
    pub id: ::uuid::Uuid,
    pub text: String,
    /// Confidence score for text recognition (required)
    #[serde(rename = "textScore")]
    pub text_score: f64,
    pub x1: f64,
    pub x2: f64,
    pub x3: f64,
    pub x4: f64,
    pub y1: f64,
    pub y2: f64,
    pub y3: f64,
    pub y4: f64,
}

impl AssetOcrResponseDTO {
    ///Creates a new [`AssetOcrResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_id: ::uuid::Uuid,
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
            asset_id,
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

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
        self
    }

    ///Sets `box_score`.
    #[must_use]
    pub fn box_score(mut self, value: f64) -> Self {
        self.box_score = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `text`.
    #[must_use]
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = value.into();
        self
    }

    ///Sets `text_score`.
    #[must_use]
    pub fn text_score(mut self, value: f64) -> Self {
        self.text_score = value;
        self
    }

    ///Sets `x1`.
    #[must_use]
    pub fn x1(mut self, value: f64) -> Self {
        self.x1 = value;
        self
    }

    ///Sets `x2`.
    #[must_use]
    pub fn x2(mut self, value: f64) -> Self {
        self.x2 = value;
        self
    }

    ///Sets `x3`.
    #[must_use]
    pub fn x3(mut self, value: f64) -> Self {
        self.x3 = value;
        self
    }

    ///Sets `x4`.
    #[must_use]
    pub fn x4(mut self, value: f64) -> Self {
        self.x4 = value;
        self
    }

    ///Sets `y1`.
    #[must_use]
    pub fn y1(mut self, value: f64) -> Self {
        self.y1 = value;
        self
    }

    ///Sets `y2`.
    #[must_use]
    pub fn y2(mut self, value: f64) -> Self {
        self.y2 = value;
        self
    }

    ///Sets `y3`.
    #[must_use]
    pub fn y3(mut self, value: f64) -> Self {
        self.y3 = value;
        self
    }

    ///Sets `y4`.
    #[must_use]
    pub fn y4(mut self, value: f64) -> Self {
        self.y4 = value;
        self
    }
}

/// Rejection reason if rejected
///
/// See: <https://api.immich.app/models/AssetRejectReason>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetRejectReason {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "unsupported-format")]
    UnsupportedFormat,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetRejectReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Duplicate => f.write_str("duplicate"),
            Self::UnsupportedFormat => f.write_str("unsupported-format"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetRejectReason {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "duplicate" => Ok(Self::Duplicate),
            "unsupported-format" => Ok(Self::UnsupportedFormat),
            _ => Err(format!("invalid value for {}: {}", "AssetRejectReason", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetRejectReason {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetRejectReason {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetRejectReason {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `AssetResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `checksum` | [`String`] | ✓ | Base64 encoded SHA1 hash |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | The UTC timestamp when the asset was originally uploaded to Immich. |
/// | `duplicate_id` | [`Uuid`](::uuid::Uuid) |  | Duplicate group ID |
/// | `duration` | `i64` | ✓ | Video/gif duration in milliseconds (null for static images) |
/// | `exif_info` | [`ExifResponseDTO`] |  |  |
/// | `file_created_at` | [`DateTime`](::chrono::DateTime) | ✓ | The actual UTC timestamp when the file was created/captured, preserving timezone information. This is the authoritative timestamp for chronological sorting within timeline groups. Combined with timezone data, this can be used to determine the exact moment the photo was taken. |
/// | `file_modified_at` | [`DateTime`](::chrono::DateTime) | ✓ | The UTC timestamp when the file was last modified on the filesystem. This reflects the last time the physical file was changed, which may be different from when the photo was originally taken. |
/// | `has_metadata` | `bool` | ✓ | Whether asset has metadata |
/// | `height` | `i64` | ✓ | Asset height |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `is_archived` | `bool` | ✓ | Is archived |
/// | `is_edited` | `bool` | ✓ | Is edited |
/// | `is_favorite` | `bool` | ✓ | Is favorite |
/// | `is_offline` | `bool` | ✓ | Is offline |
/// | `is_trashed` | `bool` | ✓ | Is trashed |
/// | `library_id` | [`Uuid`](::uuid::Uuid) |  | Library ID |
/// | `live_photo_video_id` | [`String`] |  | Live photo video ID |
/// | `local_date_time` | [`DateTime`](::chrono::DateTime) | ✓ | The local date and time when the photo/video was taken, derived from EXIF metadata. This represents the photographer's local time regardless of timezone, stored as a timezone-agnostic timestamp. Used for timeline grouping by "local" days and months. |
/// | `original_file_name` | [`String`] | ✓ | Original file name |
/// | `original_mime_type` | [`String`] |  | Original MIME type |
/// | `original_path` | [`String`] | ✓ | Original file path |
/// | `owner` | [`UserResponseDTO`] |  |  |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner user ID |
/// | `people` | [`Vec<PersonResponseDTO>`](crate::assets::PersonResponseDTO) |  |  |
/// | `resized` | `bool` |  | Is resized |
/// | `stack` | [`AssetStackResponseDTO`] |  |  |
/// | `tags` | [`Vec<TagResponseDTO>`](crate::assets::TagResponseDTO) |  |  |
/// | `thumbhash` | [`String`] | ✓ | Thumbhash for thumbnail generation (base64) also used as the c query param for thumbnail cache busting. |
/// | `type` | [`AssetTypeEnum`] | ✓ |  |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | The UTC timestamp when the asset record was last updated in the database. This is automatically maintained by the database and reflects when any field in the asset was last modified. |
/// | `visibility` | [`AssetVisibility`] | ✓ |  |
/// | `width` | `i64` | ✓ | Asset width |
///
/// See: <https://api.immich.app/models/AssetResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetResponseDTO {
    pub checksum: String,
    /// The UTC timestamp when the asset was originally uploaded to Immich. (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Duplicate group ID
    #[serde(
        rename = "duplicateId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub duplicate_id: Option<::uuid::Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    pub id: ::uuid::Uuid,
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
    /// Library ID
    #[serde(
        rename = "libraryId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub library_id: Option<::uuid::Uuid>,
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
    /// Owner user ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub people: Vec<PersonResponseDTO>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resized: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stack: Option<AssetStackResponseDTO>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tags: Vec<TagResponseDTO>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbhash: Option<String>,
    /// (required)
    #[serde(rename = "type")]
    pub type_: AssetTypeEnum,
    /// The UTC timestamp when the asset record was last updated in the database. This is automatically maintained by the database and reflects when any field in the asset was last modified. (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub visibility: AssetVisibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl AssetResponseDTO {
    ///Creates a new [`AssetResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        checksum: impl Into<String>,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        file_created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        file_modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
        has_metadata: bool,
        id: ::uuid::Uuid,
        is_archived: bool,
        is_edited: bool,
        is_favorite: bool,
        is_offline: bool,
        is_trashed: bool,
        local_date_time: ::chrono::DateTime<::chrono::offset::Utc>,
        original_file_name: impl Into<String>,
        original_path: impl Into<String>,
        owner_id: ::uuid::Uuid,
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
            id,
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
            owner_id,
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

    ///Sets `checksum`.
    #[must_use]
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = value.into();
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `duplicate_id`.
    #[must_use]
    pub fn duplicate_id(mut self, value: ::uuid::Uuid) -> Self {
        self.duplicate_id = Some(value);
        self
    }

    ///Sets or clears `duplicate_id`.
    #[must_use]
    pub fn set_duplicate_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.duplicate_id = value;
        self
    }

    ///Sets `duration`.
    #[must_use]
    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    ///Sets or clears `duration`.
    #[must_use]
    pub fn set_duration(mut self, value: Option<i64>) -> Self {
        self.duration = value;
        self
    }

    ///Sets `exif_info`.
    #[must_use]
    pub fn exif_info(mut self, value: ExifResponseDTO) -> Self {
        self.exif_info = Some(value);
        self
    }

    ///Sets or clears `exif_info`.
    #[must_use]
    pub fn set_exif_info(mut self, value: Option<ExifResponseDTO>) -> Self {
        self.exif_info = value;
        self
    }

    ///Sets `file_created_at`.
    #[must_use]
    pub fn file_created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_created_at = value;
        self
    }

    ///Sets `file_modified_at`.
    #[must_use]
    pub fn file_modified_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.file_modified_at = value;
        self
    }

    ///Sets `has_metadata`.
    #[must_use]
    pub fn has_metadata(mut self, value: bool) -> Self {
        self.has_metadata = value;
        self
    }

    ///Sets `height`.
    #[must_use]
    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    ///Sets or clears `height`.
    #[must_use]
    pub fn set_height(mut self, value: Option<i64>) -> Self {
        self.height = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `is_archived`.
    #[must_use]
    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = value;
        self
    }

    ///Sets `is_edited`.
    #[must_use]
    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = value;
        self
    }

    ///Sets `is_favorite`.
    #[must_use]
    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = value;
        self
    }

    ///Sets `is_offline`.
    #[must_use]
    pub fn is_offline(mut self, value: bool) -> Self {
        self.is_offline = value;
        self
    }

    ///Sets `is_trashed`.
    #[must_use]
    pub fn is_trashed(mut self, value: bool) -> Self {
        self.is_trashed = value;
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

    ///Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: impl Into<String>) -> Self {
        self.live_photo_video_id = Some(value.into());
        self
    }

    ///Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<String>) -> Self {
        self.live_photo_video_id = value;
        self
    }

    ///Sets `local_date_time`.
    #[must_use]
    pub fn local_date_time(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.local_date_time = value;
        self
    }

    ///Sets `original_file_name`.
    #[must_use]
    pub fn original_file_name(mut self, value: impl Into<String>) -> Self {
        self.original_file_name = value.into();
        self
    }

    ///Sets `original_mime_type`.
    #[must_use]
    pub fn original_mime_type(mut self, value: impl Into<String>) -> Self {
        self.original_mime_type = Some(value.into());
        self
    }

    ///Sets or clears `original_mime_type`.
    #[must_use]
    pub fn set_original_mime_type(mut self, value: Option<String>) -> Self {
        self.original_mime_type = value;
        self
    }

    ///Sets `original_path`.
    #[must_use]
    pub fn original_path(mut self, value: impl Into<String>) -> Self {
        self.original_path = value.into();
        self
    }

    ///Sets `owner`.
    #[must_use]
    pub fn owner(mut self, value: UserResponseDTO) -> Self {
        self.owner = Some(value);
        self
    }

    ///Sets or clears `owner`.
    #[must_use]
    pub fn set_owner(mut self, value: Option<UserResponseDTO>) -> Self {
        self.owner = value;
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `people`.
    #[must_use]
    pub fn people(mut self, value: Vec<PersonResponseDTO>) -> Self {
        self.people = value;
        self
    }

    ///Sets `resized`.
    #[must_use]
    pub fn resized(mut self, value: bool) -> Self {
        self.resized = Some(value);
        self
    }

    ///Sets or clears `resized`.
    #[must_use]
    pub fn set_resized(mut self, value: Option<bool>) -> Self {
        self.resized = value;
        self
    }

    ///Sets `stack`.
    #[must_use]
    pub fn stack(mut self, value: AssetStackResponseDTO) -> Self {
        self.stack = Some(value);
        self
    }

    ///Sets or clears `stack`.
    #[must_use]
    pub fn set_stack(mut self, value: Option<AssetStackResponseDTO>) -> Self {
        self.stack = value;
        self
    }

    ///Sets `tags`.
    #[must_use]
    pub fn tags(mut self, value: Vec<TagResponseDTO>) -> Self {
        self.tags = value;
        self
    }

    ///Sets `thumbhash`.
    #[must_use]
    pub fn thumbhash(mut self, value: impl Into<String>) -> Self {
        self.thumbhash = Some(value.into());
        self
    }

    ///Sets or clears `thumbhash`.
    #[must_use]
    pub fn set_thumbhash(mut self, value: Option<String>) -> Self {
        self.thumbhash = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: AssetTypeEnum) -> Self {
        self.type_ = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }

    ///Sets `visibility`.
    #[must_use]
    pub fn visibility(mut self, value: AssetVisibility) -> Self {
        self.visibility = value;
        self
    }

    ///Sets `width`.
    #[must_use]
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    ///Sets or clears `width`.
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
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Stack ID |
/// | `primary_asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Primary asset ID |
///
/// See: <https://api.immich.app/models/AssetStackResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetStackResponseDTO {
    /// Number of assets in stack (required)
    #[serde(rename = "assetCount")]
    pub asset_count: i64,
    pub id: ::uuid::Uuid,
    /// Primary asset ID (required)
    #[serde(rename = "primaryAssetId")]
    pub primary_asset_id: ::uuid::Uuid,
}

impl AssetStackResponseDTO {
    ///Creates a new [`AssetStackResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_count: i64, id: ::uuid::Uuid, primary_asset_id: ::uuid::Uuid) -> Self {
        Self {
            asset_count,
            id,
            primary_asset_id,
        }
    }

    ///Sets `asset_count`.
    #[must_use]
    pub fn asset_count(mut self, value: i64) -> Self {
        self.asset_count = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `primary_asset_id`.
    #[must_use]
    pub fn primary_asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.primary_asset_id = value;
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetStatsResponseDTO {
    pub images: i64,
    pub total: i64,
    pub videos: i64,
}

impl AssetStatsResponseDTO {
    ///Creates a new [`AssetStatsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(images: i64, total: i64, videos: i64) -> Self {
        Self { images, total, videos }
    }

    ///Sets `images`.
    #[must_use]
    pub fn images(mut self, value: i64) -> Self {
        self.images = value;
        self
    }

    ///Sets `total`.
    #[must_use]
    pub fn total(mut self, value: i64) -> Self {
        self.total = value;
        self
    }

    ///Sets `videos`.
    #[must_use]
    pub fn videos(mut self, value: i64) -> Self {
        self.videos = value;
        self
    }
}

/// Asset type
///
/// See: <https://api.immich.app/models/AssetTypeEnum>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetTypeEnum {
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "AUDIO")]
    Audio,
    #[serde(rename = "OTHER")]
    Other,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetTypeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Image => f.write_str("IMAGE"),
            Self::Video => f.write_str("VIDEO"),
            Self::Audio => f.write_str("AUDIO"),
            Self::Other => f.write_str("OTHER"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetTypeEnum {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "IMAGE" => Ok(Self::Image),
            "VIDEO" => Ok(Self::Video),
            "AUDIO" => Ok(Self::Audio),
            "OTHER" => Ok(Self::Other),
            _ => Err(format!("invalid value for {}: {}", "AssetTypeEnum", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetTypeEnum {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetTypeEnum {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetTypeEnum {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Upload action
///
/// See: <https://api.immich.app/models/AssetUploadAction>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetUploadAction {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "reject")]
    Reject,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetUploadAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Accept => f.write_str("accept"),
            Self::Reject => f.write_str("reject"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetUploadAction {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "accept" => Ok(Self::Accept),
            "reject" => Ok(Self::Reject),
            _ => Err(format!("invalid value for {}: {}", "AssetUploadAction", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetUploadAction {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetUploadAction {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetUploadAction {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// Asset visibility
///
/// See: <https://api.immich.app/models/AssetVisibility>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AssetVisibility {
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "timeline")]
    Timeline,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "locked")]
    Locked,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for AssetVisibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Archive => f.write_str("archive"),
            Self::Timeline => f.write_str("timeline"),
            Self::Hidden => f.write_str("hidden"),
            Self::Locked => f.write_str("locked"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for AssetVisibility {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "archive" => Ok(Self::Archive),
            "timeline" => Ok(Self::Timeline),
            "hidden" => Ok(Self::Hidden),
            "locked" => Ok(Self::Locked),
            _ => Err(format!("invalid value for {}: {}", "AssetVisibility", value).into()),
        }
    }
}

impl TryFrom<&str> for AssetVisibility {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for AssetVisibility {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for AssetVisibility {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CropParameters {
    pub height: ::std::num::NonZeroU64,
    pub width: ::std::num::NonZeroU64,
    pub x: i64,
    pub y: i64,
}

impl CropParameters {
    ///Creates a new [`CropParameters`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(height: ::std::num::NonZeroU64, width: ::std::num::NonZeroU64, x: i64, y: i64) -> Self {
        Self { height, width, x, y }
    }

    ///Sets `height`.
    #[must_use]
    pub fn height(mut self, value: ::std::num::NonZeroU64) -> Self {
        self.height = value;
        self
    }

    ///Sets `width`.
    #[must_use]
    pub fn width(mut self, value: ::std::num::NonZeroU64) -> Self {
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
pub struct ExifResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: Option<String>,
    /// Original date/time
    #[serde(
        rename = "dateTimeOriginal",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_time_original: Option<::chrono::DateTime<::chrono::offset::Utc>>,
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
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub iso: Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: Option<f64>,
    /// Lens model
    #[serde(
        rename = "lensModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lens_model: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub make: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub model: Option<String>,
    /// Modification date/time
    #[serde(
        rename = "modifyDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modify_date: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orientation: Option<String>,
    /// Projection type
    #[serde(
        rename = "projectionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub projection_type: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<::std::num::NonZeroU64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state: Option<String>,
    /// Time zone
    #[serde(rename = "timeZone", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time_zone: Option<String>,
}

impl ExifResponseDTO {
    ///Creates a new [`ExifResponseDTO`].
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

    ///Sets `date_time_original`.
    #[must_use]
    pub fn date_time_original(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.date_time_original = Some(value);
        self
    }

    ///Sets or clears `date_time_original`.
    #[must_use]
    pub fn set_date_time_original(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.date_time_original = value;
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

    ///Sets `exif_image_height`.
    #[must_use]
    pub fn exif_image_height(mut self, value: i64) -> Self {
        self.exif_image_height = Some(value);
        self
    }

    ///Sets or clears `exif_image_height`.
    #[must_use]
    pub fn set_exif_image_height(mut self, value: Option<i64>) -> Self {
        self.exif_image_height = value;
        self
    }

    ///Sets `exif_image_width`.
    #[must_use]
    pub fn exif_image_width(mut self, value: i64) -> Self {
        self.exif_image_width = Some(value);
        self
    }

    ///Sets or clears `exif_image_width`.
    #[must_use]
    pub fn set_exif_image_width(mut self, value: Option<i64>) -> Self {
        self.exif_image_width = value;
        self
    }

    ///Sets `exposure_time`.
    #[must_use]
    pub fn exposure_time(mut self, value: impl Into<String>) -> Self {
        self.exposure_time = Some(value.into());
        self
    }

    ///Sets or clears `exposure_time`.
    #[must_use]
    pub fn set_exposure_time(mut self, value: Option<String>) -> Self {
        self.exposure_time = value;
        self
    }

    ///Sets `f_number`.
    #[must_use]
    pub fn f_number(mut self, value: f64) -> Self {
        self.f_number = Some(value);
        self
    }

    ///Sets or clears `f_number`.
    #[must_use]
    pub fn set_f_number(mut self, value: Option<f64>) -> Self {
        self.f_number = value;
        self
    }

    ///Sets `file_size_in_byte`.
    #[must_use]
    pub fn file_size_in_byte(mut self, value: i64) -> Self {
        self.file_size_in_byte = Some(value);
        self
    }

    ///Sets or clears `file_size_in_byte`.
    #[must_use]
    pub fn set_file_size_in_byte(mut self, value: Option<i64>) -> Self {
        self.file_size_in_byte = value;
        self
    }

    ///Sets `focal_length`.
    #[must_use]
    pub fn focal_length(mut self, value: f64) -> Self {
        self.focal_length = Some(value);
        self
    }

    ///Sets or clears `focal_length`.
    #[must_use]
    pub fn set_focal_length(mut self, value: Option<f64>) -> Self {
        self.focal_length = value;
        self
    }

    ///Sets `iso`.
    #[must_use]
    pub fn iso(mut self, value: i64) -> Self {
        self.iso = Some(value);
        self
    }

    ///Sets or clears `iso`.
    #[must_use]
    pub fn set_iso(mut self, value: Option<i64>) -> Self {
        self.iso = value;
        self
    }

    ///Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    ///Sets or clears `latitude`.
    #[must_use]
    pub fn set_latitude(mut self, value: Option<f64>) -> Self {
        self.latitude = value;
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

    ///Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    ///Sets or clears `longitude`.
    #[must_use]
    pub fn set_longitude(mut self, value: Option<f64>) -> Self {
        self.longitude = value;
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

    ///Sets `modify_date`.
    #[must_use]
    pub fn modify_date(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.modify_date = Some(value);
        self
    }

    ///Sets or clears `modify_date`.
    #[must_use]
    pub fn set_modify_date(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.modify_date = value;
        self
    }

    ///Sets `orientation`.
    #[must_use]
    pub fn orientation(mut self, value: impl Into<String>) -> Self {
        self.orientation = Some(value.into());
        self
    }

    ///Sets or clears `orientation`.
    #[must_use]
    pub fn set_orientation(mut self, value: Option<String>) -> Self {
        self.orientation = value;
        self
    }

    ///Sets `projection_type`.
    #[must_use]
    pub fn projection_type(mut self, value: impl Into<String>) -> Self {
        self.projection_type = Some(value.into());
        self
    }

    ///Sets or clears `projection_type`.
    #[must_use]
    pub fn set_projection_type(mut self, value: Option<String>) -> Self {
        self.projection_type = value;
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

    ///Sets `time_zone`.
    #[must_use]
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    ///Sets or clears `time_zone`.
    #[must_use]
    pub fn set_time_zone(mut self, value: Option<String>) -> Self {
        self.time_zone = value;
        self
    }
}

/// Axis to mirror along
///
/// See: <https://api.immich.app/models/MirrorAxis>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MirrorAxis {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for MirrorAxis {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Horizontal => f.write_str("horizontal"),
            Self::Vertical => f.write_str("vertical"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for MirrorAxis {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "horizontal" => Ok(Self::Horizontal),
            "vertical" => Ok(Self::Vertical),
            _ => Err(format!("invalid value for {}: {}", "MirrorAxis", value).into()),
        }
    }
}

impl TryFrom<&str> for MirrorAxis {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for MirrorAxis {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for MirrorAxis {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `MirrorParameters`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `axis` | [`MirrorAxis`] | ✓ |  |
///
/// See: <https://api.immich.app/models/MirrorParameters>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MirrorParameters {
    pub axis: MirrorAxis,
}

impl MirrorParameters {
    ///Creates a new [`MirrorParameters`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(axis: MirrorAxis) -> Self {
        Self { axis }
    }

    ///Sets `axis`.
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
/// | `email` | [`String`] | ✓ | User email |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | User ID |
/// | `name` | [`String`] | ✓ | User name |
/// | `profile_changed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Profile change date |
/// | `profile_image_path` | [`String`] | ✓ | Profile image path |
/// | `role` | [`PersonUserRole`] | ✓ | Access role |
///
/// See: <https://api.immich.app/models/PeopleUserResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PeopleUserResponseDTO {
    /// (required)
    #[serde(rename = "avatarColor")]
    pub avatar_color: UserAvatarColor,
    pub email: PeopleUserResponseEmail,
    pub id: ::uuid::Uuid,
    pub name: String,
    /// Profile change date (required)
    #[serde(rename = "profileChangedAt")]
    pub profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Profile image path (required)
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    pub role: PersonUserRole,
}

impl PeopleUserResponseDTO {
    ///Creates a new [`PeopleUserResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        avatar_color: UserAvatarColor,
        email: PeopleUserResponseEmail,
        id: ::uuid::Uuid,
        name: impl Into<String>,
        profile_changed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        profile_image_path: impl Into<String>,
        role: PersonUserRole,
    ) -> Self {
        Self {
            avatar_color,
            email,
            id,
            name: name.into(),
            profile_changed_at,
            profile_image_path: profile_image_path.into(),
            role,
        }
    }

    ///Sets `avatar_color`.
    #[must_use]
    pub fn avatar_color(mut self, value: UserAvatarColor) -> Self {
        self.avatar_color = value;
        self
    }

    ///Sets `email`.
    #[must_use]
    pub fn email(mut self, value: PeopleUserResponseEmail) -> Self {
        self.email = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `profile_changed_at`.
    #[must_use]
    pub fn profile_changed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.profile_changed_at = value;
        self
    }

    ///Sets `profile_image_path`.
    #[must_use]
    pub fn profile_image_path(mut self, value: impl Into<String>) -> Self {
        self.profile_image_path = value.into();
        self
    }

    ///Sets `role`.
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
/// | `shared_by_id` | [`Uuid`](::uuid::Uuid) | ✓ |  |
///
/// See: <https://api.immich.app/models/PersonOtherResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PersonOtherResponseDTO {
    /// (required)
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    pub name: String,
    pub role: PersonUserRole,
    /// (required)
    #[serde(rename = "sharedById")]
    pub shared_by_id: ::uuid::Uuid,
}

impl PersonOtherResponseDTO {
    ///Creates a new [`PersonOtherResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(name: impl Into<String>, role: PersonUserRole, shared_by_id: ::uuid::Uuid) -> Self {
        Self {
            birth_date: None,
            name: name.into(),
            role,
            shared_by_id,
        }
    }

    ///Sets `birth_date`.
    #[must_use]
    pub fn birth_date(mut self, value: impl Into<String>) -> Self {
        self.birth_date = Some(value.into());
        self
    }

    ///Sets or clears `birth_date`.
    #[must_use]
    pub fn set_birth_date(mut self, value: Option<String>) -> Self {
        self.birth_date = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `role`.
    #[must_use]
    pub fn role(mut self, value: PersonUserRole) -> Self {
        self.role = value;
        self
    }

    ///Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_by_id = value;
        self
    }
}

/// `PersonResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`String`] | ✓ | Person date of birth |
/// | `color` | [`String`] |  | Person color (hex) |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Person ID |
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
pub struct PersonResponseDTO {
    /// Person date of birth (required)
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<::chrono::naive::NaiveDate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<String>,
    pub id: ::uuid::Uuid,
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
    ///Creates a new [`PersonResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        id: ::uuid::Uuid,
        is_hidden: bool,
        name: impl Into<String>,
        other_people: Vec<PersonOtherResponseDTO>,
        shared_by: Vec<PeopleUserResponseDTO>,
        shared_with: Vec<PeopleUserResponseDTO>,
        thumbnail_path: impl Into<String>,
    ) -> Self {
        Self {
            birth_date: None,
            color: None,
            id,
            is_favorite: None,
            is_hidden,
            name: name.into(),
            other_people,
            shared_by,
            shared_with,
            thumbnail_path: thumbnail_path.into(),
            updated_at: None,
        }
    }

    ///Sets `birth_date`.
    #[must_use]
    pub fn birth_date(mut self, value: ::chrono::naive::NaiveDate) -> Self {
        self.birth_date = Some(value);
        self
    }

    ///Sets or clears `birth_date`.
    #[must_use]
    pub fn set_birth_date(mut self, value: Option<::chrono::naive::NaiveDate>) -> Self {
        self.birth_date = value;
        self
    }

    ///Sets `color`.
    #[must_use]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    ///Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<String>) -> Self {
        self.color = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
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

    ///Sets `is_hidden`.
    #[must_use]
    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `other_people`.
    #[must_use]
    pub fn other_people(mut self, value: Vec<PersonOtherResponseDTO>) -> Self {
        self.other_people = value;
        self
    }

    ///Sets `shared_by`.
    #[must_use]
    pub fn shared_by(mut self, value: Vec<PeopleUserResponseDTO>) -> Self {
        self.shared_by = value;
        self
    }

    ///Sets `shared_with`.
    #[must_use]
    pub fn shared_with(mut self, value: Vec<PeopleUserResponseDTO>) -> Self {
        self.shared_with = value;
        self
    }

    ///Sets `thumbnail_path`.
    #[must_use]
    pub fn thumbnail_path(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_path = value.into();
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    ///Sets or clears `updated_at`.
    #[must_use]
    pub fn set_updated_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.updated_at = value;
        self
    }
}

/// Levels of access for managing people resources on behalf of another user.
///
/// See: <https://api.immich.app/models/PersonUserRole>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PersonUserRole {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for PersonUserRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Read => f.write_str("read"),
            Self::Write => f.write_str("write"),
            Self::Admin => f.write_str("admin"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for PersonUserRole {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "read" => Ok(Self::Read),
            "write" => Ok(Self::Write),
            "admin" => Ok(Self::Admin),
            _ => Err(format!("invalid value for {}: {}", "PersonUserRole", value).into()),
        }
    }
}

impl TryFrom<&str> for PersonUserRole {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PersonUserRole {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PersonUserRole {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `RotateParameters`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `angle` | `i64` | ✓ | Rotation angle in degrees |
///
/// See: <https://api.immich.app/models/RotateParameters>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct RotateParameters {
    pub angle: i64,
}

impl RotateParameters {
    ///Creates a new [`RotateParameters`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(angle: i64) -> Self {
        Self { angle }
    }

    ///Sets `angle`.
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
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Tag ID |
/// | `name` | [`String`] | ✓ | Tag name |
/// | `parent_id` | [`String`] |  | Parent tag ID |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
/// | `value` | [`String`] | ✓ | Tag value (full path) |
///
/// See: <https://api.immich.app/models/TagResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TagResponseDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<String>,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub id: ::uuid::Uuid,
    pub name: String,
    /// Parent tag ID
    #[serde(rename = "parentId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_id: Option<String>,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    pub value: String,
}

impl TagResponseDTO {
    ///Creates a new [`TagResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        id: ::uuid::Uuid,
        name: impl Into<String>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            color: None,
            created_at,
            id,
            name: name.into(),
            parent_id: None,
            updated_at,
            value: value.into(),
        }
    }

    ///Sets `color`.
    #[must_use]
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    ///Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<String>) -> Self {
        self.color = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `parent_id`.
    #[must_use]
    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    ///Sets or clears `parent_id`.
    #[must_use]
    pub fn set_parent_id(mut self, value: Option<String>) -> Self {
        self.parent_id = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }

    ///Sets `value`.
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
/// | `live_photo_video_id` | [`Uuid`](::uuid::Uuid) |  | Live photo video ID |
/// | `longitude` | `f64` |  | Longitude coordinate |
/// | `rating` | `i64` |  | Rating in range [1-5] (starred), -1 (rejected), or null (unrated) |
/// | `visibility` | [`AssetVisibility`] |  |  |
///
/// See: <https://api.immich.app/models/UpdateAssetDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct UpdateAssetDTO {
    /// Original date and time
    #[serde(
        rename = "dateTimeOriginal",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_time_original: Option<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: Option<String>,
    /// Mark as favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: Option<f64>,
    /// Live photo video ID
    #[serde(
        rename = "livePhotoVideoId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub live_photo_video_id: Option<::uuid::Uuid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rating: Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibility: Option<AssetVisibility>,
}

impl UpdateAssetDTO {
    ///Creates a new [`UpdateAssetDTO`].
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

    ///Sets `date_time_original`.
    #[must_use]
    pub fn date_time_original(mut self, value: impl Into<String>) -> Self {
        self.date_time_original = Some(value.into());
        self
    }

    ///Sets or clears `date_time_original`.
    #[must_use]
    pub fn set_date_time_original(mut self, value: Option<String>) -> Self {
        self.date_time_original = value;
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

    ///Sets `latitude`.
    #[must_use]
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    ///Sets or clears `latitude`.
    #[must_use]
    pub fn set_latitude(mut self, value: Option<f64>) -> Self {
        self.latitude = value;
        self
    }

    ///Sets `live_photo_video_id`.
    #[must_use]
    pub fn live_photo_video_id(mut self, value: ::uuid::Uuid) -> Self {
        self.live_photo_video_id = Some(value);
        self
    }

    ///Sets or clears `live_photo_video_id`.
    #[must_use]
    pub fn set_live_photo_video_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.live_photo_video_id = value;
        self
    }

    ///Sets `longitude`.
    #[must_use]
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    ///Sets or clears `longitude`.
    #[must_use]
    pub fn set_longitude(mut self, value: Option<f64>) -> Self {
        self.longitude = value;
        self
    }

    ///Sets `rating`.
    #[must_use]
    pub fn rating(mut self, value: i64) -> Self {
        self.rating = Some(value);
        self
    }

    ///Sets or clears `rating`.
    #[must_use]
    pub fn set_rating(mut self, value: Option<i64>) -> Self {
        self.rating = value;
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
