// EXPERIMENTAL: this code is a work in progress and MUST NOT be verified.
// Do not review it or treat it as a reference for correct behavior.

use crate::process::Process;

pub struct HostProcess;

impl Process for HostProcess {
    fn get_current_dir(&self) -> Option<String> {
        std::env::current_dir()
            .ok()
            .map(|path| path.to_string_lossy().into_owned())
    }

    fn create_session_dir(&self, host_home: &str) -> Option<String> {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let dir = std::path::Path::new(host_home)
            .join(".kraf")
            .join("workspace")
            .join("session")
            .join(format!("{}-{nonce}", std::process::id()));

        std::fs::create_dir_all(&dir).ok()?;

        Some(dir.to_string_lossy().into_owned())
    }

    fn create_dir(&self, path: &str) -> bool {
        std::fs::create_dir_all(path).is_ok()
    }

    fn create_dir_link(&self, target: &str, link: &str) -> bool {
        #[cfg(any(host = "linux", host = "mac"))]
        let linked = std::os::unix::fs::symlink(target, link);

        #[cfg(host = "windows")]
        let linked = std::os::windows::fs::symlink_dir(target, link);

        #[cfg(not(any(host = "linux", host = "mac", host = "windows")))]
        let linked: std::io::Result<()> = Err(std::io::ErrorKind::Unsupported.into());

        linked.is_ok()
    }
}
