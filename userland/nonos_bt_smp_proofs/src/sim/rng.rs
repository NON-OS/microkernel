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

//! Random sources for the proofs: a fixed xorshift stream, so each run is
//! the same, and a broken source that returns one octet forever.

use nonos_bt_smp::pairing::Random;

pub struct Rng(pub u64);

impl Random for Rng {
    fn fill(&mut self, out: &mut [u8]) {
        for b in out {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            *b = (self.0 >> 24) as u8;
        }
    }
}

pub struct Stuck(pub u8);

impl Random for Stuck {
    fn fill(&mut self, out: &mut [u8]) {
        out.fill(self.0);
    }
}
