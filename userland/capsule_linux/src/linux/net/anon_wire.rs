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

//! A request to net.anon's handle front, and the reading of its reply.
//! Nothing in a reply is believed before its header has been checked: it
//! arrived from another process.

use alloc::vec::Vec;

use super::anon_ops::{HDR_LEN, MAGIC, PAYLOAD_MAX, VERSION};

/// The one request id this capsule sends. A reply that does not carry it
/// back is not an answer to this request.
pub const REQUEST_ID: u32 = 1;

/// `op` carrying `body`, as net.anon's parse_req.rs reads it.
pub fn request(op: u16, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HDR_LEN + body.len());
    out.extend_from_slice(&MAGIC.to_le_bytes());
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&REQUEST_ID.to_le_bytes());
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(body);
    out
}

/// The status and body of the reply to `op` in `raw`, which holds exactly
/// the bytes that arrived; None when it is not that reply: too short, not
/// net.anon's magic or version, another op or request, or a body length
/// that is not the length that came (a reply cut short, or one with bytes
/// past its end).
pub fn reply(raw: &[u8], op: u16) -> Option<(u16, &[u8])> {
    let head = raw.get(..HDR_LEN)?;
    let u16_at = |at: usize| u16::from_le_bytes([head[at], head[at + 1]]);
    let u32_at =
        |at: usize| u32::from_le_bytes([head[at], head[at + 1], head[at + 2], head[at + 3]]);
    if u32_at(0) != MAGIC || u16_at(4) != VERSION || u16_at(6) != op {
        return None;
    }
    if u32_at(12) != REQUEST_ID {
        return None;
    }
    let body = &raw[HDR_LEN..];
    let said = usize::try_from(u32_at(16)).ok()?;
    if said != body.len() || said > PAYLOAD_MAX {
        return None;
    }
    Some((u16_at(8), body))
}

/// The bytes a call that returned `got` put in a buffer of `room`: None
/// when it is negative (no answer) or reaches past the buffer.
pub fn arrived(got: i64, room: usize) -> Option<usize> {
    let n = usize::try_from(got).ok()?;
    (n <= room).then_some(n)
}
