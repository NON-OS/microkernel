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


//! BLAKE2b against RFC 7693 and against the fork's own BLAKE2b, salted and
//! not, across the 128-byte block boundary where an off-by-one would hide.

use super::vectors::blake;
use crate::hex::hex;
use nonos_equix::Blake2b;

const HASHX_SALT: &[u8; 16] = b"HashX v1\0\0\0\0\0\0\0\0";

fn digest(out_len: usize, salt: &[u8; 16], input: &[u8]) -> Vec<u8> {
    let mut h = Blake2b::new(out_len, salt);
    h.update(input);
    let mut out = vec![0u8; out_len];
    h.finish(&mut out);
    out
}

#[test]
fn rfc_7693_appendix_a() {
    let expected = hex(
        "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d1\
         7d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923",
    );
    assert_eq!(digest(64, &[0; 16], b"abc"), expected);
}

#[test]
fn every_reference_digest_matches() {
    let all = blake();
    assert_eq!(all.len(), 7);
    for b in all {
        let salt = if b.salted { HASHX_SALT } else { &[0; 16] };
        assert_eq!(digest(b.out_len, salt, &b.input), b.digest, "{} bytes in", b.input.len());
    }
}

#[test]
fn feeding_byte_by_byte_changes_nothing() {
    for b in blake() {
        let salt = if b.salted { HASHX_SALT } else { &[0; 16] };
        let mut h = Blake2b::new(b.out_len, salt);
        for byte in &b.input {
            h.update(core::slice::from_ref(byte));
        }
        let mut out = vec![0u8; b.out_len];
        h.finish(&mut out);
        assert_eq!(out, b.digest);
    }
}

#[test]
fn the_salt_and_the_length_each_change_the_digest() {
    let plain = digest(64, &[0; 16], b"abc");
    assert_ne!(digest(64, HASHX_SALT, b"abc"), plain);
    // A short digest is not a prefix of the long one: the length is hashed in.
    assert_ne!(digest(4, &[0; 16], b"abc")[..], plain[..4]);
}

#[test]
fn a_short_output_buffer_takes_what_fits() {
    let mut h = Blake2b::new(64, &[0; 16]);
    h.update(b"abc");
    let mut out = [0u8; 8];
    h.finish(&mut out);
    assert_eq!(out[..], digest(64, &[0; 16], b"abc")[..8]);
}
