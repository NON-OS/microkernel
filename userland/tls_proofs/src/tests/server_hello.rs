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

//! A ServerHello the client is meant to accept.

use super::server_hello_vectors::{good_exts, hello, AES128};
use crate::server_hello::key_share;

#[test]
fn a_well_formed_hello_yields_the_suite_and_the_peer_key() {
    let msg = hello(AES128, &[7u8; 32], &good_exts([0xAB; 32]), [0x01; 32]);
    let (suite, key) = key_share(&msg).expect("a conforming hello must parse");
    assert_eq!(suite, AES128);
    assert_eq!(key, [0xAB; 32]);
}
#[test]
fn the_echoed_session_id_may_be_any_length() {
    for len in [0usize, 1, 16, 32] {
        let sid = vec![9u8; len];
        let msg = hello(AES128, &sid, &good_exts([0xCD; 32]), [0x02; 32]);
        let (_, key) = key_share(&msg).unwrap_or_else(|| panic!("session id of {len} bytes"));
        assert_eq!(key, [0xCD; 32]);
    }
}
