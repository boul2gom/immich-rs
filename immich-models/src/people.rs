// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `people`.

pub use crate::activities::UserResponseDTO;
pub use crate::assets::{PersonResponseDTO, PersonUserRole};

/// `AssetFaceUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `data` | [`Vec<AssetFaceUpdateItem>`](crate::people::AssetFaceUpdateItem) | ✓ | Face update items |
///
/// See: <https://api.immich.app/models/AssetFaceUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetFaceUpdateDTO {
    pub data: Vec<AssetFaceUpdateItem>,
}

impl AssetFaceUpdateDTO {
    ///Creates a new [`AssetFaceUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(data: Vec<AssetFaceUpdateItem>) -> Self {
        Self { data }
    }

    ///Sets `data`.
    #[must_use]
    pub fn data(mut self, value: Vec<AssetFaceUpdateItem>) -> Self {
        self.data = value;
        self
    }
}

/// `AssetFaceUpdateItem`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`Uuid`](::uuid::Uuid) | ✓ | Asset ID |
/// | `person_id` | [`Uuid`](::uuid::Uuid) | ✓ | Person ID |
/// | `user_id` | [`Uuid`](::uuid::Uuid) |  | User ID |
///
/// See: <https://api.immich.app/models/AssetFaceUpdateItem>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AssetFaceUpdateItem {
    /// Asset ID (required)
    #[serde(rename = "assetId")]
    pub asset_id: ::uuid::Uuid,
    /// Person ID (required)
    #[serde(rename = "personId")]
    pub person_id: ::uuid::Uuid,
    /// User ID
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<::uuid::Uuid>,
}

impl AssetFaceUpdateItem {
    ///Creates a new [`AssetFaceUpdateItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: ::uuid::Uuid, person_id: ::uuid::Uuid) -> Self {
        Self {
            asset_id,
            person_id,
            user_id: None,
        }
    }

    ///Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.asset_id = value;
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
}

/// `MergePersonDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Person IDs to merge |
///
/// See: <https://api.immich.app/models/MergePersonDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct MergePersonDTO {
    pub ids: Vec<::uuid::Uuid>,
}

impl MergePersonDTO {
    ///Creates a new [`MergePersonDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: Vec<::uuid::Uuid>) -> Self {
        Self { ids }
    }

    ///Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.ids = value;
        self
    }
}

/// `PeopleDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | IDs to process |
/// | `user_id` | [`String`] |  |  |
///
/// See: <https://api.immich.app/models/PeopleDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PeopleDeleteDTO {
    pub ids: Vec<::uuid::Uuid>,
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<String>,
}

impl PeopleDeleteDTO {
    ///Creates a new [`PeopleDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: Vec<::uuid::Uuid>) -> Self {
        Self { ids, user_id: None }
    }

    ///Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.ids = value;
        self
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    ///Sets or clears `user_id`.
    #[must_use]
    pub fn set_user_id(mut self, value: Option<String>) -> Self {
        self.user_id = value;
        self
    }
}

/// People response
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `has_next_page` | `bool` |  | Whether there are more pages |
/// | `hidden` | `i64` | ✓ | Number of hidden people |
/// | `people` | [`Vec<PersonResponseDTO>`](crate::assets::PersonResponseDTO) | ✓ |  |
/// | `total` | `i64` | ✓ | Total number of people |
///
/// See: <https://api.immich.app/models/PeopleResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PeopleResponseDTO {
    /// Whether there are more pages
    #[serde(
        rename = "hasNextPage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_next_page: Option<bool>,
    pub hidden: i64,
    pub people: Vec<PersonResponseDTO>,
    pub total: i64,
}

impl PeopleResponseDTO {
    ///Creates a new [`PeopleResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(hidden: i64, people: Vec<PersonResponseDTO>, total: i64) -> Self {
        Self {
            has_next_page: None,
            hidden,
            people,
            total,
        }
    }

    ///Sets `has_next_page`.
    #[must_use]
    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    ///Sets or clears `has_next_page`.
    #[must_use]
    pub fn set_has_next_page(mut self, value: Option<bool>) -> Self {
        self.has_next_page = value;
        self
    }

    ///Sets `hidden`.
    #[must_use]
    pub fn hidden(mut self, value: i64) -> Self {
        self.hidden = value;
        self
    }

    ///Sets `people`.
    #[must_use]
    pub fn people(mut self, value: Vec<PersonResponseDTO>) -> Self {
        self.people = value;
        self
    }

    ///Sets `total`.
    #[must_use]
    pub fn total(mut self, value: i64) -> Self {
        self.total = value;
        self
    }
}

