use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

#[doc(hidden)]
pub fn build_type_map(workspace_root: &Path) -> Result<HashMap<String, String>> {
    fn visit(dir: &Path, base: &Path, map: &mut HashMap<String, String>) -> Result<()> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit(&path, base, map)?;
                } else if path.extension().is_some_and(|e| e == "rs") {
                    let rel = path.strip_prefix(base).unwrap();
                    let mut mod_path = rel.with_extension("").to_string_lossy().replace('/', "::");
                    if mod_path.ends_with("::mod") {
                        mod_path = mod_path.strip_suffix("::mod").unwrap_or("").to_string();
                    }
                    if mod_path == "lib" {
                        continue;
                    }
                    let content =
                        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;

                    if let Ok(ast) = syn::parse_file(&content) {
                        for item in ast.items {
                            match item {
                                syn::Item::Struct(s) => {
                                    let type_name = s.ident.to_string();
                                    map.insert(type_name.clone(), format!("immich_models::{mod_path}::{type_name}"));
                                }
                                syn::Item::Enum(e) => {
                                    let type_name = e.ident.to_string();
                                    map.insert(type_name.clone(), format!("immich_models::{mod_path}::{type_name}"));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    let mut map = HashMap::new();
    let root = workspace_root.join("immich-models/src");
    visit(&root, &root, &mut map)?;
    Ok(map)
}
