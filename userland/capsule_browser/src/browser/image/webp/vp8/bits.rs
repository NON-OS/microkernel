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

/// The VP8 boolean entropy decoder (RFC 6386 section 7). `range` holds the
/// current range minus one; `bits` counts the unread bits in `value`
/// below the 8 the arithmetic works on. Reading past the data feeds zeros
/// once and then marks the stream exhausted.
pub(super) struct Bools<'a> {
    data: &'a [u8],
    pos: usize,
    value: u64,
    range: u32,
    bits: i32,
    pub eof: bool,
}

impl<'a> Bools<'a> {
    pub(super) fn new(data: &'a [u8]) -> Bools<'a> {
        let mut b = Bools { data, pos: 0, value: 0, range: 254, bits: -8, eof: false };
        b.load();
        b
    }

    fn load(&mut self) {
        while self.bits < 0 {
            if let Some(&byte) = self.data.get(self.pos) {
                self.pos += 1;
                self.value = (self.value << 8) | byte as u64;
                self.bits += 8;
            } else if !self.eof {
                self.value <<= 8;
                self.bits += 8;
                self.eof = true;
            } else {
                self.bits = 0;
            }
        }
    }

    /// One bit that is 1 with probability (256 - prob) / 256.
    pub(super) fn bit(&mut self, prob: u8) -> bool {
        self.load();
        let pos = self.bits;
        let split = (self.range * prob as u32) >> 8;
        let value = (self.value >> pos) as u32;
        let one = value > split;
        let mut range = if one {
            self.value -= ((split + 1) as u64) << pos;
            self.range - split
        } else {
            split + 1
        };
        let shift = range.leading_zeros() as i32 - 24;
        range <<= shift;
        self.bits -= shift;
        self.range = range - 1;
        one
    }
}
