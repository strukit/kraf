// EXPERIMENTAL: this code is a work in progress and MUST NOT be verified.
// Do not review it or treat it as a reference for correct behavior.

use kraf_core::environment::env_var::env_var_core::EnvVar;
use kraf_core::environment::env_var::env_var_host::{
    HOME_DIR_ENV_VAR, HOST_ENV_VAR_MAPPINGS, HOST_HOME_ENV_VAR, HostEnvVarReader, WORK_DIR_ENV_VAR,
    env_var_collect,
};
use kraf_core::environment::env_var::env_var_projector::{apply, project};
use kraf_core::environment::env_var::env_var_resolver::resolve;
use kraf_core::{ChildProcess, HostProcess, Process};

fn main() {
    println!("[kraf] trap init");

    let collected = env_var_collect(HOST_ENV_VAR_MAPPINGS, &HostEnvVarReader);

    let host_home = resolve(HOST_ENV_VAR_MAPPINGS, &collected, &[])
        .into_iter()
        .find(|env_var| env_var.key == HOST_HOME_ENV_VAR.key)
        .and_then(|env_var| env_var.value)
        .filter(|value| !value.is_empty())
        .expect("[kraf] error: $HOST_HOME is required, failed to resolve (internal)");

    let provided = [
        EnvVar {
            key: WORK_DIR_ENV_VAR.key,
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
        .find(|env_var| env_var.key == "WORK_DIR")
        .and_then(|env_var| env_var.value.clone())
        .expect("[kraf] error: $WORK_DIR is required, failed to resolve (internal)");

    let terminal = resolved
        .iter()
        .find(|env_var| env_var.key == "TERMINAL")
        .and_then(|env_var| env_var.value.clone())
        .expect("[kraf] error: $TERMINAL is required, failed to resolve (internal)");

    let home = resolved
        .iter()
        .find(|env_var| env_var.key == "HOME_DIR")
        .and_then(|env_var| env_var.value.clone())
        .expect("[kraf] error: $HOME_DIR is required, failed to resolve (internal)");

    println!("[kraf]   HOME_DIR={home}");
    println!("[kraf]   WORK_DIR={workdir}");

    let mut child = ChildProcess::new(&terminal);

    child.current_dir(&workdir);
    child.clean_env();

    apply(&project(HOST_ENV_VAR_MAPPINGS, &resolved), &mut child);

    println!("[kraf] trap ready");

    let status = child
        .status()
        .unwrap_or_else(|error| panic!("[kraf] failed to launch terminal: {error}"));

    std::process::exit(status.code().unwrap_or(1));
}
