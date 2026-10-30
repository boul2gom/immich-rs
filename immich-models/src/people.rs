// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `people`.

pub use crate::activities::UserResponseDTO;
pub use crate::assets::{PersonResponseDTO, PersonUserRole};
pub use crate::id::{AssetId, PersonId, UserId};

/// `AssetFaceUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `data` | [`Vec<AssetFaceUpdateItem>`](crate::people::AssetFaceUpdateItem) | ✓ | Face update items |
///
/// See: <https://api.immich.app/models/AssetFaceUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetFaceUpdateDTO {
    /// Face update items (required)
    pub data: Vec<AssetFaceUpdateItem>,
}

impl AssetFaceUpdateDTO {
    /// Creates a new [`AssetFaceUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(data: impl IntoIterator<Item = AssetFaceUpdateItem>) -> Self {
        Self {
            data: data.into_iter().collect(),
        }
    }
    /// Sets `data`.
    #[must_use]
    pub fn data(mut self, value: impl IntoIterator<Item = AssetFaceUpdateItem>) -> Self {
        self.data = value.into_iter().collect();
        self
    }
}

/// `AssetFaceUpdateItem`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_id` | [`AssetId`](crate::id::AssetId) | ✓ | Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `person_id` | [`PersonId`](crate::id::PersonId) | ✓ | Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `user_id` | [`UserId`](crate::id::UserId) |  | User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/AssetFaceUpdateItem>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AssetFaceUpdateItem {
    /// Asset ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "assetId")]
    pub asset_id: AssetId,
    /// Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "personId")]
    pub person_id: PersonId,
    /// User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<UserId>,
}

impl AssetFaceUpdateItem {
    /// Creates a new [`AssetFaceUpdateItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_id: impl Into<AssetId>, person_id: impl Into<PersonId>) -> Self {
        Self {
            asset_id: asset_id.into(),
            person_id: person_id.into(),
            user_id: None,
        }
    }
    /// Sets `asset_id`.
    #[must_use]
    pub fn asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.asset_id = value.into();
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
}

/// `MergePersonDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ids` | [`Vec<PersonId>`](crate::id::PersonId) | ✓ | Person IDs to merge |
///
/// See: <https://api.immich.app/models/MergePersonDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MergePersonDTO {
    /// Person IDs to merge (required)
    pub ids: Vec<PersonId>,
}

impl MergePersonDTO {
    /// Creates a new [`MergePersonDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: impl IntoIterator<Item = impl Into<PersonId>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: impl IntoIterator<Item = impl Into<PersonId>>) -> Self {
        self.ids = value.into_iter().map(Into::into).collect();
        self
    }
}

/// `PeopleDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `ids` | [`Vec<PersonId>`](crate::id::PersonId) | ✓ | IDs to process |
/// | `user_id` | [`UserId`](crate::id::UserId) |  |  |
///
/// See: <https://api.immich.app/models/PeopleDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PeopleDeleteDTO {
    /// IDs to process (required)
    pub ids: Vec<PersonId>,
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<String>,
}

impl PeopleDeleteDTO {
    /// Creates a new [`PeopleDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(ids: impl IntoIterator<Item = impl Into<PersonId>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
            user_id: None,
        }
    }
    /// Sets `ids`.
    #[must_use]
    pub fn ids(mut self, value: impl IntoIterator<Item = impl Into<PersonId>>) -> Self {
        self.ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }
    /// Sets or clears `user_id`.
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
#[non_exhaustive]
pub struct PeopleResponseDTO {
    /// Whether there are more pages
    #[serde(
        rename = "hasNextPage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_next_page: Option<bool>,
    /// Number of hidden people (required)
    pub hidden: i64,
    /// (required)
    pub people: Vec<PersonResponseDTO>,
    /// Total number of people (required)
    pub total: i64,
}

