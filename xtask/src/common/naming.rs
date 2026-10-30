//! Centralized Rust naming and keyword-escaping conventions (`C-CASE` via `heck`).

use heck::{ToSnakeCase, ToUpperCamelCase};

/// Converts an `OpenAPI` identifier (`camelCase`, `PascalCase`, `kebab-case`) to idiomatic `snake_case`.
#[must_use]
pub fn to_snake_case(input: &str) -> String {
    input.to_snake_case()
}

/// Converts a `snake_case` or `camelCase` identifier to `UpperCamelCase`, preserving `DTO`, `OAuth`, and `FFmpeg`.
#[must_use]
pub fn to_pascal_case(input: &str) -> String {
    let pascal = input.to_upper_camel_case();
    let pascal = pascal.replace("Oauth", "OAuth").replace("Ffmpeg", "FFmpeg");
    if let Some(stem) = pascal.strip_suffix("Dto") {
        format!("{stem}DTO")
    } else {
        pascal
    }
}

/// Escapes any Rust 2024 strict or reserved keyword by appending a trailing underscore (`type` -> `type_`),
/// matching `typify`'s field-naming convention.
#[must_use]
pub fn escape_rust_keyword(snaked: &str) -> String {
    match snaked {
        "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else" | "enum" | "extern"
        | "false" | "fn" | "for" | "gen" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut"
        | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super" | "trait" | "true" | "try"
        | "type" | "unsafe" | "use" | "where" | "while" | "abstract" | "become" | "box" | "do" | "final" | "macro"
        | "override" | "priv" | "typeof" | "unsized" | "virtual" | "yield" => format!("{snaked}_"),
        _ => snaked.to_string(),
    }
}

/// Applies centralized type naming conventions for `immich-rs`:
/// - `SystemConfigFooDto` -> `ConfigFooDTO` (preserving `SystemConfigDTO` itself)
/// - `AssetMediaResponseDto` -> `AssetMediaResponseDTO`
/// - Inline sub-types like `PeopleUpdateDtoItemsItem` -> `PeopleUpdateItemsItem`
#[must_use]
pub fn rename_type(name: &str) -> String {
    let mut rust_name = name.to_string();

    if rust_name.starts_with("SystemConfig") && rust_name != "SystemConfigDto" && rust_name != "SystemConfigDTO" {
        rust_name = rust_name.replacen("SystemConfig", "Config", 1);
    }

    if let Some(stem) = rust_name.strip_suffix("Dto").or_else(|| rust_name.strip_suffix("DTO")) {
        format!("{}DTO", stem.replace("Dto", "").replace("DTO", ""))
    } else {
        rust_name.replace("Dto", "").replace("DTO", "")
    }
}
