// EXPERIMENTAL: this code is a work in progress and MUST NOT be verified.
// Do not review it or treat it as a reference for correct behavior.

/// Common interface for process's
pub trait Process {
    fn get_current_dir(&self) -> Option<String>;

    fn create_session_dir(&self, host_home: &str) -> Option<String>;
}
