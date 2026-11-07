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

//! Randomness for the runs: a stream from SHA-512 of a seed and a counter,
//! so every run is the same, and a source that has run dry.

use nonos_hash::Sha512;

pub struct Det {
    seed: u8,
    n: u64,
}

impl Det {
    pub fn new(seed: u8) -> Self {
        Det { seed, n: 0 }
    }

    pub fn fill(&mut self, out: &mut [u8]) -> bool {
        for chunk in out.chunks_mut(64) {
            let mut h = Sha512::new();
            h.update(&[self.seed]);
            h.update(&self.n.to_le_bytes());
            self.n += 1;
            chunk.copy_from_slice(&h.finalize()[..chunk.len()]);
        }
        true
    }

    pub fn bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut out = [0u8; N];
        self.fill(&mut out);
        out
    }
}

/// A source that gives nothing.
pub fn dry(_: &mut [u8]) -> bool {
    false
}
