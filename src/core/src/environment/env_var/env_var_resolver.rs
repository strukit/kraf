use super::env_var_core::{EnvVar, EnvVarHostMapping, EnvVarImportMode, EnvVarSource};
use super::env_var_host::HOME_ENV_VAR;

#[must_use]
pub fn expand_tilde_var(env_var_value: &str, home_value: &str) -> String {
    env_var_value.replacen('~', home_value, 1)
}

fn extract_dollar_var_key(value: &str) -> Option<&str> {
    let after_dollar = &value[value.find('$')? + 1..];
    let end = after_dollar
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .unwrap_or(after_dollar.len());

    (end > 0).then(|| &after_dollar[..end])
}

#[must_use]
pub fn expand_dollar_var(env_var_key_raw: &str, env_var_collected: &[EnvVar]) -> String {
    let Some(env_var_key) = extract_dollar_var_key(env_var_key_raw) else {
        return env_var_key_raw.to_string();
    };

    let Some(env_var_value) = lookup_env_var_value(env_var_key, env_var_collected) else {
        return env_var_key_raw.to_string();
    };

    env_var_key_raw.replacen(&format!("${env_var_key}"), &env_var_value, 1)
}

#[must_use]
pub fn lookup_env_var_value(env_var_key: &str, env_var_collected: &[EnvVar]) -> Option<String> {
    env_var_collected
        .iter()
        .find(|env_var| env_var.key == env_var_key)
        .and_then(|env_var| env_var.value.clone())
}

fn resolve_expand(
    env_var_key_raw: &str,
    env_var_home: &str,
    env_var_collected: &[EnvVar],
) -> String {
    if env_var_key_raw.contains('~') && !env_var_home.is_empty() {
        return expand_tilde_var(env_var_key_raw, env_var_home);
    }

    if env_var_key_raw.contains('$') {
        return expand_dollar_var(env_var_key_raw, env_var_collected);
    }

    env_var_key_raw.to_string()
}

#[must_use]
pub fn resolve(
    env_var_mappings: &[EnvVarHostMapping],
    env_var_collected: &[EnvVar],
) -> Vec<EnvVar> {
    let home = lookup_env_var_value(HOME_ENV_VAR.key, env_var_collected).unwrap_or_default();

    env_var_mappings
        .iter()
        .flat_map(|env_var_unresolved| {
            let env_var_value = if env_var_unresolved.alias_import.is_empty() {
                let from_env_var_collected = (env_var_unresolved.source == EnvVarSource::Provided)
                    .then(|| lookup_env_var_value(env_var_unresolved.key, env_var_collected))
                    .flatten();

                from_env_var_collected.unwrap_or_else(|| {
                    resolve_expand(
                        env_var_unresolved.fallback.unwrap_or_default(),
                        &home,
                        env_var_collected,
                    )
                })
            } else {
                resolve_alias(env_var_unresolved, env_var_collected)
            };

            std::iter::once(EnvVar {
                key: env_var_unresolved.key,
                value: Some(env_var_value.clone()),
            })
            .chain(env_var_unresolved.alias_export.iter().copied().map(
                move |alias_export| EnvVar {
                    key: alias_export,
                    value: Some(env_var_value.clone()),
                },
            ))
        })
        .collect()
}

pub fn resolve_alias(
    env_var_unresolved: &EnvVarHostMapping,
    env_var_collected: &[EnvVar],
) -> String {
    let home = lookup_env_var_value(HOME_ENV_VAR.key, env_var_collected).unwrap_or_default();

    let value = match env_var_unresolved.alias_import_mode {
        EnvVarImportMode::FirstFound => env_var_unresolved
            .alias_import
            .iter()
            .find_map(|name| lookup_env_var_value(name, env_var_collected))
            .or_else(|| {
                Some(resolve_expand(
                    env_var_unresolved.fallback.unwrap_or_default(),
                    &home,
                    env_var_collected,
                ))
            }),

        EnvVarImportMode::PassThrough => env_var_unresolved
            .alias_import
            .iter()
            .find_map(|name| lookup_env_var_value(name, env_var_collected)),

        EnvVarImportMode::Merge => {
            let values: Vec<String> = env_var_unresolved
                .alias_import
                .iter()
                .filter_map(|name| lookup_env_var_value(name, env_var_collected))
                .filter(|val| !val.is_empty())
                .collect();

            let paths: Vec<std::path::PathBuf> =
                values.iter().flat_map(std::env::split_paths).collect();

            if paths.is_empty() {
                Some(resolve_expand(
                    env_var_unresolved.fallback.unwrap_or_default(),
                    &home,
                    env_var_collected,
                ))
            } else {
                std::env::join_paths(paths)
                    .ok()
                    .and_then(|joined| joined.into_string().ok())
            }
        }
    };

    value.unwrap_or_default()
}

pub struct EnvVarResolveKeys {
    pub env_vars_keys_import: Vec<&'static str>,
    pub env_vars_keys_export: Vec<&'static str>,
}

pub fn env_var_resolve_keys(env_var_mappings: &[EnvVarHostMapping]) -> EnvVarResolveKeys {
    let mut env_vars_keys_import = Vec::new();
    let mut env_vars_keys_export = Vec::new();

    for mapping in env_var_mappings {
        env_vars_keys_import.extend(mapping.alias_import.iter().copied());
        env_vars_keys_export.extend(mapping.alias_export.iter().copied());

        if let Some(dollar_key) = mapping.fallback.and_then(extract_dollar_var_key) {
            env_vars_keys_import.push(dollar_key);
        }
    }

    env_vars_keys_import.sort_unstable();
    env_vars_keys_import.dedup();
    env_vars_keys_export.sort_unstable();
    env_vars_keys_export.dedup();

    EnvVarResolveKeys {
        env_vars_keys_import,
        env_vars_keys_export,
    }
}
