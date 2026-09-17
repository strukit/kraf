use super::Process;

pub struct HostProcess;

impl Process for HostProcess {
    fn read_env_vars(&self) -> Vec<(String, String)> {
        std::env::vars().collect()
    }

    fn read_env_var(&self, name: &str) -> Option<String> {
        std::env::var(name).ok()
    }

    fn get_current_dir(&self) -> Option<String> {
        std::env::current_dir()
            .ok()
            .map(|path| path.to_string_lossy().into_owned())
    }

    fn set_env_var(&mut self, key: &str, value: &str) -> &mut Self {
        // SAFETY: kraf is single-threaded at the point
        unsafe {
            std::env::set_var(key, value);
        }
        self
    }

    fn get_temp_dir_random(&self) -> Option<String> {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let dir = std::env::temp_dir().join(format!("kraf-{}-{nonce}", std::process::id()));

        std::fs::create_dir_all(&dir).ok()?;

        Some(dir.to_string_lossy().into_owned())
    }

    fn clean_env(&mut self) -> &mut Self {
        let keys: Vec<String> = std::env::vars().map(|(key, _)| key).collect();

        // SAFETY: kraf is single-threaded at the point
        unsafe {
            for key in keys {
                std::env::remove_var(key);
            }
        }
        self
    }
}
