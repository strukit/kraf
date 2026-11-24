use crate::environment::env_var::env_var_collector::{EnvVarReadSet, collect};
use crate::environment::env_var::env_var_core::{
    EnvVarHostMapping, EnvVarImportMode, EnvVarReader, EnvVarSource,
};

use std::collections::HashMap;

/// In-memory reader whose environment is isolated from the real one of the
/// test process, so that tests neither read nor change it.
#[derive(Default)]
struct FakeEnvVarReader {
    env: HashMap<String, String>,
}

impl EnvVarReader for FakeEnvVarReader {
    fn read(&self, key: &str) -> Option<String> {
        self.env.get(key).cloned()
    }
}

#[test]
fn collect_picks_up_a_declared_env_var() {
    let mut reader = FakeEnvVarReader::default();
    reader.env.insert(
        "ENV_VAR_TEST_COLLECT".to_string(),
        "built-value".to_string(),
    );

    let mappings = [EnvVarHostMapping {
        key: "MY_VAR",
        source: EnvVarSource::Host,
        alias_import: &["ENV_VAR_TEST_COLLECT"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: None,
    }];

    let collected = collect(&EnvVarReadSet::new(&mappings), &reader);

    let found = collected
        .iter()
        .find(|var| var.key == "ENV_VAR_TEST_COLLECT");

    assert_eq!(
        found.and_then(|var| var.value.clone()),
        Some("built-value".to_string())
    );
}

#[test]
fn read_set_is_empty_when_no_mappings_declare_any() {
    let keys = EnvVarReadSet::new(&[]);

    assert_eq!(keys.keys(), Vec::<&str>::new().as_slice());
}

#[test]
fn read_set_holds_alias_import_but_not_dollar_var_in_fallback() {
    let mappings = [EnvVarHostMapping {
        key: "CONFIG_DIR",
        source: EnvVarSource::Provided,
        alias_import: &["XDG_CONFIG_HOME", "MY_CONFIG_DIR"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["CONFIG_DIR", "MY_CONFIG_DIR_EXPORT"],
        fallback: Some("$FOO_BASE/config"),
    }];

    let keys = EnvVarReadSet::new(&mappings);

    assert_eq!(keys.keys(), ["MY_CONFIG_DIR", "XDG_CONFIG_HOME"]);
}

#[test]
fn read_set_deduplicates_repeated_names() {
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

    let keys = EnvVarReadSet::new(&mappings);

    assert_eq!(keys.keys(), ["SHELL"]);
}
