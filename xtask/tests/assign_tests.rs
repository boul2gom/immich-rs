use std::collections::{HashMap, HashSet};

use pretty_assertions::assert_eq;
use xtask::common::assign::{RuleTracker, assign_by_name, pick_primary_tag};
use xtask::common::naming::rename_type;
use xtask::common::{camel_to_snake, snake_to_camel};
use xtask::models::ast::{
    strip_json_schema, transform_add_constructors_and_setters, transform_forward_compatible_enums,
};

#[test]
fn naming_conventions_convert_dto_and_system_config() {
    assert_eq!(rename_type("AlbumResponseDto"), "AlbumResponseDTO");
    assert_eq!(rename_type("SystemConfigFFmpegDto"), "ConfigFFmpegDTO");
    assert_eq!(rename_type("SystemConfigDto"), "SystemConfigDTO");
    assert_eq!(camel_to_snake("fileCreatedAt"), "file_created_at");
    assert_eq!(camel_to_snake("oAuth"), "o_auth");
    assert_eq!(snake_to_camel("system_config"), "SystemConfig");
}

#[test]
fn assign_by_name_routes_sysconfig_tags_and_prefixes() {
    let mut schema_tag_map: HashMap<String, HashSet<String>> = HashMap::new();
    schema_tag_map.insert(
        "CustomAlbumPayloadDto".to_string(),
        HashSet::from(["Albums".to_string()]),
    );
    schema_tag_map.insert("OldLegacyDto".to_string(), HashSet::from(["Deprecated".to_string()]));

    let mut rename_map: HashMap<String, String> = HashMap::new();
    rename_map.insert("CustomAlbumPayloadDTO".to_string(), "CustomAlbumPayloadDto".to_string());
    rename_map.insert("OldLegacyDTO".to_string(), "OldLegacyDto".to_string());

    let mut tracker = RuleTracker::new();

    // 1. System config sub-module routing
    assert_eq!(
        assign_by_name("ConfigFFmpegDTO", &schema_tag_map, &rename_map, &mut tracker),
        "system/config/ffmpeg"
    );

    // 2. Tag-based routing
    assert_eq!(
        assign_by_name("CustomAlbumPayloadDTO", &schema_tag_map, &rename_map, &mut tracker),
        "albums"
    );

    // 3. Deprecated tag routing
    assert_eq!(
        assign_by_name("OldLegacyDTO", &schema_tag_map, &rename_map, &mut tracker),
        "deprecated"
    );

    // 4. Prefix fallback routing
    assert_eq!(
        assign_by_name("TimeBucketAsset", &schema_tag_map, &rename_map, &mut tracker),
        "timeline"
    );

    // 5. Unknown schema safe fallback to "common"
    assert_eq!(
        assign_by_name("FutureBrandNewSchema", &schema_tag_map, &rename_map, &mut tracker),
        "common"
    );
}

#[test]
fn pick_primary_tag_prefers_non_deprecated_alphabetically() {
    let tags = HashSet::from([
        "Deprecated".to_string(),
        "Views".to_string(),
        "Users".to_string(),
        "Albums".to_string(),
    ]);
    assert_eq!(pick_primary_tag(&tags), Some("Albums"));

    let only_deprecated = HashSet::from(["Deprecated".to_string()]);
    assert_eq!(pick_primary_tag(&only_deprecated), Some("Deprecated"));
}

#[test]
fn strip_json_schema_removes_single_and_multiline_details() {
    let mut item: syn::ItemStruct = syn::parse_quote! {
        /// Useful description
        /// <details><summary>JSON schema</summary>{ "type": "string" }</details>
        /// <details><summary>JSON schema</summary>
        /// { "type": "object" }
        /// </details>
        /// Footer doc
        pub struct Sample;
    };

    strip_json_schema(&mut item.attrs);
    assert_eq!(item.attrs.len(), 2);
}

#[test]
fn ast_transforms_generate_forward_compatible_enums_and_dto_builders() {
    let mut file: syn::File = syn::parse_quote! {
        #[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, PartialEq, Eq)]
        pub enum SampleKind {
            #[serde(rename = "alpha")]
            Alpha,
        }

        impl std::fmt::Display for SampleKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    Self::Alpha => f.write_str("alpha"),
                }
            }
        }

        #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
        pub struct SampleDTO {
            pub name: String,
            pub count: i64,
            pub note: Option<String>,
            #[serde(default)]
            pub tags: Vec<String>,
        }
    };

    transform_forward_compatible_enums(&mut file);
    transform_add_constructors_and_setters(&mut file);

    let formatted = prettyplease::unparse(&file);
    assert!(formatted.contains("#[non_exhaustive]"));
    assert!(formatted.contains("UnknownValue"));
    assert!(formatted.contains("Self::UnknownValue => f.write_str(\"unknown\")"));
    assert!(formatted.contains("pub fn new(name: impl Into<String>, count: i64) -> Self"));
    assert!(formatted.contains("pub fn note(mut self, value: impl Into<String>) -> Self"));
    assert!(formatted.contains("pub fn set_note(mut self, value: Option<String>) -> Self"));
}
