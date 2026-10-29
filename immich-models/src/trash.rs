// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.

//! `OpenAPI` data models for `trash`.

/// `TrashResponseDTO`
///
/// | Field | Type | Required | Description |
/// |-------|------|:--------:|-------------|
/// | `count` | `i64` | ✓ | Number of items in trash |
///
/// See: <https://api.immich.app/models/TrashResponseDto>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq)]
pub struct TrashResponseDTO {
    pub count: i64,
}

impl TrashResponseDTO {
    ///Creates a new [`TrashResponseDTO`].
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
