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

//! A literal decoding table from weights. The last symbol's weight is what
//! brings the total to a power of two; codes are dealt in rising weight, then
//! rising symbol, so the table is indexed by the next `max` bits directly.

use alloc::vec::Vec;

use super::limits::HUFF_LOG;

pub struct Huff {
    pub max: u8,
    /// Per index: the symbol, and how many bits its code really takes.
    pub cells: Vec<(u8, u8)>,
}

pub fn table(weights: &[u8]) -> Option<Huff> {
    let mut sum: u32 = 0;
    for &w in weights {
        if w > HUFF_LOG {
            return None;
        }
        if w > 0 {
            sum += 1 << (w - 1);
        }
    }
    if sum == 0 {
        return None;
    }
    let max = 32 - sum.leading_zeros();
    if max > u32::from(HUFF_LOG) {
        return None;
    }
    let left = (1u32 << max) - sum;
    if !left.is_power_of_two() {
        return None;
    }
    let mut all = weights.to_vec();
    all.push((left.trailing_zeros() + 1) as u8);
    let mut cells = Vec::with_capacity(1 << max);
    for w in 1..=max {
        for (sym, _) in all.iter().enumerate().filter(|(_, &x)| u32::from(x) == w) {
            let entry = (sym as u8, (max + 1 - w) as u8);
            cells.extend(core::iter::repeat_n(entry, 1 << (w - 1)));
        }
    }
    (cells.len() == 1 << max).then_some(Huff { max: max as u8, cells })
}
