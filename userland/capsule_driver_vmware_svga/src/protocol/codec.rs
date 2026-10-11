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

use super::header::{Request, HDR_LEN, MAGIC, VERSION};

/// The request and its body, or None for a frame that is not one: wrong
/// magic or version, or a length that does not match what arrived.
pub fn parse(buf: &[u8]) -> Option<(Request, &[u8])> {
    let h = buf.first_chunk::<HDR_LEN>()?;
    if u32::from_le_bytes([h[0], h[1], h[2], h[3]]) != MAGIC
        || u16::from_le_bytes([h[4], h[5]]) != VERSION
    {
        return None;
    }
    let len = u32::from_le_bytes([h[16], h[17], h[18], h[19]]) as usize;
    if HDR_LEN.checked_add(len)? != buf.len() {
        return None;
    }
    Some((refused(buf), &buf[HDR_LEN..]))
}

/// The op, flags and request id a frame names, zeros when it is too short:
/// a refusal is answered under them, as its caller waits for a reply.
pub fn refused(buf: &[u8]) -> Request {
    let Some(h) = buf.first_chunk::<HDR_LEN>() else {
        return Request { op: 0, flags: 0, request_id: 0 };
    };
    Request {
        op: u16::from_le_bytes([h[6], h[7]]),
        flags: u16::from_le_bytes([h[8], h[9]]),
        request_id: u32::from_le_bytes([h[12], h[13], h[14], h[15]]),
    }
}

pub fn response_header(out: &mut [u8], req: &Request, payload_len: u32) {
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&req.op.to_le_bytes());
    out[8..10].copy_from_slice(&req.flags.to_le_bytes());
    out[10..12].fill(0);
    out[12..16].copy_from_slice(&req.request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
}

pub fn write_status(out: &mut [u8], status: i32) {
    out[HDR_LEN..HDR_LEN + 4].copy_from_slice(&status.to_le_bytes());
}

pub fn le_u32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}
