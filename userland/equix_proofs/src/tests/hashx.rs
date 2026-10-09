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


//! HashX against the upstream suite's published values and against the
//! fork's build, which keeps the first eight bytes of each.

use super::vectors::{hashes, seeds};
use nonos_equix::HashX;

#[test]
fn upstream_suite_values() {
    // tests.c in src/ext/equix/hashx, 32-byte outputs; the fork's build keeps
    // the first eight.
    let first = HashX::new(b"This is a test\0").expect("seed 1 is good");
    assert_eq!(first.hash(123456).to_le_bytes(), *b"\xae\xbd\xd5\x0a\xa6\x7c\x93\xaf");
    assert_eq!(first.hash(0).to_le_bytes(), *b"\x2b\x2f\x54\x56\x7d\xcb\xea\x98");
    let second = HashX::new(b"Lorem ipsum dolor sit amet\0").expect("seed 2 is good");
    assert_eq!(second.hash(123456).to_le_bytes(), *b"\xab\x3d\x15\x5b\xf4\xbb\xb0\xaa");
    assert_eq!(second.hash(987654321123456789).to_le_bytes(), *b"\x8d\xfe\xf0\x49\x7c\x32\x32\x74");
}

#[test]
fn every_reference_hash_matches() {
    let all = hashes();
    assert_eq!(all.len(), 20);
    for h in all {
        let f = HashX::new(&h.seed).expect("reference made this seed");
        assert_eq!(f.hash(h.input).to_le_bytes(), h.output, "seed {:02x?} input {}", h.seed, h.input);
    }
}

#[test]
fn seeds_the_reference_refuses_are_refused() {
    let all = seeds();
    assert_eq!(all.len(), 4);
    assert_eq!(all.iter().filter(|(_, ok)| !ok).count(), 2);
    for (seed, ok) in all {
        assert_eq!(HashX::new(&seed).is_some(), ok, "seed {seed:02x?}");
    }
}

#[test]
fn one_seed_byte_changes_every_output() {
    let a = HashX::new(b"This is a test\0").unwrap();
    let b = HashX::new(b"This is a test\x01").unwrap();
    for i in 0..64 {
        assert_ne!(a.hash(i), b.hash(i));
    }
}
