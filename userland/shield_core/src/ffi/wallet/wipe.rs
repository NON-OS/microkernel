//! Destroying the wallet on this device, the only entry point that loses money. It removes the
//! sealed seed, every note log, the view keys and the account records. The platform key is the
//! shell's to remove after this, and without it any leftover copy is noise.

use super::Wallet;
use crate::custody::Vault;
use crate::error::WalletError;

impl Wallet {
    /// Drop the session and destroy the seed and notes here. True when there was a
    /// wallet, and absent is success. The recovery phrase is the only way back.
    pub fn wipe(&self) -> Result<bool, WalletError> {
        self.drop_pending();
        *self.held()? = None;
        let vault = Vault::at(self.paths().vault.clone());
        let existed = vault.exists();
        vault.destroy()?;
        for index in 0..crate::wallet::MAX_ACCOUNTS {
            remove(&self.paths().notes_of(index))?;
        }
        remove(&self.paths().watching)?;
        remove_watched_logs(&self.paths().notes)?;
        remove(&self.paths().measurement)?;
        if self.paths().account.exists() {
            std::fs::remove_dir_all(&self.paths().account)
                .map_err(|_| WalletError::Store { source: crate::error::StoreError::Io })?;
        }
        Ok(existed)
    }
}

/// Every view-only log beside the note log, found by its name.
fn remove_watched_logs(notes: &std::path::Path) -> Result<(), WalletError> {
    let Some(dir) = notes.parent() else { return Ok(()) };
    let Ok(entries) = std::fs::read_dir(dir) else { return Ok(()) };
    for entry in entries.flatten() {
        let name = entry.file_name();
        if name.to_string_lossy().starts_with("notes-v") {
            remove(&entry.path())?;
        }
    }
    Ok(())
}

/// Remove a file this app wrote, absent counting as done. A file that stays is a failure.
fn remove(path: &std::path::Path) -> Result<(), WalletError> {
    if !path.exists() {
        return Ok(());
    }
    std::fs::remove_file(path)
        .map_err(|_| WalletError::Store { source: crate::error::StoreError::Io })
}
