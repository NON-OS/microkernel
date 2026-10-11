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

//! The header in and out. A request names its op, flags, id and payload
//! length; a reply echoes the first three and carries a status word.

use super::header::{Request, HDR_LEN, MAGIC, VERSION};

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

pub fn parse(buf: &[u8]) -> Option<Request> {
    if buf.len() < HDR_LEN || u32_at(buf, 0) != MAGIC || u16_at(buf, 4) != VERSION {
        return None;
    }
    Some(refused(buf))
}

/// The request a frame `parse` turned down is answered under: the op, flags
/// and id it names, or zeros when it is too short to name them. Its sender
/// is blocked in its call until a reply comes, so a refusal is answered too.
pub fn refused(buf: &[u8]) -> Request {
    if buf.len() < HDR_LEN {
        return Request { op: 0, flags: 0, request_id: 0, payload_len: 0 };
    }
    Request {
        op: u16_at(buf, 6),
        flags: u16_at(buf, 8),
        request_id: u32_at(buf, 12),
        payload_len: u32_at(buf, 16),
    }
}

pub fn response_header(out: &mut [u8], req: &Request, payload_len: u32) {
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&req.op.to_le_bytes());
    out[8..10].copy_from_slice(&req.flags.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&req.request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
}

pub fn write_status(out: &mut [u8], status: i32) {
    out[HDR_LEN..HDR_LEN + 4].copy_from_slice(&status.to_le_bytes());
}
