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

//! The capsule's HKDF-BLAKE3, which now wipes every block it makes, against a
//! reference written here from RFC 2104 and RFC 5869 over the BLAKE3 crate:
//! the wipes changed nothing it outputs.

use crate::crypto::hkdf_blake3::{expand, extract, hmac_blake3};

fn hmac(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut k = [0u8; 64];
    if key.len() > 64 {
        k[..32].copy_from_slice(blake3::hash(key).as_bytes());
    } else {
        k[..key.len()].copy_from_slice(key);
    }
    let inner: Vec<u8> = k.iter().map(|b| b ^ 0x36).chain(msg.iter().copied()).collect();
    let ih = blake3::hash(&inner);
    let outer: Vec<u8> = k.iter().map(|b| b ^ 0x5c).chain(ih.as_bytes().iter().copied()).collect();
    *blake3::hash(&outer).as_bytes()
}

fn reference_expand(prk: &[u8; 32], info: &[u8], len: usize) -> Vec<u8> {
    let (mut out, mut t, mut i) = (Vec::new(), Vec::new(), 1u8);
    while out.len() < len {
        t = hmac(prk, &[&t[..], info, &[i]].concat()).to_vec();
        out.extend_from_slice(&t);
        i += 1;
    }
    out.truncate(len);
    out
}

#[test]
fn hmac_matches_the_reference_for_short_and_long_keys() {
    for key in [&b"k"[..], &[7u8; 64][..], &[9u8; 65][..], &[1u8; 200][..]] {
        let mut out = [0u8; 32];
        hmac_blake3(key, b"message", &mut out).is_ok().then_some(()).expect("hashed");
        assert_eq!(out, hmac(key, b"message"), "key of {} bytes", key.len());
    }
}

#[test]
fn expand_matches_the_reference_at_every_length() {
    let mut prk = [0u8; 32];
    extract(b"salt", b"input keying material", &mut prk).is_ok().then_some(()).expect("extracted");
    assert_eq!(prk, hmac(b"salt", b"input keying material"), "extract is HMAC(salt, ikm)");
    for info in [&b""[..], &b"nym"[..]] {
        for len in 0..=130 {
            let mut out = vec![0u8; len];
            expand(&prk, info, &mut out).is_ok().then_some(()).expect("expanded");
            assert_eq!(out, reference_expand(&prk, info, len), "{len} bytes, info {info:?}");
        }
    }
    assert!(expand(&prk, b"", &mut vec![0u8; 255 * 32 + 1]).is_err(), "past 255 blocks");
}
