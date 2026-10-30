// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `libraries`.

pub use crate::id::{LibraryId, UserId};

/// `CreateLibraryDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `exclusion_patterns` | [`Vec<String>`](::std::vec::Vec) |  | Exclusion patterns (max 128) |
/// | `import_paths` | [`Vec<String>`](::std::vec::Vec) |  | Import paths (max 128) |
/// | `name` | [`String`] |  | Library name |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
///
/// See: <https://api.immich.app/models/CreateLibraryDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
    /// Library name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
    /// Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
}

impl CreateLibraryDTO {
    /// Creates a new [`CreateLibraryDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(owner_id: impl Into<UserId>) -> Self {
        Self {
            exclusion_patterns: Vec::new(),
            import_paths: Vec::new(),
            name: None,
            owner_id: owner_id.into(),
        }
    }
    /// Sets `exclusion_patterns`.
    #[must_use]
    pub fn exclusion_patterns(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclusion_patterns = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.import_paths = value.into_iter().map(Into::into).collect();
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
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
}

/// `LibraryResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `asset_count` | `i64` | ✓ | Number of assets |
/// | `created_at` | [`DateTime`](::chrono::DateTime) | ✓ | Creation date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `exclusion_patterns` | [`Vec<String>`](::std::vec::Vec) | ✓ | Exclusion patterns |
/// | `id` | [`LibraryId`](crate::id::LibraryId) | ✓ | Library ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `import_paths` | [`Vec<String>`](::std::vec::Vec) | ✓ | Import paths |
/// | `name` | [`String`] | ✓ | Library name |
/// | `owner_id` | [`UserId`](crate::id::UserId) | ✓ | Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) |
/// | `refreshed_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last refresh date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
/// | `updated_at` | [`DateTime`](::chrono::DateTime) | ✓ | Last update date (pattern: `^(?:(?:\d\d[2468][048]\|\d\d[13579][26]\|\d\d0[48]\|[02468][048]00\|[13579][26]00)-02-29\|\d{4}-(?:(?:0[13578]\|1[02])-(?:0[1-9]\|[12]\d\|3[01])\|(?:0[469]\|11)-(?:0[1-9]\|[12]\d\|30)\|(?:02)-(?:0[1-9]\|1\d\|2[0-8])))T(?:(?:[01]\d\|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z\|([+-](?:[01]\d\|2[0-3]):[0-5]\d)))$`) |
///
/// See: <https://api.immich.app/models/LibraryResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct LibraryResponseDTO {
    /// Number of assets (required)
    #[serde(rename = "assetCount")]
    pub asset_count: i64,
    /// Creation date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    /// Exclusion patterns (required)
    #[serde(rename = "exclusionPatterns")]
    pub exclusion_patterns: Vec<String>,
    /// Library ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    pub id: LibraryId,
    /// Import paths (required)
    #[serde(rename = "importPaths")]
    pub import_paths: Vec<String>,
    /// Library name (required)
    pub name: String,
    /// Owner user ID (pattern: `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`) (required)
    #[serde(rename = "ownerId")]
    pub owner_id: UserId,
    /// Last refresh date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "refreshedAt")]
    pub refreshed_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    /// Last update date (pattern: `^(?:(?:\d\d[2468][048]|\d\d[13579][26]|\d\d0[48]|[02468][048]00|[13579][26]00)-02-29|\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\d|30)|(?:02)-(?:0[1-9]|1\d|2[0-8])))T(?:(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d(?:\.\d+)?)?(?:Z|([+-](?:[01]\d|2[0-3]):[0-5]\d)))$`) (required)
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
}

