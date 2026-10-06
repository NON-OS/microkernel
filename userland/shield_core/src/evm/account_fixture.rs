//! The account of the public test phrase, for the tests that sign.

use super::EvmAccount;

/// The account of the public BIP-39 test phrase "abandon" eleven times then
/// "about", for tests that sign. Every wallet's test suite knows its key.
#[allow(clippy::unwrap_used, clippy::indexing_slicing)]
pub(crate) fn test_account() -> EvmAccount {
    let mut words = [0u16; 12];
    words[11] = 3;
    let mut seed = [0u8; 64];
    assert!(nonos_hd::bip39::seed_from_words(&words, b"", &mut seed));
    EvmAccount::from_seed(&seed).unwrap()
}
