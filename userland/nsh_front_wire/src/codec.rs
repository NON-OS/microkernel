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

//! The 12-byte header: magic, version, op, status, and the body that
//! follows. `parse` takes a frame apart and checks the magic and version;
//! `write_header` lays one down. Little-endian throughout.

use crate::{HDR_LEN, MAGIC, VERSION};

/// A parsed header and the body after it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header<'a> {
    pub op: u16,
    pub status: u16,
    pub body: &'a [u8],
}

/// Lay a header at the front of `out`, returning the whole frame length for
/// a `body_len`-byte body. `out` must hold `HDR_LEN + body_len`; the caller
/// writes the body into `out[HDR_LEN..]` itself.
pub fn write_header(out: &mut [u8], op: u16, status: u16, body_len: usize) -> Option<usize> {
    let total = HDR_LEN.checked_add(body_len)?;
    if out.len() < total {
        return None;
    }
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].copy_from_slice(&status.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    Some(total)
}

/// Take a frame apart. `None` when it is too short, or its magic or version
/// is not this wire's: a peer that does not speak it is refused, not parsed.
pub fn parse(buf: &[u8]) -> Option<Header<'_>> {
    if buf.len() < HDR_LEN {
        return None;
    }
    if le32(buf, 0) != MAGIC || le16(buf, 4) != VERSION {
        return None;
    }
    Some(Header { op: le16(buf, 6), status: le16(buf, 8), body: &buf[HDR_LEN..] })
}

fn le16(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([buf[off], buf[off + 1]])
}

fn le32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}
