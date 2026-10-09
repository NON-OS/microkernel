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

//! The backward bit stream Huffman and FSE payloads are written as: read from
//! the last byte toward the first, starting below the final byte's highest set
//! bit. Reads past the start yield zeros and are remembered as overflow.

pub struct Back<'a> {
    d: &'a [u8],
    /// Bits still unread, counting from the start of `d`; negative once overrun.
    left: isize,
}

impl<'a> Back<'a> {
    /// None when the stream is empty or its last byte has no end mark.
    pub fn new(d: &'a [u8]) -> Option<Self> {
        let last = *d.last()?;
        if last == 0 {
            return None;
        }
        let mark = 7 - last.leading_zeros() as isize;
        Some(Back { d, left: (d.len() as isize - 1) * 8 + mark })
    }

    fn bit(&self, at: isize) -> u64 {
        if at < 0 {
            return 0;
        }
        let at = at as usize;
        self.d.get(at / 8).map_or(0, |b| u64::from((b >> (at % 8)) & 1))
    }

    /// The next `n` bits (n <= 56), most significant first, without consuming.
    pub fn peek(&self, n: u32) -> u64 {
        (1..=n as isize).fold(0, |v, i| (v << 1) | self.bit(self.left - i))
    }

    pub fn read(&mut self, n: u32) -> u64 {
        let v = self.peek(n);
        self.left -= n as isize;
        v
    }

    /// Every bit read, and none beyond.
    pub fn done(&self) -> bool {
        self.left == 0
    }

    pub fn overrun(&self) -> bool {
        self.left < 0
    }
}
