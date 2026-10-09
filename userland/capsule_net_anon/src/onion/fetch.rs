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


//! Asking an HSDir for a descriptor over a BEGIN_DIR stream, and reading
//! the answer (directory_send_command and dircache's /tor/hs/3/ handler).

extern crate alloc;

use alloc::vec::Vec;

use crate::base64_encode::encode;

use super::desc::DESC_MAX;

/// The request for the descriptor stored under `blinded`. No
/// Accept-Encoding, so the HSDir answers with the document as stored; the
/// key is unpadded base64, as ed25519_public_to_base64 writes it.
pub fn request(blinded: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(96);
    out.extend_from_slice(b"GET /tor/hs/3/");
    out.extend_from_slice(&encode(blinded));
    out.extend_from_slice(b" HTTP/1.0\r\n\r\n");
    out
}

/// Why an answer held no descriptor.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Answer {
    /// Headers are not complete yet: read on.
    Partial,
    /// 404: this HSDir holds no descriptor for the key. Ask the next.
    NotFound,
    /// Any other status, or a body past DESC_MAX.
    Refused,
}

/// The body of `raw` when it is a whole 200 answer. The stream is closed by
/// the HSDir after the body, so the caller passes what it has once the
/// stream has ended.
pub fn body(raw: &[u8]) -> Result<&[u8], Answer> {
    body_within(raw, DESC_MAX)
}

/// The same, for a document whose own bound is `max`.
pub fn body_within(raw: &[u8], max: usize) -> Result<&[u8], Answer> {
    let Some(head_end) = raw.windows(4).position(|w| w == b"\r\n\r\n") else {
        return Err(if raw.len() > 4096 { Answer::Refused } else { Answer::Partial });
    };
    let status = &raw[..head_end];
    if status.starts_with(b"HTTP/1.0 404") || status.starts_with(b"HTTP/1.1 404") {
        return Err(Answer::NotFound);
    }
    if !status.starts_with(b"HTTP/1.0 200") && !status.starts_with(b"HTTP/1.1 200") {
        return Err(Answer::Refused);
    }
    let body = &raw[head_end + 4..];
    if body.is_empty() || body.len() > max {
        return Err(Answer::Refused);
    }
    Ok(body)
}
