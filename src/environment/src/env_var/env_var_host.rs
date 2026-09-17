use super::env_var_core::{EnvVar, EnvVarHostMapping};
use super::env_var_resolver::env_var_resolve_keys;

use kraf_core::Process;

// General
pub const HOSTNAME_ENV_VAR: EnvVar = EnvVar {
    key: "HOSTNAME",
    value: None,
};

pub const WORKDIR_ENV_VAR: EnvVar = EnvVar {
    key: "WORKDIR",
    value: None,
};

pub const PATH_ENV_VAR: EnvVar = EnvVar {
    key: "PATH",
    value: None,
};

// User
pub const USER_ENV_VAR: EnvVar = EnvVar {
    key: "USER",
    value: None,
};

pub const USER_LANGUAGE_ENV_VAR: EnvVar = EnvVar {
    key: "USER_LANGUAGE",
    value: None,
};

pub const USER_TIMEZONE_ENV_VAR: EnvVar = EnvVar {
    key: "USER_TIMEZONE",
    value: None,
};

// Home directories
pub const HOME_ENV_VAR: EnvVar = EnvVar {
    key: "HOME",
    value: None,
};

pub const HOME_TEMP_ENV_VAR: EnvVar = EnvVar {
    key: "HOME_TEMP",
    value: None,
};

pub const HOME_CONFIG_ENV_VAR: EnvVar = EnvVar {
    key: "HOME_CONFIG",
    value: None,
};

pub const HOME_DATA_ENV_VAR: EnvVar = EnvVar {
    key: "HOME_DATA",
    value: None,
};

pub const HOME_DATA_BIN_ENV_VAR: EnvVar = EnvVar {
    key: "HOME_DATA_BIN",
    value: None,
};

pub const HOME_CACHE_ENV_VAR: EnvVar = EnvVar {
    key: "HOME_CACHE",
    value: None,
};

pub const HOME_STATE_ENV_VAR: EnvVar = EnvVar {
    key: "HOME_STATE",
    value: None,
};

pub const HOME_SESSION_ENV_VAR: EnvVar = EnvVar {
    key: "HOME_SESSION",
    value: None,
};

// Terminal
pub const TERMINAL_ENV_VAR: EnvVar = EnvVar {
    key: "TERMINAL",
    value: None,
};

pub const TERMINAL_TYPE_ENV_VAR: EnvVar = EnvVar {
    key: "TERMINAL_TYPE",
    value: None,
};

pub const TERMINAL_COLOR_ENV_VAR: EnvVar = EnvVar {
    key: "TERMINAL_COLOR",
    value: None,
};

pub const TERMINAL_PROGRAM_ENV_VAR: EnvVar = EnvVar {
    key: "TERMINAL_PROGRAM",
    value: None,
};

pub const TERMINAL_PROGRAM_VERSION_ENV_VAR: EnvVar = EnvVar {
    key: "TERMINAL_PROGRAM_VERSION",
    value: None,
};

pub const TERMINAL_NOCOLOR_ENV_VAR: EnvVar = EnvVar {
    key: "TERMINAL_NOCOLOR",
    value: None,
};

// Platform
pub const PLATFORM_ENV_VAR: EnvVar = EnvVar {
    key: "PLATFORM",
    value: None,
};

pub const PLATFORM_FAMILY_ENV_VAR: EnvVar = EnvVar {
    key: "PLATFORM_FAMILY",
    value: None,
};

pub const PLATFORM_ARCH_ENV_VAR: EnvVar = EnvVar {
    key: "PLATFORM_ARCH",
    value: None,
};

pub const PLATFORM_VERSION_ENV_VAR: EnvVar = EnvVar {
    key: "PLATFORM_VERSION",
    value: None,
};

pub const PLATFORM_KERNEL_ENV_VAR: EnvVar = EnvVar {
    key: "PLATFORM_KERNEL",
    value: None,
};

pub const PLATFORM_KERNEL_VERSION_ENV_VAR: EnvVar = EnvVar {
    key: "PLATFORM_KERNEL_VERSION",
    value: None,
};

pub const ENV_VAR_FROM_HOST: &[&str] = &[
    HOSTNAME_ENV_VAR.key,
    WORKDIR_ENV_VAR.key,
    PATH_ENV_VAR.key,
    USER_ENV_VAR.key,
    USER_LANGUAGE_ENV_VAR.key,
    USER_TIMEZONE_ENV_VAR.key,
    HOME_ENV_VAR.key,
    HOME_TEMP_ENV_VAR.key,
    HOME_CONFIG_ENV_VAR.key,
    HOME_DATA_ENV_VAR.key,
    HOME_DATA_BIN_ENV_VAR.key,
    HOME_CACHE_ENV_VAR.key,
    HOME_STATE_ENV_VAR.key,
    HOME_SESSION_ENV_VAR.key,
    TERMINAL_ENV_VAR.key,
    TERMINAL_TYPE_ENV_VAR.key,
    TERMINAL_COLOR_ENV_VAR.key,
    TERMINAL_PROGRAM_ENV_VAR.key,
    TERMINAL_PROGRAM_VERSION_ENV_VAR.key,
    TERMINAL_NOCOLOR_ENV_VAR.key,
    PLATFORM_ENV_VAR.key,
    PLATFORM_FAMILY_ENV_VAR.key,
    PLATFORM_ARCH_ENV_VAR.key,
    PLATFORM_VERSION_ENV_VAR.key,
    PLATFORM_KERNEL_ENV_VAR.key,
    PLATFORM_KERNEL_VERSION_ENV_VAR.key,
];

pub fn env_var_process_apply(
    env_var_mappings: &[EnvVarHostMapping],
    env_vars: &[EnvVar],
    process: &mut impl Process,
) {
    let keys = env_var_resolve_keys(env_var_mappings);

    for env_var in env_vars {
        if !keys.env_vars_keys_export.contains(&env_var.key) {
            continue;
        }

        let value = env_var.value.as_deref().unwrap_or("");
        process.set_env_var(env_var.key, value);
    }
}

pub fn env_var_collect(
    env_var_mappings: &[EnvVarHostMapping],
    process: &impl Process,
) -> Vec<EnvVar> {
    let keys = env_var_resolve_keys(env_var_mappings);

    keys.env_vars_keys_import
        .iter()
        .map(|&key| EnvVar {
            key,
            value: process.read_env_var(key),
        })
        .collect()
}
