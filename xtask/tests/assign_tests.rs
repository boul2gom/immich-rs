use std::collections::{HashMap, HashSet};

use pretty_assertions::assert_eq;
use xtask::common::assign::{RuleTracker, assign_by_name, pick_primary_tag};
use xtask::common::naming::rename_type;
use xtask::common::{camel_to_snake, snake_to_camel};
use xtask::models::ast::{
    strip_json_schema, transform_add_constructors_and_setters, transform_forward_compatible_enums, transform_typed_ids,
};

#[test]
fn naming_conventions_preserve_uppercase_dto_and_snake_case_oauth() {
    assert_eq!(rename_type("AlbumResponseDto"), "AlbumResponseDTO");
    assert_eq!(rename_type("AssetMediaResponseDto"), "AssetMediaResponseDTO");
    assert_eq!(rename_type("SystemConfigFFmpegDto"), "ConfigFFmpegDTO");
    assert_eq!(rename_type("SystemConfigOAuthDto"), "ConfigOAuthDTO");
    assert_eq!(rename_type("SystemConfigDto"), "SystemConfigDTO");
    assert_eq!(rename_type("OAuthConfigDto"), "OAuthConfigDTO");
    assert_eq!(camel_to_snake("fileCreatedAt"), "file_created_at");
    assert_eq!(camel_to_snake("oAuth"), "o_auth");
    assert_eq!(camel_to_snake("unlinkOAuthAccount"), "unlink_o_auth_account");
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
    schema_tag_map.insert(
        "IntegrityReportDto".to_string(),
        HashSet::from(["Maintenance (admin)".to_string()]),
    );

    let mut rename_map: HashMap<String, String> = HashMap::new();
    rename_map.insert("CustomAlbumPayloadDTO".to_string(), "CustomAlbumPayloadDto".to_string());
    rename_map.insert("OldLegacyDTO".to_string(), "OldLegacyDto".to_string());
    rename_map.insert("IntegrityReportDTO".to_string(), "IntegrityReportDto".to_string());

    let mut tracker = RuleTracker::new();

    assert_eq!(
        assign_by_name("ConfigFFmpegDTO", &schema_tag_map, &rename_map, &mut tracker),
        "system/config/ffmpeg"
    );
    assert_eq!(
        assign_by_name("CustomAlbumPayloadDTO", &schema_tag_map, &rename_map, &mut tracker),
        "albums"
    );
    assert_eq!(
        assign_by_name("IntegrityReportDTO", &schema_tag_map, &rename_map, &mut tracker),
        "admin/maintenance"
    );
    assert_eq!(
        assign_by_name("OldLegacyDTO", &schema_tag_map, &rename_map, &mut tracker),
        "deprecated"
    );
    assert_eq!(
        assign_by_name("TimeBucketAsset", &schema_tag_map, &rename_map, &mut tracker),
        "timeline"
    );
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
fn ast_transforms_generate_lossless_unknown_enums_typed_ids_and_dto_builders() {
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
        pub struct CreateAlbumDTO {
            #[serde(rename = "albumName")]
            pub album_name: String,
            #[serde(rename = "assetIds", default)]
            pub asset_ids: Vec<uuid::Uuid>,
            pub description: Option<String>,
        }
    };

    transform_typed_ids(&mut file);
    transform_forward_compatible_enums(&mut file);
    transform_add_constructors_and_setters(&mut file);

    let formatted = prettyplease::unparse(&file);
    let compact: String = formatted.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(compact.contains("#[non_exhaustive]"));
    assert!(compact.contains("Unknown(String)"));
    assert!(compact.contains("Self::Unknown(s) => s.as_str()"));
    assert!(compact.contains("pub asset_ids: Vec<AssetId>"));
    assert!(compact.contains("pub fn new(album_name: impl Into<String>) -> Self"));
    assert!(
        compact.contains("pub fn asset_ids( mut self, value: impl IntoIterator<Item = impl Into<AssetId>>, ) -> Self")
            || compact
                .contains("pub fn asset_ids(mut self, value: impl IntoIterator<Item = impl Into<AssetId>>) -> Self"),
        "got:\n{formatted}"
    );
    assert!(compact.contains("pub fn set_description(mut self, value: Option<String>) -> Self"));
}
