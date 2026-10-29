// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `libraries`.

/// `CreateLibraryDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `exclusion_patterns` | [`Vec<String>`](::std::vec::Vec) |  | Exclusion patterns (max 128) |
/// | `import_paths` | [`Vec<String>`](::std::vec::Vec) |  | Import paths (max 128) |
/// | `name` | [`String`] |  | Library name |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner user ID |
///
/// See: <https://api.immich.app/models/CreateLibraryDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct CreateLibraryDTO {
    /// Exclusion patterns (max 128)
    #[serde(
        rename = "exclusionPatterns",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclusion_patterns: Vec<String>,
    /// Import paths (max 128)
    #[serde(rename = "importPaths", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub import_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<CreateLibraryName>,
    /// Owner user ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
}

impl CreateLibraryDTO {
    ///Creates a new [`CreateLibraryDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(owner_id: ::uuid::Uuid) -> Self {
        Self {
            exclusion_patterns: Vec::new(),
            import_paths: Vec::new(),
            name: None,
            owner_id,
        }
    }

    ///Sets `exclusion_patterns`.
    #[must_use]
    pub fn exclusion_patterns(mut self, value: Vec<String>) -> Self {
        self.exclusion_patterns = value;
        self
    }

    ///Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: Vec<String>) -> Self {
        self.import_paths = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: CreateLibraryName) -> Self {
        self.name = Some(value);
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<CreateLibraryName>) -> Self {
        self.name = value;
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CreateLibraryName(pub String);

impl ::std::ops::Deref for CreateLibraryName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for CreateLibraryName {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.is_empty() {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for CreateLibraryName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for CreateLibraryName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for CreateLibraryName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for CreateLibraryName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `LibraryResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_count` | `i64` | ✓ | Number of assets |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date |
/// | `exclusion_patterns` | [`Vec<String>`](::std::vec::Vec) | ✓ | Exclusion patterns |
/// | `id` | [`Uuid`](::uuid::Uuid) | ✓ | Library ID |
/// | `import_paths` | [`Vec<String>`](::std::vec::Vec) | ✓ | Import paths |
/// | `name` | [`String`] | ✓ | Library name |
/// | `owner_id` | [`Uuid`](::uuid::Uuid) | ✓ | Owner user ID |
/// | `refreshed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last refresh date |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date |
///
/// See: <https://api.immich.app/models/LibraryResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct LibraryResponseDTO {
    /// Number of assets (required)
    #[serde(rename = "assetCount")]
    pub asset_count: i64,
    /// Creation date (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Exclusion patterns (required)
    #[serde(rename = "exclusionPatterns")]
    pub exclusion_patterns: Vec<String>,
    pub id: ::uuid::Uuid,
    /// Import paths (required)
    #[serde(rename = "importPaths")]
    pub import_paths: Vec<String>,
    pub name: String,
    /// Owner user ID (required)
    #[serde(rename = "ownerId")]
    pub owner_id: ::uuid::Uuid,
    /// Last refresh date (required)
    #[serde(rename = "refreshedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Last update date (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl LibraryResponseDTO {
    ///Creates a new [`LibraryResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_count: i64,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        exclusion_patterns: Vec<String>,
        id: ::uuid::Uuid,
        import_paths: Vec<String>,
        name: impl Into<String>,
        owner_id: ::uuid::Uuid,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            asset_count,
            created_at,
            exclusion_patterns,
            id,
            import_paths,
            name: name.into(),
            owner_id,
            refreshed_at: None,
            updated_at,
        }
    }

    ///Sets `asset_count`.
    #[must_use]
    pub fn asset_count(mut self, value: i64) -> Self {
        self.asset_count = value;
        self
    }

    ///Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }

    ///Sets `exclusion_patterns`.
    #[must_use]
    pub fn exclusion_patterns(mut self, value: Vec<String>) -> Self {
        self.exclusion_patterns = value;
        self
    }

    ///Sets `id`.
    #[must_use]
    pub fn id(mut self, value: ::uuid::Uuid) -> Self {
        self.id = value;
        self
    }

    ///Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: Vec<String>) -> Self {
        self.import_paths = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }

    ///Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: ::uuid::Uuid) -> Self {
        self.owner_id = value;
        self
    }

    ///Sets `refreshed_at`.
    #[must_use]
    pub fn refreshed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.refreshed_at = Some(value);
        self
    }

    ///Sets or clears `refreshed_at`.
    #[must_use]
    pub fn set_refreshed_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.refreshed_at = value;
        self
    }

    ///Sets `updated_at`.
    #[must_use]
    pub fn updated_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.updated_at = value;
        self
    }
}

/// `LibraryStatsResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `photos` | `i64` | ✓ | Number of photos |
/// | `total` | `i64` | ✓ | Total number of assets |
/// | `usage` | `i64` | ✓ | Storage usage in bytes |
/// | `videos` | `i64` | ✓ | Number of videos |
///
/// See: <https://api.immich.app/models/LibraryStatsResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct LibraryStatsResponseDTO {
    pub photos: i64,
    pub total: i64,
    pub usage: i64,
    pub videos: i64,
}

impl LibraryStatsResponseDTO {
    ///Creates a new [`LibraryStatsResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(photos: i64, total: i64, usage: i64, videos: i64) -> Self {
        Self {
            photos,
            total,
            usage,
            videos,
        }
    }

    ///Sets `photos`.
    #[must_use]
    pub fn photos(mut self, value: i64) -> Self {
        self.photos = value;
        self
    }

    ///Sets `total`.
    #[must_use]
    pub fn total(mut self, value: i64) -> Self {
        self.total = value;
        self
    }

    ///Sets `usage`.
    #[must_use]
    pub fn usage(mut self, value: i64) -> Self {
        self.usage = value;
        self
    }

    ///Sets `videos`.
    #[must_use]
    pub fn videos(mut self, value: i64) -> Self {
        self.videos = value;
        self
    }
}

