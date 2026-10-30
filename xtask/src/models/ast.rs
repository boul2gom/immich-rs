use super::openapi::SchemaDoc;
use super::{HashMap, HashSet, Span, ToTokens, VisitMut, quote};

/// Applies centralized type renaming (`Dto` casing, `SystemConfig*` -> `Config*`, inline `Dto` stripping)
/// and populates `rename_map` (`final_rust_name -> original_openapi_name`).
pub fn transform_rename_types<S: std::hash::BuildHasher>(
    file: &mut syn::File,
    rename_map: &mut HashMap<String, String, S>,
) {
    let mut v = TypeRenamer::default();
    v.visit_file_mut(file);
    for (original, new_name) in v.renames {
        rename_map.insert(new_name, original);
    }
}

#[derive(Default, Debug)]
struct TypeRenamer {
    renames: HashMap<String, String>,
}

impl VisitMut for TypeRenamer {
    fn visit_ident_mut(&mut self, ident: &mut syn::Ident) {
        let name = ident.to_string();
        if name.chars().next().is_some_and(char::is_uppercase) {
            let new_name = crate::common::naming::rename_type(&name);
            if new_name != name {
                self.renames.insert(name, new_name.clone());
                *ident = syn::Ident::new(&new_name, ident.span());
            }
        }
    }
}

/// Rewrites `Uuid` fields on structs to strongly typed `immich_models::id` wrappers (`AssetId`, `AlbumId`, etc.).
pub fn transform_typed_ids(file: &mut syn::File) {
    for item in &mut file.items {
        let syn::Item::Struct(s) = item else {
            continue;
        };
        let syn::Fields::Named(named) = &mut s.fields else {
            continue;
        };
        let struct_name = s.ident.to_string();
        for field in &mut named.named {
            let wire_name = get_wire_field_name(field);
            let field_snake = crate::common::naming::to_snake_case(&wire_name);
            if let Some(id_name) = crate::common::id_mapping::resolve_struct_field_id(&struct_name, &field_snake) {
                rewrite_uuid_to_typed_id(&mut field.ty, id_name);
            }
        }
    }
}

