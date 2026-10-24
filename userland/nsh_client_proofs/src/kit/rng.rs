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

//! Deterministic randomness, so every run of a proof is the same run: a
//! SHA-512 stream over a seed and a counter.

use nonos_hash::sha512;

pub struct Det {
    seed: u8,
    ctr: u64,
}

impl Det {
    pub const fn new(seed: u8) -> Self {
        Det { seed, ctr: 0 }
    }

    pub fn fill(&mut self, out: &mut [u8]) -> bool {
        for chunk in out.chunks_mut(64) {
            let mut input = [0u8; 9];
            input[0] = self.seed;
            input[1..].copy_from_slice(&self.ctr.to_le_bytes());
            chunk.copy_from_slice(&sha512(&input)[..chunk.len()]);
            self.ctr += 1;
        }
        true
    }

    pub fn bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut out = [0u8; N];
        self.fill(&mut out);
        out
    }
}
