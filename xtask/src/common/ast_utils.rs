use std::collections::{BTreeSet, HashMap};

use proc_macro2::TokenStream;
use quote::quote;
use syn::visit_mut::VisitMut;

/// Injects the unified `@generated` header and scoped Clippy allows for generated files.
#[must_use]
pub fn inject_unified_header(code: &str) -> String {
    let header = "// @generated Made by @boul2gom with `immich-rs` - do not edit by hand.\n\
                  // Re-run `cargo xtask generate` to update.\n\
                  #![allow(clippy::struct_excessive_bools, clippy::too_many_lines)]\n\n";
    let code = code.trim_start_matches("// @generated\n\n");
    let code = code.trim_start_matches("// @generated\n");
    format!("{header}{code}")
}

/// Visitor to find all model types referenced in a client token stream and rewrite them to short imports.
#[derive(Debug)]
pub struct ImportResolver<'a, S: std::hash::BuildHasher> {
    pub type_map: &'a HashMap<String, String, S>,
    pub found_imports: BTreeSet<String>,
}

impl<S: std::hash::BuildHasher> VisitMut for ImportResolver<'_, S> {
    fn visit_path_mut(&mut self, node: &mut syn::Path) {
        if node.segments.len() == 1 {
            let ident = node.segments[0].ident.to_string();
            if let Some(fqdn) = self.type_map.get(&ident) {
                self.found_imports.insert(fqdn.clone());
            }
        } else if node.segments.len() >= 2 && node.segments[0].ident == "immich_models" {
            let last_segment = node.segments.last().unwrap().clone();
            if let Some(fqdn) = self.type_map.get(&last_segment.ident.to_string()) {
                self.found_imports.insert(fqdn.clone());
                let mut new_path = syn::Path {
                    leading_colon: None,
                    segments: syn::punctuated::Punctuated::new(),
                };
                new_path.segments.push(last_segment);
                *node = new_path;
            }
        }
        syn::visit_mut::visit_path_mut(self, node);
    }
}

/// Resolves and prepends `use immich_models::...;` statements for all referenced model types.
///
/// # Panics
/// Panics if `tokens` cannot be parsed as a valid `syn::File`.
pub fn resolve_and_inject_imports<S: std::hash::BuildHasher>(
    tokens: TokenStream,
    type_map: &HashMap<String, String, S>,
) -> TokenStream {
    let mut file: syn::File = syn::parse2(tokens).expect("Failed to parse generated client tokens");
    let mut resolver = ImportResolver {
        type_map,
        found_imports: BTreeSet::new(),
    };
    resolver.visit_file_mut(&mut file);

    let mut import_tokens = Vec::new();
    for fqdn in resolver.found_imports {
        if let Ok(path) = syn::parse_str::<syn::Path>(&fqdn) {
            import_tokens.push(quote! { use #path; });
        }
    }

    quote! {
        #(#import_tokens)*

        #file
    }
}
