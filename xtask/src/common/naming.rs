/// Applies centralized naming conventions for the immich-rs project.
/// Examples:
///   `SystemConfigDto` -> `ConfigDTO`
///   `AssetDto` -> `AssetDTO`
#[must_use]
pub fn rename_type(name: &str) -> String {
    let mut rust_name = name.to_string();

    if rust_name.starts_with("SystemConfig") && rust_name != "SystemConfigDto" && rust_name != "SystemConfigDTO" {
        rust_name = rust_name.replace("SystemConfig", "Config");
    }

    if rust_name.contains("Dto") {
        if rust_name.ends_with("Dto") {
            rust_name = rust_name.replace("Dto", "DTO");
        } else {
            rust_name = rust_name.replace("Dto", "");
        }
    }

    rust_name
}
