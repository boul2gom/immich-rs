use anyhow::{Context, Result};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamDef {
    pub description: Option<String>,
    pub name: String,
    pub in_: ParamLocation,
    pub required: bool,
    pub rust_type: String,
    pub is_array: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamLocation {
    Path,
    Query,
    Header,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyDef {
    pub rust_type: String,
    pub is_multipart: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointDef {
    pub description: Option<String>,
    pub responses_docs: Vec<(String, String)>,
    pub operation_id: String,
    pub method: String,
    pub path: String,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub params: Vec<ParamDef>,
    pub body: Option<BodyDef>,
    pub response_type: Option<String>,
    pub paginator: Option<crate::client::detect_pagination::PaginatorDef>,
    pub is_raw_response: bool,
    pub deprecated: bool,
}

fn parse_param(p: &Value) -> Result<Option<ParamDef>> {
    let p_obj = p.as_object().context("Expected parameter to be an object")?;
    let in_str = p_obj["in"].as_str().context("Missing 'in' for parameter")?;
    let in_ = match in_str {
        "path" => ParamLocation::Path,
        "query" => ParamLocation::Query,
        "header" => ParamLocation::Header,
        _ => return Ok(None),
    };
    let name = p_obj["name"]
        .as_str()
        .context("Missing 'name' for parameter")?
        .to_string();
    let required = p_obj
        .get("required")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let description = p_obj.get("description").and_then(|v| v.as_str()).map(String::from);

    let schema = &p_obj["schema"];
    let (mut rust_type, is_array) = resolve_rust_type(schema);
    if is_array {
        rust_type = format!("Vec<{rust_type}>");
    }

    Ok(Some(ParamDef {
        description,
        name,
        in_,
        required,
        rust_type,
        is_array,
    }))
}

#[doc(hidden)]
pub fn extract_endpoints(spec: &Value) -> Result<Vec<EndpointDef>> {
    let mut endpoints = Vec::new();
    let paths = spec["paths"]
        .as_object()
        .context("Missing 'paths' object in OpenAPI spec")?;

    for (path, methods) in paths {
        let methods_obj = methods.as_object().context("Expected object for path methods")?;

        let mut path_level_params = Vec::new();
        if let Some(parameters) = methods_obj.get("parameters").and_then(|v| v.as_array()) {
            for p in parameters {
                if let Some(parsed) = parse_param(p)? {
                    path_level_params.push(parsed);
                }
            }
        }

        for (method, op) in methods_obj {
            if method == "parameters" {
                continue;
            }

            let operation_id = op["operationId"]
                .as_str()
                .with_context(|| format!("Missing operationId in {method} {path}"))?
                .to_string();
            let tags = op["tags"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str())
                        .map(std::string::ToString::to_string)
                        .collect()
                })
                .unwrap_or_default();
            let summary = op["summary"].as_str().map(String::from);
            let description = op["description"].as_str().map(String::from);

            let mut params = path_level_params.clone();
            if let Some(parameters) = op["parameters"].as_array() {
                for p in parameters {
                    if let Some(parsed) = parse_param(p)? {
                        if let Some(existing) = params
                            .iter_mut()
                            .find(|ep| ep.name == parsed.name && ep.in_ == parsed.in_)
                        {
                            *existing = parsed;
                        } else {
                            params.push(parsed);
                        }
                    }
                }
            }

            // Body
            let mut body = None;
            if let Some(request_body) = op.get("requestBody")
                && let Some(content) = request_body.get("content").and_then(|c| c.as_object())
            {
                if let Some(json) = content.get("application/json") {
                    let schema = &json["schema"];
                    let (rust_type, _) = resolve_rust_type(schema);
                    body = Some(BodyDef {
                        rust_type,
                        is_multipart: false,
                    });
                } else if let Some(multipart) = content.get("multipart/form-data") {
                    let schema = &multipart["schema"];
                    let (rust_type, _) = resolve_rust_type(schema);
                    body = Some(BodyDef {
                        rust_type,
                        is_multipart: true,
                    });
                }
            }

            // Responses docs
            let mut responses_docs = Vec::new();
            if let Some(responses) = op.get("responses").and_then(|r| r.as_object()) {
                for (code, resp) in responses {
                    let desc = resp
                        .get("description")
                        .and_then(|d| d.as_str())
                        .unwrap_or_default()
                        .to_string();
                    responses_docs.push((code.clone(), desc));
                }
            }

            // Response (find 200/201/202 success)
            let mut response_type = None;
            let mut paginator = None;
            let mut is_raw_response = false;
            if let Some(responses) = op.get("responses").and_then(|r| r.as_object()) {
                let success = responses
                    .get("200")
                    .or_else(|| responses.get("201"))
                    .or_else(|| responses.get("202"));
                if let Some(resp) = success {
                    if let Some(content) = resp.get("content").and_then(|c| c.as_object()) {
                        if let Some(json) = content.get("application/json") {
                            let schema = &json["schema"];
                            let (rt, is_arr) = resolve_rust_type(schema);
                            if is_arr {
                                response_type = Some(format!("Vec<{rt}>"));
                            } else {
                                response_type = Some(rt);
                            }

                            // Detect paginator dynamically
                            paginator = crate::client::detect_pagination::detect_paginator(op, schema, spec);
                        } else if !content.is_empty() {
                            response_type = Some("reqwest::Response".to_string());
                            is_raw_response = true;
                        }
                    } else if operation_id == "getSyncStream" || path.ends_with("/stream") {
                        response_type = Some("reqwest::Response".to_string());
                        is_raw_response = true;
                    }
                }
            }

            endpoints.push(EndpointDef {
                description,
                responses_docs,
                operation_id,
                method: method.clone(),
                path: path.clone(),
                tags,
                summary,
                params,
                body,
                response_type,
                paginator,
                is_raw_response,
                deprecated: op
                    .get("deprecated")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false),
            });
        }
    }

    Ok(endpoints)
}

#[must_use]
#[doc(hidden)]
pub fn resolve_rust_type(schema: &Value) -> (String, bool) {
    if let Some(ref_str) = schema.get("$ref").and_then(|v| v.as_str()) {
        let name = ref_str.split('/').next_back().unwrap_or(ref_str);
        let rust_name = crate::common::naming::rename_type(name);
        return (rust_name, false);
    }

    let type_str = schema.get("type").and_then(|v| v.as_str()).unwrap_or("object");
    match type_str {
        "string" => {
            if schema.get("format").and_then(|v| v.as_str()) == Some("binary") {
                ("reqwest::Body".to_string(), false)
            } else if schema.get("format").and_then(|v| v.as_str()) == Some("date-time") {
                ("chrono::DateTime<chrono::Utc>".to_string(), false)
            } else if schema.get("format").and_then(|v| v.as_str()) == Some("uuid") {
                ("uuid::Uuid".to_string(), false)
            } else {
                ("String".to_string(), false)
            }
        }
        "integer" => ("i64".to_string(), false),
        "number" => ("f64".to_string(), false),
        "boolean" => ("bool".to_string(), false),
        "array" => {
            let items = &schema["items"];
            let (inner, _) = resolve_rust_type(items);
            (inner, true)
        }
        _ => ("serde_json::Value".to_string(), false),
    }
}
