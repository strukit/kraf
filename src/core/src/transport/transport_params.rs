pub struct TransportParams<T> {
    pub values: T,
}

impl TransportParams<Vec<(String, String)>> {
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values
            .iter()
            .find(|(entry_key, _)| entry_key == key)
            .map(|(_, value)| value)
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();

        if self.has(&key) {
            tracing::warn!(key = %key, "duplicate key in transport params");
        }

        self.values.push((key, value.into()));
    }

    #[must_use]
    pub fn has(&self, key: &str) -> bool {
        self.values.iter().any(|(entry_key, _)| entry_key == key)
    }
}

impl From<Vec<(String, String)>> for TransportParams<Vec<(String, String)>> {
    fn from(values: Vec<(String, String)>) -> Self {
        Self { values }
    }
}
