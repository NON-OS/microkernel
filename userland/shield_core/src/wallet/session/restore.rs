//! Restoring a wallet from a phrase somebody typed back in.
//!
//! It is the same install path as creating one, with the phrase coming from
//! the user instead of the random source, so a restore cannot end up with a
//! differently derived account than a create.

use super::Session;
use crate::custody::{parse_key, phrase_to_seed, HardwareGuard, Phrase};
use crate::error::WalletError;
use crate::wallet::paths::Paths;

impl Session {
    /// Restore a wallet from a phrase the user typed back in.
    pub fn restore(
        paths: &Paths,
        guard: &dyn HardwareGuard,
        words: &[String],
    ) -> Result<Session, WalletError> {
        let phrase = Phrase::parse(words)?;
        let seed = phrase_to_seed(&phrase)?;
        super::install::install_and_derive(paths, guard, &phrase, seed)
    }

    /// Restore a wallet from a private key the user typed or pasted.
    pub fn restore_key(
        paths: &Paths,
        guard: &dyn HardwareGuard,
        key: &str,
    ) -> Result<Session, WalletError> {
        let key = parse_key(key)?;
        super::install::install_key(paths, guard, &key)
    }
}
