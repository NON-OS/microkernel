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

//! A small seeded generator, and the ops and words the sweep draws from.

pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    pub fn below(&mut self, n: u32) -> u32 {
        (self.next() % n.max(1) as u64) as u32
    }

    /// HEALTHCHECK, a private op, each common op 0x10 to 0x17, and two
    /// the wire does not define.
    pub fn op(&mut self) -> u16 {
        match self.below(12) {
            0 => 0x01,
            1 => 0x03,
            10 => 0x18,
            11 => 0xFFFF,
            n => 0x10 + (n - 2) as u16,
        }
    }

    /// An edge of some bound, or noise.
    pub fn word(&mut self) -> u32 {
        const EDGES: [u32; 8] = [0, 1, 64, 65, 640, 800, 4096, u32::MAX];
        if self.below(9) == 0 {
            self.next() as u32
        } else {
            EDGES[self.below(8) as usize]
        }
    }
}
