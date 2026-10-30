#[cfg(all(feature = "cli-config", not(target_arch = "wasm32")))]
mod tests {
    use immich_rs::config::cli::{resolve_env_or_config_with, resolve_from_immich_cli_in, resolve_from_immich_go_in};
    use tempfile::tempdir;

    #[test]
    fn parse_immich_cli_yaml_auth() {
        let dir = tempdir().unwrap();
        let auth_file = dir.path().join("auth.yml");

        std::fs::write(
            &auth_file,
            r#"
instanceUrl: "https://photos.cli.example.com/api"
apiKey: "cli-secret-api-key"
"#,
        )
        .unwrap();

        let res = resolve_from_immich_cli_in(Some(dir.path()), None).unwrap();
        assert_eq!(res.0, "https://photos.cli.example.com/api");
        assert_eq!(res.1, "cli-secret-api-key");
    }

    #[test]
    fn parse_immich_go_toml_config() {
        let dir = tempdir().unwrap();
        let config_file = dir.path().join("immich-go.toml");

        std::fs::write(
            &config_file,
            r#"
server = "https://go.example.com/api"
api-key = "go-secret-token"
"#,
        )
        .unwrap();

        let res = resolve_from_immich_go_in(Some(dir.path()), None).unwrap();
        assert_eq!(res.0, "https://go.example.com/api");
        assert_eq!(res.1, "go-secret-token");
    }

    #[test]
    fn parse_immich_go_yaml_config() {
        let dir = tempdir().unwrap();
        let config_file = dir.path().join("immich-go.yaml");

        std::fs::write(
            &config_file,
            r#"
server: "https://go.yaml.example.com/api"
apiKey: "yaml-secret-token"
"#,
        )
        .unwrap();

        let res = resolve_from_immich_go_in(Some(dir.path()), None).unwrap();
        assert_eq!(res.0, "https://go.yaml.example.com/api");
        assert_eq!(res.1, "yaml-secret-token");
    }

    #[test]
    fn cascade_precedence_env_overrides_files() {
        let dir = tempdir().unwrap();
        let go_file = dir.path().join("immich-go.toml");
        std::fs::write(
            &go_file,
            r#"
server = "https://file.example.com/api"
api-key = "file-key"
"#,
        )
        .unwrap();

        // Environment lookup returns higher-priority values
        let env_lookup = |key: &str| match key {
            "IMMICH_INSTANCE_URL" => Some("https://env.example.com/api".to_string()),
            "IMMICH_API_KEY" => Some("env-key".to_string()),
            _ => None,
        };

        let res = resolve_env_or_config_with(env_lookup, None, Some(dir.path()), None).unwrap();

        assert_eq!(res.0, "https://env.example.com/api");
        assert_eq!(res.1, "env-key");
    }

    #[test]
    fn cascade_precedence_local_go_overrides_global_cli() {
        let dir = tempdir().unwrap();
        let local_go = dir.path().join("local");
        std::fs::create_dir_all(&local_go).unwrap();
        std::fs::write(
            local_go.join("immich-go.toml"),
            "server = \"https://local.example.com/api\"\napi-key = \"local-key\"",
        )
        .unwrap();

        let global_cli = dir.path().join("global_cli");
        std::fs::create_dir_all(&global_cli).unwrap();
        std::fs::write(
            global_cli.join("auth.yml"),
            "instanceUrl: \"https://global.example.com/api\"\napiKey: \"global-key\"",
        )
        .unwrap();

        let env_lookup = |_key: &str| None;

        let res = resolve_env_or_config_with(env_lookup, Some(&global_cli), Some(&local_go), None).unwrap();

        // Local immich-go should win over global CLI
        assert_eq!(res.0, "https://local.example.com/api");
        assert_eq!(res.1, "local-key");
    }

    #[test]
    fn cascade_reports_validation_error_when_all_missing() {
        let dir = tempdir().unwrap();
        let empty_dir = dir.path().join("empty");
        std::fs::create_dir_all(&empty_dir).unwrap();

        let env_lookup = |_key: &str| None;

        let res = resolve_env_or_config_with(env_lookup, Some(&empty_dir), Some(&empty_dir), Some(&empty_dir));

        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("no Immich configuration found"));
    }

    #[test]
    fn cli_config_trims_whitespace_from_url_and_keys() {
        let dir = tempdir().unwrap();
        let go_file = dir.path().join("immich-go.toml");
        std::fs::write(
            &go_file,
            "server = '  https://untrimmed.immich.app/api  '\napiKey = '  my-key  '\n",
        )
        .unwrap();

        let (server, key) = resolve_from_immich_go_in(Some(dir.path()), None).unwrap();
        assert_eq!(server, "https://untrimmed.immich.app/api");
        assert_eq!(key, "my-key");
    }

    #[test]
    fn cli_config_reports_error_on_corrupted_config_file() {
        let dir = tempdir().unwrap();
        let bad_file = dir.path().join("immich-go.toml");
        std::fs::write(&bad_file, "INVALID_TOML:::@@@{{{").unwrap();

        let res = resolve_from_immich_go_in(Some(dir.path()), None);
        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("failed to parse"));
    }
}
