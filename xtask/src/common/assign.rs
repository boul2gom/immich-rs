use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct RuleTracker {
    pub used_tags: HashSet<&'static str>,
    pub used_sysconfig: HashSet<&'static str>,
    pub used_prefixes: HashSet<&'static str>,
}

impl RuleTracker {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[must_use]
pub fn item_ident(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Struct(s) => Some(s.ident.to_string()),
        syn::Item::Enum(e) => Some(e.ident.to_string()),
        syn::Item::Type(t) => Some(t.ident.to_string()),
        syn::Item::Const(c) => Some(c.ident.to_string()),
        _ => None,
    }
}

fn is_builtin_ident(name: &str) -> bool {
    matches!(
        name,
        "String"
            | "str"
            | "i64"
            | "f64"
            | "bool"
            | "usize"
            | "isize"
            | "u64"
            | "u32"
            | "i32"
            | "Uuid"
            | "f32"
            | "u8"
            | "i8"
            | "u16"
            | "i16"
            | "Value"
    ) || name.starts_with("DateTime")
}

pub fn assign_module<S1: std::hash::BuildHasher, S2: std::hash::BuildHasher>(
    item: &syn::Item,
    schema_tag_map: &HashMap<String, HashSet<String, S1>, S2>,
    rename_map: &HashMap<String, String, S2>,
    tracker: &mut RuleTracker,
) -> String {
    let Some(rust_name) = item_ident(item) else {
        if let syn::Item::Impl(i) = item
            && let syn::Type::Path(tp) = i.self_ty.as_ref()
            && let Some(seg) = tp.path.segments.last()
        {
            let self_ident = seg.ident.to_string();
            // For `impl From<DomainNewtype> for String`, route to `DomainNewtype`'s module
            if is_builtin_ident(&self_ident)
                && let Some((trait_path, _)) = &i.trait_
                && let Some(trait_seg) = trait_path.segments.last()
                && let syn::PathArguments::AngleBracketed(args) = &trait_seg.arguments
                && let Some(syn::GenericArgument::Type(syn::Type::Path(arg_tp))) = args.args.first()
                && let Some(arg_seg) = arg_tp.path.segments.last()
            {
                return assign_by_name(&arg_seg.ident.to_string(), schema_tag_map, rename_map, tracker);
            }
            return assign_by_name(&self_ident, schema_tag_map, rename_map, tracker);
        }
        return "common".into();
    };

    assign_by_name(&rust_name, schema_tag_map, rename_map, tracker)
}

#[doc(hidden)]
pub fn assign_by_name<S1: std::hash::BuildHasher, S2: std::hash::BuildHasher>(
    rust_name: &str,
    schema_tag_map: &HashMap<String, HashSet<String, S1>, S2>,
    rename_map: &HashMap<String, String, S2>,
    tracker: &mut RuleTracker,
) -> String {
    for (prefix, module) in crate::common::tables::SYSCONFIG_SUB {
        if rust_name.starts_with(prefix) {
            tracker.used_sysconfig.insert(prefix);
            return (*module).to_string();
        }
    }

    let original = rename_map.get(rust_name).map_or(rust_name, std::string::String::as_str);

    if let Some(tags) = schema_tag_map.get(original)
        && let Some(tag) = pick_primary_tag(tags)
    {
        if tag == "Deprecated" {
            return "deprecated".into();
        }
        for (t, module) in crate::common::tables::TAG_MODULE {
            if *t == tag {
                tracker.used_tags.insert(t);
                if *module == "system/config" {
                    for (prefix, sub) in crate::common::tables::SYSCONFIG_SUB {
                        if rust_name.starts_with(prefix) {
                            tracker.used_sysconfig.insert(prefix);
                            return (*sub).to_string();
                        }
                    }
                }
                return (*module).to_string();
            }
        }
    }

    let mut best: Option<(&str, usize, &'static str)> = None;
    for (prefix, module) in crate::common::tables::PREFIX_MODULE {
        if rust_name.starts_with(prefix) {
            let len = prefix.len();
            if best.is_none_or(|(_, l, _)| len > l) {
                best = Some((module, len, prefix));
            }
        }
    }
    if let Some((module, _, prefix)) = best {
        tracker.used_prefixes.insert(prefix);
        return module.to_string();
    }

    if is_builtin_ident(rust_name) {
        return "common".into();
    }

    "common".into()
}

#[must_use]
pub fn pick_primary_tag<S: std::hash::BuildHasher>(tags: &HashSet<String, S>) -> Option<&str> {
    let non_deprecated: Vec<&str> = tags
        .iter()
        .map(std::string::String::as_str)
        .filter(|&t| t != "Deprecated" && t != "Views")
        .collect();

    if non_deprecated.is_empty() {
        if tags.contains("Deprecated") {
            return Some("Deprecated");
        }
        return None;
    }

    let mut sorted = non_deprecated;
    sorted.sort_unstable();
    sorted.into_iter().next()
}
