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

//! Header field lines (RFC 9112 5): a token, a colon, a value.

use nonos_http::{parse_response, HttpError};

/*
 * "Content-Length : 5" named a field "content-length " that no lookup
 * matched, so the length was ignored and the body ran to the close. RFC 9112
 * 5.1 allows no whitespace before the colon, for exactly that reason.
 */
#[test]
fn a_name_that_is_not_a_token_is_refused() {
    let spaced = b"HTTP/1.1 200 OK\r\nContent-Length : 5\r\n\r\nhello trailing";
    assert_eq!(parse_response(spaced).err(), Some(HttpError::Header));
    for name in ["Bad Name", "a\"b", "x(y)", "\x01ctl", "\u{e9}t\u{e9}"] {
        let raw = format!("HTTP/1.1 200 OK\r\n{name}: v\r\n\r\n");
        assert_eq!(parse_response(raw.as_bytes()).err(), Some(HttpError::Header), "{name:?}");
    }
}

#[test]
fn every_token_character_is_a_name_character() {
    let raw = b"HTTP/1.1 200 OK\r\nX-a.b_c~d!#$%&'*+^`|9: v\r\n\r\n";
    let r = parse_response(raw).expect("parse");
    assert_eq!(r.header("x-a.b_c~d!#$%&'*+^`|9"), Some("v"));
}

/*
 * RFC 9112 5.2: a user agent replaces an obsolete line fold with a space.
 * The continuation line had no colon of its own, so the whole response was
 * refused.
 */
#[test]
fn an_obsolete_line_fold_continues_the_value() {
    let raw = b"HTTP/1.1 200 OK\r\nX-Long: first\r\n  second\r\n\tthird\r\nContent-Length: 0\r\n\r\n";
    let r = parse_response(raw).expect("parse");
    assert_eq!(r.header("x-long"), Some("first second third"));
}

#[test]
fn a_fold_with_no_field_before_it_is_refused() {
    let raw = b"HTTP/1.1 200 OK\r\n  orphan\r\n\r\n";
    assert_eq!(parse_response(raw).err(), Some(HttpError::Header));
}

#[test]
fn values_keep_inner_colons_and_lose_outer_whitespace() {
    let raw = b"HTTP/1.1 200 OK\r\nLocation:   https://a.example:8443/x  \r\n\r\n";
    let r = parse_response(raw).expect("parse");
    assert_eq!(r.header("location"), Some("https://a.example:8443/x"));
}

/// A head of `fields` lines of `width` value bytes each, then a body.
fn head_of(fields: usize, width: usize, blank_line: bool) -> Vec<u8> {
    let mut raw = b"HTTP/1.1 200 OK\r\n".to_vec();
    for i in 0..fields {
        raw.extend_from_slice(format!("x-{i}: {}\r\n", "v".repeat(width)).as_bytes());
    }
    if blank_line {
        raw.extend_from_slice(b"Content-Length: 2\r\n\r\nok");
    }
    raw
}

/*
 * The field count was capped at 128 but not their size, so 100 fields of a
 * kilobyte each, well inside any download limit a caller sets, were all
 * collected into strings. A head is now at most 64 KiB.
 */
#[test]
fn a_head_larger_than_64_kib_is_refused() {
    assert_eq!(parse_response(&head_of(100, 1024, true)).err(), Some(HttpError::Header));
    assert_eq!(
        parse_response(&head_of(100, 1024, false)).err(),
        Some(HttpError::Header),
        "refused as too large, not left waiting as incomplete"
    );
}

#[test]
fn a_head_just_under_64_kib_is_read() {
    let r = parse_response(&head_of(60, 1000, true)).expect("parse");
    assert_eq!(r.body, b"ok");
    assert_eq!(r.headers.len(), 61);
}
