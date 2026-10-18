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

//! Requests and replies, framed, and the check a caller makes before it
//! believes a reply.

use alloc::vec::Vec;

use super::header::{Header, HDR_LEN, STATUS_LEN};

/// The header for `op` and `id`, then `body`.
pub fn request(magic: u32, op: u16, id: u32, body: &[u8]) -> Vec<u8> {
    let h = Header { op, flags: 0, id, len: len_of(0, body) };
    [&h.encode(magic)[..], body].concat()
}

/// The reply to `req`: its op, flags and id echoed, then `status` and `body`.
pub fn reply(magic: u32, req: &Header, status: i32, body: &[u8]) -> Vec<u8> {
    let len = len_of(STATUS_LEN, body);
    let h = Header { op: req.op, flags: req.flags, id: req.id, len };
    [&h.encode(magic)[..], &status.to_le_bytes(), body].concat()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReplyError {
    /// Not this service's frame, or not whole.
    Malformed,
    /// A frame for another op or request.
    Stale,
    /// The service refused: an errno or a wire status (super::status).
    Status(i32),
}

/// The body after the status of `rx`, the reply to `op` sent with `id`.
pub fn reply_body(rx: &[u8], magic: u32, op: u16, id: u32) -> Result<&[u8], ReplyError> {
    let (h, payload) = Header::parse(rx, magic).ok_or(ReplyError::Malformed)?;
    if h.op != op || h.id != id {
        return Err(ReplyError::Stale);
    }
    let status = payload.get(..STATUS_LEN).ok_or(ReplyError::Malformed)?;
    match i32::from_le_bytes(status.try_into().map_err(|_| ReplyError::Malformed)?) {
        0 => Ok(&payload[STATUS_LEN..]),
        s => Err(ReplyError::Status(s)),
    }
}

/// The largest frame either side sends: a GET's 64 KiB and a header and
/// status around it.
/// The header's length word. A body past u32 saturates it, so the frame
/// fails Header::parse's exact length check instead of naming a shorter one.
fn len_of(head: usize, body: &[u8]) -> u32 {
    head.checked_add(body.len()).and_then(|n| u32::try_from(n).ok()).unwrap_or(u32::MAX)
}

pub const FRAME_MAX: usize = HDR_LEN + STATUS_LEN + super::prove::GET_MAX;
