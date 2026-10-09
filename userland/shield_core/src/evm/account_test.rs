/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

use super::test_account;
use crate::custody::Seed;
use crate::evm::checksummed;
use crate::keys::Account;

/// The account every standard wallet derives at m/44'/60'/0'/0/0 from the
/// BIP-39 test phrase "abandon ... about", the same in every standard wallet.
const ADDRESS: &str = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";

#[test]
fn the_standard_phrase_derives_the_standard_address() {
    assert_eq!(checksummed(&test_account().address()), ADDRESS);
}

/// The shielded keys come from BLAKE3 derive-key and the account key from
/// BIP-32, and neither takes the other's output. The test checks that the two
/// derivations of one seed land on unrelated values, in both directions.
#[test]
fn the_account_key_and_the_shield_keys_are_separate_domains() {
    let mut words = [0u16; 12];
    words[11] = 3;
    let mut bytes = [0u8; 64];
    assert!(nonos_hd::bip39::seed_from_words(&words, b"", &mut bytes));
    let shield = Account::from_seed(&Seed::new(bytes)).unwrap();
    let evm = test_account().secret();
    let spend: Vec<u8> = shield.sk().iter().flat_map(|f| f.value().to_le_bytes()).collect();
    let view = shield.address().view_pk;
    for window in spend.windows(8).chain(view.windows(8)) {
        assert!(!evm.windows(8).any(|w| w == window), "a shield key shares bytes with the account");
    }
    let from_evm = Account::from_seed(&Seed::new(evm_as_seed(evm))).unwrap();
    assert_ne!(from_evm.address().spend_pk, shield.address().spend_pk);
}

/// The account key padded to a seed, to show the shield keys are not what
/// the account key derives to either.
fn evm_as_seed(key: [u8; 32]) -> [u8; 64] {
    let mut out = [0u8; 64];
    out[..32].copy_from_slice(&key);
    out
}
