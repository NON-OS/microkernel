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

//! A stream carries the next GET only when the reply on it ended exactly at
//! its Content-Length and the mirror did not say it closes (fast downloads,
//! B; install/http_reply.rs, reusable). Anything else is dropped.

use crate::install::http_reply::reusable;

const OK: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";

#[test]
fn a_whole_reply_leaves_its_stream_for_the_next_get() {
    assert!(reusable(OK));
}

#[test]
fn a_short_reply_or_bytes_past_it_drop_the_stream() {
    assert!(!reusable(&OK[..OK.len() - 1]), "short");
    assert!(!reusable(&[OK, b"HTTP/1.1"].concat()), "the next reply's start, unasked");
}

#[test]
fn a_mirror_that_closes_or_sends_no_length_drops_it() {
    assert!(!reusable(b"HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: 2\r\n\r\nhi"));
    assert!(!reusable(b"HTTP/1.1 200 OK\r\nCONNECTION: Close\r\nContent-Length: 2\r\n\r\nhi"));
    assert!(!reusable(
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\nhi\r\n0\r\n\r\n"
    ));
    assert!(reusable(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n"), "a 404 is whole");
}
