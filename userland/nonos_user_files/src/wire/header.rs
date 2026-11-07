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

//! The 16-byte header both ways.

use super::{MAGIC, VERSION};

pub const HEADER: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub op: u16,
    pub id: u32,
    pub body_len: u32,
}

impl Header {
    pub fn write(&self, out: &mut [u8; HEADER]) {
        out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
        out[4..6].copy_from_slice(&VERSION.to_le_bytes());
        out[6..8].copy_from_slice(&self.op.to_le_bytes());
        out[8..12].copy_from_slice(&self.id.to_le_bytes());
        out[12..16].copy_from_slice(&self.body_len.to_le_bytes());
    }

    /// The header of `msg` and its body, when the magic and version are
    /// this wire's and the body is exactly as long as the header says.
    pub fn read(msg: &[u8]) -> Option<(Header, &[u8])> {
        let head = msg.get(..HEADER)?;
        let word =
            |at: usize| u32::from_le_bytes([head[at], head[at + 1], head[at + 2], head[at + 3]]);
        let half = |at: usize| u16::from_le_bytes([head[at], head[at + 1]]);
        if word(0) != MAGIC || half(4) != VERSION {
            return None;
        }
        let h = Header { op: half(6), id: word(8), body_len: word(12) };
        let body = &msg[HEADER..];
        (body.len() == h.body_len as usize).then_some((h, body))
    }
}
