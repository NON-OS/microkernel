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

//! An attest is signed only after its own review (the maintainers' W7 ruling): a row
//! opens the review, which shows every signed row, the signer, the Bond and
//! the EIP-712 digest; only the review's Sign signs, under the focus guard.

use nonos_marketplace_abi::SettleMode;
use nonos_nox_config::{domain_words_at, BOND, RELEASE_ATTEST};
use sha3::{Digest, Keccak256};

use super::fixture::{entry, K, ME};
use crate::hex_id::hex;
use crate::publisher::Refused;
use crate::publisher::{attest_of, attest_review, attest_rows};
use crate::typed::attest_words;

const BOND_AT: [u8; 20] = [0x42; 20];
const CHAIN: u64 = 11_155_111;

fn keccak(b: &[u8]) -> [u8; 32] {
    Keccak256::digest(b).into()
}

/// The digest by EIP-712's own words, apart from the wallet's code.
fn by_hand(release: usize) -> [u8; 32] {
    let e = entry(SettleMode::Escrow);
    let a = attest_of(&K, &e, release, CHAIN).unwrap();
    let words = attest_words(&K, &ME, &e.publisher_pubkey, &a).unwrap();
    let mut s = RELEASE_ATTEST.typehash.to_vec();
    words.iter().for_each(|w| s.extend_from_slice(w));
    let mut m = vec![0x19, 0x01];
    m.extend_from_slice(&keccak(&domain_words_at(&BOND, CHAIN, BOND_AT)));
    m.extend_from_slice(&keccak(&s));
    keccak(&m)
}

#[test]
fn the_review_shows_the_digest_the_keyring_signs() {
    let e = entry(SettleMode::Escrow);
    let r = attest_review(&K, &e, 1, CHAIN, &ME, &BOND_AT).unwrap();
    assert_eq!(r.digest, by_hand(1));
    assert!(r.rows.contains(&("digest", hex(&by_hand(1)))), "the digest is on screen");
    assert!(r.rows.contains(&("signed by", hex(&ME))), "the signing account is named");
    assert!(r.rows.iter().any(|(k, v)| *k == "relied on by" && v.contains(&hex(&BOND_AT))));
    let a = attest_of(&K, &e, 1, CHAIN).unwrap();
    for row in attest_rows(&e, 1, &a) {
        assert!(r.rows.contains(&row), "the signed row {} is shown", row.0);
    }
}

#[test]
fn the_digest_names_the_release_the_bond_and_the_chain() {
    let e = entry(SettleMode::Escrow);
    let d = |rel, ch, bond: [u8; 20]| attest_review(&K, &e, rel, ch, &ME, &bond).unwrap().digest;
    assert_ne!(d(1, CHAIN, BOND_AT), d(0, CHAIN, BOND_AT), "another release");
    assert_ne!(d(1, CHAIN, BOND_AT), d(1, 1, BOND_AT), "another chain");
    assert_ne!(d(1, CHAIN, BOND_AT), d(1, CHAIN, [0x43; 20]), "another Bond");
    let other = attest_review(&K, &e, 1, CHAIN, &[0x12; 20], &BOND_AT).err();
    assert_eq!(other, Some(Refused::NotOurBond), "another account's bond is refused");
}
