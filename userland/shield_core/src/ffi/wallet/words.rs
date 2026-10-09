//! Showing the recovery words again, after the keystore confirms the owner. The screen that shows
//! them blocks screenshots and never copies them. A vault of version 1 kept only the seed, so it
//! has no words to show, and the shell says so.

use super::Wallet;
use crate::custody::Vault;
use crate::error::WalletError;

impl Wallet {
    /// The 24 words in order, or none when this vault never held them.
    pub fn recovery_words(&self) -> Result<Option<Vec<String>>, WalletError> {
        self.with(|_| ())?;
        let words = Vault::at(self.paths().vault.clone()).load_words(self.guard())?;
        Ok(words.map(|phrase| phrase.words()))
    }
}
