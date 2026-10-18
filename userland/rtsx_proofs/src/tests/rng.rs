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

//! A fixed-seed generator for the hostile sweeps: the same bytes on every
//! run, so a failure names its case, and no dependency.

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    pub fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }

    /// Fewer than `max` bytes, of any value.
    pub fn bytes(&mut self, max: u64) -> Vec<u8> {
        let len = self.below(max);
        (0..len).map(|_| self.next() as u8).collect()
    }

    pub fn words(&mut self) -> [u32; 4] {
        [0; 4].map(|_: u32| self.next() as u32)
    }

    /// One of `edges` half the time, any value the other half.
    pub fn edge(&mut self, edges: &[u64]) -> u64 {
        match self.below(2) {
            0 => edges[self.below(edges.len() as u64) as usize],
            _ => self.next(),
        }
    }
}
