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

//! Replies cut short, without a status, refused, or with bytes past their payload.

use super::*;

#[test]
fn a_reply_shorter_than_its_header_says_is_truncated() {
    let rx = reply(OP_GET_APP, 9, 0, b"body");
    assert_eq!(reply_body(&rx[..rx.len() - 1], OP_GET_APP, 9), Err(ReplyError::Truncated));
    let mut rx = reply(OP_GET_APP, 9, 0, b"");
    rx[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Truncated));
    let mut rx = reply(OP_GET_APP, 9, 0, b"");
    rx[16..20].copy_from_slice(&3u32.to_le_bytes());
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Truncated));
}

#[test]
fn a_reply_without_a_status_is_short() {
    let rx = reply(OP_GET_APP, 9, 0, b"");
    for n in 0..rx.len() {
        assert_eq!(reply_body(&rx[..n], OP_GET_APP, 9), Err(ReplyError::Short), "{n} bytes");
    }
}

#[test]
fn a_refusal_is_its_status_and_never_a_body() {
    let rx = reply(OP_LIST_APPS, 3, -61, b"");
    assert_eq!(reply_body(&rx, OP_LIST_APPS, 3), Err(ReplyError::Status(-61)));
    assert_eq!(status_name(-61), Some("ENODATA"));
    assert_eq!(status_name(-1), None);
}

#[test]
fn trailing_bytes_past_the_payload_are_not_the_body() {
    let mut rx = reply(OP_GET_APP, 9, 0, b"body");
    rx.extend_from_slice(b"junk");
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Ok(&b"body"[..]));
}
