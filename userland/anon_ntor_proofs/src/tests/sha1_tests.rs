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

//! SHA-1 against the published vectors.

extern crate alloc;

use alloc::vec::Vec;

use crate::hex;
use crate::sha1::Sha1;

// FIPS 180-4, and the SHA-1 sample vectors that accompany it.
const VECTORS: &[(&[u8], &str)] = &[
    (b"", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
    (b"abc", "a9993e364706816aba3e25717850c26c9cd0d89d"),
    (
        b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        "84983e441c3bd26ebaae4aa1f95129e5e54670f1",
    ),
];

fn digest(data: &[u8]) -> Vec<u8> {
    let mut hash = Sha1::new();
    hash.update(data);
    hash.finish().to_vec()
}

#[test]
fn published_vectors() {
    for (input, want) in VECTORS {
        assert_eq!(digest(input), hex(want), "input of {} bytes", input.len());
    }
}

#[test]
fn million_letters_crosses_many_blocks() {
    let mut hash = Sha1::new();
    for _ in 0..1_000 {
        hash.update(&[b'a'; 1_000]);
    }
    assert_eq!(hash.finish().to_vec(), hex("34aa973cd4c4daa4f61eeb2bdbad27316534016f"));
}
