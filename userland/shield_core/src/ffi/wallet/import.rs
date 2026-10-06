//! A wallet from a private key: its public account is the key, and it has no words.

use super::Wallet;
use crate::error::WalletError;
use crate::wallet::Session;

impl Wallet {
    /// Restore from a private key, a wallet with one account and no words.
    pub fn restore_from_key(&self, key: String) -> Result<(), WalletError> {
        self.drop_pending();
        let session = Session::restore_key(self.paths(), self.guard(), &key)?;
        self.follow(session.active());
        *self.held()? = Some(session);
        Ok(())
    }

    /// Whether this wallet came from a private key, so it has no words and one account.
    pub fn from_key(&self) -> Result<bool, WalletError> {
        self.with(|s| s.imported)
    }
}
