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

//! The properties HKDF-SHA-512 promises: a shorter output is a prefix of a
//! longer one, an empty salt is 64 zero bytes, every input changes the
//! output, and the output stops at 255 blocks.

use nonos_hash::{hkdf512_expand, hkdf512_extract, HkdfTooLong, HKDF512_MAX};

fn derive(ikm: &[u8], salt: &[u8], info: &[u8], n: usize) -> Result<Vec<u8>, HkdfTooLong> {
    let mut out = vec![0; n];
    hkdf512_expand(&hkdf512_extract(salt, ikm), info, &mut out).map(|_| out)
}

#[test]
fn a_shorter_output_is_a_prefix_of_a_longer_one() {
    let long = derive(b"ikm", b"salt", b"info", 300).unwrap();
    for n in [0, 1, 63, 64, 65, 128, 299] {
        assert_eq!(derive(b"ikm", b"salt", b"info", n).unwrap(), long[..n], "{n}");
    }
}

#[test]
fn an_empty_salt_is_sixty_four_zero_bytes() {
    assert_eq!(hkdf512_extract(b"", b"ikm"), hkdf512_extract(&[0; 64], b"ikm"));
    assert_ne!(hkdf512_extract(b"", b"ikm"), hkdf512_extract(&[1], b"ikm"));
}

#[test]
fn the_limit_is_two_hundred_fifty_five_blocks() {
    assert_eq!(HKDF512_MAX, 16_320);
    assert!(derive(b"k", b"", b"", HKDF512_MAX).is_ok());
    assert_eq!(derive(b"k", b"", b"", HKDF512_MAX + 1), Err(HkdfTooLong));
}

#[test]
fn info_and_salt_each_change_the_output() {
    let base = derive(b"ikm", b"salt", b"info", 64).unwrap();
    assert_ne!(derive(b"ikm", b"salt", b"infp", 64).unwrap(), base);
    assert_ne!(derive(b"ikm", b"salu", b"info", 64).unwrap(), base);
    assert_ne!(derive(b"ikn", b"salt", b"info", 64).unwrap(), base);
}
