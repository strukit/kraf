use crate::environment::env_var::env_var_core::{EnvVar, EnvVarHostMapping};
use crate::environment::env_var::env_var_resolver::lookup_env_var_value;

#[must_use]
pub fn project(env_var_mappings: &[EnvVarHostMapping], env_var_resolved: &[EnvVar]) -> Vec<EnvVar> {
    env_var_mappings
        .iter()
        .flat_map(|env_var_mapping| {
            let env_var_value = lookup_env_var_value(env_var_mapping.key, env_var_resolved);

            env_var_mapping
                .alias_export
                .iter()
                .copied()
                .map(move |alias_export| EnvVar {
                    key: alias_export,
                    value: env_var_value.clone(),
                })
        })
        .collect()
}
