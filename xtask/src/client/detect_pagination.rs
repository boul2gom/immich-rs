use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaginationKind {
    /// Object wrapping an items list with a boolean or cursor next field
    Wrapped {
        has_next_field: String,
        items_field: String,
        is_cursor: bool,
    },
    /// Raw JSON array response with page / size query parameters
    DirectPage {
        page_param: String,
        size_param: Option<String>,
    },
    /// Raw JSON array response with take / skip query parameters
    DirectOffset { take_param: String, skip_param: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginatorDef {
    pub kind: PaginationKind,
    pub item_inner_type: String,
}

impl PaginatorDef {
    #[must_use]
    pub fn is_cursor(&self) -> bool {
        matches!(self.kind, PaginationKind::Wrapped { is_cursor: true, .. })
    }
}

#[must_use]
#[doc(hidden)]
pub fn detect_paginator(op: &Value, response_schema: &Value, spec: &Value) -> Option<PaginatorDef> {
    let mut param_names = Vec::new();
    if let Some(params) = op.get("parameters").and_then(|p| p.as_array()) {
        for p in params {
            if let Some(name) = p.get("name").and_then(|n| n.as_str()) {
                param_names.push(name.to_string());
            }
        }
    }

    // 1. Direct array response pagination (e.g. searchMemories returning Vec<MemoryResponseDto> with page & size)
    if response_schema.get("type").and_then(|t| t.as_str()) == Some("array") {
        let items_schema = &response_schema["items"];
        let (item_inner_type, _) = super::openapi::resolve_rust_type(items_schema);

        if param_names.iter().any(|p| p == "page") {
            let size_param = param_names
                .iter()
                .find(|p| *p == "size" || *p == "pageSize" || *p == "limit")
                .map(|s| crate::common::camel_to_snake(s));

            return Some(PaginatorDef {
                kind: PaginationKind::DirectPage {
                    page_param: "page".to_string(),
                    size_param,
                },
                item_inner_type,
            });
        }

        if param_names.iter().any(|p| p == "take") && param_names.iter().any(|p| p == "skip") {
            return Some(PaginatorDef {
                kind: PaginationKind::DirectOffset {
                    take_param: "take".to_string(),
                    skip_param: "skip".to_string(),
                },
                item_inner_type,
            });
        }
    }

    // 2. Wrapped object response pagination (e.g. IntegrityReport, People, etc.)
    let mut param_name = None;
    let mut is_cursor = false;
    for name in &param_names {
        if name == "page" {
            param_name = Some(name.clone());
        } else if name == "cursor" {
            param_name = Some(name.clone());
            is_cursor = true;
        }
    }

    let _param_name = param_name?;

    let mut schema = response_schema;
    if let Some(ref_str) = schema.get("$ref").and_then(|v| v.as_str()) {
        let name = ref_str.split('/').next_back().unwrap();
        schema = spec.pointer(&format!("/components/schemas/{name}"))?;
    }

    if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
        let has_next_field = if props.contains_key("nextCursor") && is_cursor {
            Some("next_cursor".to_string())
        } else if props.contains_key("hasNextPage") {
            Some("has_next_page".to_string())
        } else if props.contains_key("nextCursor") {
            Some("next_cursor".to_string())
        } else {
            None
        };

        // Prioritize the explicit `"items"` property first before scanning other array properties
        let candidate_array = props
            .get("items")
            .filter(|v| v.get("type").and_then(|t| t.as_str()) == Some("array"))
            .map(|v| ("items", v))
            .or_else(|| {
                props
                    .iter()
                    .find(|(_, v)| v.get("type").and_then(|t| t.as_str()) == Some("array"))
                    .map(|(k, v)| (k.as_str(), v))
            });

        if let (Some(hnf), Some((arr_key, arr_val))) = (has_next_field, candidate_array) {
            let items_field = crate::common::camel_to_snake(arr_key);
            let items_schema = &arr_val["items"];
            let (rt, _) = super::openapi::resolve_rust_type(items_schema);
            let item_inner_type = if rt == "serde_json::Value" && arr_key == "items" {
                if let Some(ref_str) = response_schema.get("$ref").and_then(|v| v.as_str()) {
                    let name = ref_str.split('/').next_back().unwrap();
                    crate::common::naming::rename_type(&format!("{name}ItemsItem"))
                } else {
                    "serde_json::Value".to_string()
                }
            } else {
                rt
            };

            return Some(PaginatorDef {
                kind: PaginationKind::Wrapped {
                    has_next_field: hnf,
                    items_field,
                    is_cursor,
                },
                item_inner_type,
            });
        }
    }

    None
}
