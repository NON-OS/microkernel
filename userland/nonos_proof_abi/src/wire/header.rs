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

//! The 20-byte header both directions carry, market_proto's layout under each
//! service's own magic:
//!
//!   bytes  0..4   magic, a little-endian word ("NPRV" for nonos.prove)
//!          4..6   version, 1
//!          6..8   op (a reply echoes the request's)
//!          8..10  flags (echoed)
//!         10..12  reserved, zero
//!         12..16  request id (echoed)
//!         16..20  payload length: the bytes after the header
//!
//! A reply's payload starts with a four-byte status, zero for success.

/// "NPRV", nonos.prove's magic.
pub const PROVE_MAGIC: u32 = 0x5652_504E;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Header {
    pub op: u16,
    pub flags: u16,
    pub id: u32,
    pub len: u32,
}

impl Header {
    pub fn encode(&self, magic: u32) -> [u8; HDR_LEN] {
        let mut b = [0u8; HDR_LEN];
        b[0..4].copy_from_slice(&magic.to_le_bytes());
        b[4..6].copy_from_slice(&VERSION.to_le_bytes());
        b[6..8].copy_from_slice(&self.op.to_le_bytes());
        b[8..10].copy_from_slice(&self.flags.to_le_bytes());
        b[12..16].copy_from_slice(&self.id.to_le_bytes());
        b[16..20].copy_from_slice(&self.len.to_le_bytes());
        b
    }

    /// The header of `b` and its payload, exactly `len` bytes: `None` for
    /// another magic or version, a set reserved word, or a payload that is
    /// short or followed by anything.
    pub fn parse(b: &[u8], magic: u32) -> Option<(Header, &[u8])> {
        let at16 = |i: usize| Some(u16::from_le_bytes(b.get(i..i + 2)?.try_into().ok()?));
        let at32 = |i: usize| Some(u32::from_le_bytes(b.get(i..i + 4)?.try_into().ok()?));
        if at32(0)? != magic || at16(4)? != VERSION || at16(10)? != 0 {
            return None;
        }
        let h = Header { op: at16(6)?, flags: at16(8)?, id: at32(12)?, len: at32(16)? };
        let payload = &b[HDR_LEN..];
        (payload.len() == h.len as usize).then_some((h, payload))
    }
}
