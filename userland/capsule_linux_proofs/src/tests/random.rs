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

//! Register values the way a hostile guest picks them: mostly the edges,
//! where arithmetic wraps and bounds are off by one, and the rest anywhere.
//! Seeded, so a failure names a run that can be repeated.

use crate::linux::guest::{MMAP_BASE, PAGE, USER_MAX};

pub struct Regs(u64);

/// The values a bound is most often wrong at.
const EDGES: [u64; 16] = [
    0,
    1,
    PAGE - 1,
    PAGE,
    PAGE + 1,
    0x1_0000,
    MMAP_BASE,
    USER_MAX - PAGE,
    USER_MAX,
    USER_MAX + 1,
    i32::MAX as u64,
    u32::MAX as u64,
    i64::MAX as u64,
    1 << 63,
    u64::MAX - PAGE,
    u64::MAX,
];

impl Regs {
    pub fn new(seed: u64) -> Regs {
        Regs(seed | 1)
    }

    pub fn any(&mut self) -> u64 {
        let s = &mut self.0;
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    /// An edge, an edge nudged by a little, or anything at all.
    pub fn arg(&mut self) -> u64 {
        let pick = self.any();
        let edge = EDGES[(pick % EDGES.len() as u64) as usize];
        match (pick >> 8) % 4 {
            0 => edge,
            1 => edge.wrapping_add(self.any() % (2 * PAGE)),
            2 => edge.wrapping_sub(self.any() % (2 * PAGE)),
            _ => self.any(),
        }
    }

    /// A small value, for flags and counts that are mostly in range.
    pub fn small(&mut self, below: u64) -> u64 {
        self.any() % below.max(1)
    }
}
