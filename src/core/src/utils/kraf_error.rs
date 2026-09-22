use std::fmt;

#[derive(Debug)]
pub struct KrafError {
    pub message: String,
}

impl fmt::Display for KrafError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for KrafError {}
