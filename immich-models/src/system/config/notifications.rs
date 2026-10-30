// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! `OpenAPI` data models for `notifications`.

/// `ConfigTemplateStorageOptionDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `day_options` | [`Vec<String>`](::std::vec::Vec) | ✓ | Available day format options for storage template |
/// | `hour_options` | [`Vec<String>`](::std::vec::Vec) | ✓ | Available hour format options for storage template |
/// | `minute_options` | [`Vec<String>`](::std::vec::Vec) | ✓ | Available minute format options for storage template |
/// | `month_options` | [`Vec<String>`](::std::vec::Vec) | ✓ | Available month format options for storage template |
/// | `preset_options` | [`Vec<String>`](::std::vec::Vec) | ✓ | Available preset template options |
/// | `second_options` | [`Vec<String>`](::std::vec::Vec) | ✓ | Available second format options for storage template |
/// | `week_options` | [`Vec<String>`](::std::vec::Vec) | ✓ | Available week format options for storage template |
/// | `year_options` | [`Vec<String>`](::std::vec::Vec) | ✓ | Available year format options for storage template |
///
/// See: <https://api.immich.app/models/SystemConfigTemplateStorageOptionDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ConfigTemplateStorageOptionDTO {
    /// Available day format options for storage template (required)
    #[serde(rename = "dayOptions")]
    pub day_options: Vec<String>,
    /// Available hour format options for storage template (required)
    #[serde(rename = "hourOptions")]
    pub hour_options: Vec<String>,
    /// Available minute format options for storage template (required)
    #[serde(rename = "minuteOptions")]
    pub minute_options: Vec<String>,
    /// Available month format options for storage template (required)
    #[serde(rename = "monthOptions")]
    pub month_options: Vec<String>,
    /// Available preset template options (required)
    #[serde(rename = "presetOptions")]
    pub preset_options: Vec<String>,
    /// Available second format options for storage template (required)
    #[serde(rename = "secondOptions")]
    pub second_options: Vec<String>,
    /// Available week format options for storage template (required)
    #[serde(rename = "weekOptions")]
    pub week_options: Vec<String>,
    /// Available year format options for storage template (required)
    #[serde(rename = "yearOptions")]
    pub year_options: Vec<String>,
}

impl ConfigTemplateStorageOptionDTO {
    /// Creates a new [`ConfigTemplateStorageOptionDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        day_options: impl IntoIterator<Item = impl Into<String>>,
        hour_options: impl IntoIterator<Item = impl Into<String>>,
        minute_options: impl IntoIterator<Item = impl Into<String>>,
        month_options: impl IntoIterator<Item = impl Into<String>>,
        preset_options: impl IntoIterator<Item = impl Into<String>>,
        second_options: impl IntoIterator<Item = impl Into<String>>,
        week_options: impl IntoIterator<Item = impl Into<String>>,
        year_options: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            day_options: day_options.into_iter().map(Into::into).collect(),
            hour_options: hour_options.into_iter().map(Into::into).collect(),
            minute_options: minute_options.into_iter().map(Into::into).collect(),
            month_options: month_options.into_iter().map(Into::into).collect(),
            preset_options: preset_options.into_iter().map(Into::into).collect(),
            second_options: second_options.into_iter().map(Into::into).collect(),
            week_options: week_options.into_iter().map(Into::into).collect(),
            year_options: year_options.into_iter().map(Into::into).collect(),
        }
    }
    /// Sets `day_options`.
    #[must_use]
    pub fn day_options(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.day_options = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `hour_options`.
    #[must_use]
    pub fn hour_options(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.hour_options = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `minute_options`.
    #[must_use]
    pub fn minute_options(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.minute_options = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `month_options`.
    #[must_use]
    pub fn month_options(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.month_options = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `preset_options`.
    #[must_use]
    pub fn preset_options(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.preset_options = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `second_options`.
    #[must_use]
    pub fn second_options(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.second_options = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `week_options`.
    #[must_use]
    pub fn week_options(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.week_options = value.into_iter().map(Into::into).collect();
        self
    }
    /// Sets `year_options`.
    #[must_use]
    pub fn year_options(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.year_options = value.into_iter().map(Into::into).collect();
        self
    }
}
