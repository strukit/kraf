use super::{FileStat, FileType, Uri};
use crate::KrafError;

pub trait FileSystemProvider {
    fn stat(&self, uri: &Uri) -> Result<FileStat, KrafError>;
    fn read_directory(&self, uri: &Uri) -> Result<Vec<(String, FileType)>, KrafError>;
    fn read_file(&self, uri: &Uri) -> Result<Vec<u8>, KrafError>;
    fn write_file(&self, uri: &Uri, content: &[u8]) -> Result<(), KrafError>;
    fn digest(&self, uri: &Uri) -> Result<String, KrafError>;
}