impl PeopleResponseDTO {
    /// Creates a new [`PeopleResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(hidden: i64, people: impl IntoIterator<Item = PersonResponseDTO>, total: i64) -> Self {
        Self {
            has_next_page: None,
            hidden,
            people: people.into_iter().collect(),
            total,
        }
    }
    /// Sets `has_next_page`.
    #[must_use]
    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }
    /// Sets or clears `has_next_page`.
    #[must_use]
    pub fn set_has_next_page(mut self, value: Option<bool>) -> Self {
        self.has_next_page = value;
        self
    }
    /// Sets `hidden`.
    #[must_use]
    pub fn hidden(mut self, value: i64) -> Self {
        self.hidden = value;
        self
    }
    /// Sets `people`.
    #[must_use]
    pub fn people(mut self, value: impl IntoIterator<Item = PersonResponseDTO>) -> Self {
        self.people = value.into_iter().collect();
        self
    }
    /// Sets `total`.
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
#[non_exhaustive]
pub struct PeopleUpdateDTO {
    /// People to update (required)
    pub people: Vec<PeopleUpdateItem>,
}

impl PeopleUpdateDTO {
    /// Creates a new [`PeopleUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(people: impl IntoIterator<Item = PeopleUpdateItem>) -> Self {
        Self {
            people: people.into_iter().collect(),
        }
    }
    /// Sets `people`.
    #[must_use]
    pub fn people(mut self, value: impl IntoIterator<Item = PeopleUpdateItem>) -> Self {
        self.people = value.into_iter().collect();
        self
    }
}

/// `PeopleUpdateItem`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`String`] |  | Person date of birth |
/// | `color` | [`String`] |  | Person color (hex) (pattern: `^#?([0-9A-Fa-f]{3}\|[0-9A-Fa-f]{4}\|[0-9A-Fa-f]{6}\|[0-9A-Fa-f]{8})$`) |
/// | `feature_face_asset_id` | [`AssetId`](crate::id::AssetId) |  | Asset ID used for feature face thumbnail (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `id` | [`PersonId`](crate::id::PersonId) | ✓ | Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `is_hidden` | `bool` |  | Person visibility (hidden) |
/// | `name` | [`String`] |  | Person name |
/// | `user_id` | [`UserId`](crate::id::UserId) |  | Restrict the update to the person record of this User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\|00000000-0000-0000-0000-000000000000\|ffffffff-ffff-ffff-ffff-ffffffffffff)$`) |
///
/// See: <https://api.immich.app/models/PeopleUpdateItem>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PeopleUpdateItem {
    /// Person date of birth
    #[serde(
        rename = "birthDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub birth_date: Option<::chrono::naive::NaiveDate>,
    /// Person color (hex) (pattern: `^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$`)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<String>,
    /// Asset ID used for feature face thumbnail (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(
        rename = "featureFaceAssetId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub feature_face_asset_id: Option<AssetId>,
    /// Person ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: PersonId,
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
    /// Person name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// Restrict the update to the person record of this User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$`)
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<UserId>,
}

impl PeopleUpdateItem {
    /// Creates a new [`PeopleUpdateItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(id: impl Into<PersonId>) -> Self {
        Self {
            birth_date: None,
            color: None,
            feature_face_asset_id: None,
            id: id.into(),
            is_favorite: None,
            is_hidden: None,
            name: None,
            user_id: None,
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
    /// Sets `feature_face_asset_id`.
    #[must_use]
    pub fn feature_face_asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.feature_face_asset_id = Some(value.into());
        self
    }
    /// Sets or clears `feature_face_asset_id`.
    #[must_use]
    pub fn set_feature_face_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.feature_face_asset_id = value;
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
        self.is_hidden = Some(value);
        self
    }
    /// Sets or clears `is_hidden`.
    #[must_use]
    pub fn set_is_hidden(mut self, value: Option<bool>) -> Self {
        self.is_hidden = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
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
}

