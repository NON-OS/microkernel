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

//! What sealing has to guarantee.
//!
//! The machine root comes from the TPM, so these drive the derivation with a
//! known root instead. That is the whole point of splitting the two: the
//! properties worth proving are about what the derivation does with a root,
//! not about where the root came from.

use crate::blob::{MAGIC, OVERHEAD, VERSION};
use crate::{open, seal, subkey};

const ROOT: [u8; 32] = [0x5a; 32];
const OTHER_ROOT: [u8; 32] = [0x5b; 32];
const NONCE: [u8; 12] = [0x11; 12];

fn sealed(record: &[u8], plaintext: &[u8]) -> alloc::vec::Vec<u8> {
    let mut out = alloc::vec![0u8; plaintext.len() + OVERHEAD];
    let n = seal(&ROOT, record, plaintext, &NONCE, &mut out).expect("seal");
    out.truncate(n);
    out
}

#[test]
fn a_record_opens_to_what_was_sealed() {
    for len in [1usize, 15, 32, 72, 300] {
        let plaintext: alloc::vec::Vec<u8> = (0..len).map(|i| (i * 7) as u8).collect();
        let blob = sealed(b"wallet.account", &plaintext);
        let mut out = alloc::vec![0u8; len];
        let n = open(&ROOT, b"wallet.account", &blob, &mut out).expect("open");
        assert_eq!(n, len);
        assert_eq!(out, plaintext);
    }
}

#[test]
fn each_record_gets_a_different_key() {
    /*
     * The property the whole design rests on. If two records shared a key,
     * recovering one would open the other, and the separation this crate
     * advertises would be decoration.
     */
    let a = subkey(&ROOT, b"wallet.account").expect("a");
    let b = subkey(&ROOT, b"settings.theme").expect("b");
    assert_ne!(a, b);

    // Including names that differ by one byte, and by length alone.
    let c = subkey(&ROOT, b"settings.themf").expect("c");
    let d = subkey(&ROOT, b"settings.theme.").expect("d");
    assert_ne!(b, c);
    assert_ne!(b, d);
}

#[test]
fn the_same_record_always_derives_the_same_key() {
    let first = subkey(&ROOT, b"wallet.account").expect("first");
    for _ in 0..8 {
        assert_eq!(subkey(&ROOT, b"wallet.account").expect("again"), first);
    }
}

#[test]
fn a_different_root_derives_a_different_key() {
    // A different machine, or the same machine in a different boot state.
    let here = subkey(&ROOT, b"wallet.account").expect("here");
    let there = subkey(&OTHER_ROOT, b"wallet.account").expect("there");
    assert_ne!(here, there);
}

#[test]
fn a_record_does_not_open_under_another_name() {
    let blob = sealed(b"wallet.account", b"the seed phrase");
    let mut out = alloc::vec![0u8; 15];
    // Same length, so the header check passes and the tag is what refuses.
    assert!(open(&ROOT, b"wallet.accounx", &blob, &mut out).is_err());
    assert_eq!(out, alloc::vec![0u8; 15], "nothing is written on a failed open");
}

#[test]
fn a_record_does_not_open_on_another_machine() {
    let blob = sealed(b"wallet.account", b"the seed phrase");
    let mut out = alloc::vec![0u8; 15];
    assert!(open(&OTHER_ROOT, b"wallet.account", &blob, &mut out).is_err());
    assert_eq!(out, alloc::vec![0u8; 15]);
}

#[test]
fn every_byte_of_the_record_is_covered() {
    let blob = sealed(b"settings.theme", b"dark");
    for i in 0..blob.len() {
        let mut bad = blob.clone();
        bad[i] ^= 0x01;
        let mut out = [0u8; 4];
        assert!(
            open(&ROOT, b"settings.theme", &bad, &mut out).is_err(),
            "a flip at byte {i} was accepted"
        );
    }
}

#[test]
fn the_header_says_what_it_is() {
    let blob = sealed(b"settings.theme", b"dark");
    assert_eq!(&blob[..8], &MAGIC);
    assert_eq!(u16::from_le_bytes([blob[8], blob[9]]), VERSION);
    assert_eq!(blob[10], b"settings.theme".len() as u8);
    assert_eq!(blob[11], 0, "the reserved byte stays zero");
}

#[test]
fn a_v1_record_is_refused_rather_than_misread() {
    // Version one sealed under the machine key directly, with no per-record
    // key. Opening it under these rules would use the wrong key anyway; it
    // has to fail on the version, before any derivation.
    let mut blob = sealed(b"settings.theme", b"dark");
    blob[8] = 1;
    let mut out = [0u8; 4];
    assert!(open(&ROOT, b"settings.theme", &blob, &mut out).is_err());
}

#[test]
fn anything_that_is_not_a_record_is_refused() {
    let mut out = [0u8; 8];
    assert!(open(&ROOT, b"x", &[], &mut out).is_err());
    assert!(open(&ROOT, b"x", &[0u8; OVERHEAD], &mut out).is_err());
    assert!(open(&ROOT, b"x", &[0xFFu8; 64], &mut out).is_err());
}

#[test]
fn an_empty_or_overlong_record_name_is_refused() {
    assert!(subkey(&ROOT, b"").is_err());
    let long = [b'x'; crate::MAX_RECORD + 1];
    assert!(subkey(&ROOT, &long).is_err());
    let at_cap = [b'x'; crate::MAX_RECORD];
    assert!(subkey(&ROOT, &at_cap).is_ok(), "exactly the cap must fit");
}

#[test]
fn a_dead_entropy_source_seals_nothing() {
    /*
     * An all-zero nonce is what a stuck source returns, and two records
     * sealed with it under one key lose the key. Refusing costs a retry.
     */
    let mut out = alloc::vec![0u8; 4 + OVERHEAD];
    assert!(seal(&ROOT, b"settings.theme", b"dark", &[0u8; 12], &mut out).is_err());
}

#[test]
fn a_short_output_buffer_is_refused_rather_than_truncated() {
    let mut out = alloc::vec![0u8; OVERHEAD];
    assert!(seal(&ROOT, b"x", b"too long for this", &NONCE, &mut out).is_err());
}
