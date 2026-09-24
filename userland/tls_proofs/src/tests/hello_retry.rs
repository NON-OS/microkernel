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

//! The sentinel random that marks a HelloRetryRequest.

use crate::hello_retry::{in_buffer, HELLO_RETRY_RANDOM};

fn record(random: [u8; 32]) -> Vec<u8> {
    let mut msg = Vec::new();
    msg.push(2u8); // ServerHello
    msg.extend_from_slice(&[0, 0, 0]); // length, filled in below
    msg.extend_from_slice(&[0x03, 0x03]); // legacy version
    msg.extend_from_slice(&random);
    msg.push(0); // empty legacy session id
    msg.extend_from_slice(&[0x13, 0x01]); // cipher suite
    msg.push(0); // compression
    msg.extend_from_slice(&[0, 0]); // no extensions
    let body_len = msg.len() - 4;
    msg[1..4].copy_from_slice(&[(body_len >> 16) as u8, (body_len >> 8) as u8, body_len as u8]);

    let mut out = Vec::new();
    out.push(22u8); // handshake record
    out.extend_from_slice(&[0x03, 0x03]);
    out.extend_from_slice(&(msg.len() as u16).to_be_bytes());
    out.extend_from_slice(&msg);
    out
}

/*
 * RFC 8446 section 4.1.3: the retry is a ServerHello whose random is the SHA-256
 * of "HelloRetryRequest". Nothing else in the message distinguishes them, so a
 * client that does not compare this value reads a retry as a hello, fails to
 * parse its key share, and waits for a message that has already arrived.
 */
#[test]
fn the_sentinel_random_is_what_marks_a_retry() {
    assert!(in_buffer(&record(HELLO_RETRY_RANDOM)), "the retry is recognised");
    assert!(!in_buffer(&record([0x11; 32])), "an ordinary hello is not a retry");
}
#[test]
fn the_sentinel_is_the_published_value() {
    let expected: [u8; 32] = [
        0xCF, 0x21, 0xAD, 0x74, 0xE5, 0x9A, 0x61, 0x11, 0xBE, 0x1D, 0x8C, 0x02, 0x1E, 0x65, 0xB8,
        0x91, 0xC2, 0xA2, 0x11, 0x16, 0x7A, 0xBB, 0x8C, 0x5E, 0x07, 0x9E, 0x09, 0xE2, 0xC8, 0xA8,
        0x33, 0x9C,
    ];
    assert_eq!(HELLO_RETRY_RANDOM, expected);
}