/// `PeopleUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `people` | [`Vec<PeopleUpdateItem>`](crate::people::PeopleUpdateItem) | ✓ | People to update |
///
/// See: <https://api.immich.app/models/PeopleUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PeopleUpdateDTO {
    pub people: Vec<PeopleUpdateItem>,
}

impl PeopleUpdateDTO {
    ///Creates a new [`PeopleUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(people: Vec<PeopleUpdateItem>) -> Self {
        Self { people }
    }

    ///Sets `people`.
    #[must_use]
    pub fn people(mut self, value: Vec<PeopleUpdateItem>) -> Self {
        self.people = value;
        self
    }
}

/// `PeopleUpdateItem`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`String`] |  | Person date of birth |
/// | `color` | [`String`] |  | Person color (hex) |
/// | `feature_face_asset_id` | [`Uuid`](::uuid::Uuid) |  | Asset ID used for feature face thumbnail |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Person ID |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `is_hidden` | `bool` |  | Person visibility (hidden) |
/// | `name` | [`String`] |  | Person name |
/// | `user_id` | [`Uuid`](::uuid::Uuid) |  | Restrict the update to the person record of this User ID |
///
/// See: <https://api.immich.app/models/PeopleUpdateItem>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PeopleUpdateItem {
    /// Person date of birth
    #[serde(
        rename = "birthDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub birth_date: Option<::chrono::naive::NaiveDate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<PeopleUpdateItemColor>,
    /// Asset ID used for feature face thumbnail
    #[serde(
        rename = "featureFaceAssetId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub feature_face_asset_id: Option<::uuid::Uuid>,
    pub id: ::uuid::Uuid,
    /// Mark as favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Person visibility (hidden)
    #[serde(rename = "isHidden", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// Restrict the update to the person record of this User ID
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<::uuid::Uuid>,
}

impl PeopleUpdateItem {
    ///Creates a new [`PeopleUpdateItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: ::uuid::Uuid) -> Self {
        Self {
            birth_date: None,
            color: None,
            feature_face_asset_id: None,
            id,
            is_favorite: None,
            is_hidden: None,
            name: None,
            user_id: None,
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
    pub fn color(mut self, value: PeopleUpdateItemColor) -> Self {
        self.color = Some(value);
        self
    }

    ///Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<PeopleUpdateItemColor>) -> Self {
        self.color = value;
        self
    }

