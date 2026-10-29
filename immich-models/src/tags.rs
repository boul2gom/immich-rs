// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `tags`.

/// `TagBulkAssetsDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Asset IDs |
/// | `tag_ids` | [`Vec<Uuid>`](::uuid::Uuid) | ✓ | Tag IDs |
///
/// See: <https://api.immich.app/models/TagBulkAssetsDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TagBulkAssetsDTO {
    /// Asset IDs (required)
    #[serde(rename = "assetIds")]
    pub asset_ids: Vec<::uuid::Uuid>,
    /// Tag IDs (required)
    #[serde(rename = "tagIds")]
    pub tag_ids: Vec<::uuid::Uuid>,
}

impl TagBulkAssetsDTO {
    ///Creates a new [`TagBulkAssetsDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(asset_ids: Vec<::uuid::Uuid>, tag_ids: Vec<::uuid::Uuid>) -> Self {
        Self { asset_ids, tag_ids }
    }

    ///Sets `asset_ids`.
    #[must_use]
    pub fn asset_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.asset_ids = value;
        self
    }

    ///Sets `tag_ids`.
    #[must_use]
    pub fn tag_ids(mut self, value: Vec<::uuid::Uuid>) -> Self {
        self.tag_ids = value;
        self
    }
}

/// `TagBulkAssetsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `count` | `i64` | ✓ | Number of assets tagged |
///
/// See: <https://api.immich.app/models/TagBulkAssetsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TagBulkAssetsResponseDTO {
    pub count: i64,
}

impl TagBulkAssetsResponseDTO {
    ///Creates a new [`TagBulkAssetsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(count: i64) -> Self {
        Self { count }
    }

    ///Sets `count`.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = value;
        self
    }
}

/// `TagCreateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `color` | [`String`] |  | Tag color (hex) |
/// | `name` | [`String`] | ✓ | Tag name |
/// | `parent_id` | [`Uuid`](::uuid::Uuid) |  | Parent tag ID |
///
/// See: <https://api.immich.app/models/TagCreateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TagCreateDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<TagCreateColor>,
    pub name: TagCreateName,
    /// Parent tag ID
    #[serde(rename = "parentId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_id: Option<::uuid::Uuid>,
}

impl TagCreateDTO {
    ///Creates a new [`TagCreateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(name: TagCreateName) -> Self {
        Self {
            color: None,
            name,
            parent_id: None,
        }
    }

    ///Sets `color`.
    #[must_use]
    pub fn color(mut self, value: TagCreateColor) -> Self {
        self.color = Some(value);
        self
    }

    ///Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<TagCreateColor>) -> Self {
        self.color = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: TagCreateName) -> Self {
        self.name = value;
        self
    }

    ///Sets `parent_id`.
    #[must_use]
    pub fn parent_id(mut self, value: ::uuid::Uuid) -> Self {
        self.parent_id = Some(value);
        self
    }

    ///Sets or clears `parent_id`.
    #[must_use]
    pub fn set_parent_id(mut self, value: Option<::uuid::Uuid>) -> Self {
        self.parent_id = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TagCreateColor(pub String);

impl ::std::ops::Deref for TagCreateColor {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for TagCreateColor {
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

impl TryFrom<&str> for TagCreateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TagCreateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TagCreateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for TagCreateColor {
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
pub struct TagCreateName(pub String);

impl ::std::ops::Deref for TagCreateName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for TagCreateName {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> =
            ::std::sync::LazyLock::new(|| ::regex::Regex::new("^[^/]*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for TagCreateName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TagCreateName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TagCreateName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for TagCreateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `TagUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `color` | [`String`] |  | Tag color (hex) |
/// | `name` | [`String`] |  | Tag name |
///
/// See: <https://api.immich.app/models/TagUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct TagUpdateDTO {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: Option<TagUpdateColor>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<TagUpdateName>,
}

impl TagUpdateDTO {
    ///Creates a new [`TagUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            color: None,
            name: None,
        }
    }

    ///Sets `color`.
    #[must_use]
    pub fn color(mut self, value: TagUpdateColor) -> Self {
        self.color = Some(value);
        self
    }

    ///Sets or clears `color`.
    #[must_use]
    pub fn set_color(mut self, value: Option<TagUpdateColor>) -> Self {
        self.color = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: TagUpdateName) -> Self {
        self.name = Some(value);
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<TagUpdateName>) -> Self {
        self.name = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TagUpdateColor(pub String);

impl ::std::ops::Deref for TagUpdateColor {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for TagUpdateColor {
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

impl TryFrom<&str> for TagUpdateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TagUpdateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TagUpdateColor {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for TagUpdateColor {
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
pub struct TagUpdateName(pub String);

impl ::std::ops::Deref for TagUpdateName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for TagUpdateName {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regex::Regex> =
            ::std::sync::LazyLock::new(|| ::regex::Regex::new("^[^/]*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for TagUpdateName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for TagUpdateName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for TagUpdateName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for TagUpdateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `TagUpsertDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `tags` | [`Vec<String>`](::std::vec::Vec) | ✓ | Tag names to upsert |
///
/// See: <https://api.immich.app/models/TagUpsertDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TagUpsertDTO {
    pub tags: Vec<String>,
}

impl TagUpsertDTO {
    ///Creates a new [`TagUpsertDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(tags: Vec<String>) -> Self {
        Self { tags }
    }

    ///Sets `tags`.
    #[must_use]
    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = value;
        self
    }
}
