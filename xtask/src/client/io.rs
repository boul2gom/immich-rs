use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, Result};

use super::{ast, openapi};

/// Writes the generated client modules (including nested sub-client handles) to disk.
///
/// # Errors
/// Returns an error if writing files fails.
pub fn write_modules<S: std::hash::BuildHasher>(
    out_dir: &Path,
    mut by_module: BTreeMap<String, Vec<openapi::EndpointDef>>,
    type_map: &std::collections::HashMap<String, String, S>,
) -> Result<()> {
    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir)?;
    }

    // Collect full module hierarchy (including pure container modules like `admin` and `system`)
    let mut children_of: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let leaf_paths: Vec<String> = by_module.keys().cloned().collect();
    for module_path in &leaf_paths {
        let parts: Vec<&str> = module_path.split('/').collect();
        for depth in 0..parts.len() {
            let parent = parts[..depth].join("/");
            let child = parts[depth].to_string();
            children_of.entry(parent.clone()).or_default().insert(child);
            if !parent.is_empty() {
                by_module.entry(parent).or_default();
            }
        }
    }

    let mut vfs = crate::common::fs_tree::VirtualFs::new("mod.rs");

    for (module_path, eps) in &by_module {
        let last_part = module_path
            .split('/')
            .next_back()
            .ok_or_else(|| anyhow::anyhow!("empty module path"))?;
        vfs.set_module_doc(module_path, format!("//! API endpoints for `{last_part}`.\n"));

        let child_list: Vec<String> = children_of
            .get(module_path)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default();

        let mod_tokens = ast::emit_client_module(module_path, eps, &child_list, type_map);
        let mod_tokens = crate::common::ast_utils::resolve_and_inject_imports(mod_tokens, type_map);

        let parsed_file: syn::File =
            syn::parse2(mod_tokens).with_context(|| format!("parsing generated tokens for module {module_path}"))?;
        let code = crate::common::fmt::unparse_with_spacing(&parsed_file);

        vfs.add_file(module_path, code);
    }

    let mut accessors = String::new();
    if let Some(root_mods) = children_of.get("") {
        for m in root_mods {
            let camel_name = crate::common::snake_to_camel(m);
            let _ = write!(
                accessors,
                "    /// Returns an API handle for the [`{m}`](crate::api::{m}::{camel_name}) endpoints.\n    \
                 #[inline]\n    \
                 #[must_use]\n    \
                 pub const fn {m}(&self) -> {m}::{camel_name}<'_> {{\n        \
                 {m}::{camel_name}::new(self)\n    \
                 }}\n\n"
            );
        }
    }

    let impl_block = format!("\nimpl crate::ImmichClient {{\n{}}}\n", accessors.trim_end());
    vfs.set_module_doc("", "//! Generated API client endpoints for Immich.\n".to_string());
    vfs.add_module_content("", impl_block);
    vfs.write_to_disk(out_dir)?;

    Ok(())
}
