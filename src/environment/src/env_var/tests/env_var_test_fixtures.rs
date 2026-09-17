use crate::env_var::env_var_core::{EnvVar, EnvVarHostMapping, EnvVarImportMode, EnvVarSource};

pub(super) const ENV_VAR_MAPPING_FIXTURE: &[EnvVarHostMapping] = &[
    EnvVarHostMapping {
        key: "EDITOR",
        source: EnvVarSource::Host,
        alias_import: &["EDITOR"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("vi"),
    },
    EnvVarHostMapping {
        key: "USER_LANGUAGE",
        source: EnvVarSource::Host,
        alias_import: &["USER_LANGUAGE", "LANG"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("en_US.UTF-8"),
    },
    EnvVarHostMapping {
        key: "TERMINAL_TYPE",
        source: EnvVarSource::Host,
        alias_import: &["TERMINAL_TYPE", "TERM"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("xterm-256color"),
    },
    EnvVarHostMapping {
        key: "TMP_DIR",
        source: EnvVarSource::Host,
        alias_import: &["TMPDIR"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("~/.tmp"),
    },
    EnvVarHostMapping {
        key: "CACHE_DIR",
        source: EnvVarSource::Host,
        alias_import: &["XDG_CACHE_HOME"],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("$HOME/.cache"),
    },
    EnvVarHostMapping {
        key: "PATH",
        source: EnvVarSource::Provided,
        alias_import: &["HOME_DATA_BIN", "PATH"],
        alias_import_mode: EnvVarImportMode::Merge,
        alias_export: &[],
        fallback: None,
    },
    EnvVarHostMapping {
        key: "LD_LIBRARY_PATH",
        source: EnvVarSource::Provided,
        alias_import: &["LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH"],
        alias_import_mode: EnvVarImportMode::Merge,
        alias_export: &[],
        fallback: None,
    },
    EnvVarHostMapping {
        key: "MANPATH",
        source: EnvVarSource::Provided,
        alias_import: &["MANPATH", "MANPATH_EXTRA"],
        alias_import_mode: EnvVarImportMode::Merge,
        alias_export: &[],
        fallback: Some("/usr/share/man"),
    },
    EnvVarHostMapping {
        key: "WORKDIR",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: None,
    },
    EnvVarHostMapping {
        key: "HOME_TEMP",
        source: EnvVarSource::Provided,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("~/.tmp"),
    },
    EnvVarHostMapping {
        key: "PLATFORM_KERNEL",
        source: EnvVarSource::Host,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: Some("Darwin"),
    },
    EnvVarHostMapping {
        key: "PLATFORM_VERSION",
        source: EnvVarSource::Host,
        alias_import: &[],
        alias_import_mode: EnvVarImportMode::FirstFound,
        alias_export: &[],
        fallback: None,
    },
];

pub(super) fn env_var_collected_fixture(home: &str) -> Vec<EnvVar> {
    vec![
        EnvVar {
            key: "HOME",
            value: Some(home.to_string()),
        },
        EnvVar {
            key: "EDITOR",
            value: Some("nano".to_string()),
        },
        EnvVar {
            key: "LANG",
            value: Some("pt_BR.UTF-8".to_string()),
        },
        EnvVar {
            key: "HOME_DATA_BIN",
            value: Some("/workspace/.local/bin".to_string()),
        },
        EnvVar {
            key: "PATH",
            value: Some("/usr/bin:/bin".to_string()),
        },
        EnvVar {
            key: "DYLD_LIBRARY_PATH",
            value: Some("/usr/local/lib".to_string()),
        },
        EnvVar {
            key: "WORKDIR",
            value: Some("/workspace/project".to_string()),
        },
        EnvVar {
            key: "HOME_TEMP",
            value: Some("/custom/tmp".to_string()),
        },
        EnvVar {
            key: "PLATFORM_KERNEL",
            value: Some("Linux".to_string()),
        },
    ]
}
