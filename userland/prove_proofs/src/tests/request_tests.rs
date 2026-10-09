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

//! The request is read strictly: its fields back as written, and every length,
//! magic byte and word that is not the format's refused with its own reason.

use crate::assemble::error::Refusal;
use crate::assemble::request::{parse_request, REQUEST_MAX, VERIFIER_MAX};
use crate::fixture::{request, NONCE, VERIFIER, WINDOW};

const ROOT: [u8; 32] = [0x17; 32];

fn good() -> Vec<u8> {
    request(VERIFIER, WINDOW, &NONCE, &ROOT)
}

#[test]
fn a_request_reads_back_every_field() {
    let r = parse_request(&good()).expect("a good request");
    assert_eq!((r.window, r.nonce, r.device_root), (WINDOW, NONCE, ROOT));
    assert_eq!(r.verifier, VERIFIER);
    let longest = vec![b'v'; VERIFIER_MAX];
    let r = parse_request(&request(&longest, 0, &NONCE, &ROOT)).expect("the longest id");
    assert_eq!(r.verifier.len(), VERIFIER_MAX);
    assert_eq!(request(&longest, 0, &NONCE, &ROOT).len(), REQUEST_MAX);
}

#[test]
fn every_cut_and_every_extension_is_refused() {
    let r = good();
    for n in 0..r.len() {
        assert!(parse_request(&r[..n]).is_err(), "cut at {n}");
    }
    for extra in [0u8, b'a', 0xFF] {
        let mut longer = r.clone();
        longer.push(extra);
        assert_eq!(parse_request(&longer), Err(Refusal::RequestSize));
    }
    assert_eq!(parse_request(&vec![b'a'; REQUEST_MAX + 1]), Err(Refusal::RequestSize));
}

#[test]
fn every_magic_byte_is_checked() {
    for i in 0..8 {
        let mut r = good();
        r[i] ^= 0x20;
        assert_eq!(parse_request(&r), Err(Refusal::RequestMagic), "byte {i}");
    }
}

#[test]
fn the_length_byte_is_held_to_the_id() {
    let mut r = good();
    for (n, why) in [(0u8, Refusal::RequestVerifierLength), (129, Refusal::RequestVerifierLength)] {
        r[80] = n;
        assert_eq!(parse_request(&r), Err(why));
    }
    for n in [VERIFIER.len() as u8 - 1, VERIFIER.len() as u8 + 1, 0x80] {
        r[80] = n;
        assert_eq!(parse_request(&r), Err(Refusal::RequestSize), "length {n}");
    }
}
