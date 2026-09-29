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

//! A reply ends where its Content-Length says, not at the first empty read:
//! a mirror still fetching upstream sends nothing for a while, and reading
//! that as the end refused Kali's Release on a live boot.

use crate::install::http_reply::{body, complete, framing};
const OK: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nhello";

#[test]
fn a_reply_is_complete_only_once_every_promised_byte_is_in() {
    for cut in 0..OK.len() {
        assert!(!complete(&OK[..cut]), "complete at {cut} of {}", OK.len());
    }
    assert!(complete(OK));
    assert_eq!(body(OK.to_vec()).as_deref(), Some(&b"hello"[..]));
}

#[test]
fn the_length_header_is_read_whatever_its_case() {
    let r = b"HTTP/1.1 200 OK\r\ncontent-LENGTH:  3 \r\n\r\nabc";
    assert_eq!(framing(r), Some((r.len() - 3, Some(3))));
    assert!(complete(r));
}

#[test]
fn a_body_short_of_its_length_is_refused() {
    let short = b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nabc";
    assert!(!complete(short));
    assert_eq!(body(short.to_vec()), None);
}

#[test]
fn bytes_past_the_length_are_not_part_of_the_body() {
    let long = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nabXYZ";
    assert_eq!(body(long.to_vec()).as_deref(), Some(&b"ab"[..]));
}

#[test]
fn anything_but_a_200_is_nothing() {
    for r in [
        &b"HTTP/1.1 404 Not Found\r\nContent-Length: 3\r\n\r\nnop"[..],
        b"HTTP/1.1 302 Found\r\nLocation: x\r\n\r\n",
        b"HTTP/1.1 500 x 200 y\r\n\r\n",
    ] {
        assert_eq!(body(r.to_vec()), None);
    }
}

#[test]
fn without_a_length_the_whole_rest_is_the_body_and_never_complete() {
    let r = b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\nall of it";
    assert!(!complete(r));
    assert_eq!(body(r.to_vec()).as_deref(), Some(&b"all of it"[..]));
}

#[test]
fn a_status_line_without_a_reason_is_still_read() {
    let bare = b"HTTP/1.1 200\r\nContent-Length: 1\r\n\r\nx";
    assert_eq!(body(bare.to_vec()).as_deref(), Some(&b"x"[..]));
}
