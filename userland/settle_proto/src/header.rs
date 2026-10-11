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

//! The 20-byte header both directions carry, market_proto's layout:
//!
//!   0..4 magic "NSTL" (LE word) | 4..6 version | 6..8 op (echoed)
//!   | 8..10 flags (echoed) | 10..12 reserved, zero | 12..16 request id
//!   (echoed) | 16..20 payload length: the bytes after the header

pub const MAGIC: u32 = 0x4C54_534E;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Header {
    pub op: u16,
    pub flags: u16,
    pub request_id: u32,
    pub payload_len: u32,
}

/// The header and the payload it names, or None unless the magic, version
/// and reserved word are right and the payload arrived whole and exactly.
pub fn parse_header(buf: &[u8]) -> Option<(Header, &[u8])> {
    let h = buf.get(..HDR_LEN)?;
    let u16_at = |at: usize| u16::from_le_bytes([h[at], h[at + 1]]);
    let u32_at = |at: usize| u32::from_le_bytes([h[at], h[at + 1], h[at + 2], h[at + 3]]);
    if u32_at(0) != MAGIC || u16_at(4) != VERSION || u16_at(10) != 0 {
        return None;
    }
    let header =
        Header { op: u16_at(6), flags: u16_at(8), request_id: u32_at(12), payload_len: u32_at(16) };
    let end = HDR_LEN.checked_add(header.payload_len as usize)?;
    (end == buf.len()).then_some((header, &buf[HDR_LEN..]))
}
