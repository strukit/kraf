use crate::environment::env_var::env_var_core::{EnvVar, EnvVarHostMapping, EnvVarReader};
use crate::environment::env_var::env_var_resolver::env_var_resolve_keys;

pub mod env_var_host_keys;
pub mod env_var_host_linux;
pub mod env_var_host_mac;
pub mod env_var_host_windows;

pub use env_var_host_keys::*;

#[cfg(host = "linux")]
pub const HOST_ENV_VAR_MAPPINGS: &[EnvVarHostMapping] = env_var_host_linux::HOST_ENV_VAR_MAPPINGS;
#[cfg(host = "mac")]
pub const HOST_ENV_VAR_MAPPINGS: &[EnvVarHostMapping] = env_var_host_mac::HOST_ENV_VAR_MAPPINGS;
#[cfg(host = "windows")]
pub const HOST_ENV_VAR_MAPPINGS: &[EnvVarHostMapping] = env_var_host_windows::HOST_ENV_VAR_MAPPINGS;

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

#[cfg(test)]
#[path = "env_var_host_tests.rs"]
mod tests;
