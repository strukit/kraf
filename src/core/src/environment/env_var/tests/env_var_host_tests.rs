use crate::environment::env_var::env_var_core::{
    EnvVar, EnvVarHostMapping, EnvVarImportMode, EnvVarSource,
};
use crate::environment::env_var::env_var_host::{
    ENV_VAR_HOST_KEYS, env_var_collect, env_var_host_linux, env_var_host_mac, env_var_host_windows,
    env_var_process_apply,
};
use crate::environment::env_var::tests::env_var_test_fixtures::{
    ENV_VAR_MAPPING_FIXTURE, FakeProcess,
};

pub const LINUX_MAPPINGS: &[EnvVarHostMapping] = env_var_host_linux::HOST_ENV_VAR_MAPPINGS;
pub const MACOS_MAPPINGS: &[EnvVarHostMapping] = env_var_host_mac::HOST_ENV_VAR_MAPPINGS;
pub const WINDOWS_MAPPINGS: &[EnvVarHostMapping] = env_var_host_windows::HOST_ENV_VAR_MAPPINGS;

use crate::Process;
use std::collections::HashSet;

#[test]
fn collect_picks_up_a_declared_env_var() {
    let mut process = FakeProcess::default();
    process.set_env_var("ENV_VAR_TEST_COLLECT", "built-value");

    let mappings = [EnvVarHostMapping {
        key: "MY_VAR",
        source: EnvVarSource::Host,
        alias_import: &["ENV_VAR_TEST_COLLECT"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: None,
    }];

    let collected = env_var_collect(&mappings, &process);

    let found = collected
        .iter()
        .find(|var| var.key == "ENV_VAR_TEST_COLLECT");

    assert_eq!(
        found.and_then(|var| var.value.clone()),
        Some("built-value".to_string())
    );
}

#[test]
fn mappings_have_unique_keys() {
    fn assert_unique_env_var_mapping(mappings: &[EnvVarHostMapping]) {
        let mut seen = HashSet::new();

        for mapping in mappings {
            assert!(
                seen.insert(mapping.key),
                "key '{}' appears more than once in the mappings table",
                mapping.key
            );
        }
    }

    assert_unique_env_var_mapping(MACOS_MAPPINGS);
    assert_unique_env_var_mapping(LINUX_MAPPINGS);
    assert_unique_env_var_mapping(WINDOWS_MAPPINGS);
    assert_unique_env_var_mapping(ENV_VAR_MAPPING_FIXTURE);
}

#[test]
fn mappings_include_all_known_keys() {
    fn assert_includes_all_known_keys(mappings: &[EnvVarHostMapping], name: &str) {
        let mapping_keys: HashSet<&str> = mappings.iter().map(|mapping| mapping.key).collect();

        for known_key in ENV_VAR_HOST_KEYS {
            assert!(
                mapping_keys.contains(known_key),
                "{name} is missing known key '{known_key}' from env_var_host.rs (ENV_VAR_HOST_KEYS)"
            );
        }
    }

    assert_includes_all_known_keys(MACOS_MAPPINGS, "MACOS_MAPPINGS");
    assert_includes_all_known_keys(LINUX_MAPPINGS, "LINUX_MAPPINGS");
    assert_includes_all_known_keys(WINDOWS_MAPPINGS, "WINDOWS_MAPPINGS");
}

#[test]
fn mappings_merge_mode_has_multiple_aliases() {
    fn assert_merge_mode_has_multiple_aliases(mappings: &[EnvVarHostMapping]) {
        for mapping in mappings {
            if mapping.alias_import_mode == EnvVarImportMode::Merge {
                assert!(
                    mapping.alias_import.len() > 1,
                    "key '{}' uses EnvVarImportMode::Merge but has {} alias_import (needs more than 1)",
                    mapping.key,
                    mapping.alias_import.len()
                );
            }
        }
    }

    assert_merge_mode_has_multiple_aliases(MACOS_MAPPINGS);
    assert_merge_mode_has_multiple_aliases(LINUX_MAPPINGS);
    assert_merge_mode_has_multiple_aliases(WINDOWS_MAPPINGS);
    assert_merge_mode_has_multiple_aliases(ENV_VAR_MAPPING_FIXTURE);
}

#[test]
fn apply_writes_empty_string_for_entries_with_no_value() {
    let mappings = [EnvVarHostMapping {
        key: "HOME_TEMP",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["ENV_VAR_TEST_APPLY_EMPTY"],
        fallback: None,
    }];
    let env_vars = [EnvVar {
        key: "ENV_VAR_TEST_APPLY_EMPTY",
        value: None,
    }];

    let mut process = FakeProcess::default();
    env_var_process_apply(&mappings, &env_vars, &mut process);

    assert_eq!(
        process.read_env_var("ENV_VAR_TEST_APPLY_EMPTY"),
        Some(String::new())
    );
}

#[test]
fn apply_writes_only_what_is_declared_exportable() {
    let mappings = [EnvVarHostMapping {
        key: "ENV_VAR_TEST_APPLY_CANONICAL",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["ENV_VAR_TEST_APPLY_EXPORTABLE"],
        fallback: None,
    }];
    let env_vars = [
        EnvVar {
            key: "ENV_VAR_TEST_APPLY_CANONICAL",
            value: Some("/tmp".to_string()),
        },
        EnvVar {
            key: "ENV_VAR_TEST_APPLY_EXPORTABLE",
            value: Some("/tmp".to_string()),
        },
    ];

    let mut process = FakeProcess::default();
    env_var_process_apply(&mappings, &env_vars, &mut process);

    assert_eq!(process.read_env_var("ENV_VAR_TEST_APPLY_CANONICAL"), None);
    assert_eq!(
        process.read_env_var("ENV_VAR_TEST_APPLY_EXPORTABLE"),
        Some("/tmp".to_string())
    );
}

#[test]
fn host_source_mappings_export_every_native_imported_alias() {
    fn assert_exports_every_native_import(mappings: &[EnvVarHostMapping], name: &str) {
        for mapping in mappings {
            if mapping.source != EnvVarSource::Host {
                continue;
            }

            for imported in mapping.alias_import {
                if *imported == mapping.key {
                    continue;
                }

                assert!(
                    mapping.alias_export.contains(imported),
                    "{name}: mapping '{}' imports '{imported}' from the host but never exports it",
                    mapping.key
                );
            }
        }
    }

    assert_exports_every_native_import(MACOS_MAPPINGS, "MACOS_MAPPINGS");
    assert_exports_every_native_import(LINUX_MAPPINGS, "LINUX_MAPPINGS");
    assert_exports_every_native_import(WINDOWS_MAPPINGS, "WINDOWS_MAPPINGS");
}

#[test]
fn home_and_workdir_family_are_always_provided() {
    const ISOLATED_KEYS: &[&str] = &[
        "WORKDIR",
        "HOME",
        "HOME_TEMP",
        "HOME_CONFIG",
        "HOME_DATA",
        "HOME_DATA_BIN",
        "HOME_CACHE",
        "HOME_STATE",
        "HOME_SESSION",
    ];

    fn assert_isolated_keys_are_provided(mappings: &[EnvVarHostMapping], name: &str) {
        for mapping in mappings {
            if !ISOLATED_KEYS.contains(&mapping.key) {
                continue;
            }

            assert_eq!(
                mapping.source,
                EnvVarSource::Provided,
                "{name}: mapping '{}' is part of the isolated HOME/WORKDIR family but is source: Host",
                mapping.key
            );
        }
    }

    assert_isolated_keys_are_provided(MACOS_MAPPINGS, "MACOS_MAPPINGS");
    assert_isolated_keys_are_provided(LINUX_MAPPINGS, "LINUX_MAPPINGS");
    assert_isolated_keys_are_provided(WINDOWS_MAPPINGS, "WINDOWS_MAPPINGS");
}

#[test]
fn pass_through_mappings_only_forward_their_own_key() {
    fn assert_pass_through_shape(mappings: &[EnvVarHostMapping], name: &str) {
        for mapping in mappings {
            if mapping.alias_import_mode != EnvVarImportMode::PassThrough {
                continue;
            }

            assert_eq!(
                mapping.alias_import,
                &[mapping.key],
                "{name}: PassThrough mapping '{}' must import exactly its own key",
                mapping.key
            );
            assert!(
                mapping.fallback.is_none(),
                "{name}: PassThrough mapping '{}' must not have a fallback",
                mapping.key
            );
        }
    }

    assert_pass_through_shape(MACOS_MAPPINGS, "MACOS_MAPPINGS");
    assert_pass_through_shape(LINUX_MAPPINGS, "LINUX_MAPPINGS");
    assert_pass_through_shape(WINDOWS_MAPPINGS, "WINDOWS_MAPPINGS");
}
