//! The vault handle: a path, and the raw blob read from or written to it.

use crate::error::CustodyError;
use std::path::{Path, PathBuf};

/// A vault at a path. Holding one touches no file.
pub struct Vault {
    pub(super) path: PathBuf,
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
