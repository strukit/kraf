use crate::environment::env_var::env_var_core::{EnvVar, EnvVarHostMapping, EnvVarWriter};
use crate::environment::env_var::env_var_resolver::lookup_env_var_value;

#[derive(Debug, Clone)]
pub struct EnvVarWriteSet {
    env_vars: Vec<EnvVar>,
}

impl EnvVarWriteSet {
    #[must_use]
    pub fn new(mappings: &[EnvVarHostMapping], env_vars: &[EnvVar]) -> Self {
        let env_vars = mappings
            .iter()
            .filter_map(|mapping| {
                lookup_env_var_value(mapping.key, env_vars)
                    .filter(|value| !value.is_empty())
                    .map(|value| (mapping, value))
            })
            .flat_map(|(mapping, value)| {
                mapping
                    .alias_export
                    .iter()
                    .copied()
                    .map(move |alias_export| EnvVar {
                        key: alias_export,
                        value: Some(value.clone()),
                    })
            })
            .collect();

        Self { env_vars }
    }

    #[must_use]
    pub fn env_vars(&self) -> &[EnvVar] {
        &self.env_vars
    }
}

pub fn project(to_write: &EnvVarWriteSet, writer: &mut impl EnvVarWriter) {
    for env_var in to_write.env_vars() {
        writer.write(env_var.key, env_var.value.as_deref().unwrap_or(""));
    }
}

#[cfg(test)]
#[path = "env_var_projector_tests.rs"]
mod tests;
