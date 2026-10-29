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

//! A content tar's reply read as a part (install/http/part.rs): a 200 from
//! its start, a 206 only from the byte held and to the last, and no byte
//! past the whole. resume.rs adds the parts across broken streams.

/* part.rs's own parents, so its `super::super::http_reply` is found. */
mod install;

use install::http::part::part;

fn reply(head: &str, body: &[u8]) -> Vec<u8> {
    [head.as_bytes(), b"\r\n\r\n", body].concat()
}

#[test]
fn a_200_that_stops_short_is_a_part_from_byte_0_of_its_length() {
    let p = part(reply("HTTP/1.1 200 OK\r\nContent-Length: 10", b"abcd"), 0).unwrap();
    assert_eq!((p.from, p.total, &p.bytes[..]), (0, 10, &b"abcd"[..]));
}

#[test]
fn a_206_from_the_byte_held_to_the_last_is_added() {
    let head = "HTTP/1.1 206 Partial Content\r\nContent-Length: 6\r\nContent-Range: bytes 4-9/10";
    let p = part(reply(head, b"efghij"), 4).unwrap();
    assert_eq!((p.from, p.total, &p.bytes[..]), (4, 10, &b"efghij"[..]));
}

#[test]
fn a_206_from_another_byte_or_short_of_the_end_is_nothing() {
    let off = "HTTP/1.1 206 x\r\nContent-Length: 6\r\nContent-Range: bytes 3-8/10";
    assert!(part(reply(off, b"defghi"), 4).is_none());
    assert!(part(reply(off, b"defghi"), 3).is_none());
    let bad = "HTTP/1.1 206 x\r\nContent-Range: bytes 4-9/*";
    assert!(part(reply(bad, b"efghij"), 4).is_none());
}

#[test]
fn any_other_status_or_a_200_with_no_length_is_nothing() {
    assert!(part(reply("HTTP/1.1 404 Not Found\r\nContent-Length: 2", b"no"), 0).is_none());
    assert!(part(reply("HTTP/1.1 200 OK", b"abc"), 0).is_none());
    assert!(part(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n".to_vec(), 0).is_none());
}

#[test]
fn no_byte_past_the_whole_is_kept() {
    let p = part(reply("HTTP/1.1 200 OK\r\nContent-Length: 3", b"abcXYZ"), 0).unwrap();
    assert_eq!(&p.bytes[..], b"abc");
    let head = "HTTP/1.1 206 x\r\nContent-Range: bytes 8-9/10";
    assert_eq!(&part(reply(head, b"ijKL"), 8).unwrap().bytes[..], b"ij");
}
