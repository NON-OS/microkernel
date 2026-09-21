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

//! A ServerHello whose extensions the client did not offer.

use super::server_hello_vectors::{ext, good_exts, hello, AES128, X25519};
use crate::server_hello::key_share;

#[test]
fn a_hello_without_supported_versions_is_refused() {
    let mut share = X25519.to_be_bytes().to_vec();
    share.extend_from_slice(&32u16.to_be_bytes());
    share.extend_from_slice(&[0xEE; 32]);
    let msg = hello(AES128, &[7u8; 32], &ext(51, &share), [0x03; 32]);
    assert!(key_share(&msg).is_none());
}
#[test]
fn an_unoffered_cipher_suite_is_refused() {
    // TLS_AES_256_GCM_SHA384: what these relays prefer, and what this client
    // does not implement. Accepting it would key the session with the wrong
    // hash length and fail later, further from the cause.
    let msg = hello(0x1302, &[7u8; 32], &good_exts([0xAB; 32]), [0x04; 32]);
    assert!(key_share(&msg).is_none());
}
#[test]
fn a_key_share_that_is_not_thirty_two_bytes_is_refused() {
    for len in [0usize, 16, 31, 33, 64] {
        let mut share = X25519.to_be_bytes().to_vec();
        share.extend_from_slice(&(len as u16).to_be_bytes());
        share.extend_from_slice(&vec![0xFF; len]);
        let mut exts = ext(43, &[0x03, 0x04]);
        exts.extend_from_slice(&ext(51, &share));
        let msg = hello(AES128, &[7u8; 32], &exts, [0x05; 32]);
        assert!(key_share(&msg).is_none(), "a {len} byte key share is not x25519");
    }
}
