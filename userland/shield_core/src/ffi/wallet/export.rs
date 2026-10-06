//! Exporting keys of one account, each from the vault opened again, so the keystore confirms the
//! owner. The spend secret alone is never exported: the words are its only backup.

use super::Wallet;
use crate::custody::{Seed, Vault};
use crate::error::{CustodyError, WalletError};
use crate::evm::EvmAccount;
use crate::keys::view_key_text::view_key_text;
use crate::keys::{Account, ViewKind};

impl Wallet {
    /// The private key of the public account `index`, for another wallet. It shows nothing of the
    /// shield.
    pub fn export_public_key(&self, index: u32) -> Result<String, WalletError> {
        if index >= self.with(|s| s.count())? {
            return Err(WalletError::NoSuchAccount);
        }
        let opened = Vault::at(self.paths().vault.clone()).open_all(self.guard())?;
        let account = match opened.key.as_deref() {
            Some(key) => EvmAccount::from_key(key),
            None => EvmAccount::at(opened.seed.bytes(), index),
        }
        .ok_or(CustodyError::Mnemonic)?;
        Ok(account.exported().to_string())
    }

    /// A view key of account `index`, refused unless the build has `view-key-export`.
    pub fn export_view_key(&self, index: u32, kind: ViewKind) -> Result<String, WalletError> {
        if !view_key_export() {
            return Err(WalletError::Unavailable);
        }
        let seed = self.confirmed_seed(index)?;
        let account = Account::at(&seed, index)?;
        Ok(view_key_text(&account, kind).to_string())
    }
}

/// Whether this build exports view keys, so a screen can leave the rows out.
pub fn view_key_export() -> bool {
    cfg!(feature = "view-key-export")
}

impl Wallet {
    /// The seed, from a fresh unwrap the owner confirms, for an account the wallet shows.
    fn confirmed_seed(&self, index: u32) -> Result<Seed, WalletError> {
        if index >= self.with(|s| s.count())? {
            return Err(WalletError::NoSuchAccount);
        }
        Ok(Vault::at(self.paths().vault.clone()).load(self.guard())?)
    }
}
