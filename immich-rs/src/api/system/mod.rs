// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.
// Re-run `cargo xtask generate` to update.
#![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]

//! API endpoints for `system`.

pub mod config;
pub mod metadata;

/// Client handle for `System` endpoints.
#[derive(Debug, Clone, Copy)]
pub struct System<'a> {
    client: &'a crate::ImmichClient,
}

impl<'a> System<'a> {
    #[inline]
    pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
        Self { client }
    }
    /// Returns a sub-client handle for [`config::Config`] endpoints.
    #[must_use]
    pub const fn config(&self) -> config::Config<'a> {
        config::Config::new(self.client)
    }
    /// Returns a sub-client handle for [`metadata::Metadata`] endpoints.
    #[must_use]
    pub const fn metadata(&self) -> metadata::Metadata<'a> {
        metadata::Metadata::new(self.client)
    }
}
