use std::time::SystemTime;

pub enum FileType {
    Unknown,
    File,
    Directory,
    SymbolicLink,
}

pub struct FileStat {
    pub kind: FileType,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub size: u64,
}
