use crate::environment::env_var::env_var_core::{
    EnvVar, EnvVarHostMapping, EnvVarImportMode, EnvVarSource, EnvVarWriter,
};
use crate::environment::env_var::env_var_projector::{EnvVarWriteSet, project};

use std::collections::HashMap;

/// In-memory writer whose environment is isolated from the real one of the
/// test process, so that tests neither read nor change it.
#[derive(Default)]
struct FakeEnvVarWriter {
    env: HashMap<String, String>,
}

impl EnvVarWriter for FakeEnvVarWriter {
    fn write(&mut self, key: &str, value: &str) {
        self.env.insert(key.to_string(), value.to_string());
    }
}

#[test]
fn write_set_emits_every_alias_export_with_the_resolved_value_in_order() {
    let mapping = EnvVarHostMapping {
        key: "HOME_TEMP",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["TEMP", "TMPDIR", "TMP"],
        fallback: None,
    };
    let env_var_resolved = [EnvVar {
        key: "HOME_TEMP",
        value: Some("/workspace/.tmp".to_string()),
    }];

    let result = EnvVarWriteSet::new(&[mapping], &env_var_resolved);

    let keys: Vec<&str> = result
        .env_vars()
        .iter()
        .map(|env_var| env_var.key)
        .collect();
    let values: Vec<Option<String>> = result
        .env_vars()
        .iter()
        .map(|env_var| env_var.value.clone())
        .collect();

    assert_eq!(keys, vec!["TEMP", "TMPDIR", "TMP"]);
    assert_eq!(values, vec![Some("/workspace/.tmp".to_string()); 3]);
}

#[test]
fn write_set_skips_canonical_key_not_declared_as_export_alias() {
    let mapping = EnvVarHostMapping {
        key: "USER",
        source: EnvVarSource::Host,
        alias_import: &["USER", "LOGNAME"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["LOGNAME"],
        fallback: None,
    };
    let env_var_resolved = [EnvVar {
        key: "USER",
        value: Some("bart".to_string()),
    }];

    let result = EnvVarWriteSet::new(&[mapping], &env_var_resolved);

    let keys: Vec<&str> = result
        .env_vars()
        .iter()
        .map(|env_var| env_var.key)
        .collect();

    assert_eq!(keys, vec!["LOGNAME"]);
    assert_eq!(result.env_vars()[0].value, Some("bart".to_string()));
}

// Regression for LOGNAME=root with USER=bart, fixed in pull request: #47: the projection
// exports LOGNAME with the resolved USER value, as the production mapping declares.
#[test]
fn write_set_exports_logname_with_the_user_value() {
    let mapping = EnvVarHostMapping {
        key: "USER",
        source: EnvVarSource::Host,
        alias_import: &["USER", "LOGNAME"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["USER", "LOGNAME"],
        fallback: None,
    };
    let env_var_resolved = [EnvVar {
        key: "USER",
        value: Some("bart".to_string()),
    }];

    let result = EnvVarWriteSet::new(&[mapping], &env_var_resolved);

    let keys: Vec<&str> = result
        .env_vars()
        .iter()
        .map(|env_var| env_var.key)
        .collect();
    let values: Vec<Option<String>> = result
        .env_vars()
        .iter()
        .map(|env_var| env_var.value.clone())
        .collect();

    assert_eq!(keys, vec!["USER", "LOGNAME"]);
    assert_eq!(values, vec![Some("bart".to_string()); 2]);
}

#[test]
fn write_set_emits_nothing_for_mapping_without_alias_export() {
    let mapping = EnvVarHostMapping {
        key: "TERMINAL",
        source: EnvVarSource::Host,
        alias_import: &["SHELL"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: None,
    };
    let env_var_resolved = [EnvVar {
        key: "TERMINAL",
        value: Some("/bin/zsh".to_string()),
    }];

    let result = EnvVarWriteSet::new(&[mapping], &env_var_resolved);

    assert!(result.env_vars().is_empty());
}

#[test]
fn project_writes_every_projected_entry() {
    let mapping = EnvVarHostMapping {
        key: "HOME_TEMP",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["TMPDIR"],
        fallback: None,
    };
    let env_var_resolved = [EnvVar {
        key: "HOME_TEMP",
        value: Some("/tmp".to_string()),
    }];

    let mut writer = FakeEnvVarWriter::default();
    project(
        &EnvVarWriteSet::new(&[mapping], &env_var_resolved),
        &mut writer,
    );

    assert_eq!(writer.env.get("TMPDIR"), Some(&"/tmp".to_string()));
    assert_eq!(writer.env.get("HOME_TEMP"), None);
}

#[test]
fn write_set_skips_declaration_with_empty_value() {
    let mapping = EnvVarHostMapping {
        key: "USER_TIMEZONE",
        source: EnvVarSource::Host,
        alias_import: &["USER_TIMEZONE", "TZ"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["TZ"],
        fallback: None,
    };
    let env_var_resolved = [EnvVar {
        key: "USER_TIMEZONE",
        value: Some(String::new()),
    }];

    let result = EnvVarWriteSet::new(&[mapping], &env_var_resolved);

    assert!(result.env_vars().is_empty());
}

#[test]
fn project_writes_nothing_for_declaration_without_resolved_value() {
    let mapping = EnvVarHostMapping {
        key: "HOME_TEMP",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["TMPDIR"],
        fallback: None,
    };

    let mut writer = FakeEnvVarWriter::default();
    project(&EnvVarWriteSet::new(&[mapping], &[]), &mut writer);

    assert_eq!(writer.env.get("TMPDIR"), None);
}