fn rewrite_uuid_to_typed_id(ty: &mut syn::Type, id_name: &str) {
    let syn::Type::Path(tp) = ty else {
        return;
    };
    let Some(last) = tp.path.segments.last_mut() else {
        return;
    };
    if last.ident == "Uuid" && matches!(last.arguments, syn::PathArguments::None) {
        let id_ident = syn::Ident::new(id_name, Span::call_site());
        *ty = syn::parse_quote!(#id_ident);
        return;
    }
    if (last.ident == "Option" || last.ident == "Vec")
        && let syn::PathArguments::AngleBracketed(args) = &mut last.arguments
        && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first_mut()
    {
        rewrite_uuid_to_typed_id(inner_ty, id_name);
    }
}

/// Converts unit string enums to lossless `Unknown(String)` enums (`aws-sdk` style) with zero-allocation
/// `Deserialize` and custom `Serialize`, and appends `Unknown(::serde_json::Value)` to untagged `oneOf` enums.
#[allow(clippy::too_many_lines)]
pub fn transform_forward_compatible_enums(file: &mut syn::File) {
    let mut unit_enum_impls: HashMap<String, Vec<syn::Item>> = HashMap::new();

    for item in &mut file.items {
        let syn::Item::Enum(e) = item else {
            continue;
        };

        if !e.attrs.iter().any(|a| a.path().is_ident("non_exhaustive")) {
            e.attrs.push(syn::parse_quote!(#[non_exhaustive]));
        }

        let is_unit = !e.variants.is_empty() && e.variants.iter().all(|v| matches!(v.fields, syn::Fields::Unit));
        if is_unit {
            let enum_ident = e.ident.clone();
            let enum_name = enum_ident.to_string();

            let mut variant_pairs: Vec<(syn::Ident, String)> = Vec::with_capacity(e.variants.len());
            for variant in &mut e.variants {
                let wire = get_variant_serde_rename(variant).unwrap_or_else(|| variant.ident.to_string());
                variant.attrs.retain(|a| !a.path().is_ident("serde"));
                variant_pairs.push((variant.ident.clone(), wire));
            }

            e.attrs
                .retain(|a| !a.path().is_ident("serde") && !a.path().is_ident("derive"));
            e.attrs.insert(
                0,
                syn::parse_quote!(#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]),
            );

            let has_unknown_variant = variant_pairs.iter().any(|(ident, _)| ident == "Unknown");
            let fallback_ident = syn::Ident::new(
                if has_unknown_variant { "UnknownValue" } else { "Unknown" },
                Span::call_site(),
            );

            let unknown_variant: syn::Variant = syn::parse_quote! {
                #[doc = " Unknown value returned by a newer version of the Immich server."]
                #fallback_ident(String)
            };
            e.variants.push(unknown_variant);

            let var_idents: Vec<&syn::Ident> = variant_pairs.iter().map(|(id, _)| id).collect();
            let wire_strs: Vec<&str> = variant_pairs.iter().map(|(_, w)| w.as_str()).collect();

            let generated_impls: Vec<syn::Item> = vec![
                syn::parse_quote! {
                    impl #enum_ident {
                        #[doc = " Returns the wire string representation of this enum value."]
                        #[must_use]
                        pub fn as_str(&self) -> &str {
                            match self {
                                #(Self::#var_idents => #wire_strs,)*
                                Self::#fallback_ident(s) => s.as_str(),
                            }
                        }
                    }
                },
                syn::parse_quote! {
                    impl ::std::fmt::Display for #enum_ident {
                        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                },
                syn::parse_quote! {
                    impl From<&str> for #enum_ident {
                        fn from(value: &str) -> Self {
                            match value {
                                #(#wire_strs => Self::#var_idents,)*
                                other => Self::#fallback_ident(other.to_owned()),
                            }
                        }
                    }
                },
                syn::parse_quote! {
                    impl From<String> for #enum_ident {
                        fn from(value: String) -> Self {
                            match value.as_str() {
                                #(#wire_strs => Self::#var_idents,)*
                                _ => Self::#fallback_ident(value),
                            }
                        }
                    }
                },
                syn::parse_quote! {
                    impl From<&String> for #enum_ident {
                        fn from(value: &String) -> Self {
                            Self::from(value.as_str())
                        }
                    }
                },
                syn::parse_quote! {
                    impl ::std::str::FromStr for #enum_ident {
                        type Err = ::std::convert::Infallible;

                        fn from_str(value: &str) -> Result<Self, Self::Err> {
                            Ok(Self::from(value))
                        }
                    }
                },
                syn::parse_quote! {
                    impl ::serde::Serialize for #enum_ident {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: ::serde::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                },
                syn::parse_quote! {
                    impl<'de> ::serde::Deserialize<'de> for #enum_ident {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: ::serde::Deserializer<'de>,
                        {
                            struct EnumVisitor;

                            impl ::serde::de::Visitor<'_> for EnumVisitor {
                                type Value = #enum_ident;

                                fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                                    formatter.write_str("a string")
                                }

                                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                                where
                                    E: ::serde::de::Error,
                                {
                                    Ok(#enum_ident::from(value))
                                }

                                fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
                                where
                                    E: ::serde::de::Error,
                                {
                                    Ok(#enum_ident::from(value))
                                }
                            }

                            deserializer.deserialize_str(EnumVisitor)
                        }
                    }
                },
            ];
            unit_enum_impls.insert(enum_name, generated_impls);
        } else if has_serde_untagged(&e.attrs) {
            let has_unknown_variant = e.variants.iter().any(|v| v.ident == "Unknown");
            let fallback_ident = syn::Ident::new(
                if has_unknown_variant { "UnknownValue" } else { "Unknown" },
                Span::call_site(),
            );
            if !e.variants.iter().any(|v| v.ident == fallback_ident) {
                let unknown_variant: syn::Variant = syn::parse_quote! {
                    #[doc = " Unknown variant returned by a newer version of the Immich server."]
                    #fallback_ident(::serde_json::Value)
                };
                e.variants.push(unknown_variant);
            }
        }
    }

    // Remove typify's Display / FromStr / TryFrom impls for transformed unit enums and emit our lossless impls
    let transformed_unit_enums: HashSet<String> = unit_enum_impls.keys().cloned().collect();
    let mut new_items = Vec::with_capacity(file.items.len() + unit_enum_impls.len() * 8);
    for item in file.items.drain(..) {
        if let syn::Item::Impl(ref item_impl) = item
            && let Some((trait_path, _)) = &item_impl.trait_
            && let Some(trait_seg) = trait_path.segments.last()
            && matches!(trait_seg.ident.to_string().as_str(), "Display" | "FromStr" | "TryFrom")
            && let syn::Type::Path(tp) = &*item_impl.self_ty
            && let Some(self_seg) = tp.path.segments.last()
            && transformed_unit_enums.contains(&self_seg.ident.to_string())
        {
            continue;
        }

        let maybe_enum_name = if let syn::Item::Enum(ref e) = item {
            Some(e.ident.to_string())
        } else {
            None
        };
        new_items.push(item);
        if let Some(name) = maybe_enum_name
            && let Some(impls) = unit_enum_impls.remove(&name)
        {
            new_items.extend(impls);
        }
    }
    file.items = new_items;
}

