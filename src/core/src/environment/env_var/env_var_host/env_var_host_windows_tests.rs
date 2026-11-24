use crate::environment::env_var::env_var_core::{EnvVar, EnvVarHostMapping};
use crate::environment::env_var::env_var_projector::EnvVarWriteSet;
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
    let projected = EnvVarWriteSet::new(WINDOWS_MAPPINGS, &resolved);

    assert_eq!(
        lookup_env_var_value("ProgramFiles", projected.env_vars()),
        Some(r"C:\Program Files (x86)".to_string())
    );
    assert_eq!(
        lookup_env_var_value("ProgramW6432", projected.env_vars()),
        Some(r"C:\Program Files".to_string())
    );
}

#[test]
fn windows_comspec_keeps_the_host_value_when_the_terminal_is_another_shell() {
    let collected = [
        EnvVar {
            key: "SHELL",
            value: Some(r"C:\Program Files\Git\usr\bin\bash.exe".to_string()),
        },
        EnvVar {
            key: "COMSPEC",
            value: Some(r"C:\WINDOWS\system32\cmd.exe".to_string()),
        },
    ];

    let resolved = resolve(WINDOWS_MAPPINGS, &collected, &[]);
    let projected = EnvVarWriteSet::new(WINDOWS_MAPPINGS, &resolved);

    assert_eq!(
        lookup_env_var_value("TERMINAL", &resolved),
        Some(r"C:\Program Files\Git\usr\bin\bash.exe".to_string())
    );
    assert_eq!(
        lookup_env_var_value("SHELL", projected.env_vars()),
        Some(r"C:\Program Files\Git\usr\bin\bash.exe".to_string())
    );
    assert_eq!(
        lookup_env_var_value("COMSPEC", projected.env_vars()),
        Some(r"C:\WINDOWS\system32\cmd.exe".to_string())
    );
}
