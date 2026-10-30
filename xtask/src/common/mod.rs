pub mod assign;
pub mod ast_utils;
pub mod doc;
pub mod fmt;
pub mod fs_tree;
pub mod id_mapping;
pub mod naming;
pub mod spec;
pub mod tables;

/// Converts an `OpenAPI` identifier to `snake_case`.
#[must_use]
pub fn camel_to_snake(camel: &str) -> String {
    naming::to_snake_case(camel)
}

/// Converts a `snake_case` or `camelCase` identifier to `UpperCamelCase`.
#[must_use]
pub fn snake_to_camel(snake: &str) -> String {
    naming::to_pascal_case(snake)
}