fn has_serde_untagged(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        if !a.path().is_ident("serde") {
            return false;
        }
        let mut found = false;
        let _ = a.parse_nested_meta(|meta| {
            if meta.path.is_ident("untagged") {
                found = true;
            }
            Ok(())
        });
        found
    })
}

fn get_variant_serde_rename(variant: &syn::Variant) -> Option<String> {
    extract_serde_rename_from_attrs(&variant.attrs)
}

/// Adds `PartialEq` (and `Eq` whenever all fields transitively support `Eq`), `#[non_exhaustive]`
/// on named-field structs and enums, and normalizes `mod defaults` / `ConversionError`.
pub fn transform_add_derives(file: &mut syn::File) {
    let eq_types = compute_eq_capable_types(file);
    let mut v = DeriveAdder { eq_types };
    v.visit_file_mut(file);
}

fn compute_eq_capable_types(file: &syn::File) -> HashSet<String> {
    let mut eq_types: HashSet<String> = [
        "bool",
        "i8",
        "i16",
        "i32",
        "i64",
        "i128",
        "isize",
        "u8",
        "u16",
        "u32",
        "u64",
        "u128",
        "usize",
        "NonZeroU8",
        "NonZeroU16",
        "NonZeroU32",
        "NonZeroU64",
        "NonZeroU128",
        "NonZeroUsize",
        "NonZeroI8",
        "NonZeroI16",
        "NonZeroI32",
        "NonZeroI64",
        "NonZeroI128",
        "NonZeroIsize",
        "String",
        "Uuid",
        "DateTime",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    for (id_name, _) in crate::common::id_mapping::ALL_TYPED_IDS {
        eq_types.insert((*id_name).to_string());
    }

    // Any enum that already derives `Eq` is Eq-capable
    for item in &file.items {
        if let syn::Item::Enum(e) = item
            && has_derive_trait(&e.attrs, "Eq")
        {
            eq_types.insert(e.ident.to_string());
        }
    }

    // Fixed-point propagation across structs and enums
    loop {
        let mut changed = false;
        for item in &file.items {
            match item {
                syn::Item::Struct(s) => {
                    let name = s.ident.to_string();
                    if eq_types.contains(&name) {
                        continue;
                    }
                    let all_fields_eq = s.fields.iter().all(|f| type_is_eq(&f.ty, &eq_types));
                    if all_fields_eq {
                        eq_types.insert(name);
                        changed = true;
                    }
                }
                syn::Item::Enum(e) => {
                    let name = e.ident.to_string();
                    if eq_types.contains(&name) {
                        continue;
                    }
                    let all_variants_eq = e
                        .variants
                        .iter()
                        .all(|v| v.fields.iter().all(|f| type_is_eq(&f.ty, &eq_types)));
                    if all_variants_eq {
                        eq_types.insert(name);
                        changed = true;
                    }
                }
                _ => {}
            }
        }
        if !changed {
            break;
        }
    }

    eq_types
}

fn type_is_eq(ty: &syn::Type, eq_types: &HashSet<String>) -> bool {
    let syn::Type::Path(tp) = ty else {
        return false;
    };
    let Some(last) = tp.path.segments.last() else {
        return false;
    };
    let ident_str = last.ident.to_string();
    if ident_str == "DateTime" {
        return true;
    }
    if matches!(ident_str.as_str(), "Option" | "Vec" | "Box") {
        if let syn::PathArguments::AngleBracketed(args) = &last.arguments
            && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
        {
            return type_is_eq(inner, eq_types);
        }
        return false;
    }
    eq_types.contains(&ident_str)
}

fn has_derive_trait(attrs: &[syn::Attribute], target: &str) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("derive") {
            return false;
        }
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(target) {
                found = true;
            }
            Ok(())
        });
        found
    })
}

