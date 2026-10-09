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

use nox_shield_core::custody::{generate_phrase, phrase_to_seed, Phrase};
use nox_shield_core::error::CustodyError;
use nox_shield_core::keys::Account;

/// The BIP39 and BIP32 reference vectors are pinned in nonos_hd's own suite,
/// which is the crate that implements them. What matters here is that this
/// wallet's keys are a pure function of the phrase and nothing else.
#[test]
fn one_phrase_always_gives_one_account() {
    let phrase = generate_phrase().expect("entropy");
    let words = phrase.words();
    let first = Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account");
    let again = Phrase::parse(&words).expect("our own words parse");
    let second = Account::from_seed(&phrase_to_seed(&again).expect("seed")).expect("account");
    assert_eq!(first.address(), second.address());
}

#[test]
fn two_phrases_give_two_accounts() {
    let a = generate_phrase().expect("entropy");
    let b = generate_phrase().expect("entropy");
    let one = Account::from_seed(&phrase_to_seed(&a).expect("seed")).expect("account");
    let two = Account::from_seed(&phrase_to_seed(&b).expect("seed")).expect("account");
    assert_ne!(one.address(), two.address());
}

/// The BIP39 vector for 32 bytes of zero entropy. A random word mutation keeps the checksum
/// one time in 256, so this pair is fixed: the official vector must parse, and the same
/// phrase with a different last word must not.
#[test]
fn the_zero_entropy_vector_parses_and_its_neighbour_does_not() {
    let mut words: Vec<String> = vec!["abandon".to_string(); 23];
    words.push("art".to_string());
    Phrase::parse(&words).expect("the BIP39 zero entropy vector");

    let mut broken = words.clone();
    broken[23] = "abandon".to_string();
    assert!(matches!(Phrase::parse(&broken), Err(CustodyError::Mnemonic)));
}

/// A word outside the list is refused before any checksum arithmetic.
#[test]
fn a_word_outside_the_list_is_refused() {
    let mut words: Vec<String> = vec!["abandon".to_string(); 23];
    words.push("notaword".to_string());
    assert!(matches!(Phrase::parse(&words), Err(CustodyError::Mnemonic)));
}

#[test]
fn a_phrase_of_the_wrong_length_is_refused() {
    let phrase = generate_phrase().expect("entropy");
    let short: Vec<String> = phrase.words().into_iter().take(13).collect();
    assert!(matches!(Phrase::parse(&short), Err(CustodyError::PhraseLength)));
}

/// The BIP-39 vectors of all-zero entropy at 12, 18 and 24 words, so a phrase from any
/// other wallet restores.
#[test]
fn every_standard_length_is_read() {
    for (count, last) in [(12, "about"), (18, "agent"), (24, "art")] {
        let mut words = vec!["abandon".to_string(); count - 1];
        words.push(last.to_string());
        let phrase = Phrase::parse(&words).expect("a standard phrase");
        assert_eq!(phrase.words(), words);
    }
}
