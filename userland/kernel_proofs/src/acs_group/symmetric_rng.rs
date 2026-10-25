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

//! A seeded draw for the generated machines (symmetric_topologies.rs): a
//! xorshift, and each function's ACS from the shapes real parts show.

use super::acs_rule::{CR, FUNCTION, PORT, RR, SV};

pub(super) struct Rng(pub u64);

impl Rng {
    pub(super) fn seeded(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    pub(super) fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    /// An ACS draw: none, all on, what a function needs, part, or offered
    /// and partly off.
    pub(super) fn acs(&mut self) -> Option<(u16, u16)> {
        let draws = [
            None,
            Some((PORT, PORT)),
            Some((FUNCTION, FUNCTION)),
            Some((SV | RR, SV | RR)),
            Some((PORT, PORT & !CR)),
        ];
        draws[self.below(draws.len() as u64) as usize]
    }
}
