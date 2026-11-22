/// Common interface for process's
pub trait Process {
    fn get_current_dir(&self) -> Option<String>;

    fn create_session_dir(&self, host_home: &str) -> Option<String>;

    fn create_dir(&self, path: &str) -> bool;
}
