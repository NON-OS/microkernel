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

//! A forward bit reader, least significant bit first, for FSE table
//! descriptions. It never reads past the slice it was given.

pub struct Fwd<'a> {
    d: &'a [u8],
    pos: usize,
}

impl<'a> Fwd<'a> {
    pub fn new(d: &'a [u8]) -> Self {
        Fwd { d, pos: 0 }
    }

    /// The next `n` bits (n <= 24) without consuming them; absent bits read 0.
    pub fn peek(&self, n: u32) -> u32 {
        let mut v = 0u32;
        for i in 0..n {
            let at = self.pos + i as usize;
            let bit = self.d.get(at / 8).map_or(0, |b| (b >> (at % 8)) & 1);
            v |= (bit as u32) << i;
        }
        v
    }

    /// Consume `n` bits; None if that runs past the end.
    pub fn skip(&mut self, n: u32) -> Option<()> {
        let end = self.pos.checked_add(n as usize)?;
        (end <= self.d.len() * 8).then(|| self.pos = end)
    }

    pub fn read(&mut self, n: u32) -> Option<u32> {
        let v = self.peek(n);
        self.skip(n).map(|_| v)
    }

    /// After a zero count: 2-bit repeat fields, each adding that many zeros,
    /// a 3 meaning another field follows.
    pub fn zero_run(&mut self) -> Option<usize> {
        let mut zeros = 0;
        loop {
            let rep = self.read(2)? as usize;
            zeros += rep;
            if rep != 3 {
                return Some(zeros);
            }
        }
    }

    /// Whole bytes consumed, rounding a partial byte up.
    pub fn bytes(&self) -> usize {
        self.pos.div_ceil(8)
    }
}
