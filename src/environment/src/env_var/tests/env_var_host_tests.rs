use super::env_var_test_fixtures::ENV_VAR_MAPPING_FIXTURE;
use crate::env_var::env_var_core::{EnvVar, EnvVarHostMapping, EnvVarImportMode, EnvVarSource};
use crate::env_var::env_var_host::{ENV_VAR_FROM_HOST, env_var_collect, env_var_process_apply};
use crate::env_var::env_var_host_linux::LINUX_MAPPINGS;
use crate::env_var::env_var_host_macos::MACOS_MAPPINGS;
use crate::env_var::env_var_host_windows::WINDOWS_MAPPINGS;

use kraf_core::{HostProcess, Process};
use std::collections::HashSet;

#[test]
fn collect_picks_up_a_real_env_var_the_mapping_declares() {
    // SAFETY: test-only and single-threaded
    unsafe {
        std::env::set_var("KRAF_COLLECT_TEST", "built-value");
    }

    let mappings = [EnvVarHostMapping {
        key: "MY_VAR",
        source: EnvVarSource::Host,
        alias_import: &["KRAF_COLLECT_TEST"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: None,
    }];

    let collected = env_var_collect(&mappings, &HostProcess);

    let found = collected.iter().find(|var| var.key == "KRAF_COLLECT_TEST");

    assert_eq!(
        found.and_then(|var| var.value.clone()),
        Some("built-value".to_string())
    );

    unsafe {
        std::env::remove_var("KRAF_COLLECT_TEST");
    }
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

        for known_key in ENV_VAR_FROM_HOST {
            assert!(
                mapping_keys.contains(known_key),
                "{name} is missing known key '{known_key}' from env_var_host.rs (ENV_VAR_FROM_HOST)"
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
        alias_export: &["KRAF_TEST_APPLY_EMPTY"],
        fallback: None,
    }];
    let env_vars = [EnvVar {
        key: "KRAF_TEST_APPLY_EMPTY",
        value: None,
    }];

    env_var_process_apply(&mappings, &env_vars, &mut HostProcess);

    assert_eq!(
        HostProcess.read_env_var("KRAF_TEST_APPLY_EMPTY"),
        Some(String::new())
    );

    // SAFETY: test-only and single-threaded
    unsafe {
        std::env::remove_var("KRAF_TEST_APPLY_EMPTY");
    }
}

#[test]
fn apply_writes_only_what_is_declared_exportable() {
    let mappings = [EnvVarHostMapping {
        key: "HOME_TEMP",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["KRAF_TEST_APPLY_EXPORTABLE"],
        fallback: None,
    }];
    let env_vars = [
        EnvVar {
            key: "HOME_TEMP",
            value: Some("/tmp".to_string()),
        },
        EnvVar {
            key: "KRAF_TEST_APPLY_EXPORTABLE",
            value: Some("/tmp".to_string()),
        },
    ];

    env_var_process_apply(&mappings, &env_vars, &mut HostProcess);

    assert_eq!(HostProcess.read_env_var("HOME_TEMP"), None);
    assert_eq!(
        HostProcess.read_env_var("KRAF_TEST_APPLY_EXPORTABLE"),
        Some("/tmp".to_string())
    );

    // SAFETY: test-only and single-threaded
    unsafe {
        std::env::remove_var("KRAF_TEST_APPLY_EXPORTABLE");
    }
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