#[derive(Debug)]
struct DeriveAdder {
    eq_types: HashSet<String>,
}

impl VisitMut for DeriveAdder {
    fn visit_item_enum_mut(&mut self, item: &mut syn::ItemEnum) {
        if self.eq_types.contains(&item.ident.to_string()) {
            append_derives(&mut item.attrs, &["PartialEq", "Eq"]);
        } else {
            append_derives(&mut item.attrs, &["PartialEq"]);
        }
        if !item.attrs.iter().any(|a| a.path().is_ident("non_exhaustive")) {
            item.attrs.push(syn::parse_quote!(#[non_exhaustive]));
        }
        syn::visit_mut::visit_item_enum_mut(self, item);
    }

    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        if item.ident == "ConversionError" && !item.attrs.iter().any(|a| a.path().is_ident("derive")) {
            item.attrs.push(syn::parse_quote!(#[derive(Clone, PartialEq, Eq)]));
        } else if self.eq_types.contains(&item.ident.to_string()) {
            append_derives(&mut item.attrs, &["PartialEq", "Eq"]);
        } else {
            append_derives(&mut item.attrs, &["PartialEq"]);
        }

        match &mut item.fields {
            syn::Fields::Named(_) => {
                if !item.attrs.iter().any(|a| a.path().is_ident("non_exhaustive")) {
                    item.attrs.push(syn::parse_quote!(#[non_exhaustive]));
                }
            }
            syn::Fields::Unnamed(fields) => {
                for field in &mut fields.unnamed {
                    field.vis = syn::Visibility::Public(syn::token::Pub::default());
                }
            }
            syn::Fields::Unit => {}
        }

        syn::visit_mut::visit_item_struct_mut(self, item);
    }

    fn visit_item_mod_mut(&mut self, item: &mut syn::ItemMod) {
        if item.ident == "defaults" {
            item.attrs.push(syn::parse_quote!(#[allow(clippy::missing_panics_doc)]));
            if let Some((_, items)) = &mut item.content {
                for sub_item in items {
                    if let syn::Item::Fn(func) = sub_item {
                        func.vis = syn::Visibility::Public(syn::token::Pub::default());
                        if !func.attrs.iter().any(|a| a.path().is_ident("must_use")) {
                            func.attrs.push(syn::parse_quote!(#[must_use]));
                        }
                    }
                }
            }
        }
        syn::visit_mut::visit_item_mod_mut(self, item);
    }
}

pub fn append_derives(attrs: &mut [syn::Attribute], traits: &[&str]) {
    for attr in attrs.iter_mut() {
        if !attr.path().is_ident("derive") {
            continue;
        }
        let mut existing = HashSet::new();
        let _ = attr.parse_nested_meta(|meta| {
            if let Some(ident) = meta.path.get_ident() {
                existing.insert(ident.to_string());
            } else if let Some(last) = meta.path.segments.last() {
                existing.insert(last.ident.to_string());
            }
            Ok(())
        });
        if let syn::Meta::List(ref mut list) = attr.meta {
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

/// Adds `#[serde(skip_serializing_if = "Option::is_none")]` to optional `Option<T>` fields,
/// while omitting it on `required + nullable` fields so they serialize explicit `null`.
pub fn transform_add_skip_serializing_if<S1: std::hash::BuildHasher, S2: std::hash::BuildHasher>(
    file: &mut syn::File,
    schema_docs: &HashMap<String, SchemaDoc, S1>,
    rename_map: &HashMap<String, String, S2>,
) {
    for item in &mut file.items {
        let syn::Item::Struct(s) = item else {
            continue;
        };
        let syn::Fields::Named(named) = &mut s.fields else {
            continue;
        };
        let rust_name = s.ident.to_string();
        let original_name = rename_map.get(&rust_name).map_or(rust_name.as_str(), String::as_str);
        let schema_doc = schema_docs.get(original_name);

        for field in &mut named.named {
            if !is_option_type(&field.ty) {
                continue;
            }
            let wire_name = get_wire_field_name(field);
            let is_required = schema_doc
                .and_then(|doc| doc.properties.iter().find(|p| p.original_name == wire_name))
                .is_some_and(|p| p.required);

            if !is_required && !has_skip_serializing_if(field) {
                field
                    .attrs
                    .push(syn::parse_quote!(#[serde(skip_serializing_if = "Option::is_none")]));
            }
        }
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
pub fn extract_vec_inner(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(tp) = ty
        && let Some(seg) = tp.path.segments.last()
        && seg.ident == "Vec"
        && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
        && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
    {
        return Some(inner_ty);
    }
    None
}

#[must_use]
pub fn is_vec_type(ty: &syn::Type) -> bool {
    extract_vec_inner(ty).is_some()
}

#[must_use]
pub fn is_string_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(tp) if tp.path.segments.len() == 1 && tp.path.segments[0].ident == "String")
}

#[must_use]
pub fn is_typed_id_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(tp) = ty
        && let Some(seg) = tp.path.segments.last()
        && matches!(seg.arguments, syn::PathArguments::None)
    {
        return crate::common::id_mapping::is_typed_id_name(&seg.ident.to_string());
    }
    false
}

#[must_use]
pub fn is_uuid_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(tp) = ty
        && let Some(seg) = tp.path.segments.last()
        && matches!(seg.arguments, syn::PathArguments::None)
    {
        return seg.ident == "Uuid";
    }
    false
}

#[must_use]
pub fn is_u8_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(tp) if tp.path.segments.len() == 1 && tp.path.segments[0].ident == "u8")
}

#[must_use]
pub fn has_skip_serializing_if(field: &syn::Field) -> bool {
    field.attrs.iter().any(|a| {
        if !a.path().is_ident("serde") {
            return false;
        }
        let mut found = false;
        let _ = a.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip_serializing_if") {
                found = true;
            }
            if meta.input.peek(syn::Token![=]) {
                let value = meta.value()?;
                let _: syn::Expr = value.parse()?;
            }
            Ok(())
        });
        found
    })
}

/// Promotes trivial `typify`-generated `impl Default` blocks (where all fields are
/// `Default::default()`) to `#[derive(Default)]`, and replaces remaining `Default::default()`
/// field initializers with `None`, `Vec::new()`, or `<FieldType>::default()`.
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

    let mut derivable_structs: HashSet<String> = HashSet::new();

    file.items.retain_mut(|item| {
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
                    if func.block.stmts.len() == 1
                        && let syn::Stmt::Expr(syn::Expr::Struct(expr_struct), None) = &func.block.stmts[0]
                        && expr_struct.rest.is_none()
                        && expr_struct
                            .fields
                            .iter()
                            .all(|fv| fv.expr.to_token_stream().to_string().replace(' ', "") == "Default::default()")
                    {
                        derivable_structs.insert(s_name);
                        return false;
                    }

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
        true
    });

    if !derivable_structs.is_empty() {
        for item in &mut file.items {
            if let syn::Item::Struct(s) = item
                && derivable_structs.contains(&s.ident.to_string())
            {
                append_derives(&mut s.attrs, &["Default"]);
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

fn ergonomic_param_and_conv(
    param_ident: &syn::Ident,
    ty: &syn::Type,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    if is_string_type(ty) || is_typed_id_type(ty) {
        (quote! { #param_ident: impl Into<#ty> }, quote! { #param_ident.into() })
    } else if let Some(elem_ty) = extract_vec_inner(ty) {
        if is_u8_type(elem_ty) {
            (
                quote! { #param_ident: impl Into<Vec<u8>> },
                quote! { #param_ident.into() },
            )
        } else if is_string_type(elem_ty) || is_typed_id_type(elem_ty) || is_uuid_type(elem_ty) {
            (
                quote! { #param_ident: impl IntoIterator<Item = impl Into<#elem_ty>> },
                quote! { #param_ident.into_iter().map(Into::into).collect() },
            )
        } else {
            (
                quote! { #param_ident: impl IntoIterator<Item = #elem_ty> },
                quote! { #param_ident.into_iter().collect() },
            )
        }
    } else {
        (quote! { #param_ident: #ty }, quote! { #param_ident })
    }
}

/// Generates `pub fn new(...) -> Self` (taking only required fields) and fluent builder
/// setters `pub fn <field>(mut self, value: ...) -> Self` for all named-field structs.
#[allow(clippy::too_many_lines)]
pub fn transform_add_constructors_and_setters(file: &mut syn::File) {
    let mut existing_defaults: HashSet<String> = HashSet::new();
    for item in &file.items {
        match item {
            syn::Item::Struct(s) => {
                if has_derive_trait(&s.attrs, "Default") {
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
    let value_ident = syn::Ident::new("value", Span::call_site());

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
                } else {
                    let (param_sig, conv_expr) = ergonomic_param_and_conv(f_ident, f_ty);
                    new_params.push(param_sig);
                    if *f_ident == conv_expr.to_string() {
                        field_inits.push(quote! { #f_ident });
                    } else {
                        field_inits.push(quote! { #f_ident: #conv_expr });
                    }
                }

                let setter_doc = format!(" Sets `{f_ident}`.");
                if let Some(inner_ty) = extract_option_inner(f_ty) {
                    let (param_sig, conv_expr) = ergonomic_param_and_conv(&value_ident, inner_ty);
                    setters.push(quote! {
                        #[doc = #setter_doc]
                        #[must_use]
                        pub fn #f_ident(mut self, #param_sig) -> Self {
                            self.#f_ident = Some(#conv_expr);
                            self
                        }
                    });

                    let set_name = format!("set_{f_ident}");
                    if !field_names.contains(&set_name) {
                        let set_ident = quote::format_ident!("{set_name}");
                        let set_doc = format!(" Sets or clears `{f_ident}`.");
                        setters.push(quote! {
                            #[doc = #set_doc]
                            #[must_use]
                            pub fn #set_ident(mut self, value: Option<#inner_ty>) -> Self {
                                self.#f_ident = value;
                                self
                            }
                        });
                    }
                } else {
                    let (param_sig, conv_expr) = ergonomic_param_and_conv(&value_ident, f_ty);
                    setters.push(quote! {
                        #[doc = #setter_doc]
                        #[must_use]
                        pub fn #f_ident(mut self, #param_sig) -> Self {
                            self.#f_ident = #conv_expr;
                            self
                        }
                    });
                }
            }

            let new_doc = format!(" Creates a new [`{struct_name}`].");
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

    fn format_doc_type(
        &self,
        struct_name: &str,
        field_snake: &str,
        type_hint: &str,
        in_scope_types: &HashSet<String>,
    ) -> String {
        if let Some(id_name) = crate::common::id_mapping::resolve_struct_field_id(struct_name, field_snake) {
            if type_hint.starts_with("Vec<") {
                return format!("[`Vec<{id_name}>`](crate::id::{id_name})");
            }
            return format!("[`{id_name}`](crate::id::{id_name})");
        }

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
            return "`Vec<u8>`".to_string();
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

fn push_sanitized_prose_lines(out: &mut Vec<String>, text: &str) {
    for raw_line in text.lines() {
        out.push(crate::common::doc::sanitize_doc_line(raw_line));
    }
}

fn build_doc_attrs(lines: &[String]) -> Vec<syn::Attribute> {
    lines
        .iter()
        .map(|line| {
            let s = if line.is_empty() {
                String::new()
            } else {
                format!(" {line}")
            };
            syn::parse_quote!(#[doc = #s])
        })
        .collect()
}

fn normalize_existing_doc_attrs(attrs: &mut Vec<syn::Attribute>) {
    strip_json_schema(attrs);
    for attr in attrs.iter_mut() {
        if !attr.path().is_ident("doc") {
            continue;
        }
        if let syn::Meta::NameValue(ref nv) = attr.meta
            && let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(ref lit_str),
                ..
            }) = nv.value
        {
            let val = lit_str.value();
            let sanitized = crate::common::doc::sanitize_doc_line(val.trim());
            let formatted = if sanitized.is_empty() {
                String::new()
            } else {
                format!(" {sanitized}")
            };
            *attr = syn::parse_quote!(#[doc = #formatted]);
        }
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
                push_sanitized_prose_lines(&mut lines, desc);
            } else {
                lines.push(format!("`{rust_name}`"));
            }
            lines.push(String::new());
            lines.push(format!("See: <https://api.immich.app/models/{}>", doc.original_name));
            if doc.is_deprecated {
                lines.push(String::new());
                lines.push("**Deprecated**".into());
            }
            let mut new_attrs = build_doc_attrs(&lines);
            let old_attrs: Vec<syn::Attribute> = item.attrs.drain(..).filter(|a| !a.path().is_ident("doc")).collect();
            new_attrs.extend(old_attrs);
            item.attrs = new_attrs;
        } else {
            normalize_existing_doc_attrs(&mut item.attrs);
        }

        for variant in &mut item.variants {
            normalize_existing_doc_attrs(&mut variant.attrs);
        }

        syn::visit_mut::visit_item_enum_mut(self, item);
    }

    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        let rust_name = item.ident.to_string();
        if let Some(doc) = self.lookup(&rust_name).cloned() {
            let mut doc_lines: Vec<String> = Vec::new();

            if let Some(desc) = &doc.description {
                push_sanitized_prose_lines(&mut doc_lines, desc);
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
                    let field_snake = crate::common::naming::to_snake_case(&prop.original_name);
                    let field_escaped = crate::common::naming::escape_rust_keyword(&field_snake);
                    let req = if prop.required { "✓" } else { "" };
                    let type_display = self.format_doc_type(&rust_name, &field_snake, &prop.type_hint, &in_scope_types);
                    let desc = crate::common::doc::sanitize_table_cell(prop.description.as_deref().unwrap_or(""));
                    doc_lines.push(format!("| `{field_escaped}` | {type_display} | {req} | {desc} |"));
                }
            }

            doc_lines.push(String::new());
            doc_lines.push(format!("See: <https://api.immich.app/models/{}>", doc.original_name));

            if doc.is_deprecated {
                doc_lines.push(String::new());
                doc_lines.push("**Deprecated** — this type may be removed in a future Immich release.".into());
            }

            let mut new_attrs = build_doc_attrs(&doc_lines);
            let old_attrs: Vec<syn::Attribute> = item.attrs.drain(..).filter(|a| !a.path().is_ident("doc")).collect();
            new_attrs.extend(old_attrs);
            item.attrs = new_attrs;

            for field in &mut item.fields {
                let wire_name = get_wire_field_name(field);
                field.attrs.retain(|a| !a.path().is_ident("doc"));

                if let Some(prop) = doc.properties.iter().find(|p| p.original_name == wire_name) {
                    if let Some(desc) = &prop.description {
                        let mut f_lines = Vec::new();
                        push_sanitized_prose_lines(&mut f_lines, desc);
                        if prop.required {
                            if let Some(last) = f_lines.last_mut() {
                                if last.is_empty() {
                                    *last = "(required)".to_string();
                                } else {
                                    last.push_str(" (required)");
                                }
                            } else {
                                f_lines.push("(required)".to_string());
                            }
                        }
                        let mut f_doc_attrs = build_doc_attrs(&f_lines);
                        f_doc_attrs.append(&mut field.attrs);
                        field.attrs = f_doc_attrs;
                    } else if prop.required {
                        field.attrs.insert(0, syn::parse_quote!(#[doc = " (required)"]));
                    }
                }
            }
        } else {
            normalize_existing_doc_attrs(&mut item.attrs);
            for field in &mut item.fields {
                normalize_existing_doc_attrs(&mut field.attrs);
            }
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

fn extract_serde_rename_from_attrs(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let mut rename_val: Option<String> = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") && meta.input.peek(syn::Token![=]) {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;
                rename_val = Some(lit.value());
            } else if meta.input.peek(syn::Token![=]) {
                let value = meta.value()?;
                let _: syn::Expr = value.parse()?;
            }
            Ok(())
        });
        if rename_val.is_some() {
            return rename_val;
        }
    }
    None
}

#[must_use]
pub fn get_serde_rename(field: &syn::Field) -> Option<String> {
    extract_serde_rename_from_attrs(&field.attrs)
}

#[must_use]
pub fn get_wire_field_name(field: &syn::Field) -> String {
    get_serde_rename(field).unwrap_or_else(|| {
        field
            .ident
            .as_ref()
            .map(std::string::ToString::to_string)
            .unwrap_or_default()
    })
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
                self.used_imports.insert((mod_path.clone(), ident));
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
    fn visit_attribute_mut(&mut self, attr: &mut syn::Attribute) {
        if attr.path().is_ident("serde")
            && let syn::Meta::List(ref list) = attr.meta
            && let Ok(mut metas) =
                list.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        {
            let mut changed = false;
            for meta in &mut metas {
                if let syn::Meta::NameValue(nv) = meta
                    && nv.path.is_ident("default")
                    && let syn::Expr::Lit(expr_lit) = &mut nv.value
                    && let syn::Lit::Str(lit_str) = &mut expr_lit.lit
                    && let Some(rest) = lit_str.value().strip_prefix("defaults::")
                {
                    let new_val = format!("crate::common::defaults::{rest}");
                    *lit_str = syn::LitStr::new(&new_val, lit_str.span());
                    changed = true;
                }
            }
            if changed {
                *attr = syn::parse_quote!(#[serde(#metas)]);
            }
        }
        syn::visit_mut::visit_attribute_mut(self, attr);
    }

    fn visit_expr_path_mut(&mut self, i: &mut syn::ExprPath) {
        if let Some(qself) = &mut i.qself {
            self.visit_type_mut(&mut qself.ty);
            return;
        }
        syn::visit_mut::visit_expr_path_mut(self, i);
    }

    fn visit_type_path_mut(&mut self, i: &mut syn::TypePath) {
        if let Some(qself) = &mut i.qself {
            self.visit_type_mut(&mut qself.ty);
            return;
        }
        syn::visit_mut::visit_type_path_mut(self, i);
    }

    fn visit_path_mut(&mut self, node: &mut syn::Path) {
        syn::visit_mut::visit_path_mut(self, node);

        // Rewrite `self::error::ConversionError` or `error::ConversionError` -> `crate::common::error::ConversionError`
        if node.leading_colon.is_none() {
            let is_self_conv_err = node.segments.len() == 3
                && node.segments[0].ident == "self"
                && node.segments[1].ident == "error"
                && node.segments[2].ident == "ConversionError";
            let is_rel_conv_err = node.segments.len() == 2
                && node.segments[0].ident == "error"
                && node.segments[1].ident == "ConversionError";
            if is_self_conv_err || is_rel_conv_err {
                *node = syn::parse_quote!(crate::common::error::ConversionError);
                return;
            }
        }

        if node.segments.len() >= 3 && node.leading_colon.is_some() && node.segments[0].ident == "std" {
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
        if let syn::Expr::If(expr_if) = expr
            && let syn::Expr::Unary(unary) = &*expr_if.cond
            && matches!(unary.op, syn::UnOp::Not(_))
            && let Some((_, else_expr)) = &mut expr_if.else_branch
            && let syn::Expr::Block(else_block) = &mut **else_expr
        {
            let positive_cond = (*unary.expr).clone();
            *expr_if.cond = positive_cond;
            std::mem::swap(&mut expr_if.then_branch, &mut else_block.block);
        }

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
