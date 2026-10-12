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

//! Which answers to a request are believed: its own call's, and no other's.

use super::*;

#[test]
fn a_request_carries_the_markets_magic_and_its_own_id() {
    let tx = request(OP_LIST_APPS, 7, b"xy").unwrap();
    assert_eq!(tx.len(), HDR_LEN + 2);
    assert_eq!(&tx[0..4], b"TKMN", "NMKT as a little-endian word");
    assert_eq!(u16::from_le_bytes([tx[6], tx[7]]), OP_LIST_APPS);
    assert_eq!(u32::from_le_bytes(tx[12..16].try_into().unwrap()), 7);
    assert_eq!(u32::from_le_bytes(tx[16..20].try_into().unwrap()), 2);
}

#[test]
fn the_answer_to_this_call_is_believed() {
    let rx = reply(OP_GET_APP, 9, 0, b"body");
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Ok(&b"body"[..]));
}

#[test]
fn an_answer_to_an_earlier_call_is_stale() {
    let rx = reply(OP_GET_APP, 8, 0, b"body");
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Stale));
    let rx = reply(OP_LIST_APPS, 9, 0, b"body");
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Stale));
}

#[test]
fn another_protocols_reply_is_foreign() {
    let mut rx = reply(OP_GET_APP, 9, 0, b"body");
    rx[0..4].copy_from_slice(&0x4E43_4D50u32.to_le_bytes());
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Foreign));
    let mut rx = reply(OP_GET_APP, 9, 0, b"body");
    rx[4] = 2;
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Foreign));
}
