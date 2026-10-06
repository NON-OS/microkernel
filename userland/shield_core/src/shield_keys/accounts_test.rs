/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

//! The vectors of accounts 0, 1 and 2 for the phrase "abandon" eleven times then "about", for an
//! independent implementation to reproduce. A change here is a change to every wallet.

use super::receiving_address::receiving_address_text;
use super::Account;
use crate::custody::Seed;
use crate::evm::{checksummed, EvmAccount};

#[path = "accounts_vectors.rs"]
mod accounts_vectors;
use accounts_vectors::VECTORS;

pub(super) fn seed() -> Seed {
    let mut words = [0u16; 12];
    words[11] = 3;
    let mut bytes = [0u8; 64];
    assert!(nonos_hd::bip39::seed_from_words(&words, b"", &mut bytes));
    Seed::new(bytes)
}

fn hex(words: [u64; 4]) -> String {
    words.iter().map(|w| format!("{w:016x}")).collect()
}

/// What one account shows: `spend_pk`, `nk`, BLAKE3 of the X-Wing key, the public address.
fn row(index: u32) -> [String; 4] {
    let seed = seed();
    let account = Account::at(&seed, index).unwrap();
    let words = |f: [nonos_stark::field::Fp; 4]| hex(f.map(|e| e.value()));
    let ek = account.receive().encapsulation_key();
    let evm = EvmAccount::at(seed.bytes(), index).unwrap();
    [
        words(account.spend_pk()),
        words(account.nk()),
        blake3::hash(&ek).to_hex().to_string(),
        checksummed(&evm.address()),
    ]
}

#[test]
fn accounts_zero_to_two_match_the_vectors() {
    for (i, expected) in (0u32..).zip(VECTORS.iter()) {
        let account = Account::at(&seed(), i).unwrap();
        let nox1 = receiving_address_text(&account.receiving_address());
        let got = row(i);
        assert_eq!(got.each_ref().map(String::as_str), expected[..4], "account {i}");
        assert_eq!(blake3::hash(nox1.as_bytes()).to_hex().as_str(), expected[4], "account {i}");
    }
}

/// Each index names a different account in every key, the public one included.
#[test]
fn no_two_accounts_share_a_key() {
    let rows: Vec<[String; 4]> = (0..4).map(row).collect();
    for (i, a) in rows.iter().enumerate() {
        for b in rows.iter().skip(i + 1) {
            assert!(a.iter().zip(b.iter()).all(|(x, y)| x != y));
        }
    }
}
