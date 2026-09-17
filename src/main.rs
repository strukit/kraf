use kraf::core::{ChildProcess, HostProcess, Process};
use kraf::environment::env_var::env_var_core::EnvVar;
use kraf::environment::env_var::env_var_host::{env_var_collect, env_var_process_apply};
use kraf::environment::env_var::env_var_resolver::resolve;

#[cfg(target_os = "linux")]
use kraf::environment::env_var::env_var_host_linux::LINUX_MAPPINGS as PLATFORM_MAPPINGS;
#[cfg(target_os = "macos")]
use kraf::environment::env_var::env_var_host_macos::MACOS_MAPPINGS as PLATFORM_MAPPINGS;
#[cfg(target_os = "windows")]
use kraf::environment::env_var::env_var_host_windows::WINDOWS_MAPPINGS as PLATFORM_MAPPINGS;

fn main() {
    let workdir = HostProcess.get_current_dir();
    let homedir = HostProcess.get_temp_dir_random();

    println!("[kraf] trap init");

    let mut collected = env_var_collect(PLATFORM_MAPPINGS, &HostProcess);

    collected.push(EnvVar {
        key: "WORKDIR",
        value: workdir.clone(),
    });

    collected.push(EnvVar {
        key: "HOME",
        value: homedir,
    });

    let resolved = resolve(PLATFORM_MAPPINGS, &collected);

    let workdir = resolved
        .iter()
        .find(|env_var| env_var.key == "WORKDIR")
        .and_then(|env_var| env_var.value.clone())
        .expect("[kraf] error: $WORKDIR is required, failed to resolve (internal)");

    let terminal = resolved
        .iter()
        .find(|env_var| env_var.key == "TERMINAL")
        .and_then(|env_var| env_var.value.clone())
        .expect("[kraf] error: $TERMINAL is required, failed to resolve (internal)");

    let home = resolved
        .iter()
        .find(|env_var| env_var.key == "HOME")
        .and_then(|env_var| env_var.value.clone())
        .expect("[kraf] error: $HOME is required, failed to resolve (internal)");

    println!("[kraf]   HOME={home}");
    println!("[kraf]   WORKDIR={workdir}");

    let mut child = ChildProcess::new(&terminal);

    child.current_dir(&workdir);
    child.clean_env();

    env_var_process_apply(PLATFORM_MAPPINGS, &resolved, &mut child);

    println!("[kraf] trap ready");

    let status = child
        .status()
        .unwrap_or_else(|error| panic!("[kraf] failed to launch terminal: {error}"));

    std::process::exit(status.code().unwrap_or(1));
}
