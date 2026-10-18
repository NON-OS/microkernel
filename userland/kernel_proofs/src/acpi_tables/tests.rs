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

//! A table reaches the capsule only whole: its own signature, its own
//! length, and bytes summing to zero.

use super::table_check::{declared_length, verify, TableError, MAX_TABLE};

/// A table with `sig`, `body` after the header, and its checksum fixed.
fn table(sig: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let mut t = vec![0u8; 36];
    t[..4].copy_from_slice(sig);
    t.extend_from_slice(body);
    let len = t.len() as u32;
    t[4..8].copy_from_slice(&len.to_le_bytes());
    let sum = t.iter().fold(0u8, |a, &b| a.wrapping_add(b));
    t[9] = 0u8.wrapping_sub(sum);
    t
}

#[test]
fn a_sound_table_passes_and_any_one_byte_changed_fails() {
    let t = table(b"SSDT", &[0x10, 0x05, 0x5C, 0x5F, 0x53, 0x42]);
    assert_eq!(verify(&t, *b"SSDT"), Ok(()));
    for i in 0..t.len() {
        let mut bad = t.clone();
        bad[i] ^= 0x01;
        assert!(verify(&bad, *b"SSDT").is_err(), "byte {i}");
    }
}

#[test]
fn length_and_signature_are_checked_before_the_sum() {
    let t = table(b"DSDT", &[0; 4]);
    assert_eq!(verify(&t, *b"SSDT"), Err(TableError::Signature));
    assert_eq!(verify(&t[..t.len() - 1], *b"DSDT"), Err(TableError::Length), "cut short");
    let mut long = t.clone();
    long.push(0);
    assert_eq!(verify(&long, *b"DSDT"), Err(TableError::Length), "trailing bytes");
    let mut huge = t.clone();
    huge[4..8].copy_from_slice(&((MAX_TABLE as u32) + 1).to_le_bytes());
    assert_eq!(declared_length(&huge, *b"DSDT"), Err(TableError::Length));
    assert_eq!(declared_length(&t[..7], *b"DSDT"), Err(TableError::Length));
    let mut short = t.clone();
    short[4..8].copy_from_slice(&35u32.to_le_bytes());
    assert_eq!(declared_length(&short, *b"DSDT"), Err(TableError::Length), "under a header");
}

#[test]
fn the_facs_has_no_checksum_and_a_short_header() {
    let mut facs = vec![0u8; 64];
    facs[..4].copy_from_slice(b"FACS");
    facs[4..8].copy_from_slice(&64u32.to_le_bytes());
    facs[20] = 0x5A;
    assert_eq!(verify(&facs, *b"FACS"), Ok(()));
    assert_eq!(declared_length(&facs[..8], *b"FACS"), Ok(64));
}
