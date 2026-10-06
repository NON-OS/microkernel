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

//! Where a body ends (RFC 9112 6.3): Transfer-Encoding over Content-Length,
//! one length or none, and no body at all where the status says so.

use nonos_http::{parse_response, HttpError};

fn body(raw: &[u8]) -> Result<Vec<u8>, HttpError> {
    parse_response(raw).map(|r| r.body)
}

/*
 * Two lengths that disagree are two framings of one stream, the shape of
 * response splitting. The first one was believed.
 */
#[test]
fn conflicting_content_lengths_are_refused() {
    let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nContent-Length: 7\r\n\r\nhello, world";
    assert_eq!(body(raw), Err(HttpError::Body));
    let listed = b"HTTP/1.1 200 OK\r\nContent-Length: 5, 7\r\n\r\nhello, world";
    assert_eq!(body(listed), Err(HttpError::Body));
}

#[test]
fn a_repeated_identical_length_is_one_length() {
    let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 5, 5\r\nContent-Length: 5\r\n\r\nhello!";
    assert_eq!(body(raw), Ok(b"hello".to_vec()));
}

/// A length is digits and nothing else; the integer parser also took a sign.
#[test]
fn a_length_that_is_not_only_digits_is_refused() {
    for value in ["+5", "-5", "5x", "0x5", "", " ", "5 5", "99999999999999999999999"] {
        let raw = format!("HTTP/1.1 200 OK\r\nContent-Length: {value}\r\n\r\nhello");
        assert_eq!(body(raw.as_bytes()), Err(HttpError::Body), "{value:?}");
    }
}

/*
 * The coding applied last is the one that frames the body. "gzip, chunked"
 * is chunked; it was read as not chunked and the chunk framing handed on as
 * body bytes.
 */
#[test]
fn chunked_as_the_final_coding_is_decoded() {
    let raw = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip, chunked\r\n\r\n3\r\nabc\r\n0\r\n\r\n";
    assert_eq!(body(raw), Ok(b"abc".to_vec()));
    let split = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip\r\nTransfer-Encoding: Chunked\r\n\r\n3\r\nabc\r\n0\r\n\r\n";
    assert_eq!(body(split), Ok(b"abc".to_vec()), "codings listed over two fields, any case");
}

#[test]
fn a_coding_after_chunked_runs_the_body_to_the_close() {
    let raw = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked, gzip\r\nContent-Length: 2\r\n\r\nraw bytes";
    assert_eq!(body(raw), Ok(b"raw bytes".to_vec()), "neither chunked nor the length frames it");
}

#[test]
fn transfer_encoding_overrides_content_length() {
    let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\nTransfer-Encoding: chunked\r\n\r\n3\r\nabc\r\n0\r\n\r\n";
    assert_eq!(body(raw), Ok(b"abc".to_vec()));
}

/*
 * 204 and 304 never carry a body, and a 304 may still state the length of
 * the representation it stands for. That length was waited for and the
 * response refused as short.
 */
#[test]
fn no_content_and_not_modified_have_no_body() {
    let not_modified = b"HTTP/1.1 304 Not Modified\r\nContent-Length: 1234\r\nETag: \"x\"\r\n\r\n";
    assert_eq!(body(not_modified), Ok(Vec::new()));
    let no_content = b"HTTP/1.1 204 No Content\r\nTransfer-Encoding: chunked\r\n\r\n";
    assert_eq!(body(no_content), Ok(Vec::new()));
}

#[test]
fn no_length_and_no_coding_runs_to_the_close() {
    let raw = b"HTTP/1.0 200 OK\r\nServer: old\r\n\r\neverything that followed";
    assert_eq!(body(raw), Ok(b"everything that followed".to_vec()));
}
