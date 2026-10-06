//! The note secrets a deposit hands the pool are canonical Goldilocks elements, by rejection.
//!
//! The pool reverts `NonCanonicalFieldElement` on a `spendPk` or `blinding` at or above the
//! modulus, and masking would skew the distribution. The sampler rejects a word at or above
//! `P` and redraws. These pin both values below `P` over enough draws to expose masking.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use nonos_stark::field::P;
use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::keys::Account;
use nox_shield_core::notes::fresh_blinding;

fn account() -> Account {
    let phrase = generate_phrase().expect("entropy");
    Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account")
}

#[test]
fn every_fresh_blinding_word_is_canonical() {
    for _ in 0..2048 {
        for word in fresh_blinding().expect("entropy") {
            assert!(word < P, "a blinding word is at or above the modulus");
        }
    }
}

#[test]
fn the_spend_key_the_address_publishes_is_canonical() {
    for word in account().address().spend_pk {
        assert!(word < P, "a spend key word is at or above the modulus");
    }
}
