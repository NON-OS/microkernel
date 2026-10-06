//! Writing a new vault from a phrase, then opening the wallet it holds. The words are sealed with
//! the seed, so the owner can see them again after confirming.

use super::open::build;
use super::Session;
use crate::custody::{seed_of_key, HardwareGuard, Phrase, Seed, Vault};
use crate::error::WalletError;
use crate::wallet::paths::Paths;

/// Install the seed, then keep the accounts after account 0 for a search to look at.
pub(super) fn install_and_derive(
    paths: &Paths,
    guard: &dyn HardwareGuard,
    phrase: &Phrase,
    seed: Seed,
) -> Result<Session, WalletError> {
    Vault::at(paths.vault.clone()).store_with_words(&seed, phrase, guard)?;
    let mut session = build(paths, &seed, None)?;
    session.derive_candidates(paths, &seed)?;
    Ok(session)
}

pub(super) fn install(
    paths: &Paths,
    guard: &dyn HardwareGuard,
    phrase: &Phrase,
    seed: Seed,
) -> Result<Session, WalletError> {
    Vault::at(paths.vault.clone()).store_with_words(&seed, phrase, guard)?;
    build(paths, &seed, None)
}

/// Install a private key brought in, and open the account it gives.
pub(super) fn install_key(
    paths: &Paths,
    guard: &dyn HardwareGuard,
    key: &[u8; 32],
) -> Result<Session, WalletError> {
    Vault::at(paths.vault.clone()).store_key(key, guard)?;
    build(paths, &seed_of_key(key), Some(key))
}
