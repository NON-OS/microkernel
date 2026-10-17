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

use super::disk::Disk;
use super::rng::Rng;

/// The primary's entries: up to 8 partitions in random slots, and every
/// other slot unused (type zero) though it may still name usable sectors.
pub fn place(r: &mut Rng, d: &mut Disk, first: u64, last: u64, n: u64) {
    let parts = spans(r, first, last, n.min(8));
    let mut slots: Vec<usize> = (0..n as usize).collect();
    for i in 0..n as usize {
        // An unused entry may still name sectors inside the usable range.
        let (a, b) = (r.range(first, last), r.range(first, last));
        let e = d.entry(0, i);
        e[..16].fill(0);
        e[32..48].copy_from_slice(&[a.min(b).to_le_bytes(), a.max(b).to_le_bytes()].concat());
    }
    for (a, b) in parts {
        let slot = slots.swap_remove(r.range(0, slots.len() as u64 - 1) as usize);
        let e = d.entry(0, slot);
        e[..16].copy_from_slice(&kind(r));
        e[32..40].copy_from_slice(&a.to_le_bytes());
        e[40..48].copy_from_slice(&b.to_le_bytes());
    }
}

/// Up to `most` partitions in `first..=last`, each before the next.
fn spans(r: &mut Rng, first: u64, last: u64, most: u64) -> Vec<(u64, u64)> {
    let mut at: Vec<u64> = (0..2 * r.range(0, most)).map(|_| r.range(first, last)).collect();
    at.sort_unstable();
    at.dedup();
    at.chunks_exact(2).map(|p| (p[0], p[1])).collect()
}

/// A random partition type, a NONOS one one time in two.
fn kind(r: &mut Rng) -> [u8; 16] {
    use crate::kernel_gpt::places::{NONOS_DATA, NONOS_PLAN, NONOS_STORE};
    let mut t: [u8; 16] = r.bytes(16).try_into().unwrap();
    t[0] |= 1;
    [t, t, t, NONOS_STORE, NONOS_PLAN, NONOS_DATA][r.range(0, 5) as usize]
}
