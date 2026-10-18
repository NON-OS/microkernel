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

//! The header both ways: magic, version, op, flags, a reserved half-word,
//! the request id and the payload length, little endian. A reply echoes
//! the request's op, flags and id, and its payload starts with the status.

/// "NSDR": NONOS SD reader.
pub const MAGIC: u32 = 0x4E53_4452;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Request {
    pub op: u16,
    pub flags: u16,
    pub request_id: u32,
    pub payload_len: u32,
}

fn le16(b: &[u8; HDR_LEN], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn le32(b: &[u8; HDR_LEN], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

/// `None` for fewer than twenty bytes, another magic or another version.
pub fn decode_request(buf: &[u8]) -> Option<Request> {
    let h: &[u8; HDR_LEN] = buf.get(..HDR_LEN)?.try_into().ok()?;
    if le32(h, 0) != MAGIC || le16(h, 4) != VERSION {
        return None;
    }
    let (op, flags, request_id, payload_len) = (le16(h, 6), le16(h, 8), le32(h, 12), le32(h, 16));
    Some(Request { op, flags, request_id, payload_len })
}

/// The reply's header and status into `out`, for a payload of `body`
/// bytes after the status.
pub fn encode_reply(out: &mut [u8; HDR_LEN + STATUS_LEN], req: &Request, status: i32, body: u32) {
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&req.op.to_le_bytes());
    out[8..10].copy_from_slice(&req.flags.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&req.request_id.to_le_bytes());
    out[16..20].copy_from_slice(&(STATUS_LEN as u32).saturating_add(body).to_le_bytes());
    out[20..24].copy_from_slice(&status.to_le_bytes());
}
