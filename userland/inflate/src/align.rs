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

//! Byte-level access for stored blocks and trailers.

use super::bits::Bits;

impl<'a> Bits<'a> {
    /// Drops the bits left of a partly read byte and gives back the whole
    /// bytes the buffer held, so aligned data is read straight from input.
    /// Returns the offset of the next unread byte.
    pub fn align_to_bytes(&mut self) -> usize {
        self.pos -= (self.cnt >> 3) as usize;
        self.buf = 0;
        self.cnt = 0;
        self.pos
    }

    pub fn input(&self) -> &'a [u8] {
        self.d
    }

    /// Continues bit reading at byte offset `pos` (after `align_to_bytes`).
    pub fn skip_to(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Bytes read so far, a partly read byte counting as read.
    pub fn consumed(&self) -> usize {
        self.pos - (self.cnt >> 3) as usize
    }

    /// True once every input byte has been loaded into the buffer.
    pub fn drained(&self) -> bool {
        self.pos >= self.d.len()
    }
}
