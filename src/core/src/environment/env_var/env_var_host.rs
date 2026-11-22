use crate::environment::env_var::env_var_core::{EnvVar, EnvVarHostMapping, EnvVarReader};
use crate::environment::env_var::env_var_resolver::env_var_resolve_keys;

#[path = "env_var_host_linux.rs"]
pub mod env_var_host_linux;

#[path = "env_var_host_mac.rs"]
pub mod env_var_host_mac;

#[path = "env_var_host_windows.rs"]
pub mod env_var_host_windows;

#[cfg(host = "linux")]
pub const HOST_ENV_VAR_MAPPINGS: &[EnvVarHostMapping] = env_var_host_linux::HOST_ENV_VAR_MAPPINGS;
#[cfg(host = "mac")]
pub const HOST_ENV_VAR_MAPPINGS: &[EnvVarHostMapping] = env_var_host_mac::HOST_ENV_VAR_MAPPINGS;
#[cfg(host = "windows")]
pub const HOST_ENV_VAR_MAPPINGS: &[EnvVarHostMapping] = env_var_host_windows::HOST_ENV_VAR_MAPPINGS;

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
pub const HOST_HOME_ENV_VAR: EnvVar = EnvVar {
    key: "HOST_HOME",
    value: None,
};

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

pub const ENV_VAR_HOST_KEYS: &[&str] = &[
    HOSTNAME_ENV_VAR.key,
    WORKDIR_ENV_VAR.key,
    PATH_ENV_VAR.key,
    USER_ENV_VAR.key,
    USER_LANGUAGE_ENV_VAR.key,
    USER_TIMEZONE_ENV_VAR.key,
    HOST_HOME_ENV_VAR.key,
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

pub struct HostEnvVarReader;

impl EnvVarReader for HostEnvVarReader {
    fn read(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
}

pub fn env_var_collect(
    env_var_mappings: &[EnvVarHostMapping],
    reader: &impl EnvVarReader,
) -> Vec<EnvVar> {
    let keys = env_var_resolve_keys(env_var_mappings);

    keys.env_vars_keys_import
        .iter()
        .map(|&key| EnvVar {
            key,
            value: reader.read(key),
        })
        .collect()
}
