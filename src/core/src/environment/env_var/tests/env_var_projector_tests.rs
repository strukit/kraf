use crate::environment::env_var::env_var_core::{
    EnvVar, EnvVarHostMapping, EnvVarImportMode, EnvVarSource,
};
use crate::environment::env_var::env_var_projector::{apply, project};
use crate::environment::env_var::tests::env_var_test_fixtures::FakeEnvVarWriter;

#[test]
fn project_emits_every_alias_export_with_the_resolved_value_in_order() {
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

    let result = project(&[mapping], &env_var_resolved);

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
fn project_skips_canonical_key_not_declared_as_export_alias() {
    let mapping = EnvVarHostMapping {
        key: "LOGNAME",
        source: EnvVarSource::Host,
        alias_import: &["USER", "LOGNAME"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["USER"],
        fallback: None,
    };
    let env_var_resolved = [EnvVar {
        key: "LOGNAME",
        value: Some("bart".to_string()),
    }];

    let result = project(&[mapping], &env_var_resolved);

    let keys: Vec<&str> = result
        .env_vars()
        .iter()
        .map(|env_var| env_var.key)
        .collect();

    assert_eq!(keys, vec!["USER"]);
    assert_eq!(result.env_vars()[0].value, Some("bart".to_string()));
}

#[test]
fn project_emits_nothing_for_mapping_without_alias_export() {
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

    let result = project(&[mapping], &env_var_resolved);

    assert!(result.env_vars().is_empty());
}

#[test]
fn apply_writes_every_projected_entry() {
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
    apply(&project(&[mapping], &env_var_resolved), &mut writer);

    assert_eq!(writer.env.get("TMPDIR"), Some(&"/tmp".to_string()));
    assert_eq!(writer.env.get("HOME_TEMP"), None);
}

#[test]
fn apply_writes_empty_string_for_entries_with_no_value() {
    let mapping = EnvVarHostMapping {
        key: "HOME_TEMP",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &["TMPDIR"],
        fallback: None,
    };

    let mut writer = FakeEnvVarWriter::default();
    apply(&project(&[mapping], &[]), &mut writer);

    assert_eq!(writer.env.get("TMPDIR"), Some(&String::new()));
}
