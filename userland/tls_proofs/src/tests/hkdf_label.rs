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

//! The labels a TLS 1.3 key schedule is required to produce.

use crate::hkdf_label::hkdf_label;

/*
 * RFC 8446 section 7.1. Written out byte for byte rather than rebuilt from the
 * same expression the implementation uses, because a test that constructs its
 * expectation the way the code does agrees with the code about any mistake they
 * share.
 *
 * The record key for AES-128-GCM: sixteen bytes out, label "key", no context.
 */
#[test]
fn the_record_key_label_is_exact() {
    let got = hkdf_label(16, b"key", &[]).expect("a short label always fits");
    let want: &[u8] = &[
        0x00, 0x10, // length: 16
        0x09, // label length: six for "tls13 " plus three for "key"
        b't', b'l', b's', b'1', b'3', b' ', b'k', b'e', b'y', 0x00, // empty context
    ];
    assert_eq!(got, want);
}
#[test]
fn the_record_nonce_label_is_exact() {
    let got = hkdf_label(12, b"iv", &[]).expect("fits");
    let want: &[u8] = &[0x00, 0x0C, 0x08, b't', b'l', b's', b'1', b'3', b' ', b'i', b'v', 0x00];
    assert_eq!(got, want);
}
#[test]
fn a_traffic_secret_label_carries_its_context() {
    let context = [0xAB; 32];
    let got = hkdf_label(32, b"s hs traffic", &context).expect("fits");
    assert_eq!(&got[0..2], &[0x00, 0x20], "thirty two bytes requested");
    assert_eq!(got[2], 18, "six for the prefix and twelve for the label");
    assert_eq!(&got[3..21], b"tls13 s hs traffic");
    assert_eq!(got[21], 32, "the context length precedes the context");
    assert_eq!(&got[22..], &context);
    assert_eq!(got.len(), 54);
}
