/// Common interface for process's
pub trait Process {
    fn read_env_vars(&self) -> Vec<(String, String)>;

    fn read_env_var(&self, name: &str) -> Option<String>;

    fn get_current_dir(&self) -> Option<String>;

    fn get_temp_dir_random(&self) -> Option<String>;

    fn set_env_var(&mut self, key: &str, value: &str) -> &mut Self;

    fn clean_env(&mut self) -> &mut Self;
}
