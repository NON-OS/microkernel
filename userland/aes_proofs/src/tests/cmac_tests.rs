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

//! AES-CMAC against RFC 4493, 4: the four examples under its one key, and
//! a message split into parts at every boundary MACs as the whole.

use crate::hex::hex;
use nonos_aes::{aes_cmac, CMAC_BYTES};

const KEY: &str = "2b7e151628aed2a6abf7158809cf4f3c";
const MSG: &str = concat!(
    "6bc1bee22e409f96e93d7e117393172a",
    "ae2d8a571e03ac9c9eb76fac45af8e51",
    "30c81c46a35ce411e5fbc1191a0a52ef",
    "f69f2445df4f9b17ad2b417be66c3710",
);

fn key() -> [u8; 16] {
    hex(KEY).try_into().unwrap()
}

#[test]
fn rfc4493_examples() {
    let m = hex(MSG);
    let cases = [
        (0, "bb1d6929e95937287fa37d129b756746"),
        (16, "070a16b46b4d4144f79bdd9dd04a287c"),
        (40, "dfa66747de9ae63030ca32611497c827"),
        (64, "51f0bebf7e3b9d92fc49741779363cfe"),
    ];
    for (len, tag) in cases {
        let mac: [u8; CMAC_BYTES] = aes_cmac(&key(), &[&m[..len]]);
        assert_eq!(mac.to_vec(), hex(tag), "message length {len}");
    }
}

// Every split of every prefix: the block held back for the subkey must not
// depend on where one part ends and the next begins.
#[test]
fn parts_are_one_message() {
    let m = hex(MSG);
    for len in 0..=m.len() {
        let whole = aes_cmac(&key(), &[&m[..len]]);
        for cut in 0..=len {
            let split = aes_cmac(&key(), &[&m[..cut], &[], &m[cut..len]]);
            assert_eq!(split, whole, "length {len} cut at {cut}");
        }
        if len > 0 {
            assert_ne!(aes_cmac(&key(), &[&m[..len - 1]]), whole, "length {len}");
        }
    }
}