    ///Sets `feature_face_asset_id`.
    #[must_use]
    pub fn feature_face_asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.feature_face_asset_id = Some(value);
        self
    }

    ///Sets or clears `feature_face_asset_id`.
    #[must_use]
    pub fn set_feature_face_asset_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.feature_face_asset_id = value;
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
        self.is_hidden = Some(value);
        self
    }

    ///Sets or clears `is_hidden`.
    #[must_use]
    pub fn set_is_hidden(mut self, value: Option<bool>) -> Self {
        self.is_hidden = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
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
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PeopleUpdateItemColor(pub String);

impl ::std::ops::Deref for PeopleUpdateItemColor {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for PeopleUpdateItemColor {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new("^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for PeopleUpdateItemColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PeopleUpdateItemColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PeopleUpdateItemColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for PeopleUpdateItemColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PeopleUserResponseEmail(pub String);

impl ::std::ops::Deref for PeopleUserResponseEmail {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for PeopleUserResponseEmail {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new(
                    "^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[\\p{L}\\p{M}\\p{N}.!#$%&'*+/=?^_`{|}~-]+@[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?(?:\\.[\\p{L}\\p{N}](?:[\\p{L}\\p{M}\\p{N}-]{0,61}[\\p{L}\\p{M}\\p{N}])?)*$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for PeopleUserResponseEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PeopleUserResponseEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PeopleUserResponseEmail {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for PeopleUserResponseEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `PeopleUsersUpsertDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `person_ids` | [`Vec<Uuid>`](::uuid::Uuid) |  | Person IDs, required when type is omitted |
/// | `role` | [`PersonUserRole`] | ✓ | Role that should be applied |
/// | `shared_with_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | User IDs that should be given access to the person |
/// | `type` | [`PeopleUsersUpsertType`] |  |  |
///
/// See: <https://api.immich.app/models/PeopleUsersUpsertDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PeopleUsersUpsertDTO {
    /// Person IDs, required when type is omitted
    #[serde(rename = "personIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub person_ids: Vec<::uuid::Uuid>,
    pub role: PersonUserRole,
    /// User IDs that should be given access to the person (required)
    #[serde(rename = "sharedWithIds")]
    pub shared_with_ids: Vec<::uuid::Uuid>,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<PeopleUsersUpsertType>,
}

impl PeopleUsersUpsertDTO {
    ///Creates a new [`PeopleUsersUpsertDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(role: PersonUserRole, shared_with_ids: Vec<::uuid::Uuid>) -> Self {
        Self {
            person_ids: Vec::new(),
            role,
            shared_with_ids,
            type_: None,
        }
    }

    ///Sets `person_ids`.
    #[must_use]
    pub fn person_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.person_ids = value;
        self
    }

    ///Sets `role`.
    #[must_use]
    pub fn role(mut self, value: PersonUserRole) -> Self {
        self.role = value;
        self
    }

    ///Sets `shared_with_ids`.
    #[must_use]
    pub fn shared_with_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.shared_with_ids = value;
        self
    }

    ///Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: PeopleUsersUpsertType) -> Self {
        self.type_ = Some(value);
        self
    }

    ///Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<PeopleUsersUpsertType>) -> Self {
        self.type_ = value;
        self
    }
}

/// Which people to update when personIds is omitted
///
/// See: <https://api.immich.app/models/PeopleUsersUpsertType>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PeopleUsersUpsertType {
    #[serde(rename = "everyone")]
    Everyone,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for PeopleUsersUpsertType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Everyone => f.write_str("everyone"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for PeopleUsersUpsertType {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "everyone" => Ok(Self::Everyone),
            _ => Err(format!("invalid value for {}: {}", "PeopleUsersUpsertType", value).into()),
        }
    }
}

impl TryFrom<&str> for PeopleUsersUpsertType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PeopleUsersUpsertType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PeopleUsersUpsertType {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

/// `PersonCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`String`] |  | Person date of birth |
/// | `color` | [`String`] |  | Person color (hex) |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `is_hidden` | `bool` |  | Person visibility (hidden) |
/// | `name` | [`String`] |  | Person name |
///
/// See: <https://api.immich.app/models/PersonCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct PersonCreateDTO {
    /// Person date of birth
    #[serde(
        rename = "birthDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub birth_date: Option<::chrono::naive::NaiveDate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<PersonCreateColor>,
    /// Mark as favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Person visibility (hidden)
    #[serde(rename = "isHidden", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
}

impl PersonCreateDTO {
    ///Creates a new [`PersonCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            birth_date: None,
            color: None,
            is_favorite: None,
            is_hidden: None,
            name: None,
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
    pub fn color(mut self, value: PersonCreateColor) -> Self {
        self.color = Some(value);
        self
    }

    ///Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<PersonCreateColor>) -> Self {
        self.color = value;
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
        self.is_hidden = Some(value);
        self
    }

    ///Sets or clears `is_hidden`.
    #[must_use]
    pub fn set_is_hidden(mut self, value: Option<bool>) -> Self {
        self.is_hidden = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PersonCreateColor(pub String);

impl ::std::ops::Deref for PersonCreateColor {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for PersonCreateColor {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new("^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for PersonCreateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PersonCreateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PersonCreateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for PersonCreateColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `PersonDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `user_id` | [`String`] |  |  |
///
/// See: <https://api.immich.app/models/PersonDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct PersonDeleteDTO {
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<String>,
}

impl PersonDeleteDTO {
    ///Creates a new [`PersonDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { user_id: None }
    }

    ///Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    ///Sets or clears `user_id`.
    #[must_use]
    pub fn set_user_id(mut self, value: Option<String>) -> Self {
        self.user_id = value;
        self
    }
}

/// `PersonStatisticsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `assets` | `i64` | ✓ | Number of assets |
///
/// See: <https://api.immich.app/models/PersonStatisticsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PersonStatisticsResponseDTO {
    pub assets: i64,
}

impl PersonStatisticsResponseDTO {
    ///Creates a new [`PersonStatisticsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(assets: i64) -> Self {
        Self { assets }
    }

    ///Sets `assets`.
    #[must_use]
    pub fn assets(mut self, value: i64) -> Self {
        self.assets = value;
        self
    }
}

