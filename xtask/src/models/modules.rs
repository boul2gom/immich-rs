use std::collections::{HashMap, HashSet};

use crate::common::assign::{RuleTracker, assign_module};

pub(crate) fn split_modules(
    file: &syn::File,
    schema_tag_map: &HashMap<String, HashSet<String>>,
    rename_map: &HashMap<String, String>,
) -> HashMap<String, Vec<syn::Item>> {
    let mut out: HashMap<String, Vec<syn::Item>> = HashMap::new();

    let use_items: Vec<syn::Item> = file
        .items
        .iter()
        .filter(|i| matches!(i, syn::Item::Use(_)))
        .cloned()
        .collect();

    let mut tracker = RuleTracker::new();

    for item in &file.items {
        if matches!(item, syn::Item::Use(_)) {
            continue;
        }
        let module = assign_module(item, schema_tag_map, rename_map, &mut tracker);
        out.entry(module).or_default().push(item.clone());
    }

    if !use_items.is_empty() {
        for items in out.values_mut() {
            let mut with_uses = use_items.clone();
            with_uses.append(items);
            *items = with_uses;
        }
    }

    out
}
