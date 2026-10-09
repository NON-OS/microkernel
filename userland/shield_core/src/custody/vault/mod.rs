//! The seed at rest, sealed under a file key that the platform keystore wraps.
//! Reading it needs the file and the device.

mod destroy;
mod load;
mod store;
mod words;

pub use load::Opened;

use crate::error::CustodyError;
use nonos_seal::NONCE_LEN;
use std::path::{Path, PathBuf};

/// The file key is fresh per vault, so a single fixed nonce is used once.
pub(super) const NONCE: [u8; NONCE_LEN] = [0u8; NONCE_LEN];

/// A vault at a path. Holding one touches no file.
pub struct Vault {
    path: PathBuf,
}

impl Vault {
    /// Address a vault at a path. Nothing is read or written until `store` or `load`.
    pub fn at(path: impl Into<PathBuf>) -> Vault {
        Vault { path: path.into() }
    }

    /// Whether a vault is already stored here.
    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    pub(super) fn read_blob(&self) -> Result<Vec<u8>, CustodyError> {
        std::fs::read(&self.path).map_err(|_| CustodyError::VaultShape)
    }

    pub(super) fn write_blob(&self, blob: &[u8]) -> Result<(), CustodyError> {
        std::fs::write(&self.path, blob).map_err(|_| CustodyError::VaultShape)
    }
}
