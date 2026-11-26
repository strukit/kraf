// EXPERIMENTAL: this code is a work in progress and MUST NOT be verified.
// Do not review it or treat it as a reference for correct behavior.

mod process;

use kraf_core::environment::env_var::env_var_collector::{EnvVarReadSet, collect};
use kraf_core::environment::env_var::env_var_core::{EnvVar, EnvVarReader};
use kraf_core::environment::env_var::env_var_host::HOST_ENV_VAR_MAPPINGS;
use kraf_core::environment::env_var::env_var_keys::{
    HOME_DIR_ENV_VAR, HOST_HOME_ENV_VAR, HOST_WORKDIR_ENV_VAR,
};
use kraf_core::environment::env_var::env_var_projector::{EnvVarWriteSet, project};
use kraf_core::environment::env_var::env_var_resolver::resolve;
use process::{ChildProcess, HostProcess, Process};

pub struct HostEnvVarReader;

impl EnvVarReader for HostEnvVarReader {
    fn read(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
}

fn main() {
    println!("[kraf] trap init");

    let collected = collect(
        &EnvVarReadSet::new(HOST_ENV_VAR_MAPPINGS),
        &HostEnvVarReader,
    );

    let host_home = resolve(HOST_ENV_VAR_MAPPINGS, &collected, &[])
        .into_iter()
        .find(|env_var| env_var.key == HOST_HOME_ENV_VAR.key)
        .and_then(|env_var| env_var.value)
        .filter(|value| !value.is_empty())
        .expect("[kraf] error: $HOST_HOME is required, failed to resolve (internal)");

    let provided = [
        EnvVar {
            key: HOST_WORKDIR_ENV_VAR.key,
            value: HostProcess.get_current_dir(),
        },
        EnvVar {
            key: HOME_DIR_ENV_VAR.key,
            value: HostProcess.create_session_dir(&host_home),
        },
    ];

    let resolved = resolve(HOST_ENV_VAR_MAPPINGS, &collected, &provided);

    let workdir = resolved
        .iter()
        .find(|env_var| env_var.key == HOST_WORKDIR_ENV_VAR.key)
        .and_then(|env_var| env_var.value.clone())
        .expect("[kraf] error: $WORKDIR is required, failed to resolve (internal)");

    let terminal = resolved
        .iter()
        .find(|env_var| env_var.key == "TERMINAL")
        .and_then(|env_var| env_var.value.clone())
        .expect("[kraf] error: $TERMINAL is required, failed to resolve (internal)");

    let home = resolved
        .iter()
        .find(|env_var| env_var.key == HOME_DIR_ENV_VAR.key)
        .and_then(|env_var| env_var.value.clone())
        .expect("[kraf] error: $HOME is required, failed to resolve (internal)");

    println!("[kraf]   HOME={home}");
    println!("[kraf]   WORKDIR={workdir}");

    let mut child = ChildProcess::new(&terminal);

    child.current_dir(&workdir);
    child.clean_env();

    project(
        &EnvVarWriteSet::new(HOST_ENV_VAR_MAPPINGS, &resolved),
        &mut child,
    );

    println!("[kraf] trap ready");

    let status = child
        .status()
        .unwrap_or_else(|error| panic!("[kraf] failed to launch terminal: {error}"));

    std::process::exit(status.code().unwrap_or(1));
}
