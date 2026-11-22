use crate::environment::env_var::env_var_core::EnvVarWriter;

pub struct ChildProcess {
    command: std::process::Command,
}

impl ChildProcess {
    #[must_use]
    pub fn new(program: &str) -> Self {
        Self {
            command: std::process::Command::new(program),
        }
    }

    pub fn current_dir(&mut self, dir: &str) -> &mut Self {
        self.command.current_dir(dir);
        self
    }

    pub fn clean_env(&mut self) -> &mut Self {
        self.command.env_clear();
        self
    }

    /// # Errors
    /// Returns an error if the process fails to spawn, matching
    /// `std::process::Command::status`.
    pub fn status(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.command.status()
    }
}

impl EnvVarWriter for ChildProcess {
    fn write(&mut self, key: &str, value: &str) {
        self.command.env(key, value);
    }
}
