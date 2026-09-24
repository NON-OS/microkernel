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

//! What in a buffer is, and is not, a retry.

use crate::hello_retry::{in_buffer, is_hello_retry, HELLO_RETRY_RANDOM};

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

#[test]
fn a_partial_record_is_not_a_retry() {
    let full = record(HELLO_RETRY_RANDOM);
    for cut in 0..full.len() {
        assert!(!in_buffer(&full[..cut]), "{cut} bytes is not yet a decision");
    }
    assert!(in_buffer(&full), "and the whole record is");
}
#[test]
fn only_a_handshake_record_can_be_a_retry() {
    let mut app = record(HELLO_RETRY_RANDOM);
    app[0] = 23; // application_data
    assert!(!in_buffer(&app));
}
#[test]
fn another_handshake_message_is_not_a_retry() {
    let mut msg = vec![8u8, 0, 0, 40]; // EncryptedExtensions
    msg.extend_from_slice(&[0x03, 0x03]);
    msg.extend_from_slice(&HELLO_RETRY_RANDOM);
    assert!(!is_hello_retry(&msg));
}
