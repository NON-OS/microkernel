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

//! LZMA's range decoder. A read past the chunk yields zero; `finished`
//! then fails, so an overrun is caught once, at the chunk's end.

pub struct Range<'a> {
    d: &'a [u8],
    pos: usize,
    pub(super) range: u32,
    pub(super) code: u32,
}

impl<'a> Range<'a> {
    /// A chunk opens with a zero byte and four bytes of code.
    pub fn new(d: &'a [u8]) -> Option<Self> {
        let head = d.get(..5)?;
        if head[0] != 0 {
            return None;
        }
        let code = u32::from_be_bytes([head[1], head[2], head[3], head[4]]);
        Some(Range { d, pos: 5, range: u32::MAX, code })
    }

    pub(super) fn normalize(&mut self) {
        if self.range < 1 << 24 {
            let b = self.d.get(self.pos).copied().unwrap_or(0);
            self.pos += 1;
            self.range <<= 8;
            self.code = self.code << 8 | u32::from(b);
        }
    }

    pub fn bit(&mut self, p: &mut u16) -> u32 {
        let bound = (self.range >> 11) * u32::from(*p);
        let bit = if self.code < bound {
            *p += (2048 - *p) >> 5;
            self.range = bound;
            0
        } else {
            *p -= *p >> 5;
            self.code -= bound;
            self.range -= bound;
            1
        };
        self.normalize();
        bit
    }

    /// Every byte of the chunk read, none beyond, and the code run to zero.
    pub fn finished(&self) -> bool {
        self.code == 0 && self.pos == self.d.len()
    }
}
