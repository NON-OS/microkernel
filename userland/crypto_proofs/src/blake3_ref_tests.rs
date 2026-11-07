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

// The kernel's BLAKE3 (src/crypto/hash/blake3) against the BLAKE3 team's
// reference crate, on every input length through four chunks, on lengths
// across the tree's levels, fed whole and in uneven pieces, for the plain,
// keyed, derive-key and extended-output modes. The official vectors
// (blake3_tests.rs) pin both to the specification.

use crate::hash::blake3::{
    blake3_derive_key, blake3_hash, blake3_hash_xof, blake3_keyed_hash, Hasher,
};

const KEY: &[u8; 32] = b"whats the Elvish word for friend";

fn input(n: usize) -> Vec<u8> {
    (0..n).map(|i| (i % 251) as u8).collect()
}

fn lengths() -> Vec<usize> {
    let mut v: Vec<usize> = (0..=4100).collect();
    for k in [8191, 8192, 8193, 16384, 31744, 65535, 102400, 1 << 20] {
        v.push(k);
    }
    v
}

#[test]
fn plain_and_keyed_hashes_match_the_reference_at_every_length() {
    for n in lengths() {
        let x = input(n);
        assert_eq!(blake3_hash(&x), *blake3::hash(&x).as_bytes(), "hash, {n} bytes");
        assert_eq!(blake3_keyed_hash(KEY, &x), *blake3::keyed_hash(KEY, &x).as_bytes(), "keyed, {n} bytes");
    }
}

#[test]
fn uneven_updates_give_the_one_shot_digest() {
    let x = input(9000);
    for step in [1usize, 3, 63, 64, 65, 1023, 1024, 1025, 4097] {
        let mut h = Hasher::new();
        for piece in x.chunks(step) {
            h.update(piece);
        }
        assert_eq!(h.finalize(), *blake3::hash(&x).as_bytes(), "step {step}");
    }
}

#[test]
fn derive_key_and_long_output_match_the_reference() {
    for n in [0usize, 1, 64, 1024, 1025, 5000] {
        let x = input(n);
        let mut ours = [0u8; 131];
        blake3_derive_key("nonos launch proofs 2026", &x, &mut ours);
        let mut theirs = [0u8; 131];
        let mut r = blake3::Hasher::new_derive_key("nonos launch proofs 2026");
        r.update(&x);
        r.finalize_xof().fill(&mut theirs);
        assert_eq!(ours[..], theirs[..], "derive, {n} bytes");

        blake3_hash_xof(&x, &mut ours);
        blake3::Hasher::new().update(&x).finalize_xof().fill(&mut theirs);
        assert_eq!(ours[..], theirs[..], "xof, {n} bytes");
    }
}
