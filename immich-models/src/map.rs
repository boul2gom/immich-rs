// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `map`.

pub use crate::sync::{SyncAckV1, SyncCompleteV1, SyncResetV1};

/// `MapReverseGeocodeResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `city` | [`String`] | ✓ | City name |
/// | `country` | [`String`] | ✓ | Country name |
/// | `state` | [`String`] | ✓ | State/Province name |
///
/// See: <https://api.immich.app/models/MapReverseGeocodeResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct MapReverseGeocodeResponseDTO {
    /// City name (required)
    pub city: Option<String>,
    /// Country name (required)
    pub country: Option<String>,
    /// State/Province name (required)
    pub state: Option<String>,
}

impl MapReverseGeocodeResponseDTO {
    /// Creates a new [`MapReverseGeocodeResponseDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            state: None,
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
}

impl Default for MapReverseGeocodeResponseDTO {
    fn default() -> Self {
        Self::new()
    }
}

impl From<SyncAckV1> for ::serde_json::Map<String, ::serde_json::Value> {
    fn from(value: SyncAckV1) -> Self {
        value.0
    }
}

impl From<SyncCompleteV1> for ::serde_json::Map<String, ::serde_json::Value> {
    fn from(value: SyncCompleteV1) -> Self {
        value.0
    }
}

impl From<SyncResetV1> for ::serde_json::Map<String, ::serde_json::Value> {
    fn from(value: SyncResetV1) -> Self {
        value.0
    }
}