impl LibraryResponseDTO {
    /// Creates a new [`LibraryResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        asset_count: i64,
        created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        exclusion_patterns: impl IntoIterator<Item = impl Into<String>>,
        id: impl Into<LibraryId>,
        import_paths: impl IntoIterator<Item = impl Into<String>>,
        name: impl Into<String>,
        owner_id: impl Into<UserId>,
        updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ) -> Self {
        Self {
            asset_count,
            created_at,
            exclusion_patterns: exclusion_patterns.into_iter().map(Into::into).collect(),
            id: id.into(),
            import_paths: import_paths.into_iter().map(Into::into).collect(),
            name: name.into(),
            owner_id: owner_id.into(),
            refreshed_at: None,
            updated_at,
        }
    }
    /// Sets `asset_count`.
    #[must_use]
    pub fn asset_count(mut self, value: i64) -> Self {
        self.asset_count = value;
        self
    }
    /// Sets `created_at`.
    #[must_use]
    pub fn created_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.created_at = value;
        self
    }
    /// Sets `exclusion_patterns`.
    #[must_use]
    pub fn exclusion_patterns(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclusion_patterns = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `id`.
    #[must_use]
    pub fn id(mut self, value: impl Into<LibraryId>) -> Self {
        self.id = value.into();
        self
    }
    /// Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.import_paths = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `name`.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = value.into();
        self
    }
    /// Sets `owner_id`.
    #[must_use]
    pub fn owner_id(mut self, value: impl Into<UserId>) -> Self {
        self.owner_id = value.into();
        self
    }
    /// Sets `refreshed_at`.
    #[must_use]
    pub fn refreshed_at(mut self, value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        self.refreshed_at = Some(value);
        self
    }
    /// Sets or clears `refreshed_at`.
    #[must_use]
    pub fn set_refreshed_at(mut self, value: Option<::chrono::DateTime<::chrono::offset::Utc>>) -> Self {
        self.refreshed_at = value;
        self
    }
    /// Sets `updated_at`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct LibraryStatsResponseDTO {
    /// Number of photos (required)
    pub photos: i64,
    /// Total number of assets (required)
    pub total: i64,
    /// Storage usage in bytes (required)
    pub usage: i64,
    /// Number of videos (required)
    pub videos: i64,
}

impl LibraryStatsResponseDTO {
    /// Creates a new [`LibraryStatsResponseDTO`].
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
    /// Sets `photos`.
    #[must_use]
    pub fn photos(mut self, value: i64) -> Self {
        self.photos = value;
        self
    }
    /// Sets `total`.
    #[must_use]
    pub fn total(mut self, value: i64) -> Self {
        self.total = value;
        self
    }
    /// Sets `usage`.
    #[must_use]
    pub fn usage(mut self, value: i64) -> Self {
        self.usage = value;
        self
    }
    /// Sets `videos`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Library name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
}

impl UpdateLibraryDTO {
    /// Creates a new [`UpdateLibraryDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            exclusion_patterns: Vec::new(),
            import_paths: Vec::new(),
            name: None,
        }
    }
    /// Sets `exclusion_patterns`.
    #[must_use]
    pub fn exclusion_patterns(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclusion_patterns = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.import_paths = value.into_iter().map(Into::into).collect();
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

/// `ValidateLibraryDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `exclusion_patterns` | [`Vec<String>`](::std::vec::Vec) |  | Exclusion patterns (max 128) |
/// | `import_paths` | [`Vec<String>`](::std::vec::Vec) |  | Import paths to validate (max 128) |
///
/// See: <https://api.immich.app/models/ValidateLibraryDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Creates a new [`ValidateLibraryDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            exclusion_patterns: Vec::new(),
            import_paths: Vec::new(),
        }
    }
    /// Sets `exclusion_patterns`.
    #[must_use]
    pub fn exclusion_patterns(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclusion_patterns = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.import_paths = value.into_iter().map(Into::into).collect();
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ValidateLibraryImportPathResponseDTO {
    /// Import path (required)
    #[serde(rename = "importPath")]
    pub import_path: String,
    /// Is valid (required)
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    /// Validation message
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: Option<String>,
}

impl ValidateLibraryImportPathResponseDTO {
    /// Creates a new [`ValidateLibraryImportPathResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(import_path: impl Into<String>, is_valid: bool) -> Self {
        Self {
            import_path: import_path.into(),
            is_valid,
            message: None,
        }
    }
    /// Sets `import_path`.
    #[must_use]
    pub fn import_path(mut self, value: impl Into<String>) -> Self {
        self.import_path = value.into();
        self
    }
    /// Sets `is_valid`.
    #[must_use]
    pub fn is_valid(mut self, value: bool) -> Self {
        self.is_valid = value;
        self
    }
    /// Sets `message`.
    #[must_use]
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }
    /// Sets or clears `message`.
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct ValidateLibraryResponseDTO {
    /// Validation results for import paths
    #[serde(rename = "importPaths", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub import_paths: Vec<ValidateLibraryImportPathResponseDTO>,
}

impl ValidateLibraryResponseDTO {
    /// Creates a new [`ValidateLibraryResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            import_paths: Vec::new(),
        }
    }
    /// Sets `import_paths`.
    #[must_use]
    pub fn import_paths(mut self, value: impl IntoIterator<Item = ValidateLibraryImportPathResponseDTO>) -> Self {
        self.import_paths = value.into_iter().collect();
        self
    }
}
