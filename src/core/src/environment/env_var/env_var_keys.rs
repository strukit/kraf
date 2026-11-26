use crate::environment::env_var::env_var_core::EnvVar;

// Host
pub const HOST_NAME_ENV_VAR: EnvVar = EnvVar {
    key: "HOST_NAME",
    value: None,
};

pub const HOST_WORKDIR_ENV_VAR: EnvVar = EnvVar {
    key: "HOST_WORKDIR",
    value: None,
};

pub const HOST_HOME_ENV_VAR: EnvVar = EnvVar {
    key: "HOST_HOME",
    value: None,
};

pub const HOST_PATH_ENV_VAR: EnvVar = EnvVar {
    key: "HOST_PATH",
    value: None,
};

pub const HOST_DATA_ENV_VAR: EnvVar = EnvVar {
    key: "HOST_DATA",
    value: None,
};

pub const HOST_CONFIG_ENV_VAR: EnvVar = EnvVar {
    key: "HOST_CONFIG",
    value: None,
};

// User
pub const USER_NAME_ENV_VAR: EnvVar = EnvVar {
    key: "USER_NAME",
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
pub const HOME_DIR_ENV_VAR: EnvVar = EnvVar {
    key: "HOME_DIR",
    value: None,
};

pub const HOME_PATH_ENV_VAR: EnvVar = EnvVar {
    key: "HOME_PATH",
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

pub const ENV_VAR_KEYS: &[&str] = &[
    // Host
    HOST_NAME_ENV_VAR.key,
    HOST_WORKDIR_ENV_VAR.key,
    HOST_HOME_ENV_VAR.key,
    HOST_PATH_ENV_VAR.key,
    HOST_DATA_ENV_VAR.key,
    HOST_CONFIG_ENV_VAR.key,
    // User
    USER_NAME_ENV_VAR.key,
    USER_LANGUAGE_ENV_VAR.key,
    USER_TIMEZONE_ENV_VAR.key,
    // Home directories
    HOME_DIR_ENV_VAR.key,
    HOME_PATH_ENV_VAR.key,
    HOME_TEMP_ENV_VAR.key,
    HOME_CONFIG_ENV_VAR.key,
    HOME_DATA_ENV_VAR.key,
    HOME_DATA_BIN_ENV_VAR.key,
    HOME_CACHE_ENV_VAR.key,
    HOME_STATE_ENV_VAR.key,
    HOME_SESSION_ENV_VAR.key,
    // Terminal
    TERMINAL_ENV_VAR.key,
    TERMINAL_TYPE_ENV_VAR.key,
    TERMINAL_COLOR_ENV_VAR.key,
    TERMINAL_PROGRAM_ENV_VAR.key,
    TERMINAL_PROGRAM_VERSION_ENV_VAR.key,
    TERMINAL_NOCOLOR_ENV_VAR.key,
    // Platform
    PLATFORM_ENV_VAR.key,
    PLATFORM_FAMILY_ENV_VAR.key,
    PLATFORM_ARCH_ENV_VAR.key,
    PLATFORM_VERSION_ENV_VAR.key,
    PLATFORM_KERNEL_ENV_VAR.key,
    PLATFORM_KERNEL_VERSION_ENV_VAR.key,
];
