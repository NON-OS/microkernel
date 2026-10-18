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

//! Replies: the request's header repeated with the body's length, then
//! an i32 status, then the body (at most 252 bytes).

use crate::bytes::{i32_at, put};
use crate::errno::{Errno, EINVAL};
use crate::header::{Header, HDR_LEN, MAX_REPLY_BODY};

/// Write a reply to `req` into `out`; the number of bytes written.
pub fn reply(req: &Header, status: Errno, body: &[u8], out: &mut [u8]) -> Result<usize, Errno> {
    if body.len() > MAX_REPLY_BODY {
        return Err(EINVAL);
    }
    let h = Header { payload_len: (4 + body.len()) as u32, ..*req };
    h.encode(out)?;
    let ok = put(out, HDR_LEN, &status.to_le_bytes()) && put(out, HDR_LEN + 4, body);
    if !ok {
        return Err(EINVAL);
    }
    Ok(HDR_LEN + 4 + body.len())
}

/// A reply as the compositor reads it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reply<'a> {
    pub header: Header,
    pub status: Errno,
    pub body: &'a [u8],
}

impl<'a> Reply<'a> {
    /// Read a reply and hold it to the request it answers: same op,
    /// output and id, a status, a body no longer than allowed.
    pub fn decode(frame: &'a [u8], asked: &Header) -> Result<Self, Errno> {
        let header = Header::decode(frame)?;
        let same = header.op == asked.op
            && header.output == asked.output
            && header.request_id == asked.request_id;
        let len = header.payload_len as usize;
        if !same || !(4..=4 + MAX_REPLY_BODY).contains(&len) {
            return Err(EINVAL);
        }
        let body = frame.get(HDR_LEN + 4..HDR_LEN + len).ok_or(EINVAL)?;
        let status = i32_at(frame, HDR_LEN).ok_or(EINVAL)?;
        Ok(Reply { header, status, body })
    }
}
