use super::openapi::SchemaDoc;
use super::{HashMap, HashSet, Span, ToTokens, VisitMut, camel_to_snake, quote, snake_to_camel};

pub fn transform_rename_dto<S: std::hash::BuildHasher>(
    file: &mut syn::File,
    rename_map: &mut HashMap<String, String, S>,
) {
    let mut v = DtoRenamer::default();
    v.visit_file_mut(file);
    for (original, new_name) in &v.renames {
        rename_map.insert(new_name.clone(), original.clone());
    }
}

#[derive(Default, Debug)]
pub struct DtoRenamer {
    renames: HashMap<String, String>,
}

impl VisitMut for DtoRenamer {
    fn visit_ident_mut(&mut self, ident: &mut syn::Ident) {
        let name = ident.to_string();
        if name.chars().next().is_some_and(char::is_uppercase) && name.contains("Dto") {
            let new_name = crate::common::naming::rename_type(&name);
            self.renames.insert(name, new_name.clone());
            *ident = syn::Ident::new(&new_name, ident.span());
        }
    }
}

/// Rename `SystemConfig*` → `Config*`, except `SystemConfigDTO` itself.
/// Updates `rename_map` so that the final name still maps to the original `OpenAPI` name.
pub fn transform_rename_system_config<S: std::hash::BuildHasher>(
    file: &mut syn::File,
    rename_map: &mut HashMap<String, String, S>,
) {
    let mut v = SystemConfigRenamer::default();
    v.visit_file_mut(file);

    for (intermediate, final_name) in &v.renames {
        if let Some(original) = rename_map.remove(intermediate) {
            rename_map.insert(final_name.clone(), original);
        } else {
            rename_map.insert(final_name.clone(), intermediate.clone());
        }
    }
}

#[derive(Default, Debug)]
pub struct SystemConfigRenamer {
    renames: HashMap<String, String>,
}

impl VisitMut for SystemConfigRenamer {
    fn visit_ident_mut(&mut self, ident: &mut syn::Ident) {
        let name = ident.to_string();
        if name.starts_with("SystemConfig") && name != "SystemConfigDTO" {
            let new_name = crate::common::naming::rename_type(&name);
            if new_name != name {
                self.renames.insert(name.clone(), new_name.clone());
                *ident = syn::Ident::new(&new_name, ident.span());
            }
        }
    }
}

/// Add `PartialEq` to all types; `Eq` + `Hash` to unit enums; `Clone, PartialEq, Eq` to `ConversionError`.
pub fn transform_add_derives(file: &mut syn::File) {
    let mut v = DeriveAdder;
    v.visit_file_mut(file);
}

#[derive(Debug)]
pub struct DeriveAdder;

impl VisitMut for DeriveAdder {
    fn visit_item_enum_mut(&mut self, item: &mut syn::ItemEnum) {
        append_derives(&mut item.attrs, &["PartialEq"]);

        let is_simple = item.variants.iter().all(|v| matches!(v.fields, syn::Fields::Unit));
        if is_simple {
            append_derives(&mut item.attrs, &["Copy", "Eq", "Hash"]);
        }

        syn::visit_mut::visit_item_enum_mut(self, item);
    }

    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        if item.ident == "ConversionError" && !item.attrs.iter().any(|a| a.path().is_ident("derive")) {
            let attr: syn::Attribute = syn::parse_quote!(#[derive(Clone, PartialEq, Eq)]);
            item.attrs.push(attr);
        } else {
            append_derives(&mut item.attrs, &["PartialEq"]);
        }

        if let syn::Fields::Unnamed(fields) = &mut item.fields {
            for field in &mut fields.unnamed {
                field.vis = syn::Visibility::Public(syn::token::Pub::default());
            }
        }
        syn::visit_mut::visit_item_struct_mut(self, item);
    }

