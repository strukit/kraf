use std::path::PathBuf;
use url::Url;

pub struct Uri {
    raw: String,
}

impl Uri {
    pub fn parse(input: &str) -> Result<Self, url::ParseError> {
        Url::parse(input)?;
        Ok(Self {
            raw: input.to_string(),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }

    pub fn to_file_path(&self) -> Option<PathBuf> {
        Url::parse(&self.raw).ok()?.to_file_path().ok()
    }

    pub fn scheme(&self) -> String {
        Url::parse(&self.raw)
            .map(|url| url.scheme().to_string())
            .unwrap_or_default()
    }
}
