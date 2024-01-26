#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bip39 error: `{0}`")]
    Bip39(String),
    #[error("Secp256k1 error: `{0}`")]
    Secp256k1(#[from] secp256k1::Error),
    #[error("Aes256Gcm error: `{0}`")]
    Aes256Gcm(String),
    #[error("HD Invalid Child Number")]
    HDInvalidChildNumber,
    #[error("HD Invalid Derivation Path")]
    HDInvalidDerivationPath,
    #[error("HD Invalid Extended PrivKey")]
    HDInvalidExtendedPrivKey,
    #[error("HD Secp256k1 error: `{0}`")]
    HDSecp256k1(String),
}

impl From<tiny_hderive::Error> for Error {
    fn from(value: tiny_hderive::Error) -> Self {
        match value {
            tiny_hderive::Error::Secp256k1(e) => Self::HDSecp256k1(e.to_string()),
            tiny_hderive::Error::InvalidChildNumber => Self::HDInvalidChildNumber,
            tiny_hderive::Error::InvalidDerivationPath => Self::HDInvalidDerivationPath,
            tiny_hderive::Error::InvalidExtendedPrivKey => Self::HDInvalidExtendedPrivKey,
        }
    }
}

impl From<aes_gcm::Error> for Error {
    fn from(value: aes_gcm::Error) -> Self {
        Self::Aes256Gcm(value.to_string())
    }
}
