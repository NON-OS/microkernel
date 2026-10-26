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

//! The seeded generator the sweep draws from.

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    pub fn range(&mut self, lo: u64, hi: u64) -> u64 {
        match hi - lo {
            u64::MAX => self.next(),
            span => lo + self.next() % (span + 1),
        }
    }

    pub fn one_in(&mut self, n: u64) -> bool {
        self.next().is_multiple_of(n)
    }

    pub fn one_of(&mut self, v: &[u64]) -> u64 {
        v[self.range(0, v.len() as u64 - 1) as usize]
    }

    pub fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.next() as u8).collect()
    }
}

/// Fewer rounds under Miri; the full count is the native sweep's.
pub fn rounds(full: u64) -> u64 {
    if cfg!(miri) {
        100
    } else {
        full
    }
}
