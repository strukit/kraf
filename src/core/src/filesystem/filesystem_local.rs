use std::fs;

use super::{FileStat, FileSystemProvider, FileType, Uri};
use crate::KrafError;
use sha2::{Digest, Sha256};

pub struct FileSystemLocal;

impl FileSystemLocal {
    fn resolve(uri: &Uri) -> Result<std::path::PathBuf, KrafError> {
        uri.to_file_path().map_err(|()| KrafError {
            message: format!("uri não é um caminho de arquivo local: {}", uri.as_str()),
        })
    }
}

impl FileSystemProvider for FileSystemLocal {
    fn stat(&self, uri: &Uri) -> Result<FileStat, KrafError> {
        let path = Self::resolve(uri)?;

        let metadata = fs::metadata(&path).map_err(|error| KrafError {
            message: error.to_string(),
        })?;

        let kind = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.is_symlink() {
            FileType::SymbolicLink
        } else if metadata.is_file() {
            FileType::File
        } else {
            FileType::Unknown
        };

        Ok(FileStat {
            kind,
            created: metadata.created().map_err(|error| KrafError {
                message: error.to_string(),
            })?,
            modified: metadata.modified().map_err(|error| KrafError {
                message: error.to_string(),
            })?,
            size: metadata.len(),
        })
    }

    fn read_directory(&self, uri: &Uri) -> Result<Vec<(String, FileType)>, KrafError> {
        let path = Self::resolve(uri)?;

        let entries = fs::read_dir(&path).map_err(|error| KrafError {
            message: error.to_string(),
        })?;

        let mut result = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|error| KrafError {
                message: error.to_string(),
            })?;

            let file_type = entry.file_type().map_err(|error| KrafError {
                message: error.to_string(),
            })?;

            let kind = if file_type.is_dir() {
                FileType::Directory
            } else if file_type.is_symlink() {
                FileType::SymbolicLink
            } else if file_type.is_file() {
                FileType::File
            } else {
                FileType::Unknown
            };

            result.push((entry.file_name().to_string_lossy().into_owned(), kind));
        }

        Ok(result)
    }

    fn read_file(&self, uri: &Uri) -> Result<Vec<u8>, KrafError> {
        let path = Self::resolve(uri)?;

        fs::read(&path).map_err(|error| KrafError {
            message: error.to_string(),
        })
    }

    fn write_file(&self, uri: &Uri, content: &[u8]) -> Result<(), KrafError> {
        let path = Self::resolve(uri)?;

        fs::write(&path, content).map_err(|error| KrafError {
            message: error.to_string(),
        })
    }

    fn digest(&self, uri: &Uri) -> Result<String, KrafError> {
        let content = self.read_file(uri)?;

        Ok(format!("sha256:{:x}", Sha256::digest(content)))
    }
}
