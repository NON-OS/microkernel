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

//! The partitions a sound table names, and their types.

use super::rng::Rng;

/// Up to `most` partitions in `first..=last`, each before the next.
pub fn spans(r: &mut Rng, first: u64, last: u64, most: u64) -> Vec<(u64, u64)> {
    let mut at: Vec<u64> = (0..2 * r.range(0, most)).map(|_| r.range(first, last)).collect();
    at.sort_unstable();
    at.dedup();
    at.chunks_exact(2).map(|p| (p[0], p[1])).collect()
}

/// A random partition type, a NONOS one one time in two.
pub fn kind(r: &mut Rng) -> [u8; 16] {
    use crate::kernel_gpt::places::{NONOS_DATA, NONOS_PLAN, NONOS_STORE};
    let mut t: [u8; 16] = r.bytes(16).try_into().unwrap();
    t[0] |= 1;
    [t, t, t, NONOS_STORE, NONOS_PLAN, NONOS_DATA][r.range(0, 5) as usize]
}
