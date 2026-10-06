/*
 * A test asserts by panicking, and a measurement prints what it read, so the
 * lints that forbid panicking and indexing are off here and nowhere else.
 */
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod keystore;

use keystore::{scratch, RefusingGuard, TestGuard};
use nox_shield_core::custody::{generate_phrase, phrase_to_seed, Vault};
use nox_shield_core::error::CustodyError;
use nox_shield_core::keys::Account;

#[test]
fn a_sealed_seed_round_trips_and_a_refusing_guard_does_not_open_it() {
    let dir = scratch("vault");
    let vault = Vault::at(dir.join("vault.seal"));
    let phrase = generate_phrase().expect("entropy");
    let seed = phrase_to_seed(&phrase).expect("seed");
    vault.store(&seed, &TestGuard).expect("store");
    let opened = vault.load(&TestGuard).expect("load");
    let expected = Account::from_seed(&seed).expect("account").address();
    assert_eq!(Account::from_seed(&opened).expect("account").address(), expected);
    assert!(matches!(vault.load(&RefusingGuard), Err(CustodyError::Guard)));
    assert_eq!(vault.store(&seed, &TestGuard), Err(CustodyError::VaultPresent));
    assert!(vault.load_words(&TestGuard).expect("a version 1 vault opens").is_none());
}

/// A version 2 vault opens to the same seed and gives back the words, and only to the device.
#[test]
fn a_vault_with_words_gives_them_back() {
    let dir = scratch("vault-words");
    let vault = Vault::at(dir.join("vault.seal"));
    let phrase = generate_phrase().expect("entropy");
    let seed = phrase_to_seed(&phrase).expect("seed");
    vault.store_with_words(&seed, &phrase, &TestGuard).expect("store");
    let opened = vault.load(&TestGuard).expect("load");
    let expected = Account::from_seed(&seed).expect("account").address();
    assert_eq!(Account::from_seed(&opened).expect("account").address(), expected);
    let words = vault.load_words(&TestGuard).expect("load").expect("words");
    assert_eq!(words.words(), phrase.words());
    assert!(matches!(vault.load_words(&RefusingGuard), Err(CustodyError::Guard)));
}
