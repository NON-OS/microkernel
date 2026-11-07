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

use crate::admission_cache::key::key;
use crate::admission_cache::table::Table;

#[test]
fn moving_a_byte_between_inputs_changes_the_key() {
    assert_ne!(key(b"d", &[b"ab", b"c"]), key(b"d", &[b"a", b"bc"]));
    assert_ne!(key(b"d", &[b"abc"]), key(b"d", &[b"abc", b""]));
    assert_ne!(key(b"d", &[]), key(b"d", &[b""]));
}

#[test]
fn the_domain_separates_keys_over_the_same_inputs() {
    assert_ne!(
        key(b"nonos.admission.signature.v1", &[b"x"]),
        key(b"nonos.admission.attest.v1", &[b"x"])
    );
    assert_ne!(key(b"ab", &[b"c"]), key(b"a", &[b"bc"]));
}

#[test]
fn every_input_byte_reaches_the_key() {
    let base: Vec<u8> = (0..200u8).collect();
    let k = key(b"d", &[&base, &[7; 32]]);
    for i in 0..base.len() {
        let mut flipped = base.clone();
        flipped[i] ^= 1;
        assert_ne!(key(b"d", &[&flipped, &[7; 32]]), k, "byte {i}");
    }
}

#[test]
fn the_key_is_blake3_over_length_prefixed_parts() {
    let mut h = blake3::Hasher::new();
    for part in [&b"dom"[..], b"one", b"two"] {
        h.update(&(part.len() as u64).to_le_bytes());
        h.update(part);
    }
    assert_eq!(key(b"dom", &[b"one", b"two"]), *h.finalize().as_bytes());
}

#[test]
fn an_empty_table_knows_nothing() {
    let t: Table<4> = Table::new();
    assert!(!t.contains(&[0; 32]));
    assert!(!t.contains(&[1; 32]));
}

#[test]
fn the_table_keeps_what_it_is_given() {
    let mut t: Table<4> = Table::new();
    t.insert([1; 32]);
    t.insert([2; 32]);
    assert!(t.contains(&[1; 32]) && t.contains(&[2; 32]));
    assert!(!t.contains(&[3; 32]));
}

#[test]
fn a_full_table_lets_the_oldest_go_first() {
    let mut t: Table<3> = Table::new();
    for i in 1..=5u8 {
        t.insert([i; 32]);
    }
    assert!(!t.contains(&[1; 32]) && !t.contains(&[2; 32]));
    for i in 3..=5u8 {
        assert!(t.contains(&[i; 32]), "{i}");
    }
}

#[test]
fn a_key_kept_twice_takes_one_place() {
    let mut t: Table<2> = Table::new();
    t.insert([9; 32]);
    t.insert([9; 32]);
    t.insert([8; 32]);
    assert!(t.contains(&[9; 32]) && t.contains(&[8; 32]));
}

#[test]
fn clearing_forgets_everything() {
    let mut t: Table<4> = Table::new();
    t.insert([1; 32]);
    t.insert([0; 32]);
    t.clear();
    assert!(!t.contains(&[1; 32]));
    assert!(!t.contains(&[0; 32]));
}

#[test]
fn a_zero_sized_table_keeps_nothing() {
    let mut t: Table<0> = Table::new();
    t.insert([1; 32]);
    assert!(!t.contains(&[1; 32]));
}
