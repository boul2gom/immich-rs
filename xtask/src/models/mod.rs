pub mod ast;
mod io;
mod modules;
mod openapi;
mod utils;

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result};
use ast::{
    ImportCollector, transform_add_constructors_and_setters, transform_add_derives, transform_add_skip_serializing_if,
    transform_clean_std_paths, transform_fix_default_impls, transform_fix_from_str, transform_forward_compatible_enums,
    transform_inject_docs, transform_rename_dto, transform_rename_system_config,
};
use io::write_modules;
use modules::split_modules;
use openapi::{build_root_schema, build_schema_tag_map, extract_schema_docs, patch_openapi_v3_to_jsonschema};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::visit_mut::VisitMut;
use utils::{camel_to_snake, snake_to_camel};

/// Entry point for the `models` generation process.
///
/// # Panics
/// Panics if the `OpenAPI` spec cannot be parsed or if the AST modification fails unexpectedly.
///
/// # Errors
/// Returns an error if the generation process fails.
pub fn run(workspace_root: &Path, version: Option<&str>) -> Result<()> {
    let (spec, resolved_version) = crate::common::spec::fetch_and_load_spec(workspace_root, version)?;

    // Extract documentation metadata before any transformation
    let schema_docs = extract_schema_docs(&spec);
    let schema_tag_map = build_schema_tag_map(&spec);

    // Pre-process: OpenAPI 3.0 to JSON Schema translation
    let mut schemas_val = spec["components"]["schemas"].clone();
    patch_openapi_v3_to_jsonschema(&mut schemas_val);

    // Build schemars RootSchema
    let root_schema = build_root_schema(&schemas_val).context("building schemars RootSchema")?;

    // Generate types with typify
    println!("[xtask] Generating Rust types (typify)...");
    let settings = typify::TypeSpaceSettings::default();
    let mut type_space = typify::TypeSpace::new(&settings);
    type_space
        .add_root_schema(root_schema)
        .context("typify: adding root schema")?;
    let tokens = type_space.to_stream();

    // Parse to syn::File
    let mut file: syn::File = syn::parse2(tokens).context("parsing generated TokenStream")?;

    // AST transforms — order matters
    println!("[xtask] Applying AST transforms...");
    let mut rename_map: HashMap<String, String> = HashMap::new(); // final Rust name → original OpenAPI name
    transform_rename_dto(&mut file, &mut rename_map);
    transform_rename_system_config(&mut file, &mut rename_map);
    transform_clean_std_paths(&mut file);
    crate::models::ast::transform_binary_types(&mut file);
    transform_add_derives(&mut file);
    transform_forward_compatible_enums(&mut file);
    transform_add_skip_serializing_if(&mut file);
    transform_fix_from_str(&mut file);
    transform_fix_default_impls(&mut file);
    transform_add_constructors_and_setters(&mut file);

    let mut tracker = crate::common::assign::RuleTracker::new();
    let mut type_to_module: HashMap<String, String> = HashMap::new();
    for item in &file.items {
        if let Some(ident) = crate::common::assign::item_ident(item) {
            let m = crate::common::assign::assign_module(item, &schema_tag_map, &rename_map, &mut tracker);
            type_to_module.insert(ident, m);
        }
    }

    transform_inject_docs(&mut file, &schema_docs, &rename_map, &type_to_module);

    // Split items across modules
    println!("[xtask] Splitting {} items across modules...", file.items.len());
    let module_items = split_modules(&file, &schema_tag_map, &rename_map);

    let non_empty = module_items.values().filter(|v| !v.is_empty()).count();
    println!("[xtask] {non_empty} non-empty modules");

    // Write files
    let out_dir = workspace_root.join("immich-models/src");
    write_modules(&out_dir, &module_items, &resolved_version)?;

    crate::common::fmt::format_crate(workspace_root, "immich-models")?;

    println!("[xtask] ✓ Generation complete.");
    Ok(())
}
