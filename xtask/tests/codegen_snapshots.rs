use std::collections::HashMap;

use xtask::client::ast::emit_client_module;
use xtask::client::openapi::{BodyDef, BodyKind, EndpointDef, ParamDef, ParamLocation};
use xtask::models::ast::{
    transform_add_constructors_and_setters, transform_forward_compatible_enums, transform_rename_types,
    transform_typed_ids,
};

#[test]
fn codegen_models_ast_transforms_match_snapshot() {
    let mut file: syn::File = syn::parse_quote! {
        #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum ReactionType {
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "like")]
            Like,
        }

        #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, PartialEq, Eq)]
        pub struct ActivityCreateDto {
            #[serde(rename = "albumId")]
            pub album_id: ::uuid::Uuid,
            #[serde(rename = "assetId", default, skip_serializing_if = "::std::option::Option::is_none")]
            pub asset_id: Option<::uuid::Uuid>,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            pub comment: Option<String>,
            #[serde(rename = "type")]
            pub type_: ReactionType,
        }
    };

    let mut rename_map = HashMap::new();
    transform_rename_types(&mut file, &mut rename_map);
    transform_typed_ids(&mut file);
    transform_forward_compatible_enums(&mut file);
    transform_add_constructors_and_setters(&mut file);

    let rendered = prettyplease::unparse(&file);
    insta::assert_snapshot!("models_ast_enum_and_dto", rendered);
}

#[test]
fn codegen_client_builder_and_urlencoded_endpoint_match_snapshot() {
    let endpoints = vec![
        EndpointDef {
            description: None,
            responses_docs: vec![("200".to_string(), "Asset details".to_string())],
            operation_id: "getAssetInfo".to_string(),
            method: "get".to_string(),
            path: "/assets/{id}".to_string(),
            tags: vec!["Assets".to_string()],
            summary: Some("Retrieve an asset".to_string()),
            params: vec![
                ParamDef {
                    description: Some("Asset identifier".to_string()),
                    name: "id".to_string(),
                    in_: ParamLocation::Path,
                    required: true,
                    rust_type: "uuid::Uuid".to_string(),
                    is_array: false,
                },
                ParamDef {
                    description: Some("Optional share key".to_string()),
                    name: "key".to_string(),
                    in_: ParamLocation::Query,
                    required: false,
                    rust_type: "String".to_string(),
                    is_array: false,
                },
            ],
            body: None,
            response_type: Some("AssetResponseDTO".to_string()),
            paginator: None,
            is_raw_response: false,
            deprecated: false,
        },
        EndpointDef {
            description: None,
            responses_docs: vec![("200".to_string(), "Back-channel logout".to_string())],
            operation_id: "logoutOAuth".to_string(),
            method: "post".to_string(),
            path: "/oauth/backchannel-logout".to_string(),
            tags: vec!["Assets".to_string()],
            summary: Some("Back-channel OAuth logout".to_string()),
            params: vec![],
            body: Some(BodyDef {
                rust_type: "OAuthLogoutDTO".to_string(),
                kind: BodyKind::FormUrlEncoded,
            }),
            response_type: None,
            paginator: None,
            is_raw_response: false,
            deprecated: false,
        },
    ];

    let mut type_map = HashMap::new();
    type_map.insert(
        "AssetResponseDTO".to_string(),
        "immich_models::assets::AssetResponseDTO".to_string(),
    );
    type_map.insert(
        "OAuthLogoutDTO".to_string(),
        "immich_models::authentication::OAuthLogoutDTO".to_string(),
    );

    let tokens = emit_client_module("assets", &endpoints, &["faces".to_string()], &type_map);
    let file: syn::File = syn::parse2(tokens).expect("generated tokens must parse as valid Rust AST");
    let rendered = prettyplease::unparse(&file);
    insta::assert_snapshot!("client_ast_builder_and_subclient", rendered);
}
