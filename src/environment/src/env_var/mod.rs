pub mod env_var_core;
pub mod env_var_host;
pub mod env_var_resolver;

#[cfg(any(target_os = "linux", test))]
pub mod env_var_host_linux;
#[cfg(any(target_os = "macos", test))]
pub mod env_var_host_macos;
#[cfg(any(target_os = "windows", test))]
pub mod env_var_host_windows;

#[cfg(test)]
mod tests;
