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

//! Tampering with a signature the chain actually checks.

extern crate alloc;

use alloc::vec::Vec;

use crate::link::constants::{CERT_ED_ID_SIGN, CERT_ED_SIGN_LINK};
use crate::link::{bind, BindError};
use crate::vectors::{CAPTURED_AT, CERTS, IDENTITY, LEAF};

#[test]
fn a_tampered_signature_is_refused() {
    for want in [CERT_ED_ID_SIGN, CERT_ED_SIGN_LINK] {
        let certs = flip_last_byte_of(want);
        let verdict = bind(&certs, LEAF, &IDENTITY, CAPTURED_AT);
        assert_eq!(verdict, Err(BindError::BadSignature), "cert type {want}");
    }
}

/// A byte flipped at the end of the named certificate, which is inside its
fn flip_last_byte_of(cert_type: u8) -> Vec<u8> {
    let mut out: Vec<u8> = CERTS.to_vec();
    let mut at = 1usize;
    for _ in 0..CERTS[0] {
        let len = u16::from_be_bytes([out[at + 1], out[at + 2]]) as usize;
        if out[at] == cert_type {
            out[at + 2 + len] ^= 0x01;
            return out;
        }
        at += 3 + len;
    }
    panic!("cert type {cert_type} not in the captured cell");
}

#[test]
fn an_expired_chain_is_refused() {
    let far_future = CAPTURED_AT + 400 * 24 * 3_600;
    assert_eq!(bind(CERTS, LEAF, &IDENTITY, far_future), Err(BindError::Expired));
}

#[test]
fn a_truncated_cell_is_refused_not_read() {
    for cut in [0usize, 1, 4, 32, 700] {
        let verdict = bind(&CERTS[..cut], LEAF, &IDENTITY, CAPTURED_AT);
        assert!(verdict.is_err(), "a cell cut to {cut} bytes must not bind");
    }
}
