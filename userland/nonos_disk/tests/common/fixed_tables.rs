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

//! The seeded tables the fixed-sector sweep reads.

use crate::fixed_sweep::{entry, EDGES, ESP, FIXED_HI, FIXED_LO};
use crate::kernel_gpt::places::{NONOS_DATA, NONOS_PLAN, NONOS_STORE};
use crate::kernel_gpt::Entry;

pub struct Rng(pub u64);

impl Rng {
    pub fn below(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n
    }
}

/// One partition of a foreign type, an ESP or one of NONOS's three, at the fixed
/// edges, below them, above them or anywhere low; one time in three NONOS's
/// store and plan named as well, far above.
pub fn table(r: &mut Rng) -> ([u8; 16], u64, u64, Vec<Entry>) {
    let kind = [[r.below(255) as u8 + 1; 16], ESP, NONOS_STORE, NONOS_PLAN, NONOS_DATA]
        [r.below(5) as usize];
    let (a, b) = match r.below(4) {
        0 => (EDGES[r.below(6) as usize], EDGES[r.below(6) as usize]),
        1 => (34 + r.below(FIXED_LO), FIXED_LO - 1),
        2 => (FIXED_HI + r.below(1 << 30), FIXED_HI + (1 << 30) + r.below(1 << 20)),
        _ => (34 + r.below(1 << 20), 34 + r.below(1 << 21)),
    };
    let mut entries = vec![entry(kind, a.min(b), a.max(b))];
    if r.below(3) == 0 {
        let far = FIXED_HI + (1 << 31);
        entries.push(entry(NONOS_STORE, far, far + 99));
        entries.push(entry(NONOS_PLAN, far << 1, (far << 1) + 1));
    }
    (kind, a, b, entries)
}