/// `PersonUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`String`] |  | Person date of birth |
/// | `color` | [`String`] |  | Person color (hex) |
/// | `feature_face_asset_id` | [`Uuid`](::uuid::Uuid) |  | Asset ID used for feature face thumbnail |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `is_hidden` | `bool` |  | Person visibility (hidden) |
/// | `name` | [`String`] |  | Person name |
/// | `user_id` | [`Uuid`](::uuid::Uuid) |  | Restrict the update to the person record of this User ID |
///
/// See: <https://api.immich.app/models/PersonUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct PersonUpdateDTO {
    /// Person date of birth
    #[serde(
        rename = "birthDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub birth_date: Option<::chrono::naive::NaiveDate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<PersonUpdateColor>,
    /// Asset ID used for feature face thumbnail
    #[serde(
        rename = "featureFaceAssetId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub feature_face_asset_id: Option<::uuid::Uuid>,
    /// Mark as favorite
    #[serde(
        rename = "isFavorite",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_favorite: Option<bool>,
    /// Person visibility (hidden)
    #[serde(rename = "isHidden", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// Restrict the update to the person record of this User ID
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<::uuid::Uuid>,
}

impl PersonUpdateDTO {
    ///Creates a new [`PersonUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            birth_date: None,
            color: None,
            feature_face_asset_id: None,
            is_favorite: None,
            is_hidden: None,
            name: None,
            user_id: None,
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
    pub fn color(mut self, value: PersonUpdateColor) -> Self {
        self.color = Some(value);
        self
    }

    ///Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<PersonUpdateColor>) -> Self {
        self.color = value;
        self
    }

    ///Sets `feature_face_asset_id`.
    #[must_use]
    pub fn feature_face_asset_id(mut self, value: ::uuid::Uuid) -> Self {
        self.feature_face_asset_id = Some(value);
        self
    }

    ///Sets or clears `feature_face_asset_id`.
    #[must_use]
    pub fn set_feature_face_asset_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.feature_face_asset_id = value;
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
        self.is_hidden = Some(value);
        self
    }

    ///Sets or clears `is_hidden`.
    #[must_use]
    pub fn set_is_hidden(mut self, value: Option<bool>) -> Self {
        self.is_hidden = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
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
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PersonUpdateColor(pub String);

impl ::std::ops::Deref for PersonUpdateColor {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for PersonUpdateColor {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
            ::regex::Regex::new("^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$").unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for PersonUpdateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PersonUpdateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PersonUpdateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for PersonUpdateColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `PersonUsersDeleteDTO`
///
/// See: <https://api.immich.app/models/PersonUsersDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PersonUsersDeleteDTO(pub Vec<PersonUsersDeleteItem>);

impl ::std::ops::Deref for PersonUsersDeleteDTO {
    type Target = Vec<PersonUsersDeleteItem>;
    fn deref(&self) -> &Vec<PersonUsersDeleteItem> {
        &self.0
    }
}

impl From<Vec<PersonUsersDeleteItem>> for PersonUsersDeleteDTO {
    fn from(value: Vec<PersonUsersDeleteItem>) -> Self {
        Self(value)
    }
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PersonUsersDeleteItem {
    ///Person ID
    #[serde(rename = "personId")]
    pub person_id: ::uuid::Uuid,
    ///User ID of the user that gave access to the person
    #[serde(
        rename = "sharedById",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub shared_by_id: Option<::uuid::Uuid>,
    ///User ID of the user that was given access to the person
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: ::uuid::Uuid,
}

impl PersonUsersDeleteItem {
    ///Creates a new [`PersonUsersDeleteItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(person_id: ::uuid::Uuid, shared_with_id: ::uuid::Uuid) -> Self {
        Self {
            person_id,
            shared_by_id: None,
            shared_with_id,
        }
    }

    ///Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: ::uuid::Uuid) -> Self {
        self.person_id = value;
        self
    }

    ///Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_by_id = Some(value);
        self
    }

    ///Sets or clears `shared_by_id`.
    #[must_use]
    pub fn set_shared_by_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.shared_by_id = value;
        self
    }

    ///Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_with_id = value;
        self
    }
}