/// `PeopleUsersUpsertDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `person_ids` | [`Vec<PersonId>`](crate::id::PersonId) |  | Person IDs, required when type is omitted |
/// | `role` | [`PersonUserRole`] | ✓ | Role that should be applied |
/// | `shared_with_ids` | [`Vec<UserId>`](crate::id::UserId) | ✓ | User IDs that should be given access to the person |
/// | `type_` | [`PeopleUsersUpsertType`] |  |  |
///
/// See: <https://api.immich.app/models/PeopleUsersUpsertDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PeopleUsersUpsertDTO {
    /// Person IDs, required when type is omitted
    #[serde(rename = "personIds", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub person_ids: Vec<PersonId>,
    /// Role that should be applied (required)
    pub role: PersonUserRole,
    /// User IDs that should be given access to the person (required)
    #[serde(rename = "sharedWithIds")]
    pub shared_with_ids: Vec<UserId>,
    #[serde(rename = "type", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: Option<PeopleUsersUpsertType>,
}

impl PeopleUsersUpsertDTO {
    /// Creates a new [`PeopleUsersUpsertDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(role: PersonUserRole, shared_with_ids: impl IntoIterator<Item = impl Into<UserId>>) -> Self {
        Self {
            person_ids: Vec::new(),
            role,
            shared_with_ids: shared_with_ids.into_iter().map(Into::into).collect(),
            type_: None,
        }
    }
    /// Sets `person_ids`.
    #[must_use]
    pub fn person_ids(mut self, value: impl IntoIterator<Item = impl Into<PersonId>>) -> Self {
        self.person_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `role`.
    #[must_use]
    pub fn role(mut self, value: PersonUserRole) -> Self {
        self.role = value;
        self
    }
    /// Sets `shared_with_ids`.
    #[must_use]
    pub fn shared_with_ids(mut self, value: impl IntoIterator<Item = impl Into<UserId>>) -> Self {
        self.shared_with_ids = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `type_`.
    #[must_use]
    pub fn type_(mut self, value: PeopleUsersUpsertType) -> Self {
        self.type_ = Some(value);
        self
    }
    /// Sets or clears `type_`.
    #[must_use]
    pub fn set_type_(mut self, value: Option<PeopleUsersUpsertType>) -> Self {
        self.type_ = value;
        self
    }
}

/// Which people to update when personIds is omitted
///
/// See: <https://api.immich.app/models/PeopleUsersUpsertType>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PeopleUsersUpsertType {
    Everyone,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl PeopleUsersUpsertType {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Everyone => "everyone",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for PeopleUsersUpsertType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for PeopleUsersUpsertType {
    fn from(value: &str) -> Self {
        match value {
            "everyone" => Self::Everyone,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for PeopleUsersUpsertType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "everyone" => Self::Everyone,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for PeopleUsersUpsertType {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for PeopleUsersUpsertType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for PeopleUsersUpsertType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for PeopleUsersUpsertType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = PeopleUsersUpsertType;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(PeopleUsersUpsertType::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(PeopleUsersUpsertType::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}

/// `PersonCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `birth_date` | [`String`] |  | Person date of birth |
/// | `color` | [`String`] |  | Person color (hex) (pattern: `^#?([0-9A-Fa-f]{3}\|[0-9A-Fa-f]{4}\|[0-9A-Fa-f]{6}\|[0-9A-Fa-f]{8})$`) |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `is_hidden` | `bool` |  | Person visibility (hidden) |
/// | `name` | [`String`] |  | Person name |
///
/// See: <https://api.immich.app/models/PersonCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub struct PersonCreateDTO {
    /// Person date of birth
    #[serde(
        rename = "birthDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub birth_date: Option<::chrono::naive::NaiveDate>,
    /// Person color (hex) (pattern: `^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$`)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<String>,
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
    /// Person name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
}

impl PersonCreateDTO {
    /// Creates a new [`PersonCreateDTO`].
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
        self.is_hidden = Some(value);
        self
    }
    /// Sets or clears `is_hidden`.
    #[must_use]
    pub fn set_is_hidden(mut self, value: Option<bool>) -> Self {
        self.is_hidden = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
}

/// `PersonDeleteDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `user_id` | [`UserId`](crate::id::UserId) |  |  |
///
/// See: <https://api.immich.app/models/PersonDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct PersonDeleteDTO {
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<String>,
}

impl PersonDeleteDTO {
    /// Creates a new [`PersonDeleteDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self { user_id: None }
    }
    /// Sets `user_id`.
    #[must_use]
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }
    /// Sets or clears `user_id`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PersonStatisticsResponseDTO {
    /// Number of assets (required)
    pub assets: i64,
}

impl PersonStatisticsResponseDTO {
    /// Creates a new [`PersonStatisticsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(assets: i64) -> Self {
        Self { assets }
    }
    /// Sets `assets`.
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
/// | `color` | [`String`] |  | Person color (hex) (pattern: `^#?([0-9A-Fa-f]{3}\|[0-9A-Fa-f]{4}\|[0-9A-Fa-f]{6}\|[0-9A-Fa-f]{8})$`) |
/// | `feature_face_asset_id` | [`AssetId`](crate::id::AssetId) |  | Asset ID used for feature face thumbnail (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `is_favorite` | `bool` |  | Mark as favorite |
/// | `is_hidden` | `bool` |  | Person visibility (hidden) |
/// | `name` | [`String`] |  | Person name |
/// | `user_id` | [`UserId`](crate::id::UserId) |  | Restrict the update to the person record of this User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\|00000000-0000-0000-0000-000000000000\|ffffffff-ffff-ffff-ffff-ffffffffffff)$`) |
///
/// See: <https://api.immich.app/models/PersonUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
#[non_exhaustive]
pub struct PersonUpdateDTO {
    /// Person date of birth
    #[serde(
        rename = "birthDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub birth_date: Option<::chrono::naive::NaiveDate>,
    /// Person color (hex) (pattern: `^#?([0-9A-Fa-f]{3}|[0-9A-Fa-f]{4}|[0-9A-Fa-f]{6}|[0-9A-Fa-f]{8})$`)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<String>,
    /// Asset ID used for feature face thumbnail (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)
    #[serde(
        rename = "featureFaceAssetId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub feature_face_asset_id: Option<AssetId>,
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
    /// Person name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// Restrict the update to the person record of this User ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$`)
    #[serde(rename = "userId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user_id: Option<UserId>,
}

impl PersonUpdateDTO {
    /// Creates a new [`PersonUpdateDTO`].
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
    /// Sets `feature_face_asset_id`.
    #[must_use]
    pub fn feature_face_asset_id(mut self, value: impl Into<AssetId>) -> Self {
        self.feature_face_asset_id = Some(value.into());
        self
    }
    /// Sets or clears `feature_face_asset_id`.
    #[must_use]
    pub fn set_feature_face_asset_id(mut self, value: Option<AssetId>) -> Self {
        self.feature_face_asset_id = value;
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
        self.is_hidden = Some(value);
        self
    }
    /// Sets or clears `is_hidden`.
    #[must_use]
    pub fn set_is_hidden(mut self, value: Option<bool>) -> Self {
        self.is_hidden = value;
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<String>) -> Self {
        self.name = value;
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
}

/// `PersonUsersDeleteDTO`
///
/// See: <https://api.immich.app/models/PersonUsersDeleteDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
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

/// `PersonUsersDeleteDtoItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PersonUsersDeleteItem {
    /// Person ID
    #[serde(rename = "personId")]
    pub person_id: PersonId,
    /// User ID of the user that gave access to the person
    #[serde(
        rename = "sharedById",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub shared_by_id: Option<UserId>,
    /// User ID of the user that was given access to the person
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: UserId,
}

impl PersonUsersDeleteItem {
    /// Creates a new [`PersonUsersDeleteItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(person_id: impl Into<PersonId>, shared_with_id: impl Into<UserId>) -> Self {
        Self {
            person_id: person_id.into(),
            shared_by_id: None,
            shared_with_id: shared_with_id.into(),
        }
    }
    /// Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: impl Into<PersonId>) -> Self {
        self.person_id = value.into();
        self
    }
    /// Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_by_id = Some(value.into());
        self
    }
    /// Sets or clears `shared_by_id`.
    #[must_use]
    pub fn set_shared_by_id(mut self, value: Option<UserId>) -> Self {
        self.shared_by_id = value;
        self
    }
    /// Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_with_id = value.into();
        self
    }
}

