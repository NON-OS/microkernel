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

use crate::error::Error;

/// Bits of the input, least significant bit of each byte first.
pub(crate) struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
    acc: u64,
    n: u32,
}

impl<'a> Bits<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Bits<'a> {
        Bits { data, pos: 0, acc: 0, n: 0 }
    }

    /// The next `k` (0..=32) bits unconsumed, and how many of them the
    /// input really has; missing bits read as zero.
    pub(crate) fn peek(&mut self, k: u32) -> (u32, u32) {
        while self.n <= 56 && self.pos < self.data.len() {
            self.acc |= (self.data[self.pos] as u64) << self.n;
            self.pos += 1;
            self.n += 8;
        }
        ((self.acc & ((1u64 << k) - 1)) as u32, self.n.min(k))
    }

    /// Consume `k` bits that `peek` reported present.
    pub(crate) fn skip(&mut self, k: u32) {
        self.acc >>= k;
        self.n -= k;
    }

    /// The next `k` (0..=32) bits as an integer, first bit lowest.
    pub(crate) fn read(&mut self, k: u32) -> Result<u32, Error> {
        match self.peek(k) {
            (v, have) if have == k => {
                self.skip(k);
                Ok(v)
            }
            _ => Err(Error::Truncated),
        }
    }

    /// How many bits are buffered ahead of the input position.
    pub(crate) fn buffered(&self) -> u32 {
        self.n
    }

    /// `len` input bytes past those buffered, or None when too few are left.
    pub(crate) fn raw(&mut self, len: usize) -> Option<&'a [u8]> {
        let s = self.data.get(self.pos..self.pos.checked_add(len)?)?;
        self.pos += len;
        Some(s)
    }
}
