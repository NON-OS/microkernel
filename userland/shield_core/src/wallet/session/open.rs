//! Creating a wallet and unlocking one. The seed is dropped before either returns.

use super::book::Book;
use super::slot::Slot;
use super::Session;
use crate::custody::{generate_phrase, phrase_to_seed, HardwareGuard, Phrase, Seed, Vault};
use crate::error::WalletError;
use crate::wallet::paths::Paths;
use crate::wallet::watch::Watching;

impl Session {
    /// Create a wallet. The phrase is returned once for display, and kept only in the vault.
    pub fn create(
        paths: &Paths,
        guard: &dyn HardwareGuard,
    ) -> Result<(Session, Phrase), WalletError> {
        let phrase = generate_phrase()?;
        let seed = phrase_to_seed(&phrase)?;
        let session = super::install::install(paths, guard, &phrase, seed)?;
        Ok((session, phrase))
    }
}

impl Session {
    /// Open an existing wallet. The guard unwrap is where the device authenticates the user.
    pub fn unlock(paths: &Paths, guard: &dyn HardwareGuard) -> Result<Session, WalletError> {
        let opened = Vault::at(paths.vault.clone()).open_all(guard)?;
        build(paths, &opened.seed, opened.key.as_deref())
    }

    /// Whether a wallet is already stored at these paths.
    pub fn stored(paths: &Paths) -> bool {
        Vault::at(paths.vault.clone()).exists()
    }
}

/// Open every account the book names, each from its own store.
pub(super) fn build(
    paths: &Paths,
    seed: &Seed,
    key: Option<&[u8; 32]>,
) -> Result<Session, WalletError> {
    let book = Book::read(&paths.account);
    let first = match key {
        Some(key) => Slot::imported(paths, seed, key)?,
        None => Slot::open(paths, seed, 0)?,
    };
    let more = (1..book.count).map(|i| Slot::open(paths, seed, i)).collect::<Result<_, _>>()?;
    let watching = Watching::open(paths, seed)?;
    Ok(Session {
        first,
        more,
        active: book.active,
        watching,
        candidates: Vec::new(),
        leaves_seen: 0,
        imported: key.is_some(),
        next_public: key.is_none().then(|| next_public(seed, book.count)).flatten(),
        ahead: key.map_or_else(|| ahead(paths, seed, book.count), |_| Vec::new()),
    })
}

/// The addresses mode 1 of OP_FRESH_ADDRESS may hand out (v1.4.52).
fn ahead(paths: &Paths, seed: &Seed, count: u32) -> Vec<(u32, [u8; 20])> {
    let highest = crate::wallet::fresh::highest_handed(&paths.account);
    crate::wallet::fresh::derive_ahead(
        seed.bytes(),
        crate::wallet::fresh::first_once(count, highest),
    )
}

/// The public address of account `index`, next after the last shown.
pub(super) fn next_public(seed: &Seed, index: u32) -> Option<[u8; 20]> {
    crate::evm::EvmAccount::at(seed.bytes(), index).map(|a| a.address())
}
