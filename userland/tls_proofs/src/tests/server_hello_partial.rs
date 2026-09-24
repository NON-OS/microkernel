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

//! Bytes that are not a ServerHello at all.

use super::server_hello_vectors::{ext, good_exts, hello, AES128, X25519};
use crate::hello_retry::HELLO_RETRY_RANDOM;
use crate::server_hello::key_share;

#[test]
fn a_retry_does_not_parse_as_a_hello() {
    // A retry's key_share extension holds the group it wants and nothing else.
    let share = X25519.to_be_bytes();
    let mut exts = ext(43, &[0x03, 0x04]);
    exts.extend_from_slice(&ext(51, &share));
    let msg = hello(AES128, &[7u8; 32], &exts, HELLO_RETRY_RANDOM);
    assert!(key_share(&msg).is_none(), "a retry carries a group, not a key");
}
#[test]
fn no_prefix_of_a_hello_parses() {
    let msg = hello(AES128, &[7u8; 32], &good_exts([0xAB; 32]), [0x06; 32]);
    for cut in 0..msg.len() {
        assert!(key_share(&msg[..cut]).is_none(), "{cut} bytes is not a hello");
    }
    assert!(key_share(&msg).is_some());
}
