use crate::environment::env_var::env_var_core::EnvVarHostMapping;

pub mod env_var_host_linux;
pub mod env_var_host_mac;
pub mod env_var_host_windows;

#[cfg(host = "linux")]
pub const HOST_ENV_VAR_MAPPINGS: &[EnvVarHostMapping] = env_var_host_linux::HOST_ENV_VAR_MAPPINGS;
#[cfg(host = "mac")]
pub const HOST_ENV_VAR_MAPPINGS: &[EnvVarHostMapping] = env_var_host_mac::HOST_ENV_VAR_MAPPINGS;
#[cfg(host = "windows")]
pub const HOST_ENV_VAR_MAPPINGS: &[EnvVarHostMapping] = env_var_host_windows::HOST_ENV_VAR_MAPPINGS;

#[cfg(test)]
#[path = "env_var_host_tests.rs"]
mod tests;
