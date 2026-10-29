use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, Result};

use super::{ast, openapi};

/// Write the generated client modules to disk.
///
/// # Errors
/// Returns an error if writing files fails.
pub fn write_modules<S: std::hash::BuildHasher>(
    out_dir: &Path,
    by_module: BTreeMap<String, Vec<openapi::EndpointDef>>,
    type_map: &std::collections::HashMap<String, String, S>,
) -> Result<()> {
    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir)?;
    }

    let mut vfs = crate::common::fs_tree::VirtualFs::new("mod.rs");
    let mut lib_mods: Vec<String> = Vec::new();

    for (module_name, eps) in by_module {
        let last_part = module_name
            .split('/')
            .next_back()
            .ok_or_else(|| anyhow::anyhow!("empty parts"))?;
        vfs.set_module_doc(&module_name, format!("//! API endpoints for `{last_part}`.\n"));
        let mod_tokens = ast::emit_client_module(last_part, &eps, type_map);
        let mod_tokens = crate::common::ast_utils::resolve_and_inject_imports(mod_tokens, type_map);

        let parsed_file: syn::File =
            syn::parse2(mod_tokens).with_context(|| format!("parsing generated tokens for module {module_name}"))?;
        let mut code = prettyplease::unparse(&parsed_file);
        crate::common::fmt::post_process_formatting(&mut code);

        vfs.add_file(&module_name, code);
        lib_mods.push(module_name);
    }

    let mut accessors = String::new();
    let mut sorted_libs = lib_mods.clone();
    sorted_libs.sort();
    for m in sorted_libs {
        let rust_path = m.replace('/', "::");
        let parts: Vec<&str> = m.split('/').collect();
        let last_part = parts.last().ok_or_else(|| anyhow::anyhow!("empty parts"))?;
        let camel_name = crate::common::snake_to_camel(last_part);
        let fn_name = m.replace('/', "_");
        let _ = write!(
            accessors,
            "    /// Returns an API handle for the [`{last_part}`](crate::api::{rust_path}::{camel_name}) endpoints.\n    #[must_use]\n    pub fn {fn_name}(&self) -> {rust_path}::{camel_name}<'_> {{\n        {rust_path}::{camel_name}::new(self)\n    }}\n"
        );
    }

    let impl_block = format!("\nimpl crate::ImmichClient {{\n{accessors}}}\n");
    vfs.set_module_doc("", "//! Generated API client endpoints for Immich.\n".to_string());
    vfs.add_module_content("", impl_block);
    vfs.write_to_disk(out_dir)?;

    Ok(())
}
