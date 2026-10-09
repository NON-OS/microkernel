// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The keyring keeps sealed words beside an account only when they derive
//! it. Words kept beside another wallet's key gave the shield that other
//! wallet's private address, and further accounts its keys. Checked with the
//! BIP39 vector "abandon x11 about", whose account 0 is the well known
//! 0x9858EfFD232B4033E47d90003D41EC34EcaEda94.

use crate::server::words_own::{words_derive, MAX_ACCOUNT_INDEX};

const ABANDON: u16 = 0;
const ABOUT: u16 = 3;

fn vector() -> [u16; 12] {
    let mut w = [ABANDON; 12];
    w[11] = ABOUT;
    w
}

/// The vector's account key at m/44'/60'/0'/0/0.
const ACCOUNT_0: [u8; 32] = [
    0x1a, 0xb4, 0x2c, 0xc4, 0x12, 0xb6, 0x18, 0xbd, 0xea, 0x3a, 0x59, 0x9e, 0x3c, 0x9b, 0xae, 0x19,
    0x9e, 0xbf, 0x03, 0x08, 0x95, 0xb0, 0x39, 0xe9, 0xdb, 0x1e, 0x30, 0xda, 0xfb, 0x12, 0xb7, 0x27,
];

fn key_at(index: u32) -> [u8; 32] {
    let mut seed = [0u8; 64];
    assert!(nonos_hd::bip39::seed_from_words(&vector(), b"", &mut seed));
    let mut key = [0u8; 32];
    let pubkey = |sk: &[u8; 32]| {
        let mut out = [0u8; 65];
        (crate::server::secp::pubkey(sk, &mut out) == 65).then_some(out)
    };
    assert!(nonos_hd::derive_eth_key_at(&seed, pubkey, index, &mut key));
    key
}

#[test]
fn the_vector_derives_its_well_known_account() {
    assert_eq!(key_at(0), ACCOUNT_0);
    assert!(words_derive(&vector(), &ACCOUNT_0));
}

#[test]
fn every_account_the_wallet_derives_is_its_own() {
    for index in 0..=MAX_ACCOUNT_INDEX {
        assert!(words_derive(&vector(), &key_at(index)), "account {index}");
    }
}

#[test]
fn words_beside_another_key_are_refused() {
    let mut other = ACCOUNT_0;
    other[31] ^= 1;
    assert!(!words_derive(&vector(), &other));
    assert!(!words_derive(&vector(), &key_at(MAX_ACCOUNT_INDEX + 1)), "past the last account");
    let mut wrong = vector();
    wrong[0] = 1;
    assert!(!words_derive(&wrong, &ACCOUNT_0), "other words");
    assert!(!words_derive(&[], &ACCOUNT_0), "no words");
}
