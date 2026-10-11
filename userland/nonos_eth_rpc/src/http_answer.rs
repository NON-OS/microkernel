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

//! An answer's HTTP framing, read strictly: the node said 200, and the body
//! is all there. The JSON readers search the body as text, so a body cut
//! short, or one in chunks whose size lines would split a value, is never
//! handed on as an answer.

use alloc::vec::Vec;

use crate::http_parts::{dechunk, decimal, header};

/// What the HTTP layer of an answer says.
#[derive(Debug, PartialEq, Eq)]
pub enum Http {
    /// The whole body, chunks joined.
    Body(Vec<u8>),
    /// The node answered another status.
    Status(u16),
    /// The answer stops before its body is whole, or does not read as HTTP.
    Incomplete,
}

/// Read the HTTP framing of a whole decrypted answer.
pub fn http_answer(resp: &[u8]) -> Http {
    let Some(head_end) = resp.windows(4).position(|w| w == b"\r\n\r\n") else {
        return Http::Incomplete;
    };
    let head = &resp[..head_end];
    let body = &resp[head_end + 4..];
    let status = head
        .strip_prefix(b"HTTP/1.1 ")
        .or_else(|| head.strip_prefix(b"HTTP/1.0 "))
        .and_then(|s| s.get(..3))
        .and_then(decimal);
    match status {
        Some(200) => {}
        Some(code) => return Http::Status(code as u16),
        None => return Http::Incomplete,
    }
    let chunked =
        header(head, b"transfer-encoding").is_some_and(|v| v.eq_ignore_ascii_case(b"chunked"));
    if chunked {
        return dechunk(body).map_or(Http::Incomplete, Http::Body);
    }
    match header(head, b"content-length") {
        Some(len) => match decimal(len) {
            Some(n) if body.len() >= n => Http::Body(body[..n].to_vec()),
            _ => Http::Incomplete,
        },
        /* No length: the body runs to the close of an authenticated stream. */
        None => Http::Body(body.to_vec()),
    }
}
