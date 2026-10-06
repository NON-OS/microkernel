//! Create, restore, unlock and lock. The phrase is returned once, kept only in the vault.

use super::Wallet;
use crate::error::WalletError;
use crate::ffi::AddressParts;
use crate::store::Balance;
use crate::wallet::Session;

impl Wallet {
    pub fn stored(&self) -> bool {
        Session::stored(self.paths())
    }

    /// Create a wallet and return its phrase once, for the shell to show and drop.
    pub fn create(&self) -> Result<Vec<String>, WalletError> {
        self.drop_pending();
        let (session, phrase) = Session::create(self.paths(), self.guard())?;
        self.follow(session.active());
        *self.held()? = Some(session);
        Ok(phrase.words())
    }

    /// Restore from a phrase the user typed back in.
    pub fn restore(&self, words: Vec<String>) -> Result<(), WalletError> {
        self.drop_pending();
        let session = Session::restore(self.paths(), self.guard(), &words)?;
        self.follow(session.active());
        *self.held()? = Some(session);
        Ok(())
    }

    /// Unlock the stored wallet, where the keystore authenticates the user.
    pub fn unlock(&self) -> Result<(), WalletError> {
        self.drop_pending();
        let session = Session::unlock(self.paths(), self.guard())?;
        self.follow(session.active());
        *self.held()? = Some(session);
        Ok(())
    }

    /// Drop the session, and with it every key derived from the seed. A read of the pool
    /// under way stops at its next request rather than run on for a store now closed.
    pub fn lock(&self) -> Result<(), WalletError> {
        self.stops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.drop_pending();
        *self.held()? = None;
        Ok(())
    }

    /// Whether a session is open, failing on a poisoned lock instead of guessing no.
    pub fn unlocked(&self) -> Result<bool, WalletError> {
        Ok(self.held()?.is_some())
    }

    pub fn address(&self) -> Result<String, WalletError> {
        Ok(self.with(|s| s.address())?.to_text())
    }

    /// The two keys this wallet's address carries, in hex, so a holder can check it.
    pub fn address_parts(&self) -> Result<AddressParts, WalletError> {
        Ok(AddressParts::of(&self.with(|s| s.address())?))
    }

    /// One balance per asset, ETH and NOX, from this wallet's own notes.
    pub fn balances(&self) -> Result<Vec<Balance>, WalletError> {
        self.with(|s| s.balances())
    }
}
