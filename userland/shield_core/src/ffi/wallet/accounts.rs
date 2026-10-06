//! The accounts of the wallet as the shells see them: how many, which is active, adding one and
//! choosing one. Adding one unlocks the vault again, so the keystore confirms the owner.

use super::Wallet;
use crate::custody::Vault;
use crate::error::WalletError;
use crate::evm::checksummed;
use crate::ffi::AccountSummary;

impl Wallet {
    /// Every account in order, each with its public address. The active one is flagged.
    pub fn accounts(&self) -> Result<Vec<AccountSummary>, WalletError> {
        self.with(|s| {
            (0..s.count())
                .filter_map(|i| s.at(i).map(|slot| (i, slot)))
                .map(|(index, slot)| AccountSummary {
                    index,
                    public_address: checksummed(&slot.evm().address()),
                    active: index == s.active(),
                })
                .collect()
        })
    }

    /// Add the next account and make it active. The vault is opened again for its seed.
    pub fn add_account(&self) -> Result<u32, WalletError> {
        self.drop_pending();
        let seed = Vault::at(self.paths().vault.clone()).load(self.guard())?;
        let paths = self.paths().clone();
        let index = self.with_mut(|s| s.add(&paths, &seed))?;
        self.follow(index);
        Ok(index)
    }

    /// Make account `index` the active one. A send waiting for a yes is dropped.
    pub fn select_account(&self, index: u32) -> Result<(), WalletError> {
        self.drop_pending();
        let paths = self.paths().clone();
        self.with_mut(|s| s.select(&paths, index))?;
        self.follow(index);
        Ok(())
    }
}
