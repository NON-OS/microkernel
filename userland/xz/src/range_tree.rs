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

//! Direct bits, and bit trees over the range decoder: most significant bit
//! first, and the reverse order the distance and alignment bits use.

use super::range::Range;

impl Range<'_> {
    pub fn direct(&mut self, n: u32) -> u32 {
        let mut v = 0u32;
        for _ in 0..n {
            self.range >>= 1;
            self.code = self.code.wrapping_sub(self.range);
            let t = 0u32.wrapping_sub(self.code >> 31);
            self.code = self.code.wrapping_add(self.range & t);
            self.normalize();
            v = (v << 1).wrapping_add(t.wrapping_add(1));
        }
        v
    }

    pub fn tree(&mut self, probs: &mut [u16], bits: u32) -> u32 {
        let mut m = 1usize;
        for _ in 0..bits {
            m = (m << 1) + self.bit(&mut probs[m]) as usize;
        }
        m as u32 - (1 << bits)
    }

    pub fn reverse(&mut self, probs: &mut [u16], bits: u32) -> u32 {
        let (mut m, mut v) = (1usize, 0u32);
        for i in 0..bits {
            let bit = self.bit(&mut probs[m]);
            m = (m << 1) + bit as usize;
            v |= bit << i;
        }
        v
    }
}
