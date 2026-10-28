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

//! The NSO1 wire S1's SERVE_ONE speaks (docs/design/serve-one.md): one
//! IPC message is a twelve-byte head, "NSO1", the op, a zero, the
//! connection and the length of what follows, then those bytes. A frame
//! carries at most 64 KiB.

use alloc::vec::Vec;

pub const MAGIC: &[u8; 4] = b"NSO1";
pub const OPEN: u8 = 1;
pub const WRITE: u8 = 2;
pub const READ: u8 = 3;
pub const CLOSE: u8 = 4;
pub const HEAD: usize = 12;
pub const FRAME_MAX: usize = 64 << 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Head {
    pub op: u8,
    pub conn: u16,
    pub len: u32,
}

/// One message: the head for `op` on `conn`, then `data`, cut to a frame.
pub fn message(op: u8, conn: u16, data: &[u8]) -> Vec<u8> {
    let data = &data[..data.len().min(FRAME_MAX)];
    let mut out = Vec::with_capacity(HEAD + data.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&[op, 0]);
    out.extend_from_slice(&conn.to_le_bytes());
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(data);
    out
}

/// The head of a reply and the bytes its length names, or None when the
/// reply is short, is not NSO1, or names more than it holds.
pub fn parse(buf: &[u8]) -> Option<(Head, &[u8])> {
    if buf.len() < HEAD || &buf[..4] != MAGIC {
        return None;
    }
    let conn = u16::from_le_bytes([buf[6], buf[7]]);
    let len = u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]);
    let body = buf.get(HEAD..HEAD + len as usize)?;
    Some((Head { op: buf[4], conn, len }, body))
}
