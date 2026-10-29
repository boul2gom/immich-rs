use std::path::Path;

use anyhow::{Context, Result};

/// Centralized utility to load and parse the `OpenAPI` spec.
///
/// # Errors
/// Returns an error if the file cannot be read or parsed as valid JSON.
pub fn load_openapi_spec(workspace_root: &Path) -> Result<serde_json::Value> {
    let spec_path = workspace_root.join("open-api/immich-openapi-specs.json");
    println!("[xtask] Reading OpenAPI spec from {}...", spec_path.display());
    let spec_json = std::fs::read_to_string(&spec_path).with_context(|| format!("reading {}", spec_path.display()))?;
    serde_json::from_str(&spec_json).context("parsing OpenAPI spec")
}

/// Fetches the spec (if needed) and loads it into a `serde_json::Value`.
/// Returns the parsed spec and the resolved version tag.
///
/// # Errors
/// Returns an error if the network request fails, or if parsing fails.
pub fn fetch_and_load_spec(workspace_root: &Path, version: Option<&str>) -> Result<(serde_json::Value, String)> {
    let (spec_changed, resolved_version) = crate::fetch_spec(version)?;
    if !spec_changed {
        println!("[xtask] Note: spec unchanged — regenerating anyway (codegen rules may have changed).");
    }

    let spec = load_openapi_spec(workspace_root)?;
    Ok((spec, resolved_version))
}
