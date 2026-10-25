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

//! The hello preamble: each of its ten bytes is judged, and a peer speaking
//! another version is told so.

use super::hello::client;
use nsh_wire::{ClientHello, WireError, CLIENT_HELLO_LEN};

#[test]
fn each_preamble_byte_is_judged() {
    let want = [
        WireError::Magic,
        WireError::Magic,
        WireError::Magic,
        WireError::Magic,
        WireError::Version,
        WireError::Version,
        WireError::Version,
        WireError::Version,
        WireError::Reserved,
        WireError::Reserved,
    ];
    for (i, err) in want.iter().enumerate() {
        let mut bytes = client();
        bytes[i] ^= 0x40;
        assert_eq!(ClientHello::parse(&bytes), Err(*err), "byte {i}");
    }
}

#[test]
fn another_version_is_told_version_whatever_its_size() {
    // Version 1, whose cap could not take a document with a full event log,
    // is told VERSION at the hello, before any proof is sent.
    for (v, size) in [(1, 2000), (3, 600)] {
        let mut other = client();
        other[4] = v;
        other.truncate(size);
        assert_eq!(ClientHello::parse(&other), Err(WireError::Version), "version {v}");
    }
    assert_eq!(WireError::Version.reason().code(), 2);
    assert_eq!(WireError::BodyVersion.reason().code(), 1, "only a hello is told VERSION");
}

#[test]
fn the_encoder_refuses_a_buffer_that_is_too_small() {
    let mut out = vec![0u8; CLIENT_HELLO_LEN - 1];
    let h = ClientHello { random: &[0; 32], kem_ek: &[0; 1216] };
    assert_eq!(h.encode(&mut out), Err(WireError::NoRoom));
}
