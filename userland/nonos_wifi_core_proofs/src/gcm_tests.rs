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

//! GCM against published vectors: test cases 13 to 16 of McGrew and Viega, "The Galois/Counter
//! Mode of Operation (GCM)" (the AES-256 cases, also in NIST's GCM
//! validation set): here an empty and a one-block message under the zero
//! key; `gcm_key_tests` has the rest.

use nonos_wifi_core::gcmp::gcm::{open, seal};

pub fn h(s: &str) -> Vec<u8> {
    (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()).collect()
}

pub fn case(k: &str, iv: &str, aad: &str, p: &str, c: &str, t: &str) {
    let (k, iv): ([u8; 32], [u8; 12]) = (h(k).try_into().unwrap(), h(iv).try_into().unwrap());
    let sealed = seal(&k, &iv, &h(aad), &h(p));
    assert_eq!(sealed, [h(c), h(t)].concat());
    assert_eq!(open(&k, &iv, &h(aad), &sealed), Some(h(p)));
}

// Recalled, the code reproduces it; source not reachable here.
#[test]
fn gcm_case_13_empty_message() {
    let z = "0".repeat(64);
    case(&z, &"0".repeat(24), "", "", "", "530f8afbc74536b9a963b4f1c4cb738b");
}

// Recalled, the code reproduces it; source not reachable here.
#[test]
fn gcm_case_14_one_zero_block() {
    let z = "0".repeat(64);
    let p = "0".repeat(32);
    case(
        &z,
        &"0".repeat(24),
        "",
        &p,
        "cea7403d4d606b6e074ec5d3baf39d18",
        "d0d1c8a799996bf0265b98b5d48ab919",
    );
}

#[test]
fn a_changed_tag_aad_or_ciphertext_opens_nothing() {
    let (k, iv) = ([0x42; 32], [7u8; 12]);
    let sealed = seal(&k, &iv, b"aad", b"payload");
    for i in 0..sealed.len() {
        let mut s = sealed.clone();
        s[i] ^= 1;
        assert_eq!(open(&k, &iv, b"aad", &s), None, "flip at {i}");
    }
    assert_eq!(open(&k, &iv, b"aae", &sealed), None);
    assert_eq!(open(&k, &iv, b"aad", &sealed[..15]), None);
}