/// `PersonUsersResponseDTO`
///
/// See: <https://api.immich.app/models/PersonUsersResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
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

/// `PersonUsersResponseDtoItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PersonUsersResponseItem {
    /// Person ID
    #[serde(rename = "personId")]
    pub person_id: PersonId,
    /// Access role
    pub role: PersonUserRole,
    /// The user that gave access to this person
    #[serde(rename = "sharedBy")]
    pub shared_by: UserResponseDTO,
    /// User ID of the user that gave access to this person
    #[serde(rename = "sharedById")]
    pub shared_by_id: UserId,
    /// The user that was given access to this person
    #[serde(rename = "sharedWith")]
    pub shared_with: UserResponseDTO,
    /// User ID of the user that was given access to this person
    #[serde(rename = "sharedWithId")]
    pub shared_with_id: UserId,
}

impl PersonUsersResponseItem {
    /// Creates a new [`PersonUsersResponseItem`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        person_id: impl Into<PersonId>,
        role: PersonUserRole,
        shared_by: UserResponseDTO,
        shared_by_id: impl Into<UserId>,
        shared_with: UserResponseDTO,
        shared_with_id: impl Into<UserId>,
    ) -> Self {
        Self {
            person_id: person_id.into(),
            role,
            shared_by,
            shared_by_id: shared_by_id.into(),
            shared_with,
            shared_with_id: shared_with_id.into(),
        }
    }
    /// Sets `person_id`.
    #[must_use]
    pub fn person_id(mut self, value: impl Into<PersonId>) -> Self {
        self.person_id = value.into();
        self
    }
    /// Sets `role`.
    #[must_use]
    pub fn role(mut self, value: PersonUserRole) -> Self {
        self.role = value;
        self
    }
    /// Sets `shared_by`.
    #[must_use]
    pub fn shared_by(mut self, value: UserResponseDTO) -> Self {
        self.shared_by = value;
        self
    }
    /// Sets `shared_by_id`.
    #[must_use]
    pub fn shared_by_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_by_id = value.into();
        self
    }
    /// Sets `shared_with`.
    #[must_use]
    pub fn shared_with(mut self, value: UserResponseDTO) -> Self {
        self.shared_with = value;
        self
    }
    /// Sets `shared_with_id`.
    #[must_use]
    pub fn shared_with_id(mut self, value: impl Into<UserId>) -> Self {
        self.shared_with_id = value.into();
        self
    }
}

/// Sharing direction
///
/// See: <https://api.immich.app/models/SharingDirection>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SharingDirection {
    SharedBy,
    SharedWith,
    /// Unknown value returned by a newer version of the Immich server.
    Unknown(String),
}

impl SharingDirection {
    /// Returns the wire string representation of this enum value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::SharedBy => "shared-by",
            Self::SharedWith => "shared-with",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl ::std::fmt::Display for SharingDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for SharingDirection {
    fn from(value: &str) -> Self {
        match value {
            "shared-by" => Self::SharedBy,
            "shared-with" => Self::SharedWith,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl From<String> for SharingDirection {
    fn from(value: String) -> Self {
        match value.as_str() {
            "shared-by" => Self::SharedBy,
            "shared-with" => Self::SharedWith,
            _ => Self::Unknown(value),
        }
    }
}

impl From<&String> for SharingDirection {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl ::std::str::FromStr for SharingDirection {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}

impl ::serde::Serialize for SharingDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for SharingDirection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct EnumVisitor;
        impl ::serde::de::Visitor<'_> for EnumVisitor {
            type Value = SharingDirection;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str("a string")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SharingDirection::from(value))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SharingDirection::from(value))
            }
        }
        deserializer.deserialize_str(EnumVisitor)
    }
}
