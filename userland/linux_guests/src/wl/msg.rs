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

//! One request: object id, then size and opcode in one word, then the args.

pub struct Msg {
    buf: Vec<u8>,
    opcode: u16,
}

impl Msg {
    pub fn new(object: u32, opcode: u16) -> Msg {
        let mut buf = Vec::with_capacity(64);
        buf.extend_from_slice(&object.to_le_bytes());
        buf.extend_from_slice(&[0; 4]);
        Msg { buf, opcode }
    }

    pub fn u32(mut self, v: u32) -> Msg {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    // Length with the terminator, the bytes, the NUL, padding to a word.
    pub fn string(mut self, s: &[u8]) -> Msg {
        self.buf.extend_from_slice(&(s.len() as u32 + 1).to_le_bytes());
        self.buf.extend_from_slice(s);
        self.buf.push(0);
        while self.buf.len() % 4 != 0 {
            self.buf.push(0);
        }
        self
    }

    pub fn bytes(mut self) -> Vec<u8> {
        let word = ((self.buf.len() as u32) << 16) | self.opcode as u32;
        self.buf[4..8].copy_from_slice(&word.to_le_bytes());
        self.buf
    }
}

/// An event's header: the object it is for, its opcode, and its body.
pub fn split(rx: &[u8]) -> Option<(u32, u16, &[u8], usize)> {
    let head = rx.get(..8)?;
    let object = u32::from_le_bytes([head[0], head[1], head[2], head[3]]);
    let word = u32::from_le_bytes([head[4], head[5], head[6], head[7]]);
    let size = (word >> 16) as usize;
    if size < 8 {
        return None;
    }
    let body = rx.get(8..size)?;
    Some((object, word as u16, body, size))
}

pub fn word(body: &[u8], at: usize) -> u32 {
    body.get(at..at + 4).map_or(0, |w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}
