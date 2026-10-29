use super::{Context, HashMap, HashSet, Result};

#[derive(Default, Clone, Debug)]
pub struct SchemaDoc {
    pub original_name: String,
    pub description: Option<String>,
    pub is_deprecated: bool,
    pub properties: Vec<PropDoc>,
}

#[derive(Default, Clone, Debug)]
pub struct PropDoc {
    /// camelCase name from the `OpenAPI` spec.
    pub original_name: String,
    pub description: Option<String>,
    pub type_hint: String,
    pub required: bool,
}

pub fn extract_schema_docs(spec: &serde_json::Value) -> HashMap<String, SchemaDoc> {
    let Some(schemas) = spec.pointer("/components/schemas").and_then(|v| v.as_object()) else {
        return HashMap::new();
    };

    let tag_map = build_schema_tag_map(spec);

    let deprecated_only: HashSet<String> = tag_map
        .iter()
        .filter(|(_, tags)| tags.iter().all(|t| t == "Deprecated"))
        .map(|(name, _)| name.clone())
        .collect();

    let mut docs = HashMap::new();

    for (name, schema) in schemas {
        let required: HashSet<String> = schema
            .get("required")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(String::from).collect())
            .unwrap_or_default();

        let properties: Vec<PropDoc> = schema
            .get("properties")
            .and_then(|v| v.as_object())
            .map(|props| {
                props
                    .iter()
                    .map(|(prop_name, prop_schema)| {
                        let is_required = required.contains(prop_name);
                        PropDoc {
                            required: is_required,
                            original_name: prop_name.clone(),
                            description: prop_schema
                                .get("description")
                                .and_then(|v| v.as_str())
                                .map(String::from),
                            type_hint: extract_type_hint(prop_schema),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        docs.insert(
            name.clone(),
            SchemaDoc {
                original_name: name.clone(),
                description: schema.get("description").and_then(|v| v.as_str()).map(String::from),
                is_deprecated: deprecated_only.contains(name),
                properties,
            },
        );
    }

    docs
}

pub fn extract_type_hint(schema: &serde_json::Value) -> String {
    if let Some(ref_str) = schema.get("$ref").and_then(|v| v.as_str()) {
        return ref_str.split('/').next_back().unwrap_or("unknown").to_string();
    }
    let fmt = schema.get("format").and_then(|v| v.as_str()).unwrap_or("");
    let ty = schema.get("type").and_then(|v| v.as_str()).unwrap_or("");
    match (ty, fmt) {
        ("string", "uuid") => "Uuid".into(),
        ("string", "date-time") => "DateTime".into(),
        ("string", "binary") => "Bytes (as String)".into(),
        ("string", _) => "String".into(),
        ("integer", _) => "i64".into(),
        ("number", _) => "f64".into(),
        ("boolean", _) => "bool".into(),
        ("array", _) => {
            let inner = schema.get("items").map_or_else(|| "unknown".into(), extract_type_hint);
            format!("Vec<{inner}>")
        }
        _ => {
            for key in &["allOf", "oneOf", "anyOf"] {
                if let Some(first) = schema.get(key).and_then(|v| v.as_array()).and_then(|a| a.first()) {
                    return extract_type_hint(first);
                }
            }
            "object".into()
        }
    }
}

/// Map each schema name → the set of `OpenAPI` tags that reference it.
pub fn build_schema_tag_map(spec: &serde_json::Value) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    let Some(paths) = spec.get("paths").and_then(|v| v.as_object()) else {
        return map;
    };

    for (_path, methods) in paths {
        let Some(methods) = methods.as_object() else {
            continue;
        };
        for (_method, op) in methods {
            let Some(op) = op.as_object() else {
                continue;
            };
            let tags: HashSet<String> = op
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|t| t.as_str()).map(String::from).collect())
                .unwrap_or_default();

            if tags.is_empty() {
                continue;
            }

            collect_refs(&serde_json::Value::Object(op.clone()), &tags, &mut map);
        }
    }

    // Transitively propagate tags through nested $refs in components/schemas
    if let Some(schemas) = spec.pointer("/components/schemas").and_then(|v| v.as_object()) {
        loop {
            let mut changed = false;
            for (schema_name, schema_val) in schemas {
                let Some(existing_tags) = map.get(schema_name).cloned() else {
                    continue;
                };
                if existing_tags.is_empty() {
                    continue;
                }
                let mut nested = HashMap::new();
                collect_refs(schema_val, &existing_tags, &mut nested);
                for (child_name, child_tags) in nested {
                    let entry = map.entry(child_name).or_default();
                    let before = entry.len();
                    entry.extend(child_tags);
                    if entry.len() > before {
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
    }

    map
}

pub fn collect_refs(val: &serde_json::Value, tags: &HashSet<String>, map: &mut HashMap<String, HashSet<String>>) {
    match val {
        serde_json::Value::Object(obj) => {
            if let Some(r) = obj.get("$ref").and_then(|v| v.as_str())
                && let Some(name) = r.strip_prefix("#/components/schemas/")
            {
                map.entry(name.to_string()).or_default().extend(tags.iter().cloned());
            }
            for v in obj.values() {
                collect_refs(v, tags, map);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                collect_refs(v, tags, map);
            }
        }
        _ => {}
    }
}

// ─── $ref rewriting ───────────────────────────────────────────────────────────

pub fn patch_openapi_v3_to_jsonschema(val: &mut serde_json::Value) {
    match val {
        serde_json::Value::Object(map) => {
            // Recurse first so inner objects are patched
            for v in map.values_mut() {
                patch_openapi_v3_to_jsonschema(v);
            }

            // 1. Rewrite $ref
            if let Some(r) = map.get_mut("$ref")
                && let Some(s) = r.as_str()
                && let Some(name) = s.strip_prefix("#/components/schemas/")
            {
                *r = serde_json::Value::String(format!("#/definitions/{name}"));
            }

            // 2. Strip default: null
            if map.get("default").is_some_and(serde_json::Value::is_null) {
                map.remove("default");
            }

            // 2b. Translate OpenAPI 3.0 boolean exclusiveMinimum / exclusiveMaximum to JSON Schema Draft 7 numbers
            if let Some(excl_min) = map.get("exclusiveMinimum").and_then(serde_json::Value::as_bool) {
                map.remove("exclusiveMinimum");
                if excl_min && let Some(min_val) = map.remove("minimum") {
                    map.insert("exclusiveMinimum".to_string(), min_val);
                }
            }
            if let Some(excl_max) = map.get("exclusiveMaximum").and_then(serde_json::Value::as_bool) {
                map.remove("exclusiveMaximum");
                if excl_max && let Some(max_val) = map.remove("maximum") {
                    map.insert("exclusiveMaximum".to_string(), max_val);
                }
            }

            // 3. Handle nullable: true
            let is_nullable = map
                .get("nullable")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            if is_nullable {
                map.remove("nullable");
                let original_schema = serde_json::Value::Object(map.clone());
                map.clear();
                map.insert(
                    "anyOf".to_string(),
                    serde_json::Value::Array(vec![original_schema, serde_json::json!({ "type": "null" })]),
                );
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                patch_openapi_v3_to_jsonschema(v);
            }
        }
        _ => {}
    }
}

// ─── Root schema building ─────────────────────────────────────────────────────

pub fn build_root_schema(schemas_val: &serde_json::Value) -> Result<schemars::schema::RootSchema> {
    let mut root = schemars::schema::RootSchema::default();
    if let Some(obj) = schemas_val.as_object() {
        for (name, schema_val) in obj {
            let schema: schemars::schema::Schema =
                serde_json::from_value(schema_val.clone()).with_context(|| format!("parsing schema '{name}'"))?;
            root.definitions.insert(name.clone(), schema);
        }
    }
    Ok(root)
}

// ─── AST transforms ───────────────────────────────────────────────────────────
