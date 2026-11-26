use crate::environment::env_var::env_var_core::{
    EnvVarHostMapping, EnvVarImportMode, EnvVarSource,
};
use crate::environment::env_var::env_var_host::{
    env_var_host_linux, env_var_host_mac, env_var_host_windows,
};
use crate::environment::env_var::env_var_keys::ENV_VAR_KEYS;

pub const LINUX_MAPPINGS: &[EnvVarHostMapping] = env_var_host_linux::HOST_ENV_VAR_MAPPINGS;
pub const MACOS_MAPPINGS: &[EnvVarHostMapping] = env_var_host_mac::HOST_ENV_VAR_MAPPINGS;
pub const WINDOWS_MAPPINGS: &[EnvVarHostMapping] = env_var_host_windows::HOST_ENV_VAR_MAPPINGS;

use std::collections::HashSet;

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
}

#[test]
fn mappings_include_all_known_keys() {
    fn assert_includes_all_known_keys(mappings: &[EnvVarHostMapping], name: &str) {
        let mapping_keys: HashSet<&str> = mappings.iter().map(|mapping| mapping.key).collect();

        for known_key in ENV_VAR_KEYS {
            assert!(
                mapping_keys.contains(known_key),
                "{name} is missing known key '{known_key}' from env_var_keys.rs (ENV_VAR_KEYS)"
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
}

#[test]
fn host_source_mappings_export_every_native_imported_alias() {
    fn assert_exports_every_native_import(mappings: &[EnvVarHostMapping], name: &str) {
        for mapping in mappings {
            if mapping.source != EnvVarSource::Host || mapping.alias_export.is_empty() {
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
        "HOST_WORKDIR",
        "HOME_DIR",
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
                "{name}: mapping '{}' is part of the isolated HOME_DIR/HOST_WORKDIR family but is source: Host",
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
            assert_eq!(
                mapping.alias_export,
                &[mapping.key],
                "{name}: PassThrough mapping '{}' must export exactly its own key",
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

#[test]
fn derived_canonical_keys_are_not_exported() {
    fn is_derived(key: &str) -> bool {
        key.starts_with("PLATFORM")
            || key.starts_with("TERMINAL")
            || key.starts_with("USER_")
            || key.starts_with("HOME_")
    }

    fn assert_derived_keys_are_not_exported(mappings: &[EnvVarHostMapping], name: &str) {
        for mapping in mappings {
            for alias_export in mapping.alias_export {
                assert!(
                    !is_derived(alias_export),
                    "{name}: mapping '{}' exports the derived canonical key '{alias_export}'",
                    mapping.key
                );
            }
        }
    }

    assert_derived_keys_are_not_exported(MACOS_MAPPINGS, "MACOS_MAPPINGS");
    assert_derived_keys_are_not_exported(LINUX_MAPPINGS, "LINUX_MAPPINGS");
    assert_derived_keys_are_not_exported(WINDOWS_MAPPINGS, "WINDOWS_MAPPINGS");
}

#[test]
fn canonical_keys_are_never_exported() {
    fn assert_canonical_keys_are_not_exported(mappings: &[EnvVarHostMapping], name: &str) {
        let canonical_keys: HashSet<&str> = mappings.iter().map(|mapping| mapping.key).collect();

        for mapping in mappings {
            if mapping.alias_import_mode == EnvVarImportMode::PassThrough {
                continue;
            }

            for alias_export in mapping.alias_export {
                assert!(
                    !canonical_keys.contains(alias_export),
                    "{name}: mapping '{}' exports the canonical key '{alias_export}'",
                    mapping.key
                );
            }
        }
    }

    assert_canonical_keys_are_not_exported(MACOS_MAPPINGS, "MACOS_MAPPINGS");
    assert_canonical_keys_are_not_exported(LINUX_MAPPINGS, "LINUX_MAPPINGS");
    assert_canonical_keys_are_not_exported(WINDOWS_MAPPINGS, "WINDOWS_MAPPINGS");
}

#[test]
fn host_data_and_config_have_no_fallback() {
    fn assert_no_fallback(mappings: &[EnvVarHostMapping], name: &str) {
        for mapping in mappings {
            if mapping.key != "HOST_DATA" && mapping.key != "HOST_CONFIG" {
                continue;
            }

            assert!(
                mapping.fallback.is_none(),
                "{name}: mapping '{}' declares a fallback",
                mapping.key
            );
        }
    }

    assert_no_fallback(MACOS_MAPPINGS, "MACOS_MAPPINGS");
    assert_no_fallback(LINUX_MAPPINGS, "LINUX_MAPPINGS");
    assert_no_fallback(WINDOWS_MAPPINGS, "WINDOWS_MAPPINGS");
}

#[test]
fn home_path_does_not_import_the_host_path() {
    fn assert_home_path_is_isolated(mappings: &[EnvVarHostMapping], name: &str) {
        let home_path = mappings
            .iter()
            .find(|mapping| mapping.key == "HOME_PATH")
            .unwrap_or_else(|| panic!("{name}: missing HOME_PATH mapping"));

        assert!(
            home_path.alias_import.is_empty() && home_path.fallback.is_some(),
            "{name}: HOME_PATH imports from the host or has no fallback"
        );
    }

    assert_home_path_is_isolated(MACOS_MAPPINGS, "MACOS_MAPPINGS");
    assert_home_path_is_isolated(LINUX_MAPPINGS, "LINUX_MAPPINGS");
    assert_home_path_is_isolated(WINDOWS_MAPPINGS, "WINDOWS_MAPPINGS");
}

#[test]
fn windows_system_variables_pass_through_only_on_windows() {
    const WINDOWS_SYSTEM_KEYS: &[&str] = &["SystemRoot", "windir", "SystemDrive"];

    for key in WINDOWS_SYSTEM_KEYS {
        let mapping = WINDOWS_MAPPINGS
            .iter()
            .find(|mapping| mapping.key == *key)
            .unwrap_or_else(|| panic!("WINDOWS_MAPPINGS: missing mapping '{key}'"));

        assert_eq!(
            mapping.alias_import_mode,
            EnvVarImportMode::PassThrough,
            "WINDOWS_MAPPINGS: mapping '{key}' does not pass through"
        );
    }

    for mapping in MACOS_MAPPINGS.iter().chain(LINUX_MAPPINGS) {
        assert!(
            !WINDOWS_SYSTEM_KEYS.contains(&mapping.key),
            "mapping '{}' is declared outside the Windows mapping",
            mapping.key
        );
    }
}

#[test]
fn pass_through_keys_belong_to_one_platform() {
    let platforms = [MACOS_MAPPINGS, LINUX_MAPPINGS, WINDOWS_MAPPINGS];

    for mappings in platforms {
        for mapping in mappings {
            if mapping.alias_import_mode != EnvVarImportMode::PassThrough {
                continue;
            }

            let declared_in = platforms
                .iter()
                .filter(|other| other.iter().any(|other| other.key == mapping.key))
                .count();

            assert_eq!(
                declared_in, 1,
                "PassThrough mapping '{}' is declared by more than one platform",
                mapping.key
            );
        }
    }
}

#[test]
fn host_home_is_imported_from_the_host_and_not_exported() {
    fn assert_host_home(mappings: &[EnvVarHostMapping], native_home: &str, name: &str) {
        let host_home = mappings
            .iter()
            .find(|mapping| mapping.key == "HOST_HOME")
            .unwrap_or_else(|| panic!("{name}: missing HOST_HOME mapping"));

        assert_eq!(host_home.source, EnvVarSource::Host, "{name}");
        assert_eq!(host_home.alias_import, &[native_home], "{name}");
        assert!(host_home.alias_export.is_empty(), "{name}");
    }

    assert_host_home(MACOS_MAPPINGS, "HOME", "MACOS_MAPPINGS");
    assert_host_home(LINUX_MAPPINGS, "HOME", "LINUX_MAPPINGS");
    assert_host_home(WINDOWS_MAPPINGS, "USERPROFILE", "WINDOWS_MAPPINGS");
}
