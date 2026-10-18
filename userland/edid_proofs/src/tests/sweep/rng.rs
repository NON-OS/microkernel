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

//! A small seeded generator, so a failing case replays from its seed.

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed | 1)
    }

    pub fn next(&mut self) -> u64 {
        // xorshift64 with the multiply finish: full period, cheap, and
        // good enough to reach every byte value and length.
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A value in `lo..=hi`; `lo` when the range is empty.
    pub fn range(&mut self, lo: u32, hi: u32) -> u32 {
        if hi <= lo {
            return lo;
        }
        lo + (self.next() % (hi - lo + 1) as u64) as u32
    }

    /// A value in `lo..=hi` whose size is spread evenly over the powers
    /// of two, so tiny values come up as often as huge ones.
    pub fn sized(&mut self, lo: u32, hi: u32) -> u32 {
        let top = lo + (hi.saturating_sub(lo) >> self.range(0, 31));
        self.range(lo, top)
    }

    pub fn byte(&mut self) -> u8 {
        self.next() as u8
    }

    /// True once in `n`.
    pub fn one_in(&mut self, n: u32) -> bool {
        self.range(1, n.max(1)) == 1
    }

    pub fn fill(&mut self, b: &mut [u8]) {
        b.iter_mut().for_each(|v| *v = self.byte());
    }

    pub fn bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut b = [0u8; N];
        self.fill(&mut b);
        b
    }
}
