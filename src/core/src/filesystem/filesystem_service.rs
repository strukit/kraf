use super::{FileSystemProvider, Uri};
use crate::KrafError;

pub struct FileSystemService {
    providers: Vec<(String, Box<dyn FileSystemProvider>)>,
}

impl FileSystemService {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn register(&mut self, scheme: impl Into<String>, provider: Box<dyn FileSystemProvider>) {
        self.providers.push((scheme.into(), provider));
    }

    pub fn resolve(&self, uri: &Uri) -> Result<&dyn FileSystemProvider, KrafError> {
        self.providers
            .iter()
            .find(|(scheme, _)| scheme == &uri.scheme())
            .map(|(_, provider)| provider.as_ref())
            .ok_or_else(|| KrafError {
                message: format!("no provider registered for scheme: {}", uri.scheme()),
            })
    }
}

impl Default for FileSystemService {
    fn default() -> Self {
        Self::new()
    }
}
