use crate::environment::env_var::env_var_core::{EnvVar, EnvVarHostMapping, EnvVarWriter};
use crate::environment::env_var::env_var_resolver::lookup_env_var_value;

/// Environment variables that the mappings allow a managed process to receive.
/// Only [`project`] builds it, so [`apply`] writes nothing outside the mappings.
#[derive(Debug, Clone)]
pub struct EnvVarProjection(Vec<EnvVar>);

impl EnvVarProjection {
    #[must_use]
    pub fn env_vars(&self) -> &[EnvVar] {
        &self.0
    }
}

#[must_use]
pub fn project(
    env_var_mappings: &[EnvVarHostMapping],
    env_var_resolved: &[EnvVar],
) -> EnvVarProjection {
    let env_vars = env_var_mappings
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
        .collect();

    EnvVarProjection(env_vars)
}

pub fn apply(env_var_projection: &EnvVarProjection, writer: &mut impl EnvVarWriter) {
    for env_var in env_var_projection.env_vars() {
        writer.write(env_var.key, env_var.value.as_deref().unwrap_or(""));
    }
}
