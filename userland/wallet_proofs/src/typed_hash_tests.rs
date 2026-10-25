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

//! The hashes the keyring signs with, pinned and checked against the strings
//! they are of: a change to a type string or a domain is a change to every
//! signature it makes, and has to be made here as well as in the config.

use crate::server::typed::table::ENTRIES;
use nonos_nox_config::EIP712_DOMAIN_TYPE;
use sha3::{Digest, Keccak256};

pub fn keccak(b: &[u8]) -> Option<[u8; 32]> {
    Some(Keccak256::digest(b).into())
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn hash_hex(s: &str) -> String {
    hex(&keccak(s.as_bytes()).unwrap())
}

#[test]
fn the_type_hashes() {
    let want = [
        "6481006cfbd6bb98804dafc05500eb2c23dbdb5206a1d046cdb9b2621c2780fe",
        "f3a44af7b15b9b54ed9ce73eb2a58f9b0154713e3265acc20e98701170b1c88f",
        "2c3027b70e6539f80e45782d694163977de6df1bafcba6fa319d7a6c6283faf0",
    ];
    for (e, want) in ENTRIES.iter().zip(want) {
        assert_eq!(hash_hex(e.typed.type_string), want, "entry {}", e.id);
        assert_eq!(hex(&e.typed.typehash), want, "entry {}", e.id);
    }
}

#[test]
fn the_domain_hashes() {
    assert_eq!(
        hash_hex(EIP712_DOMAIN_TYPE),
        "8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f"
    );
    let names: Vec<_> =
        ENTRIES.iter().map(|e| (e.typed.domain.name, e.typed.domain.version)).collect();
    assert_eq!(names, [("NONOS Settlement", "1"), ("NONOS Bond", "1"), ("NONOS Bond", "1")]);
    assert_eq!(
        hash_hex("NONOS Settlement"),
        "9fcd4fbd152ffd7f0627344c2cd9dcf7f407d40b5aaf3e3126b3b34452daccd2"
    );
    assert_eq!(
        hash_hex("NONOS Bond"),
        "b4dfeee30953d2d8114d7740b1d8af1ff5a81505abd3efda74a42d5c4c3c6e0d"
    );
}
