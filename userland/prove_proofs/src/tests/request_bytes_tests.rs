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

//! No byte of a request is ignored: a changed byte is refused or read as a
//! changed field, and each id byte is held to printable ASCII without space.

use crate::assemble::error::Refusal;
use crate::assemble::request::parse_request;
use crate::fixture::{request, NONCE, VERIFIER, WINDOW};

fn good() -> Vec<u8> {
    request(VERIFIER, WINDOW, &NONCE, &[0x17; 32])
}

#[test]
fn every_byte_is_read() {
    let r = good();
    let before = parse_request(&r).expect("a good request");
    for i in 0..r.len() {
        for bit in 0..8 {
            let mut g = r.clone();
            g[i] ^= 1 << bit;
            if let Ok(after) = parse_request(&g) {
                assert_ne!(after, before, "byte {i} bit {bit} was not read");
            }
        }
    }
}

#[test]
fn every_id_byte_outside_printable_ascii_is_refused() {
    let r = good();
    for i in 81..r.len() {
        for b in 0..=255u8 {
            let mut g = r.clone();
            g[i] = b;
            let got = parse_request(&g);
            if (0x21..=0x7E).contains(&b) {
                assert!(got.is_ok(), "byte {b:#x} at {i}");
            } else {
                assert_eq!(got, Err(Refusal::RequestVerifierByte), "byte {b:#x} at {i}");
            }
        }
    }
}
