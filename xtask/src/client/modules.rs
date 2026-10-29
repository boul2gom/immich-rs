use std::collections::BTreeMap;

use super::openapi;

pub fn split_endpoints(endpoints: Vec<openapi::EndpointDef>) -> BTreeMap<String, Vec<openapi::EndpointDef>> {
    let mut by_module: BTreeMap<String, Vec<openapi::EndpointDef>> = BTreeMap::new();
    for ep in endpoints {
        let tag = ep.tags.first().map_or("common", std::string::String::as_str);
        let mut module = "common".to_string();
        for (t, m) in crate::common::tables::TAG_MODULE {
            if t == &tag {
                module = m.to_string();
                break;
            }
        }
        by_module.entry(module).or_default().push(ep);
    }
    by_module
}
