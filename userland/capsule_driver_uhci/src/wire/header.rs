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

//! The 20-byte header: magic, version, op, flags, a reserved u16, request
//! id and payload length, little-endian. A reply echoes the op, flags and
//! id, and its payload starts with an i32 status.

pub const MAGIC: u32 = 0x4E58_4843;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Request {
    pub op: u16,
    pub flags: u16,
    pub request_id: u32,
    pub payload_len: u32,
}

pub fn decode(buf: &[u8]) -> Option<Request> {
    let h = buf.get(..HDR_LEN)?;
    let u16_at = |i: usize| u16::from_le_bytes([h[i], h[i + 1]]);
    let u32_at = |i: usize| u32::from_le_bytes([h[i], h[i + 1], h[i + 2], h[i + 3]]);
    if u32_at(0) != MAGIC || u16_at(4) != VERSION {
        return None;
    }
    Some(Request {
        op: u16_at(6),
        flags: u16_at(8),
        request_id: u32_at(12),
        payload_len: u32_at(16),
    })
}

/// The reply header for `req` and the status after it; returns where the
/// reply's data starts.
pub fn encode_header(out: &mut [u8], req: &Request, status: i32, data_len: usize) -> usize {
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&req.op.to_le_bytes());
    out[8..10].copy_from_slice(&req.flags.to_le_bytes());
    out[10..12].copy_from_slice(&[0, 0]);
    out[12..16].copy_from_slice(&req.request_id.to_le_bytes());
    out[16..20].copy_from_slice(&((STATUS_LEN + data_len) as u32).to_le_bytes());
    out[20..24].copy_from_slice(&status.to_le_bytes());
    HDR_LEN + STATUS_LEN
}
