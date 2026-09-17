use crate::env_var::env_var_core::{EnvVar, EnvVarHostMapping, EnvVarImportMode, EnvVarSource};

use super::env_var_test_fixtures::{ENV_VAR_MAPPING_FIXTURE, env_var_collected_fixture};
use crate::env_var::env_var_resolver::{
    env_var_resolve_keys, expand_dollar_var, expand_tilde_var, lookup_env_var_value, resolve,
    resolve_alias,
};

#[test]
fn resolve_keys_are_empty_when_no_mappings_declare_any() {
    let keys = env_var_resolve_keys(&[]);

    assert!(keys.env_vars_keys_import.is_empty());
    assert!(keys.env_vars_keys_export.is_empty());
}

#[test]
fn resolve_keys_collects_alias_import_and_dollar_var_in_fallback() {
    let mappings = [EnvVarHostMapping {
        key: "CONFIG_DIR",
        source: EnvVarSource::Provided,
        alias_import: &["XDG_CONFIG_HOME", "MY_CONFIG_DIR"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["CONFIG_DIR", "MY_CONFIG_DIR_EXPORT"],
        fallback: Some("$FOO_BASE/config"),
    }];

    let keys = env_var_resolve_keys(&mappings);

    assert_eq!(
        keys.env_vars_keys_import,
        vec!["FOO_BASE", "MY_CONFIG_DIR", "XDG_CONFIG_HOME"]
    );
    assert_eq!(
        keys.env_vars_keys_export,
        vec!["CONFIG_DIR", "MY_CONFIG_DIR_EXPORT"]
    );
}

#[test]
fn resolve_keys_deduplicates_repeated_names() {
    let mappings = [
        EnvVarHostMapping {
            key: "A",
            source: EnvVarSource::Host,
            alias_import: &["SHELL"],
            alias_import_mode: EnvVarImportMode::FirstFound,
            alias_export: &[],
            fallback: None,
        },
        EnvVarHostMapping {
            key: "B",
            source: EnvVarSource::Host,
            alias_import: &["SHELL"],
            alias_import_mode: EnvVarImportMode::FirstFound,
            alias_export: &[],
            fallback: Some("$SHELL/fallback"),
        },
    ];

    let keys = env_var_resolve_keys(&mappings);

    assert_eq!(keys.env_vars_keys_import, vec!["SHELL"]);
}

#[test]
fn expands_leading_tilde_with_preresolved_home() {
    let result = expand_tilde_var("~/.tmp", "/workspace");

    assert_eq!(result, "/workspace/.tmp");
}

#[test]
fn expands_dollar_var_reference() {
    let env_var_collected = env_var_collected_fixture("/workspace");

    let result = expand_dollar_var("$HOME/.cache", &env_var_collected);

    assert_eq!(result, "/workspace/.cache");
}

#[test]
fn expand_dollar_var_keeps_value_when_var_is_missing() {
    let result = expand_dollar_var("$HOME/.cache", &[]);

    assert_eq!(result, "$HOME/.cache");
}

#[test]
fn expand_dollar_var_keeps_value_when_dollar_has_no_name_after() {
    let result = expand_dollar_var("$", &[]);

    assert_eq!(result, "$");
}

#[test]
fn expand_dollar_var_keeps_value_when_dollar_is_followed_by_non_alnum() {
    let result = expand_dollar_var("$/foo", &[]);

    assert_eq!(result, "$/foo");
}

#[test]
fn resolves_to_fallback_when_no_alias_import() {
    let env_var_mapping = EnvVarHostMapping {
        key: "TEST_VAR",
        source: EnvVarSource::Host,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("test-fallback"),
    };

    let result = resolve_alias(&env_var_mapping, &[]);

    assert_eq!(result, "test-fallback");
}

#[test]
fn resolve_alias_expands_tilde_in_fallback() {
    let env_var_mapping = EnvVarHostMapping {
        key: "TMP_DIR",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("~/.tmp"),
    };
    let env_var_collected = env_var_collected_fixture("/workspace");

    let result = resolve_alias(&env_var_mapping, &env_var_collected);

    assert_eq!(result, "/workspace/.tmp");
}

#[test]
fn resolve_alias_expands_dollar_home_in_fallback() {
    let env_var_mapping = EnvVarHostMapping {
        key: "CACHE_DIR",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("$HOME/.cache"),
    };
    let env_var_collected = env_var_collected_fixture("/workspace");

    let result = resolve_alias(&env_var_mapping, &env_var_collected);

    assert_eq!(result, "/workspace/.cache");
}

#[test]
fn resolve_alias_keeps_tilde_when_home_is_missing() {
    let env_var_mapping = EnvVarHostMapping {
        key: "TMP_DIR",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("~/.tmp"),
    };

    let result = resolve_alias(&env_var_mapping, &[]);

    assert_eq!(result, "~/.tmp");
}

#[test]
fn lookup_env_var_value_finds_value_by_key() {
    let env_var_collected = vec![EnvVar {
        key: "SHELL",
        value: Some("/bin/zsh".to_string()),
    }];

    let result = lookup_env_var_value("SHELL", &env_var_collected);

    assert_eq!(result, Some("/bin/zsh".to_string()));
}

#[test]
fn lookup_env_var_value_returns_none_when_missing() {
    let env_var_collected: Vec<EnvVar> = vec![];

    let result = lookup_env_var_value("SHELL", &env_var_collected);

    assert_eq!(result, None);
}

#[test]
fn resolve_emits_an_entry_for_key_and_for_every_alias_export() {
    let mapping = EnvVarHostMapping {
        key: "HOME_TEMP",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["TEMP", "TMPDIR", "TMP"],
        fallback: Some("/tmp"),
    };

    let result = resolve(&[mapping], &[]);

    let keys: Vec<&str> = result.iter().map(|env_var| env_var.key).collect();
    let values: Vec<Option<String>> = result.iter().map(|env_var| env_var.value.clone()).collect();

    assert_eq!(keys, vec!["HOME_TEMP", "TEMP", "TMPDIR", "TMP"]);
    assert_eq!(
        values,
        vec![
            Some("/tmp".to_string()),
            Some("/tmp".to_string()),
            Some("/tmp".to_string()),
            Some("/tmp".to_string())
        ]
    );
}

#[test]
fn resolve_handles_every_mapping_case_at_once() {
    let env_var_collected = env_var_collected_fixture("/workspace");
    let result = resolve(ENV_VAR_MAPPING_FIXTURE, &env_var_collected);

    #[cfg(target_os = "windows")]
    let path_expected = "/workspace/.local/bin;/usr/bin:/bin";

    #[cfg(not(target_os = "windows"))]
    let path_expected = "/workspace/.local/bin:/usr/bin:/bin";

    let expected: &[(&str, &str)] = &[
        ("EDITOR", "nano"),
        ("USER_LANGUAGE", "pt_BR.UTF-8"),
        ("TERMINAL_TYPE", "xterm-256color"),
        ("TMP_DIR", "/workspace/.tmp"),
        ("CACHE_DIR", "/workspace/.cache"),
        ("PATH", path_expected),
        ("LD_LIBRARY_PATH", "/usr/local/lib"),
        ("MANPATH", "/usr/share/man"),
        ("WORKDIR", "/workspace/project"),
        ("HOME_TEMP", "/custom/tmp"),
        ("PLATFORM_KERNEL", "Darwin"),
        ("PLATFORM_VERSION", ""),
    ];

    assert_eq!(result.len(), ENV_VAR_MAPPING_FIXTURE.len());

    for (key, expected_value) in expected {
        let resolved = result.iter().find(|r| r.key == *key).unwrap();

        assert_eq!(
            resolved.value,
            Some(expected_value.to_string()),
            "mismatch for key {key}"
        );
    }
}

#[test]
fn resolve_alias_merges_multiple_paths_with_colon_separator() {
    let env_var_mapping = EnvVarHostMapping {
        key: "PATH",
        source: EnvVarSource::Provided,
        alias_import: &["BIN_A", "BIN_B"],
        alias_import_mode: EnvVarImportMode::Merge,
        alias_export: &[],
        fallback: None,
    };

    let env_var_collected = vec![
        EnvVar {
            key: "BIN_A",
            value: Some("/custom/bin".to_string()),
        },
        EnvVar {
            key: "BIN_B",
            value: Some("/usr/bin".to_string()),
        },
    ];

    let result = resolve_alias(&env_var_mapping, &env_var_collected);

    #[cfg(target_os = "windows")]
    let expected = "/custom/bin;/usr/bin".to_string();

    #[cfg(not(target_os = "windows"))]
    let expected = "/custom/bin:/usr/bin".to_string();

    assert_eq!(result, expected);
}
