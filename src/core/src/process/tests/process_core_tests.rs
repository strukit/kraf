use crate::environment::env_var::env_var_core::EnvVar;
use crate::process::Process;
use std::collections::HashMap;

/// In-memory process whose environment is isolated from the real one of the
/// test process, so that tests neither read nor change it.
#[derive(Default)]
struct FakeProcess {
    env: HashMap<String, String>,
}

impl Process for FakeProcess {
    fn read_env_vars(&self) -> Vec<(String, String)> {
        self.env
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    fn read_env_var(&self, name: &str) -> Option<String> {
        self.env.get(name).cloned()
    }

    fn get_current_dir(&self) -> Option<String> {
        None
    }

    fn get_temp_dir_random(&self) -> Option<String> {
        None
    }

    fn set_env_var(&mut self, key: &str, value: &str) -> &mut Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    fn clean_env(&mut self) -> &mut Self {
        self.env.clear();
        self
    }
}

#[test]
fn apply_env_vars_writes_every_entry() {
    let env_vars = [EnvVar {
        key: "TMPDIR",
        value: Some("/tmp".to_string()),
    }];

    let mut process = FakeProcess::default();
    process.apply_env_vars(&env_vars);

    assert_eq!(process.read_env_var("TMPDIR"), Some("/tmp".to_string()));
}

#[test]
fn apply_env_vars_writes_empty_string_for_entries_with_no_value() {
    let env_vars = [EnvVar {
        key: "TMPDIR",
        value: None,
    }];

    let mut process = FakeProcess::default();
    process.apply_env_vars(&env_vars);

    assert_eq!(process.read_env_var("TMPDIR"), Some(String::new()));
}
