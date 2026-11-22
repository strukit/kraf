use crate::environment::env_var::env_var_core::{EnvVar, EnvVarHostMapping};
use crate::environment::env_var::env_var_projector::project;
use crate::environment::env_var::env_var_resolver::{lookup_env_var_value, resolve};

const WINDOWS_MAPPINGS: &[EnvVarHostMapping] = super::HOST_ENV_VAR_MAPPINGS;

#[test]
fn windows_program_folders_pass_through_unchanged() {
    let collected = [
        EnvVar {
            key: "ProgramFiles",
            value: Some(r"C:\Program Files (x86)".to_string()),
        },
        EnvVar {
            key: "ProgramW6432",
            value: Some(r"C:\Program Files".to_string()),
        },
    ];

    let resolved = resolve(WINDOWS_MAPPINGS, &collected, &[]);
    let projected = project(WINDOWS_MAPPINGS, &resolved);

    assert_eq!(
        lookup_env_var_value("SYSTEM_PROGRAMS", &resolved),
        Some(r"C:\Program Files".to_string())
    );
    assert_eq!(
        lookup_env_var_value("ProgramFiles", projected.env_vars()),
        Some(r"C:\Program Files (x86)".to_string())
    );
    assert_eq!(
        lookup_env_var_value("ProgramW6432", projected.env_vars()),
        Some(r"C:\Program Files".to_string())
    );
}
