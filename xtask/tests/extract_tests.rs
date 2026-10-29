use pretty_assertions::assert_eq;
use serde_json::json;
use xtask::client::openapi::*;

#[test]
fn test_extract_endpoints_with_path_level_params_fusion_and_types() {
    let spec = json!({
        "paths": {
            "/users/{id}": {
                "parameters": [
                    {
                        "name": "id",
                        "in": "path",
                        "required": true,
                        "description": "User identifier",
                        "schema": {
                            "type": "string",
                            "format": "uuid"
                        }
                    }
                ],
                "get": {
                    "operationId": "getUser",
                    "tags": ["Users"],
                    "summary": "Get user by ID",
                    "parameters": [
                        {
                            "name": "includeDeleted",
                            "in": "query",
                            "required": false,
                            "schema": {
                                "type": "boolean"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "User found",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/UserResponseDTO"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    let endpoints = extract_endpoints(&spec).unwrap();
    assert_eq!(endpoints.len(), 1);
    let ep = &endpoints[0];

    let expected = EndpointDef {
        description: None,
        responses_docs: vec![("200".to_string(), "User found".to_string())],
        operation_id: "getUser".to_string(),
        method: "get".to_string(),
        path: "/users/{id}".to_string(),
        tags: vec!["Users".to_string()],
        summary: Some("Get user by ID".to_string()),
        params: vec![
            ParamDef {
                description: Some("User identifier".to_string()),
                name: "id".to_string(),
                in_: ParamLocation::Path,
                required: true,
                rust_type: "uuid::Uuid".to_string(),
                is_array: false,
            },
            ParamDef {
                description: None,
                name: "includeDeleted".to_string(),
                in_: ParamLocation::Query,
                required: false,
                rust_type: "bool".to_string(),
                is_array: false,
            },
        ],
        body: None,
        response_type: Some("UserResponseDTO".to_string()),
        paginator: None,
        is_raw_response: false,
        deprecated: false,
    };

    assert_eq!(*ep, expected);
}

#[test]
fn test_extract_endpoints_detects_raw_streaming_and_deprecated() {
    let spec = json!({
        "paths": {
            "/sync/stream": {
                "post": {
                    "operationId": "getSyncStream",
                    "tags": ["Sync"],
                    "deprecated": true,
                    "requestBody": {
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/SyncConfigDTO"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Streaming changes",
                            "content": {
                                "application/octet-stream": {
                                    "schema": {
                                        "type": "string",
                                        "format": "binary"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    let endpoints = extract_endpoints(&spec).unwrap();
    assert_eq!(endpoints.len(), 1);
    let ep = &endpoints[0];

    assert_eq!(ep.operation_id, "getSyncStream");
    assert!(ep.deprecated, "deprecated flag should be extracted");
    assert!(ep.is_raw_response, "octet-stream should be detected as raw response");
    assert_eq!(
        ep.body,
        Some(BodyDef {
            rust_type: "SyncConfigDTO".to_string(),
            is_multipart: false,
        })
    );
}

#[test]
fn test_extract_endpoints_detects_multipart_and_array_query() {
    let spec = json!({
        "paths": {
            "/assets": {
                "post": {
                    "operationId": "uploadAsset",
                    "tags": ["Assets"],
                    "parameters": [
                        {
                            "name": "tagIds",
                            "in": "query",
                            "required": false,
                            "schema": {
                                "type": "array",
                                "items": {
                                    "type": "string",
                                    "format": "uuid"
                                }
                            }
                        }
                    ],
                    "requestBody": {
                        "content": {
                            "multipart/form-data": {
                                "schema": {
                                    "$ref": "#/components/schemas/AssetUploadDTO"
                                }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "Created"
                        }
                    }
                }
            }
        }
    });

    let endpoints = extract_endpoints(&spec).unwrap();
    assert_eq!(endpoints.len(), 1);
    let ep = &endpoints[0];

    assert_eq!(ep.params.len(), 1);
    assert_eq!(ep.params[0].name, "tagIds");
    assert_eq!(ep.params[0].rust_type, "Vec<uuid::Uuid>");
    assert!(
        ep.params[0].is_array,
        "array query parameter should be detected as array"
    );
    assert_eq!(
        ep.body,
        Some(BodyDef {
            rust_type: "AssetUploadDTO".to_string(),
            is_multipart: true,
        })
    );
}
