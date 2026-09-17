use super::Process;

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

    /// # Errors
    /// Returns an error if the process fails to spawn, matching
    /// `std::process::Command::status`.
    pub fn status(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.command.status()
    }
}

impl Process for ChildProcess {
    fn read_env_vars(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    fn read_env_var(&self, _name: &str) -> Option<String> {
        None
    }

    fn get_current_dir(&self) -> Option<String> {
        None
    }

    fn get_temp_dir_random(&self) -> Option<String> {
        None
    }

    fn set_env_var(&mut self, key: &str, value: &str) -> &mut Self {
        self.command.env(key, value);
        self
    }

    fn clean_env(&mut self) -> &mut Self {
        self.command.env_clear();
        self
    }
}
