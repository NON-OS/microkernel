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

//! Drawing one relay from a weighted list.

/// Walks the cumulative weight, so position in the list carries no advantage.
/// A consensus arrives in a stable order, so a drawer that leaned on the front
pub fn pick(weights: &[u64], roll: u64) -> Option<usize> {
    let total: u64 = weights.iter().copied().fold(0u64, |a, b| a.saturating_add(b));
    if total == 0 {
        return None;
    }
    let mut target = roll % total;
    for (index, weight) in weights.iter().enumerate() {
        if target < *weight {
            return Some(index);
        }
        target -= *weight;
    }
    /*
     * Unreachable while the sum above is the sum walked here. Last candidate
     * rather than a panic: this runs per circuit and an abort takes the
     * transport down.
     */
    Some(weights.len().saturating_sub(1))
}

/// Consensus bandwidth scaled by a position weight, both in ten thousandths.
pub fn scale(bandwidth: u32, position_weight: u32) -> u64 {
    (bandwidth as u64).saturating_mul(position_weight as u64) / 10_000
}
