use pretty_assertions::assert_eq;
use serde_json::json;
use xtask::client::openapi::{BodyDef, BodyKind, EndpointDef, ParamDef, ParamLocation, extract_endpoints};

#[test]
fn extract_endpoints_fuses_path_level_params_and_normalizes_dto_names() {
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
                                        "$ref": "#/components/schemas/UserResponseDto"
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

    assert_eq!(endpoints[0], expected);
}

#[test]
fn extract_endpoints_detects_raw_streaming_json_body_and_deprecated_flag() {
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
                                    "$ref": "#/components/schemas/SyncConfigDto"
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
            kind: BodyKind::Json,
        })
    );
}

#[test]
fn extract_endpoints_detects_multipart_and_form_urlencoded_content_types() {
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
                                    "$ref": "#/components/schemas/AssetUploadDto"
                                }
                            }
                        }
                    },
                    "responses": {
                        "201": { "description": "Created" }
                    }
                }
            },
            "/oauth/mobile-redirect": {
                "post": {
                    "operationId": "logoutOAuth",
                    "tags": ["Authentication"],
                    "requestBody": {
                        "content": {
                            "application/x-www-form-urlencoded": {
                                "schema": {
                                    "$ref": "#/components/schemas/OAuthLogoutDto"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": { "description": "Logged out" }
                    }
                }
            }
        }
    });

    let endpoints = extract_endpoints(&spec).unwrap();
    assert_eq!(endpoints.len(), 2);

    let upload = endpoints.iter().find(|e| e.operation_id == "uploadAsset").unwrap();
    assert_eq!(upload.params.len(), 1);
    assert_eq!(upload.params[0].name, "tagIds");
    assert_eq!(upload.params[0].rust_type, "Vec<uuid::Uuid>");
    assert!(upload.params[0].is_array);
    assert_eq!(
        upload.body,
        Some(BodyDef {
            rust_type: "AssetUploadDTO".to_string(),
            kind: BodyKind::Multipart,
        })
    );

    let oauth = endpoints.iter().find(|e| e.operation_id == "logoutOAuth").unwrap();
    assert_eq!(
        oauth.body,
        Some(BodyDef {
            rust_type: "OAuthLogoutDTO".to_string(),
            kind: BodyKind::FormUrlEncoded,
        })
    );
}