/// `PersonUsersResponseDTO`
///
/// See: <https://api.immich.app/models/PersonUsersResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct PersonUsersResponseDTO(pub Vec<PersonUsersResponseItem>);

impl ::std::ops::Deref for PersonUsersResponseDTO {
    type Target = Vec<PersonUsersResponseItem>;
    fn deref(&self) -> &Vec<PersonUsersResponseItem> {
        &self.0
    }
}

impl From<Vec<PersonUsersResponseItem>> for PersonUsersResponseDTO {
    fn from(value: Vec<PersonUsersResponseItem>) -> Self {
        Self(value)
    }
}

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct PersonUsersResponseItem {
    ///Person ID
    #[serde(rename = "personId")]
    pub person_id: ::uuid::Uuid,
    ///Access role
    pub role: PersonUserRole,
    ///The user that gave access to this person
    #[serde(rename = "sharedBy")]
    pub shared_by: UserResponseDTO,
    ///User ID of the user that gave access to this person
    #[serde(rename = "sharedById")]
    pub shared_by_id: ::uuid::Uuid,
    ///The user that was given access to this person
    #[serde(rename = "sharedWith")]
    pub shared_with: UserResponseDTO,
    ///User ID of the user that was given access to this person
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: ::uuid::Uuid,
}

impl PersonUsersResponseItem {
    ///Creates a new [`PersonUsersResponseItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        person_id: ::uuid::Uuid,
        role: PersonUserRole,
        shared_by: UserResponseDTO,
        shared_by_id: ::uuid::Uuid,
        shared_with: UserResponseDTO,
        shared_with_id: ::uuid::Uuid,
    ) -> Self {
        Self {
            person_id,
            role,
            shared_by,
            shared_by_id,
            shared_with,
            shared_with_id,
        }
    }

    ///Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: ::uuid::Uuid) -> Self {
        self.person_id = value;
        self
    }

    ///Sets `role`.
    #[must_use]
    pub fn role(mut self, value: PersonUserRole) -> Self {
        self.role = value;
        self
    }

    ///Sets `shared_by`.
    #[must_use]
    pub fn shared_by(mut self, value: UserResponseDTO) -> Self {
        self.shared_by = value;
        self
    }

    ///Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_by_id = value;
        self
    }

    ///Sets `shared_with`.
    #[must_use]
    pub fn shared_with(mut self, value: UserResponseDTO) -> Self {
        self.shared_with = value;
        self
    }

    ///Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: ::uuid::Uuid) -> Self {
        self.shared_with_id = value;
        self
    }
}

/// Sharing direction
///
/// See: <https://api.immich.app/models/SharingDirection>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SharingDirection {
    #[serde(rename = "shared-by")]
    SharedBy,
    #[serde(rename = "shared-with")]
    SharedWith,
    ///Unknown value returned by a newer version of the Immich server.
    #[serde(other)]
    UnknownValue,
}

#[allow(clippy::match_same_arms)]
impl ::std::fmt::Display for SharingDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SharedBy => f.write_str("shared-by"),
            Self::SharedWith => f.write_str("shared-with"),
            Self::UnknownValue => f.write_str("unknown"),
        }
    }
}

impl ::std::str::FromStr for SharingDirection {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        match value {
            "shared-by" => Ok(Self::SharedBy),
            "shared-with" => Ok(Self::SharedWith),
            _ => Err(format!("invalid value for {}: {}", "SharingDirection", value).into()),
        }
    }
}

impl TryFrom<&str> for SharingDirection {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for SharingDirection {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for SharingDirection {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}
