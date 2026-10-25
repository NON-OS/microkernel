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
//! Random fields for the sweeps: xorshift64*, seeded, so a failure repeats.

pub struct Rng(pub u64);

impl Rng {
    pub fn word(&mut self) -> u32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }

    /// Mostly plausible values, sometimes tiny, sometimes any 32-bit value.
    pub fn field(&mut self) -> u32 {
        match self.word() % 8 {
            0 => self.word(),
            1 => self.word() % 8,
            _ => self.word() % 9000,
        }
    }

    /// One direction's [active, blank, front, sync]: front and sync usually
    /// drawn inside the blank, so near-valid timings are common.
    pub fn side(&mut self) -> [u32; 4] {
        let (active, blank) = (self.field(), self.field());
        if self.word().is_multiple_of(4) {
            return [active, blank, self.field(), self.field()];
        }
        let room = blank.saturating_add(2).max(1);
        [active, blank, self.word() % room, self.word() % room]
    }
}
