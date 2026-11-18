use sha2::{Digest, Sha256};
use sha3::Sha3_256;

// Each digest starts with its algorithm name, so digests of different algorithms never look equal.
#[derive(Clone, Copy)]
pub enum DigestAlgorithm {
    Sha2,
    Sha3,
    None,
}

impl DigestAlgorithm {
    // SHA2 is the default: it is the fastest measured on the CI runners.
    pub fn default_algorithm() -> Self {
        Self::Sha2
    }

    fn prefix(self) -> &'static str {
        match self {
            Self::Sha2 => "sha256",
            Self::Sha3 => "sha3-256",
            Self::None => "none",
        }
    }
}

enum State {
    Sha2(Sha256),
    Sha3(Sha3_256),
    None,
}

pub struct Hasher {
    algorithm: DigestAlgorithm,
    state: State,
}

impl Hasher {
    pub fn new(algorithm: DigestAlgorithm) -> Self {
        let state = match algorithm {
            DigestAlgorithm::Sha2 => State::Sha2(Sha256::new()),
            DigestAlgorithm::Sha3 => State::Sha3(Sha3_256::new()),
            DigestAlgorithm::None => State::None,
        };

        Self { algorithm, state }
    }

    pub fn update(&mut self, chunk: &[u8]) {
        match &mut self.state {
            State::Sha2(hasher) => hasher.update(chunk),
            State::Sha3(hasher) => hasher.update(chunk),
            State::None => {}
        }
    }

    pub fn finish(self) -> String {
        let bytes = match self.state {
            State::Sha2(hasher) => hasher.finalize().to_vec(),
            State::Sha3(hasher) => hasher.finalize().to_vec(),
            State::None => return "none".to_string(),
        };

        let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();

        format!("{}:{hex}", self.algorithm.prefix())
    }
}

pub fn digest(algorithm: DigestAlgorithm, content: &[u8]) -> String {
    let mut hasher = Hasher::new(algorithm);
    hasher.update(content);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::{DigestAlgorithm, digest};

    #[test]
    fn sha2_digest_has_its_prefix() {
        assert_eq!(
            digest(DigestAlgorithm::Sha2, b"abc"),
            "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn sha3_digest_has_its_prefix() {
        assert_eq!(
            digest(DigestAlgorithm::Sha3, b"abc"),
            "sha3-256:3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
        );
    }
}
