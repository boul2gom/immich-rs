use std::path::Path;

use anyhow::{Context, Result};

/// Loads and parses the local `open-api/immich-openapi-specs.json` file.
///
/// # Errors
/// Returns an error if the file cannot be read or parsed as valid JSON.
pub fn load_openapi_spec(workspace_root: &Path) -> Result<(serde_json::Value, String)> {
    let spec_path = workspace_root.join("open-api/immich-openapi-specs.json");
    println!("[xtask] Reading OpenAPI spec from {}...", spec_path.display());
    let spec_json = std::fs::read_to_string(&spec_path).with_context(|| format!("reading {}", spec_path.display()))?;
    let spec: serde_json::Value = serde_json::from_str(&spec_json).context("parsing OpenAPI spec")?;
    let raw_ver = spec
        .pointer("/info/version")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("0.0.0");
    let tag = if raw_ver.starts_with('v') {
        raw_ver.to_string()
    } else {
        format!("v{raw_ver}")
    };
    Ok((spec, tag))
}

/// Fetches the spec from GitHub (unless `offline` is `true`) and loads it once into memory.
///
/// # Errors
/// Returns an error if the network request or JSON parsing fails.
pub fn resolve_spec(
    workspace_root: &Path,
    version: Option<&str>,
    offline: bool,
) -> Result<(serde_json::Value, String)> {
    if offline {
        let (spec, local_ver) = load_openapi_spec(workspace_root)?;
        let resolved = version.map_or(local_ver, str::to_owned);
        println!("[xtask] Offline mode — using local spec ({resolved}).");
        return Ok((spec, resolved));
    }

    let (spec_changed, resolved_version) = crate::fetch_spec(version)?;
    if !spec_changed {
        println!("[xtask] Note: spec unchanged — regenerating anyway (codegen rules may have changed).");
    }

    let (spec, _) = load_openapi_spec(workspace_root)?;
    Ok((spec, resolved_version))
}
