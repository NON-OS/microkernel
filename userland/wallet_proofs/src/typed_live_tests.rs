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

//! An entry with a contract deployed, as one will look once
//! nonos_nox_config names it: the path that signs, end to end but the key.

use super::typed_hash_tests::keccak;
use crate::server::typed::domain::{Domain, DOMAIN_TYPE};
use crate::server::typed::entry::Entry;
use crate::server::typed::field::{Field, Kind};
use crate::server::typed::hashes::hashes;

static D: Domain = Domain { name: "T", version: "1" };
pub static LIVE: Entry = Entry {
    id: 99,
    type_string: "T(uint64 n,address who)",
    domain: &D,
    fields: &[
        Field { kind: Kind::Uint(64), is_signer: false },
        Field { kind: Kind::Address, is_signer: true },
    ],
    callers: &[b"x"],
    contracts: [None, Some([0x55; 20])],
};

fn k(b: &[u8]) -> [u8; 32] {
    keccak(b).unwrap()
}
/// The three hashes, against EIP-712 written out by hand.
#[test]
fn the_hashes_are_eip712s() {
    let mut words = [[0u8; 32]; 2].concat();
    words[31] = 5;
    words[32 + 12..].copy_from_slice(&[0x77; 20]);
    let h = hashes(&LIVE, 11_155_111, &[0x55; 20], &words, keccak).unwrap();
    let dom = [
        k(DOMAIN_TYPE.as_bytes()).as_slice(),
        &k(b"T"),
        &k(b"1"),
        &[[0u8; 24].as_slice(), &11_155_111u64.to_be_bytes()].concat(),
        &[[0u8; 12].as_slice(), &[0x55; 20]].concat(),
    ]
    .concat();
    assert_eq!(h.domain_separator, k(&dom));
    assert_eq!(h.struct_hash, k(&[k(b"T(uint64 n,address who)").as_slice(), &words].concat()));
    let digest = k(&[[0x19, 0x01].as_slice(), &h.domain_separator, &h.struct_hash].concat());
    assert_eq!(h.digest, digest);
}

#[test]
fn a_failed_hash_signs_nothing() {
    let words = [[0u8; 32]; 2].concat();
    assert!(hashes(&LIVE, 11_155_111, &[0x55; 20], &words, |_| None).is_none());
}
