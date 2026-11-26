use crate::environment::env_var::env_var_core::{EnvVar, EnvVarHostMapping, EnvVarReader};

#[derive(Debug, Clone)]
pub struct EnvVarReadSet {
    keys: Vec<&'static str>,
}

impl EnvVarReadSet {
    #[must_use]
    pub fn new(mappings: &[EnvVarHostMapping]) -> Self {
        let mut keys: Vec<&'static str> = mappings
            .iter()
            .flat_map(|mapping| mapping.alias_import.iter().copied())
            .collect();

        keys.sort_unstable();
        keys.dedup();

        Self { keys }
    }

    #[must_use]
    pub fn keys(&self) -> &[&'static str] {
        &self.keys
    }
}

pub fn collect(to_read: &EnvVarReadSet, reader: &impl EnvVarReader) -> Vec<EnvVar> {
    to_read
        .keys()
        .iter()
        .map(|&key| EnvVar {
            key,
            value: reader.read(key),
        })
        .collect()
}

#[cfg(test)]
#[path = "env_var_collector_tests.rs"]
mod tests;
