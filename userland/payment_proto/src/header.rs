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
//!   0..4 magic "NPAY" (LE word) | 4..6 version | 6..8 op (echoed)
//!   | 8..10 flags (echoed) | 10..12 reserved, zero | 12..16 request id
//!   (echoed) | 16..20 payload length: the bytes after the header
//!
//! A reply's payload starts with a four-byte status, zero for success.

use crate::bytes::fixed;

pub const MAGIC: u32 = 0x5941_504E;
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

/// The header of `buf` and the payload it names, or None unless the magic,
/// version and reserved word are right and the payload arrived whole and
/// exactly: a frame with bytes past its payload is not this wire's.
pub fn parse_header(buf: &[u8]) -> Option<(Header, &[u8])> {
    let u16_at = |i: usize| fixed::<2>(buf, i).map(u16::from_le_bytes);
    let u32_at = |i: usize| fixed::<4>(buf, i).map(u32::from_le_bytes);
    if u32_at(0)? != MAGIC || u16_at(4)? != VERSION || u16_at(10)? != 0 {
        return None;
    }
    let (op, flags, request_id, payload_len) = (u16_at(6)?, u16_at(8)?, u32_at(12)?, u32_at(16)?);
    let header = Header { op, flags, request_id, payload_len };
    let end = HDR_LEN.checked_add(usize::try_from(payload_len).ok()?)?;
    if end != buf.len() {
        return None;
    }
    Some((header, buf.get(HDR_LEN..)?))
}
