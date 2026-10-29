pub mod ast;
pub mod detect_pagination;
pub mod io;
pub mod modules;
pub mod openapi;
pub mod paginator;
pub mod utils;

use std::path::Path;

use anyhow::Result;

#[doc(hidden)]
pub fn run(workspace_root: &Path, version: Option<&str>) -> Result<()> {
    let (spec, _resolved_version) = crate::common::spec::fetch_and_load_spec(workspace_root, version)?;

    let endpoints = openapi::extract_endpoints(&spec)?;
    println!("[xtask] Extracted {} endpoints", endpoints.len());

    let by_module = modules::split_endpoints(endpoints);
    let type_map = utils::build_type_map(workspace_root)?;
    let out_dir = workspace_root.join("immich-rs/src/api");

    io::write_modules(&out_dir, by_module, &type_map)?;

    crate::common::fmt::format_crate(workspace_root, "immich-rs")?;

    Ok(())
}
