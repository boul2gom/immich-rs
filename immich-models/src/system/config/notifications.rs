// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
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
    ///Creates a new [`ConfigTemplateStorageOptionDTO`].
    #[must_use]
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools, clippy::similar_names)]
    pub fn new(
        day_options: Vec<String>,
        hour_options: Vec<String>,
        minute_options: Vec<String>,
        month_options: Vec<String>,
        preset_options: Vec<String>,
        second_options: Vec<String>,
        week_options: Vec<String>,
        year_options: Vec<String>,
    ) -> Self {
        Self {
            day_options,
            hour_options,
            minute_options,
            month_options,
            preset_options,
            second_options,
            week_options,
            year_options,
        }
    }

    ///Sets `day_options`.
    #[must_use]
    pub fn day_options(mut self, value: Vec<String>) -> Self {
        self.day_options = value;
        self
    }

    ///Sets `hour_options`.
    #[must_use]
    pub fn hour_options(mut self, value: Vec<String>) -> Self {
        self.hour_options = value;
        self
    }

    ///Sets `minute_options`.
    #[must_use]
    pub fn minute_options(mut self, value: Vec<String>) -> Self {
        self.minute_options = value;
        self
    }

    ///Sets `month_options`.
    #[must_use]
    pub fn month_options(mut self, value: Vec<String>) -> Self {
        self.month_options = value;
        self
    }

    ///Sets `preset_options`.
    #[must_use]
    pub fn preset_options(mut self, value: Vec<String>) -> Self {
        self.preset_options = value;
        self
    }

    ///Sets `second_options`.
    #[must_use]
    pub fn second_options(mut self, value: Vec<String>) -> Self {
        self.second_options = value;
        self
    }

    ///Sets `week_options`.
    #[must_use]
    pub fn week_options(mut self, value: Vec<String>) -> Self {
        self.week_options = value;
        self
    }

    ///Sets `year_options`.
    #[must_use]
    pub fn year_options(mut self, value: Vec<String>) -> Self {
        self.year_options = value;
        self
    }
}
