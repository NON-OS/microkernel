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

//! The chunked coding (RFC 9112 7.1): a hex size, CRLF, that many bytes,
//! CRLF, until a zero size.

use nonos_http::{parse_response, HttpError};

fn chunked(wire: &[u8]) -> Result<Vec<u8>, HttpError> {
    let mut raw = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n".to_vec();
    raw.extend_from_slice(wire);
    parse_response(&raw).map(|r| r.body)
}

/*
 * Each chunk's data is followed by CRLF. Two bytes were skipped there
 * unread, so a chunk whose size understated its data had its tail read as
 * the next size line, or a stray byte pair taken as the terminator.
 */
#[test]
fn chunk_data_not_followed_by_crlf_is_refused() {
    assert_eq!(chunked(b"3\r\nabcde\r\n0\r\n\r\n"), Err(HttpError::Chunk));
    assert_eq!(chunked(b"3\r\nabcXY0\r\n\r\n"), Err(HttpError::Chunk));
}

#[test]
fn whitespace_before_an_extension_or_the_line_end_is_allowed() {
    assert_eq!(chunked(b"3 ;name=v\r\nabc\r\n0\r\n\r\n"), Ok(b"abc".to_vec()));
    assert_eq!(chunked(b"3\t\r\nabc\r\n0 \r\n\r\n"), Ok(b"abc".to_vec()));
}

#[test]
fn sizes_that_overflow_or_are_not_hex_are_refused() {
    assert_eq!(chunked(b"ffffffffffffffffff\r\nabc\r\n0\r\n\r\n"), Err(HttpError::Chunk));
    assert_eq!(chunked(b"\r\nabc\r\n0\r\n\r\n"), Err(HttpError::Chunk));
    assert_eq!(chunked(b" 3\r\nabc\r\n0\r\n\r\n"), Err(HttpError::Chunk));
    assert_eq!(chunked(b"3 4\r\nabc\r\n0\r\n\r\n"), Err(HttpError::Chunk));
}

#[test]
fn a_size_larger_than_what_arrived_is_short() {
    assert_eq!(chunked(b"7fffffffffffffff\r\nabc\r\n0\r\n\r\n"), Err(HttpError::Body));
}

#[test]
fn trailers_after_the_last_chunk_are_ignored() {
    assert_eq!(chunked(b"3\r\nabc\r\n0\r\nExpires: never\r\n\r\n"), Ok(b"abc".to_vec()));
}

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

/// Any body, cut into chunks of any sizes in any case of hex, decodes back
/// to itself.
#[test]
fn any_body_chunked_any_way_decodes_back_to_itself() {
    let mut s = 0xDEC0_DE01u32;
    for _ in 0..20_000 {
        let body: Vec<u8> = (0..xorshift(&mut s) % 600).map(|_| xorshift(&mut s) as u8).collect();
        let mut wire = Vec::new();
        let mut at = 0;
        while at < body.len() {
            let n = (1 + xorshift(&mut s) as usize % 97).min(body.len() - at);
            let size = if xorshift(&mut s) % 2 == 0 { format!("{n:x}") } else { format!("{n:X}") };
            wire.extend_from_slice(size.as_bytes());
            if xorshift(&mut s) % 4 == 0 {
                wire.extend_from_slice(b";ext=1");
            }
            wire.extend_from_slice(b"\r\n");
            wire.extend_from_slice(&body[at..at + n]);
            wire.extend_from_slice(b"\r\n");
            at += n;
        }
        wire.extend_from_slice(b"0\r\n\r\n");
        assert_eq!(chunked(&wire), Ok(body));
    }
}
