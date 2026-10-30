use std::collections::BTreeMap;

use super::openapi;

#[must_use]
pub fn split_endpoints(endpoints: Vec<openapi::EndpointDef>) -> BTreeMap<String, Vec<openapi::EndpointDef>> {
    let mut by_module: BTreeMap<String, Vec<openapi::EndpointDef>> = BTreeMap::new();
    for ep in endpoints {
        let tag = ep
            .tags
            .iter()
            .map(std::string::String::as_str)
            .find(|&t| t != "Deprecated")
            .or_else(|| ep.tags.first().map(std::string::String::as_str))
            .unwrap_or("common");
        let mut module = "common".to_string();
        for (t, m) in crate::common::tables::TAG_MODULE {
            if *t == tag {
                module = (*m).to_string();
                break;
            }
        }
        by_module.entry(module).or_default().push(ep);
    }
    by_module
}
