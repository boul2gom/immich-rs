pub mod ast;
mod io;
mod modules;
pub mod openapi;

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result};
use ast::{
    ImportCollector, transform_add_constructors_and_setters, transform_add_derives, transform_add_skip_serializing_if,
    transform_binary_types, transform_clean_std_paths, transform_fix_default_impls, transform_fix_from_str,
    transform_forward_compatible_enums, transform_inject_docs, transform_rename_types, transform_typed_ids,
};
use io::write_modules;
use modules::split_modules;
use openapi::{build_root_schema, build_schema_tag_map, extract_schema_docs, patch_openapi_v3_to_jsonschema};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::visit_mut::VisitMut;

/// In-memory mapping from generated `Rust` model identifier to its fully-qualified `immich_models::...` path.
pub type ModelIndex = HashMap<String, String>;

/// Generates `immich-models` from an already-loaded `OpenAPI` specification and returns the in-memory [`ModelIndex`].
///
/// # Errors
/// Returns an error if schema translation, AST generation, disk I/O, or formatting fails.
pub fn run_with_spec(workspace_root: &Path, spec: &serde_json::Value, resolved_version: &str) -> Result<ModelIndex> {
    let (module_items, model_index) = prepare_models(spec)?;

    let non_empty = module_items.values().filter(|v| !v.is_empty()).count();
    println!("[xtask] {non_empty} non-empty model modules");

    let out_dir = workspace_root.join("immich-models/src");
    write_modules(&out_dir, &module_items, resolved_version)?;

    crate::common::fmt::format_crate(workspace_root, "immich-models")?;

    println!("[xtask] ✓ immich-models generation complete.");
    Ok(model_index)
}

/// Builds the [`ModelIndex`] in memory from an `OpenAPI` specification without writing files to disk.
///
/// # Errors
/// Returns an error if schema translation or `typify` generation fails.
pub fn build_model_index(spec: &serde_json::Value, _resolved_version: &str) -> Result<ModelIndex> {
    let (_, model_index) = prepare_models(spec)?;
    Ok(model_index)
}

/// Convenience entry point for fetching/loading the spec and generating `immich-models`.
///
/// # Errors
/// Returns an error if fetching, parsing, or generating fails.
pub fn run(workspace_root: &Path, version: Option<&str>) -> Result<ModelIndex> {
    let (spec, resolved_version) = crate::common::spec::resolve_spec(workspace_root, version, false)?;
    run_with_spec(workspace_root, &spec, &resolved_version)
}

fn prepare_models(spec: &serde_json::Value) -> Result<(HashMap<String, Vec<syn::Item>>, ModelIndex)> {
    // Extract documentation metadata before any schema transformation
    let schema_docs = extract_schema_docs(spec);
    let schema_tag_map = build_schema_tag_map(spec);

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
    transform_rename_types(&mut file, &mut rename_map);
    transform_clean_std_paths(&mut file);
    transform_binary_types(&mut file);
    transform_typed_ids(&mut file);
    transform_forward_compatible_enums(&mut file);
    transform_add_derives(&mut file);
    transform_add_skip_serializing_if(&mut file, &schema_docs, &rename_map);
    transform_fix_from_str(&mut file);
    transform_fix_default_impls(&mut file);
    transform_add_constructors_and_setters(&mut file);

    let mut tracker = crate::common::assign::RuleTracker::new();
    let mut type_to_module: HashMap<String, String> = HashMap::new();
    for (id_name, _) in crate::common::id_mapping::ALL_TYPED_IDS {
        type_to_module.insert((*id_name).to_string(), "id".to_string());
    }
    for item in &file.items {
        if let Some(ident) = crate::common::assign::item_ident(item) {
            let m = crate::common::assign::assign_module(item, &schema_tag_map, &rename_map, &mut tracker);
            type_to_module.insert(ident, m);
        }
    }

    transform_inject_docs(&mut file, &schema_docs, &rename_map, &type_to_module);

    println!("[xtask] Splitting {} items across modules...", file.items.len());
    let module_items = split_modules(&file, &schema_tag_map, &rename_map);

    let mut model_index = ModelIndex::new();
    for (id_name, _) in crate::common::id_mapping::ALL_TYPED_IDS {
        model_index.insert((*id_name).to_string(), format!("immich_models::id::{id_name}"));
    }
    for (module_path, items) in &module_items {
        let mod_rs = module_path.replace('/', "::");
        for item in items {
            if let Some(ident) = crate::common::assign::item_ident(item) {
                model_index.insert(ident.clone(), format!("immich_models::{mod_rs}::{ident}"));
            }
        }
    }

    Ok((module_items, model_index))
}
