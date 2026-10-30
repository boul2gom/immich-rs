pub mod ast;
pub mod detect_pagination;
pub mod io;
pub mod modules;
pub mod openapi;
pub mod paginator;

use std::path::Path;

use anyhow::Result;

/// Generates `immich-rs/src/api` from an already-loaded `OpenAPI` specification and in-memory [`crate::models::ModelIndex`].
///
/// # Errors
/// Returns an error if endpoint extraction, file writing, or formatting fails.
pub fn run_with_spec(
    workspace_root: &Path,
    spec: &serde_json::Value,
    model_index: &crate::models::ModelIndex,
) -> Result<()> {
    let endpoints = openapi::extract_endpoints(spec)?;
    println!("[xtask] Extracted {} endpoints", endpoints.len());

    let by_module = modules::split_endpoints(endpoints);
    let out_dir = workspace_root.join("immich-rs/src/api");

    io::write_modules(&out_dir, by_module, model_index)?;

    crate::common::fmt::format_crate(workspace_root, "immich-rs")?;

    println!("[xtask] ✓ immich-rs client generation complete.");
    Ok(())
}

/// Convenience entry point for fetching/loading the spec and generating `immich-rs/src/api`.
///
/// # Errors
/// Returns an error if fetching, model indexing, or client generation fails.
pub fn run(workspace_root: &Path, version: Option<&str>) -> Result<()> {
    let (spec, resolved_version) = crate::common::spec::resolve_spec(workspace_root, version, false)?;
    let model_index = crate::models::build_model_index(&spec, &resolved_version)?;
    run_with_spec(workspace_root, &spec, &model_index)
}
