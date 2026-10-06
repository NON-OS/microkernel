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
//! Turning received bytes into a response.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::super::error::HttpError;
use super::chunk::decode;
use super::framing::{content_length, transfer_codings, Codings};
use super::head::head;
use super::types::Response;

/// How many interim responses may come before the final one. Servers send
/// one or two (100 Continue, 103 Early Hints); more is a loop, not a reply.
const MAX_INTERIM: usize = 8;

/// Parse a complete response.
///
/// The body length comes from the headers rather than from how much arrived:
/// a short read has to be an error, or a truncated pack would be handed on as
/// though it were whole.
pub fn parse_response(raw: &[u8]) -> Result<Response, HttpError> {
    let mut rest = raw;
    /*
     * An interim (1xx) response comes ahead of the final one and is not it
     * (RFC 9110 15.2): taking a 103 Early Hints as the answer handed the real
     * response on as its body. 101 is final; what follows it is no longer
     * HTTP.
     */
    for _ in 0..=MAX_INTERIM {
        let (code, fields, after) = head(rest)?;
        if (100..200).contains(&code) && code != 101 {
            rest = after;
            continue;
        }
        let body = body(code, &fields, after)?;
        return Ok(Response { status: code, headers: fields, body });
    }
    Err(HttpError::StatusLine)
}

/*
 * RFC 9112 6.3, in its order: no body for 1xx, 204 and 304 whatever the
 * fields say; then Transfer-Encoding, which overrides any Content-Length;
 * then one Content-Length; else the body runs to the close, which is what
 * Connection: close asks for.
 */
fn body(code: u16, fields: &[(String, String)], after: &[u8]) -> Result<Vec<u8>, HttpError> {
    if (100..200).contains(&code) || code == 204 || code == 304 {
        return Ok(Vec::new());
    }
    match transfer_codings(fields) {
        Codings::ChunkedLast => return decode(after),
        Codings::Other => return Ok(Vec::from(after)),
        Codings::None => {}
    }
    match content_length(fields)? {
        Some(want) => after.get(..want).map(Vec::from).ok_or(HttpError::Body),
        None => Ok(Vec::from(after)),
    }
}
