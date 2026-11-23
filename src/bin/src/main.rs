// EXPERIMENTAL: this code is a work in progress and MUST NOT be verified.
// Do not review it or treat it as a reference for correct behavior.

mod process;

use kraf_core::environment::env_var::env_var_core::{EnvVar, EnvVarReader, EnvVarWriter};
use kraf_core::environment::env_var::env_var_host::{
    HOME_DATA_BIN_ENV_VAR, HOME_DIR_ENV_VAR, HOME_PATH_ENV_VAR, HOST_ENV_VAR_MAPPINGS,
    HOST_HOME_ENV_VAR, WORK_DIR_ENV_VAR, env_var_collect,
};
use kraf_core::environment::env_var::env_var_projector::{apply, project};
use kraf_core::environment::env_var::env_var_resolver::{
    expand_tilde_var, lookup_env_var_value, resolve,
};
use process::{ChildProcess, HostProcess, Process};

pub struct HostEnvVarReader;

impl EnvVarReader for HostEnvVarReader {
    fn read(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
}

/// Source the session PATH inherits from, chosen with `--inherit`.
#[derive(PartialEq)]
enum Inherit {
    System,
    User,
    Passthrough,
}

fn parse_inherit() -> Vec<Inherit> {
    let inherit: Vec<Inherit> = std::env::args()
        .skip(1)
        .filter_map(|arg| arg.strip_prefix("--inherit=").map(str::to_owned))
        .flat_map(|value| value.split(',').map(str::to_owned).collect::<Vec<_>>())
        .map(|source| match source.as_str() {
            "system" => Inherit::System,
            "user" => Inherit::User,
            "passthrough" => Inherit::Passthrough,
            "workspace" => {
                eprintln!("[kraf] error: --inherit=workspace is not implemented yet");
                std::process::exit(2);
            }
            other => {
                eprintln!("[kraf] error: unknown --inherit source: {other}");
                std::process::exit(2);
            }
        })
        .collect();

    if inherit.contains(&Inherit::Passthrough)
        && inherit.iter().any(|source| *source != Inherit::Passthrough)
    {
        eprintln!("[kraf] error: --inherit=passthrough cannot be combined with other sources");
        std::process::exit(2);
    }

    if inherit.is_empty() {
        return vec![Inherit::User];
    }

    inherit
}

fn mapping_fallback(key: &str) -> &'static str {
    HOST_ENV_VAR_MAPPINGS
        .iter()
        .find(|mapping| mapping.key == key)
        .and_then(|mapping| mapping.fallback)
        .unwrap_or_default()
}

/// Builds the session PATH: the host PATH unchanged for passthrough;
/// otherwise the host user's bin directory, when inherited, before the system
/// fallback.
fn inherited_path(
    inherit: &[Inherit],
    collected: &[EnvVar],
    host_home: &str,
    home_dir: Option<&str>,
) -> Option<String> {
    if inherit.contains(&Inherit::Passthrough) {
        return lookup_env_var_value("PATH", collected);
    }

    let mut entries = Vec::new();

    if inherit.contains(&Inherit::User) {
        let user_bin = expand_tilde_var(mapping_fallback(HOME_DATA_BIN_ENV_VAR.key), host_home);
        entries.push(std::path::PathBuf::from(user_bin));
    }

    let system = mapping_fallback(HOME_PATH_ENV_VAR.key);
    let system = home_dir.map_or_else(|| system.to_string(), |home| expand_tilde_var(system, home));
    entries.extend(std::env::split_paths(&system));

    std::env::join_paths(entries)
        .ok()
        .and_then(|path| path.into_string().ok())
}

/// Links the session's macOS Library to the host, where programs keep their state.
///
/// Caches link to `$HOME_CACHE`. Keychains link to the host's, because the login
/// keychain lives in the real `Library/Keychains`, and Claude Code reads its stored
/// credentials from there.
fn link_mac_library(host_home: &str, resolved: &[EnvVar]) {
    let value = |key: &str| {
        resolved
            .iter()
            .find(|env_var| env_var.key == key)
            .and_then(|env_var| env_var.value.clone())
            .filter(|value| !value.is_empty())
    };

    let Some(home) = value("HOME_DIR") else {
        return;
    };

    let library = std::path::Path::new(&home).join("Library");
    let library_path = library.to_string_lossy().into_owned();

    if !HostProcess.create_dir(&library_path) {
        eprintln!("[kraf] warning: failed to create {library_path}");
        return;
    }

    if let Some(cache) = value("HOME_CACHE") {
        let caches = library.join("Caches").to_string_lossy().into_owned();

        if !HostProcess.create_dir_link(&cache, &caches) {
            eprintln!("[kraf] warning: failed to link {caches} to $HOME_CACHE at {cache}");
        }
    }

    let keychains = library.join("Keychains").to_string_lossy().into_owned();
    let host_keychains = std::path::Path::new(host_home)
        .join("Library")
        .join("Keychains")
        .to_string_lossy()
        .into_owned();

    if !HostProcess.create_dir_link(&host_keychains, &keychains) {
        eprintln!("[kraf] warning: failed to link {keychains} to {host_keychains}");
    }
}

fn main() {
    println!("[kraf] trap init");

    let inherit = parse_inherit();

    let collected = env_var_collect(HOST_ENV_VAR_MAPPINGS, &HostEnvVarReader);

    let host_home = resolve(HOST_ENV_VAR_MAPPINGS, &collected, &[])
        .into_iter()
        .find(|env_var| env_var.key == HOST_HOME_ENV_VAR.key)
        .and_then(|env_var| env_var.value)
        .filter(|value| !value.is_empty())
        .expect("[kraf] error: $HOST_HOME is required, failed to resolve (internal)");

    let home_dir = HostProcess.create_session_dir(&host_home);

    let provided = [
        EnvVar {
            key: WORK_DIR_ENV_VAR.key,
            value: HostProcess.get_current_dir(),
        },
        EnvVar {
            key: HOME_PATH_ENV_VAR.key,
            value: inherited_path(&inherit, &collected, &host_home, home_dir.as_deref()),
        },
        EnvVar {
            key: HOME_DIR_ENV_VAR.key,
            value: home_dir,
        },
    ];

    let resolved = resolve(HOST_ENV_VAR_MAPPINGS, &collected, &provided);

    // Session directories that programs expect to exist, such as TEMP.
    for key in [
        "HOME_TEMP",
        "HOME_CONFIG",
        "HOME_DATA",
        "HOME_DATA_BIN",
        "HOME_CACHE",
        "HOME_STATE",
        "HOME_SESSION",
    ] {
        let dir = resolved
            .iter()
            .find(|env_var| env_var.key == key)
            .and_then(|env_var| env_var.value.clone())
            .filter(|value| !value.is_empty());

        if let Some(dir) = dir
            && !HostProcess.create_dir(&dir)
        {
            eprintln!("[kraf] warning: failed to create ${key} at {dir}");
        }
    }

    if cfg!(host = "mac") {
        link_mac_library(&host_home, &resolved);
    }

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

    // Temporary: mark the cmd prompt as a Kraf session, keeping the host prompt.
    if cfg!(windows) {
        let prompt = resolved
            .iter()
            .find(|env_var| env_var.key == "PROMPT")
            .and_then(|env_var| env_var.value.clone())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "$P$G".to_string());

        child.write("PROMPT", &format!("[kraf] {prompt}"));
    }

    println!("[kraf] trap ready");

    let status = child
        .status()
        .unwrap_or_else(|error| panic!("[kraf] failed to launch terminal: {error}"));

    std::process::exit(status.code().unwrap_or(1));
}