/// `UpdateLibraryDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `exclusion_patterns` | [`Vec<String>`](::std::vec::Vec) |  | Exclusion patterns (max 128) |
/// | `import_paths` | [`Vec<String>`](::std::vec::Vec) |  | Import paths (max 128) |
/// | `name` | [`String`] |  | Library name |
///
/// See: <https://api.immich.app/models/UpdateLibraryDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct UpdateLibraryDTO {
    /// Exclusion patterns (max 128)
    #[serde(
        rename = "exclusionPatterns",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclusion_patterns: Vec<String>,
    /// Import paths (max 128)
    #[serde(rename = "importPaths", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub import_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<UpdateLibraryName>,
}

impl UpdateLibraryDTO {
    ///Creates a new [`UpdateLibraryDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            exclusion_patterns: Vec::new(),
            import_paths: Vec::new(),
            name: None,
        }
    }

    ///Sets `exclusion_patterns`.
    #[must_use]
    pub fn exclusion_patterns(mut self, value: Vec<String>) -> Self {
        self.exclusion_patterns = value;
        self
    }

    ///Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: Vec<String>) -> Self {
        self.import_paths = value;
        self
    }

    ///Sets `name`.
    #[must_use]
    pub fn name(mut self, value: UpdateLibraryName) -> Self {
        self.name = Some(value);
        self
    }

    ///Sets or clears `name`.
    #[must_use]
    pub fn set_name(mut self, value: Option<UpdateLibraryName>) -> Self {
        self.name = value;
        self
    }
}

#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UpdateLibraryName(pub String);

impl ::std::ops::Deref for UpdateLibraryName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl ::std::str::FromStr for UpdateLibraryName {
    type Err = crate::common::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        if value.is_empty() {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for UpdateLibraryName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for UpdateLibraryName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for UpdateLibraryName {
    type Error = crate::common::error::ConversionError;
    fn try_from(value: String) -> Result<Self, crate::common::error::ConversionError> {
        value.parse()
    }
}

impl<'de> ::serde::Deserialize<'de> for UpdateLibraryName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: crate::common::error::ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

/// `ValidateLibraryDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `exclusion_patterns` | [`Vec<String>`](::std::vec::Vec) |  | Exclusion patterns (max 128) |
/// | `import_paths` | [`Vec<String>`](::std::vec::Vec) |  | Import paths to validate (max 128) |
///
/// See: <https://api.immich.app/models/ValidateLibraryDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct ValidateLibraryDTO {
    /// Exclusion patterns (max 128)
    #[serde(
        rename = "exclusionPatterns",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclusion_patterns: Vec<String>,
    /// Import paths to validate (max 128)
    #[serde(rename = "importPaths", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub import_paths: Vec<String>,
}

impl ValidateLibraryDTO {
    ///Creates a new [`ValidateLibraryDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            exclusion_patterns: Vec::new(),
            import_paths: Vec::new(),
        }
    }

    ///Sets `exclusion_patterns`.
    #[must_use]
    pub fn exclusion_patterns(mut self, value: Vec<String>) -> Self {
        self.exclusion_patterns = value;
        self
    }

    ///Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: Vec<String>) -> Self {
        self.import_paths = value;
        self
    }
}

/// `ValidateLibraryImportPathResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `import_path` | [`String`] | ✓ | Import path |
/// | `is_valid` | `bool` | ✓ | Is valid |
/// | `message` | [`String`] |  | Validation message |
///
/// See: <https://api.immich.app/models/ValidateLibraryImportPathResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ValidateLibraryImportPathResponseDTO {
    /// Import path (required)
    #[serde(rename = "importPath")]
    pub import_path: String,
    /// Is valid (required)
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: Option<String>,
}

impl ValidateLibraryImportPathResponseDTO {
    ///Creates a new [`ValidateLibraryImportPathResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(import_path: impl Into<String>, is_valid: bool) -> Self {
        Self {
            import_path: import_path.into(),
            is_valid,
            message: None,
        }
    }

    ///Sets `import_path`.
    #[must_use]
    pub fn import_path(mut self, value: impl Into<String>) -> Self {
        self.import_path = value.into();
        self
    }

    ///Sets `is_valid`.
    #[must_use]
    pub fn is_valid(mut self, value: bool) -> Self {
        self.is_valid = value;
        self
    }

    ///Sets `message`.
    #[must_use]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    ///Sets or clears `message`.
    #[must_use]
    pub fn set_message(mut self, value: Option<String>) -> Self {
        self.message = value;
        self
    }
}

/// `ValidateLibraryResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `import_paths` | [`Vec<ValidateLibraryImportPathResponseDTO>`](crate::libraries::ValidateLibraryImportPathResponseDTO) |  | Validation results for import paths |
///
/// See: <https://api.immich.app/models/ValidateLibraryResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Default)]
pub struct ValidateLibraryResponseDTO {
    /// Validation results for import paths
    #[serde(rename = "importPaths", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub import_paths: Vec<ValidateLibraryImportPathResponseDTO>,
}

impl ValidateLibraryResponseDTO {
    ///Creates a new [`ValidateLibraryResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            import_paths: Vec::new(),
        }
    }

    ///Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: Vec<ValidateLibraryImportPathResponseDTO>) -> Self {
        self.import_paths = value;
        self
    }
}
