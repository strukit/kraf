#[derive(Debug, Clone)]
pub struct EnvVar {
    pub key: &'static str,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvVarSource {
    Host,
    Provided,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvVarImportMode {
    FirstFound,
    Merge,
    PassThrough,
}

#[derive(Debug, Clone, Copy)]
pub struct EnvVarHostMapping {
    pub key: &'static str,
    pub source: EnvVarSource,
    pub fallback: Option<&'static str>,
    pub alias_import: &'static [&'static str],
    pub alias_import_mode: EnvVarImportMode,
    pub alias_export: &'static [&'static str],
}
