//! Destroying a vault. This loses money and cannot be undone.
//! The file is overwritten, then unlinked. Flash may keep old blocks, so the real defence is the
//! shell destroying the wrapping key, which leaves any surviving ciphertext as noise.

use super::Vault;
use crate::error::CustodyError;

impl Vault {
    /// Overwrite and remove the vault. An absent vault is success.
    pub fn destroy(&self) -> Result<(), CustodyError> {
        if !self.exists() {
            return Ok(());
        }
        let length = std::fs::metadata(&self.path).map_err(|_| CustodyError::VaultShape)?.len();
        let length = usize::try_from(length).map_err(|_| CustodyError::VaultShape)?;
        self.write_blob(&vec![0u8; length])?;
        std::fs::remove_file(&self.path).map_err(|_| CustodyError::VaultShape)
    }
}