    fn visit_item_mod_mut(&mut self, item: &mut syn::ItemMod) {
        if item.ident == "defaults" {
            item.attrs.push(syn::parse_quote!(#[allow(clippy::missing_panics_doc)]));
        }
        syn::visit_mut::visit_item_mod_mut(self, item);
    }
}

pub fn append_derives(attrs: &mut [syn::Attribute], traits: &[&str]) {
    for attr in attrs.iter_mut() {
        if !attr.path().is_ident("derive") {
            continue;
        }
        if let syn::Meta::List(ref mut list) = attr.meta {
            let existing = list.tokens.to_string();
            let to_add: Vec<proc_macro2::Ident> = traits
                .iter()
                .filter(|&&t| !existing.contains(t))
                .map(|&t| proc_macro2::Ident::new(t, Span::call_site()))
                .collect();
            if !to_add.is_empty() {
                let extra = quote! { , #(#to_add),* };
                list.tokens.extend(extra);
            }
        }
        return;
    }
}

/// Add `#[non_exhaustive]` and `#[serde(other)] UnknownValue` to all unit enums,
/// and update their `Display` implementations accordingly.
pub fn transform_forward_compatible_enums(file: &mut syn::File) {
    let mut unit_enums: HashSet<String> = HashSet::new();

    for item in &mut file.items {
        if let syn::Item::Enum(e) = item {
            let is_unit = !e.variants.is_empty() && e.variants.iter().all(|v| matches!(v.fields, syn::Fields::Unit));
            if is_unit {
                unit_enums.insert(e.ident.to_string());
                if !e.attrs.iter().any(|a| a.path().is_ident("non_exhaustive")) {
                    e.attrs.push(syn::parse_quote!(#[non_exhaustive]));
                }
                if !e.variants.iter().any(|v| v.ident == "UnknownValue") {
                    let unknown_variant: syn::Variant = syn::parse_quote! {
                        #[doc = "Unknown value returned by a newer version of the Immich server."]
                        #[serde(other)]
                        UnknownValue
                    };
                    e.variants.push(unknown_variant);
                }
            }
        }
    }

    for item in &mut file.items {
        if let syn::Item::Impl(item_impl) = item
            && let Some((trait_path, _)) = &item_impl.trait_
            && trait_path.segments.last().is_some_and(|s| s.ident == "Display")
            && let syn::Type::Path(tp) = &*item_impl.self_ty
            && let Some(seg) = tp.path.segments.last()
            && unit_enums.contains(&seg.ident.to_string())
        {
            item_impl
                .attrs
                .push(syn::parse_quote!(#[allow(clippy::match_same_arms)]));
            for impl_item in &mut item_impl.items {
                if let syn::ImplItem::Fn(func) = impl_item
                    && func.sig.ident == "fmt"
                {
                    for stmt in &mut func.block.stmts {
                        if let syn::Stmt::Expr(syn::Expr::Match(expr_match), _) = stmt {
                            let arm: syn::Arm = syn::parse_quote! {
                                Self::UnknownValue => f.write_str("unknown"),
                            };
                            expr_match.arms.push(arm);
                        }
                    }
                }
            }
        }
    }
}

/// Add `#[serde(skip_serializing_if = "Option::is_none")]` to all `Option<T>` fields
/// that don't already have it.
pub fn transform_add_skip_serializing_if(file: &mut syn::File) {
    let mut v = SkipSerializingIfAdder;
    v.visit_file_mut(file);
}

#[derive(Debug)]
pub struct SkipSerializingIfAdder;

impl VisitMut for SkipSerializingIfAdder {
    fn visit_field_mut(&mut self, field: &mut syn::Field) {
        if is_option_type(&field.ty) && !has_skip_serializing_if(field) {
            let attr: syn::Attribute = syn::parse_quote!(#[serde(skip_serializing_if = "Option::is_none")]);
            field.attrs.push(attr);
        }
        syn::visit_mut::visit_field_mut(self, field);
    }
}

#[must_use]
pub fn is_option_type(ty: &syn::Type) -> bool {
    extract_option_inner(ty).is_some()
}

#[must_use]
pub fn extract_option_inner(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(tp) = ty
        && let Some(seg) = tp.path.segments.last()
        && seg.ident == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
        && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
    {
        return Some(inner_ty);
    }
    None
}

#[must_use]
pub fn is_vec_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(tp) if tp.path.segments.last().is_some_and(|s| s.ident == "Vec"))
}

#[must_use]
pub fn is_string_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(tp) if tp.path.segments.len() == 1 && tp.path.segments[0].ident == "String")
}

#[must_use]
pub fn has_skip_serializing_if(field: &syn::Field) -> bool {
    field
        .attrs
        .iter()
        .any(|a| a.path().is_ident("serde") && a.to_token_stream().to_string().contains("skip_serializing_if"))
}

/// Replace `Default::default()` in `typify`-generated `impl Default` blocks with
/// `None` (for `Option<T>`), `Vec::new()` (for `Vec<T>`), or `<FieldType>::default()`.
pub fn transform_fix_default_impls(file: &mut syn::File) {
    let mut struct_fields: HashMap<(String, String), syn::Type> = HashMap::new();
    for item in &file.items {
        if let syn::Item::Struct(s) = item
            && let syn::Fields::Named(named) = &s.fields
        {
            let s_name = s.ident.to_string();
            for f in &named.named {
                if let Some(f_ident) = &f.ident {
                    struct_fields.insert((s_name.clone(), f_ident.to_string()), f.ty.clone());
                }
            }
        }
    }

    for item in &mut file.items {
        if let syn::Item::Impl(item_impl) = item
            && let Some((trait_path, _)) = &item_impl.trait_
            && trait_path.segments.last().is_some_and(|s| s.ident == "Default")
            && let syn::Type::Path(tp) = &*item_impl.self_ty
            && let Some(seg) = tp.path.segments.last()
        {
            let s_name = seg.ident.to_string();
            for impl_item in &mut item_impl.items {
                if let syn::ImplItem::Fn(func) = impl_item
                    && func.sig.ident == "default"
                {
                    for stmt in &mut func.block.stmts {
                        if let syn::Stmt::Expr(syn::Expr::Struct(expr_struct), _) = stmt {
                            for fv in &mut expr_struct.fields {
                                if let syn::Member::Named(f_ident) = &fv.member {
                                    let expr_str = fv.expr.to_token_stream().to_string();
                                    if expr_str.replace(' ', "") == "Default::default()"
                                        && let Some(f_ty) = struct_fields.get(&(s_name.clone(), f_ident.to_string()))
                                    {
                                        if is_option_type(f_ty) {
                                            fv.expr = syn::parse_quote!(None);
                                        } else if is_vec_type(f_ty) {
                                            fv.expr = syn::parse_quote!(Vec::new());
                                        } else {
                                            fv.expr = syn::parse_quote!(<#f_ty>::default());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn extract_field_default(field: &syn::Field) -> Option<syn::Expr> {
    if is_option_type(&field.ty) {
        return Some(syn::parse_quote!(None));
    }

    for attr in &field.attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let mut default_expr: Option<syn::Expr> = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("default") {
                if meta.input.peek(syn::Token![=]) {
                    let value = meta.value()?;
                    let lit: syn::LitStr = value.parse()?;
                    if let Ok(expr) = syn::parse_str::<syn::Expr>(&format!("{}()", lit.value())) {
                        default_expr = Some(expr);
                    }
                } else if is_vec_type(&field.ty) {
                    default_expr = Some(syn::parse_quote!(Vec::new()));
                } else {
                    let ty = &field.ty;
                    default_expr = Some(syn::parse_quote!(<#ty>::default()));
                }
            } else if meta.input.peek(syn::Token![=]) {
                let value = meta.value()?;
                let _: syn::Expr = value.parse()?;
            }
            Ok(())
        });
        if default_expr.is_some() {
            return default_expr;
        }
    }

    None
}

/// Generate `pub fn new(...) -> Self` (taking only required fields) and fluent builder
/// setters `pub fn <field>(mut self, value: ...) -> Self` for all named-field structs.
pub fn transform_add_constructors_and_setters(file: &mut syn::File) {
    let mut existing_defaults: HashSet<String> = HashSet::new();
    for item in &file.items {
        match item {
            syn::Item::Struct(s) => {
                let has_derive_default = s
                    .attrs
                    .iter()
                    .any(|a| a.path().is_ident("derive") && a.to_token_stream().to_string().contains("Default"));
                if has_derive_default {
                    existing_defaults.insert(s.ident.to_string());
                }
            }
            syn::Item::Impl(i) => {
                if let Some((trait_path, _)) = &i.trait_
                    && trait_path.segments.last().is_some_and(|s| s.ident == "Default")
                    && let syn::Type::Path(tp) = &*i.self_ty
                    && let Some(seg) = tp.path.segments.last()
                {
                    existing_defaults.insert(seg.ident.to_string());
                }
            }
            _ => {}
        }
    }

    let mut new_items: Vec<syn::Item> = Vec::with_capacity(file.items.len() * 2);

    for item in file.items.drain(..) {
        if let syn::Item::Struct(ref s) = item
            && let syn::Fields::Named(ref named) = s.fields
        {
            let struct_ident = s.ident.clone();
            let struct_name = struct_ident.to_string();

            let mut new_params: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut field_inits: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut setters: Vec<proc_macro2::TokenStream> = Vec::new();
            let field_names: HashSet<String> = named
                .named
                .iter()
                .filter_map(|f| f.ident.as_ref().map(std::string::ToString::to_string))
                .collect();

            for field in &named.named {
                let Some(f_ident) = &field.ident else {
                    continue;
                };
                let f_ty = &field.ty;

                if let Some(default_expr) = extract_field_default(field) {
                    field_inits.push(quote! { #f_ident: #default_expr });
                } else if is_string_type(f_ty) {
                    new_params.push(quote! { #f_ident: impl Into<String> });
                    field_inits.push(quote! { #f_ident: #f_ident.into() });
                } else {
                    new_params.push(quote! { #f_ident: #f_ty });
                    field_inits.push(quote! { #f_ident });
                }

                let setter_doc = format!("Sets `{f_ident}`.");
                if let Some(inner_ty) = extract_option_inner(f_ty) {
                    if is_string_type(inner_ty) {
                        setters.push(quote! {
                            #[doc = #setter_doc]
                            #[must_use]
                            pub fn #f_ident(mut self, value: impl Into<String>) -> Self {
                                self.#f_ident = Some(value.into());
                                self
                            }
                        });
                    } else {
                        setters.push(quote! {
                            #[doc = #setter_doc]
                            #[must_use]
                            pub fn #f_ident(mut self, value: #inner_ty) -> Self {
                                self.#f_ident = Some(value);
                                self
                            }
                        });
                    }
                    let set_name = format!("set_{f_ident}");
                    if !field_names.contains(&set_name) {
                        let set_ident = quote::format_ident!("{set_name}");
                        let set_doc = format!("Sets or clears `{f_ident}`.");
                        setters.push(quote! {
                            #[doc = #set_doc]
                            #[must_use]
                            pub fn #set_ident(mut self, value: Option<#inner_ty>) -> Self {
                                self.#f_ident = value;
                                self
                            }
                        });
                    }
                } else if is_string_type(f_ty) {
                    setters.push(quote! {
                        #[doc = #setter_doc]
                        #[must_use]
                        pub fn #f_ident(mut self, value: impl Into<String>) -> Self {
                            self.#f_ident = value.into();
                            self
                        }
                    });
                } else {
                    setters.push(quote! {
                        #[doc = #setter_doc]
                        #[must_use]
                        pub fn #f_ident(mut self, value: #f_ty) -> Self {
                            self.#f_ident = value;
                            self
                        }
                    });
                }
            }

            let new_doc = format!("Creates a new [`{struct_name}`].");
            let impl_block: syn::Item = syn::parse_quote! {
                impl #struct_ident {
                    #[doc = #new_doc]
                    #[must_use]
                    #[allow(
                        clippy::too_many_arguments,
                        clippy::fn_params_excessive_bools,
                        clippy::similar_names
                    )]
                    pub fn new(#(#new_params),*) -> Self {
                        Self {
                            #(#field_inits),*
                        }
                    }

                    #(#setters)*
                }
            };

            let needs_default_impl = new_params.is_empty() && !existing_defaults.contains(&struct_name);
            new_items.push(item);
            new_items.push(impl_block);
            if needs_default_impl {
                let default_impl: syn::Item = syn::parse_quote! {
                    impl Default for #struct_ident {
                        fn default() -> Self {
                            Self::new()
                        }
                    }
                };
                new_items.push(default_impl);
            }
        } else {
            new_items.push(item);
        }
    }

    file.items = new_items;
}

// ─── Documentation injection ──────────────────────────────────────────────────

pub fn transform_inject_docs<S1: std::hash::BuildHasher, S2: std::hash::BuildHasher, S3: std::hash::BuildHasher>(
    file: &mut syn::File,
    schema_docs: &HashMap<String, SchemaDoc, S1>,
    rename_map: &HashMap<String, String, S2>,
    type_to_module: &HashMap<String, String, S3>,
) {
    let mut v = DocInjector {
        schema_docs,
        rename_map,
        type_to_module,
    };
    v.visit_file_mut(file);
}

#[derive(Debug)]
pub struct DocInjector<'a, S1: std::hash::BuildHasher, S2: std::hash::BuildHasher, S3: std::hash::BuildHasher> {
    schema_docs: &'a HashMap<String, SchemaDoc, S1>,
    rename_map: &'a HashMap<String, String, S2>,
    type_to_module: &'a HashMap<String, String, S3>,
}

impl<S1: std::hash::BuildHasher, S2: std::hash::BuildHasher, S3: std::hash::BuildHasher> DocInjector<'_, S1, S2, S3> {
    fn lookup(&self, rust_name: &str) -> Option<&SchemaDoc> {
        let original = self
            .rename_map
            .get(rust_name)
            .map_or(rust_name, std::string::String::as_str);
        self.schema_docs.get(original)
    }

    fn format_doc_type(&self, type_hint: &str, in_scope_types: &HashSet<String>) -> String {
        if type_hint == "Uuid" {
            return "[`Uuid`](::uuid::Uuid)".to_string();
        }
        if type_hint == "DateTime" {
            return "[`DateTime`](::chrono::DateTime)".to_string();
        }
        if type_hint == "String" {
            return "[`String`]".to_string();
        }
        if type_hint == "Bytes (as String)" {
            return "`String`".to_string();
        }
        if let Some(inner) = type_hint.strip_prefix("Vec<").and_then(|s| s.strip_suffix('>')) {
            if inner == "Uuid" {
                return "[`Vec<Uuid>`](::uuid::Uuid)".to_string();
            }
            if inner == "DateTime" {
                return "[`Vec<DateTime>`](::chrono::DateTime)".to_string();
            }
            if inner == "String" {
                return "[`Vec<String>`](::std::vec::Vec)".to_string();
            }
            let rust_inner = crate::common::naming::rename_type(inner);
            if let Some(mod_path) = self.type_to_module.get(&rust_inner) {
                let mod_rs = mod_path.replace('/', "::");
                return format!("[`Vec<{rust_inner}>`](crate::{mod_rs}::{rust_inner})");
            }
            return format!("`Vec<{rust_inner}>`");
        }
        if type_hint.chars().next().is_some_and(char::is_uppercase) {
            let rust_type = crate::common::naming::rename_type(type_hint);
            if let Some(mod_path) = self.type_to_module.get(&rust_type) {
                if in_scope_types.contains(&rust_type) {
                    return format!("[`{rust_type}`]");
                }
                let mod_rs = mod_path.replace('/', "::");
                return format!("[`{rust_type}`](crate::{mod_rs}::{rust_type})");
            }
            return format!("`{rust_type}`");
        }
        format!("`{type_hint}`")
    }
}

impl<S1: std::hash::BuildHasher, S2: std::hash::BuildHasher, S3: std::hash::BuildHasher> VisitMut
    for DocInjector<'_, S1, S2, S3>
{
    fn visit_item_enum_mut(&mut self, item: &mut syn::ItemEnum) {
        let rust_name = item.ident.to_string();
        if let Some(doc) = self.lookup(&rust_name) {
            let mut lines: Vec<String> = Vec::new();
            if let Some(desc) = &doc.description {
                lines.push(desc.clone());
            }
            lines.push(String::new());
            lines.push(format!("See: <https://api.immich.app/models/{}>", doc.original_name));
            if doc.is_deprecated {
                lines.push(String::new());
                lines.push("**Deprecated**".into());
            }
            let mut new_attrs: Vec<syn::Attribute> = lines
                .iter()
                .map(|line| {
                    let s = if line.is_empty() {
                        String::new()
                    } else {
                        format!(" {line}")
                    };
                    syn::parse_quote!(#[doc = #s])
                })
                .collect();
            let old_attrs: Vec<syn::Attribute> = item.attrs.drain(..).filter(|a| !a.path().is_ident("doc")).collect();
            new_attrs.extend(old_attrs);
            item.attrs = new_attrs;
        } else {
            strip_json_schema(&mut item.attrs);
        }
        syn::visit_mut::visit_item_enum_mut(self, item);
    }

    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        let rust_name = item.ident.to_string();
        if let Some(doc) = self.lookup(&rust_name) {
            let doc = doc.clone();
            let mut doc_lines: Vec<String> = Vec::new();

            if let Some(desc) = &doc.description {
                doc_lines.push(desc.clone());
            } else {
                doc_lines.push(format!("`{rust_name}`"));
            }

            if !doc.properties.is_empty() {
                let mut in_scope_types = HashSet::new();
                if let Some(this_mod) = self.type_to_module.get(&rust_name) {
                    for (t_name, t_mod) in self.type_to_module {
                        if t_mod == this_mod {
                            in_scope_types.insert(t_name.clone());
                        }
                    }
                }
                for f in &item.fields {
                    if let Some(inner) = extract_option_inner(&f.ty) {
                        if let syn::Type::Path(tp) = inner
                            && let Some(seg) = tp.path.segments.last()
                        {
                            in_scope_types.insert(seg.ident.to_string());
                        }
                    } else if let syn::Type::Path(tp) = &f.ty
                        && let Some(seg) = tp.path.segments.last()
                    {
                        in_scope_types.insert(seg.ident.to_string());
                    }
                }
                doc_lines.push(String::new());
                doc_lines.push("| Field | Type | Required | Description |".into());
                doc_lines.push("|-------|------|:--------:|-------------|".into());
                for prop in &doc.properties {
                    let field = camel_to_snake(&prop.original_name);
                    let req = if prop.required { "✓" } else { "" };
                    let type_display = self.format_doc_type(&prop.type_hint, &in_scope_types);
                    let desc = prop.description.as_deref().unwrap_or("");
                    doc_lines.push(format!("| `{field}` | {type_display} | {req} | {desc} |"));
                }
            }

            doc_lines.push(String::new());
            doc_lines.push(format!("See: <https://api.immich.app/models/{}>", doc.original_name));

            if doc.is_deprecated {
                doc_lines.push(String::new());
                doc_lines.push("**Deprecated** — this type may be removed in a future Immich release.".into());
            }

            let mut new_attrs: Vec<syn::Attribute> = doc_lines
                .iter()
                .map(|line| {
                    let s = if line.is_empty() {
                        String::new()
                    } else {
                        format!(" {line}")
                    };
                    syn::parse_quote!(#[doc = #s])
                })
                .collect();
            let old_attrs: Vec<syn::Attribute> = item.attrs.drain(..).filter(|a| !a.path().is_ident("doc")).collect();
            new_attrs.extend(old_attrs);
            item.attrs = new_attrs;
        } else {
            item.attrs.retain(|attr| !attr.path().is_ident("doc"));
        }

        let rust_name = item.ident.to_string();
        if let Some(doc) = self.lookup(&rust_name) {
            let props = doc.properties.clone();
            for field in &mut item.fields {
                let serde_name = get_serde_rename(field).unwrap_or_else(|| {
                    field
                        .ident
                        .as_ref()
                        .map(|i| snake_to_camel(&i.to_string()))
                        .unwrap_or_default()
                });

                field.attrs.retain(|a| !a.path().is_ident("doc"));

                if let Some(prop) = props.iter().find(|p| p.original_name == serde_name) {
                    if let Some(desc) = &prop.description {
                        let final_desc = if prop.required {
                            format!("{desc} (required)")
                        } else {
                            desc.clone()
                        };
                        let attr_str = format!(" {final_desc}");
                        let attr: syn::Attribute = syn::parse_quote!(#[doc = #attr_str]);
                        field.attrs.insert(0, attr);
                    } else if prop.required {
                        let attr: syn::Attribute = syn::parse_quote!(#[doc = " (required)"]);
                        field.attrs.insert(0, attr);
                    }
                }
            }
        } else {
            strip_json_schema(&mut item.attrs);
        }
        syn::visit_mut::visit_item_struct_mut(self, item);
    }
}

pub fn strip_json_schema(attrs: &mut Vec<syn::Attribute>) {
    let mut keep = Vec::new();
    let mut in_schema = false;
    for attr in attrs.drain(..) {
        if attr.path().is_ident("doc") {
            let s = attr.to_token_stream().to_string();
            if s.contains("<details><summary>JSON schema</summary>") {
                if !s.contains("</details>") {
                    in_schema = true;
                }
                continue;
            }
            if in_schema && s.contains("</details>") {
                in_schema = false;
                continue;
            }
            if in_schema {
                continue;
            }
        }
        keep.push(attr);
    }
    *attrs = keep;
}

#[must_use]
pub fn get_serde_rename(field: &syn::Field) -> Option<String> {
    for attr in &field.attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let raw = attr.to_token_stream().to_string();
        if let Some(pos) = raw.find("rename") {
            let after = &raw[pos + "rename".len()..];
            if let Some(eq) = after.find('=') {
                let val = after[eq + 1..].trim();
                if val.starts_with('"')
                    && let Some(end) = val[1..].find('"')
                {
                    return Some(val[1..=end].to_string());
                }
            }
        }
    }
    None
}

#[derive(Debug)]
pub struct ImportCollector<'a> {
    pub type_to_module: &'a HashMap<String, String>,
    pub current_module: &'a str,
    pub used_imports: HashSet<(String, String)>,
}

impl syn::visit_mut::VisitMut for ImportCollector<'_> {
    fn visit_path_mut(&mut self, node: &mut syn::Path) {
        if node.segments.len() == 1 {
            let ident = node.segments[0].ident.to_string();
            if let Some(mod_path) = self.type_to_module.get(&ident)
                && mod_path != self.current_module
            {
                self.used_imports.insert((mod_path.clone(), ident));
            }
        } else if node.segments.len() >= 2 && node.segments[0].ident == "super" {
            let ident = node.segments[1].ident.to_string();
            if let Some(mod_path) = self.type_to_module.get(&ident)
                && mod_path != self.current_module
            {
                self.used_imports.insert((mod_path.clone(), ident.clone()));
            }
        } else if node.segments.len() >= 2 && node.segments[0].ident == "defaults" {
            let ident = node.segments[1].ident.to_string();
            let mut new_path: syn::Path = syn::parse_str(&format!("crate::common::defaults::{ident}")).unwrap();
            if let Some(last_seg) = new_path.segments.last_mut() {
                last_seg.arguments = node.segments[1].arguments.clone();
            }
            *node = new_path;
        }
        syn::visit_mut::visit_path_mut(self, node);
    }
}

pub fn transform_binary_types(file: &mut syn::File) {
    struct BinaryVisitor {
        current_struct: Option<String>,
    }
    impl syn::visit_mut::VisitMut for BinaryVisitor {
        fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
            self.current_struct = Some(item.ident.to_string());
            syn::visit_mut::visit_item_struct_mut(self, item);
            self.current_struct = None;
        }

        fn visit_field_mut(&mut self, field: &mut syn::Field) {
            let Some(ident) = &field.ident else {
                return;
            };
            let name = ident.to_string();
            let struct_name = self.current_struct.as_deref().unwrap_or("");

            let make_binary = matches!(
                (struct_name, name.as_str()),
                ("AssetMediaCreateDTO", "asset_data" | "sidecar_data")
            );

            if make_binary {
                if is_option_type(&field.ty) {
                    field.ty = syn::parse_quote!(Option<Vec<u8>>);
                } else {
                    field.ty = syn::parse_quote!(Vec<u8>);
                }
            }
            syn::visit_mut::visit_field_mut(self, field);
        }
    }
    syn::visit_mut::VisitMut::visit_file_mut(&mut BinaryVisitor { current_struct: None }, file);
}

#[derive(Debug)]
pub struct PathCleaner;
impl syn::visit_mut::VisitMut for PathCleaner {
    fn visit_expr_path_mut(&mut self, i: &mut syn::ExprPath) {
        if i.qself.is_some() {
            if let Some(qself) = &mut i.qself {
                self.visit_type_mut(&mut qself.ty);
            }
            return;
        }
        syn::visit_mut::visit_expr_path_mut(self, i);
    }

    fn visit_type_path_mut(&mut self, i: &mut syn::TypePath) {
        if i.qself.is_some() {
            if let Some(qself) = &mut i.qself {
                self.visit_type_mut(&mut qself.ty);
            }
            return;
        }
        syn::visit_mut::visit_type_path_mut(self, i);
    }

    fn visit_path_mut(&mut self, node: &mut syn::Path) {
        syn::visit_mut::visit_path_mut(self, node);
        if node.segments.len() >= 3 && node.leading_colon.is_some() {
            let seg0 = node.segments[0].ident.to_string();
            if seg0 == "std" {
                let seg1 = node.segments[1].ident.to_string();
                let seg2 = node.segments[2].ident.to_string();

                if (seg1 == "string" && seg2 == "String")
                    || (seg1 == "option" && seg2 == "Option")
                    || (seg1 == "vec" && seg2 == "Vec")
                    || (seg1 == "result" && seg2 == "Result")
                    || (seg1 == "convert" && seg2 == "TryFrom")
                    || (seg1 == "convert" && seg2 == "From")
                    || (seg1 == "primitive")
                {
                    let mut new_segments = syn::punctuated::Punctuated::new();
                    for seg in node.segments.iter().skip(2) {
                        new_segments.push(seg.clone());
                    }
                    node.segments = new_segments;
                    node.leading_colon = None;
                }
            }
        }
    }
}

pub fn transform_clean_std_paths(file: &mut syn::File) {
    syn::visit_mut::VisitMut::visit_file_mut(&mut PathCleaner, file);
}

#[derive(Debug)]
pub struct FromStrFixer {
    current_type: Option<String>,
    strip_lifetime: bool,
}

impl syn::visit_mut::VisitMut for FromStrFixer {
    fn visit_item_impl_mut(&mut self, item: &mut syn::ItemImpl) {
        if let Some((path, _)) = &item.trait_
            && path
                .segments
                .last()
                .is_some_and(|s| s.ident == "FromStr" || s.ident == "TryFrom")
        {
            if let syn::Type::Path(tp) = &*item.self_ty
                && let Some(seg) = tp.path.segments.last()
            {
                self.current_type = Some(seg.ident.to_string());
            }
            if path.segments.last().is_some_and(|s| s.ident == "TryFrom") && !item.generics.params.is_empty() {
                item.generics = syn::Generics::default();
                self.strip_lifetime = true;
            }
            syn::visit_mut::visit_item_impl_mut(self, item);
            self.current_type = None;
            self.strip_lifetime = false;
            return;
        }
        syn::visit_mut::visit_item_impl_mut(self, item);
    }

    fn visit_type_reference_mut(&mut self, i: &mut syn::TypeReference) {
        if self.strip_lifetime {
            i.lifetime = None;
        }
        syn::visit_mut::visit_type_reference_mut(self, i);
    }

    fn visit_arm_mut(&mut self, arm: &mut syn::Arm) {
        if let Some(type_name) = &self.current_type
            && let syn::Pat::Wild(_) = arm.pat
            && let syn::Expr::Call(call) = &*arm.body
            && let syn::Expr::Path(path) = &*call.func
            && path.path.segments.last().is_some_and(|s| s.ident == "Err")
        {
            let new_body: syn::Expr = syn::parse_quote! {
                Err(format!("invalid value for {}: {}", #type_name, value).into())
            };
            *arm.body = new_body;
        }
        syn::visit_mut::visit_arm_mut(self, arm);
    }

    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        if let Some(type_name) = &self.current_type
            && matches!(expr, syn::Expr::Call(_))
            && expr.to_token_stream().to_string().replace(' ', "") == "Err(\"invalidvalue\".into())"
        {
            *expr = syn::parse_quote! {
                Err(format!("invalid value for {}: {}", #type_name, value).into())
            };
            return;
        }
        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

pub fn transform_fix_from_str(file: &mut syn::File) {
    syn::visit_mut::VisitMut::visit_file_mut(
        &mut FromStrFixer {
            current_type: None,
            strip_lifetime: false,
        },
        file,
    );
}
