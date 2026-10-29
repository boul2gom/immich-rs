// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `metadata`.

/// `AdminOnboardingUpdateDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `is_onboarded` | `bool` | ✓ | Is admin onboarded |
///
/// See: <https://api.immich.app/models/AdminOnboardingUpdateDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct AdminOnboardingUpdateDTO {
    /// Is admin onboarded (required)
    #[serde(rename = "isOnboarded")]
    pub is_onboarded: bool,
}

impl AdminOnboardingUpdateDTO {
    ///Creates a new [`AdminOnboardingUpdateDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(is_onboarded: bool) -> Self {
        Self { is_onboarded }
    }

    ///Sets `is_onboarded`.
    #[must_use]
    pub fn is_onboarded(mut self, value: bool) -> Self {
        self.is_onboarded = value;
        self
    }
}

/// `ReverseGeocodingStateResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `last_import_file_name` | [`String`] | ✓ | Last import file name |
/// | `last_update` | [`String`] | ✓ | Last update timestamp |
///
/// See: <https://api.immich.app/models/ReverseGeocodingStateResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct ReverseGeocodingStateResponseDTO {
    /// Last import file name (required)
    #[serde(rename = "lastImportFileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_import_file_name: Option<String>,
    /// Last update timestamp (required)
    #[serde(rename = "lastUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
}

impl ReverseGeocodingStateResponseDTO {
    ///Creates a new [`ReverseGeocodingStateResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            last_import_file_name: None,
            last_update: None,
        }
    }

    ///Sets `last_import_file_name`.
    #[must_use]
    pub fn last_import_file_name(mut self, value: impl Into<String>) -> Self {
        self.last_import_file_name = Some(value.into());
        self
    }

    ///Sets or clears `last_import_file_name`.
    #[must_use]
    pub fn set_last_import_file_name(mut self, value: Option<String>) -> Self {
        self.last_import_file_name = value;
        self
    }

    ///Sets `last_update`.
    #[must_use]
    pub fn last_update(mut self, value: impl Into<String>) -> Self {
        self.last_update = Some(value.into());
        self
    }

    ///Sets or clears `last_update`.
    #[must_use]
    pub fn set_last_update(mut self, value: Option<String>) -> Self {
        self.last_update = value;
        self
    }
}

impl Default for ReverseGeocodingStateResponseDTO {
    fn default() -> Self {
        Self::new()
    }
}
