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

//! Placement that steps over a domain's reserved ranges, so no grant is
//! ever given an IOVA a firmware-reserved range holds. Pure, so the proofs
//! run it.

use super::iova_space::place;

/// Place `length` bytes at or above `next` as `place` does, and past every
/// reserved span `(start, end)` it would meet. Each try that meets one
/// starts again past the furthest it met, and starts only rise, so a span
/// once passed is never met again: at most one try per span, then one clear.
pub(super) fn place_avoiding(
    next: u64,
    length: u64,
    reserved: &[(u64, u64)],
) -> Option<(u64, u64)> {
    let mut at = next;
    for _ in 0..=reserved.len() {
        let (start, end) = place(at, length)?;
        let met = reserved.iter().filter(|&&(s, e)| start < e && s < end).map(|&(_, e)| e).max();
        match met {
            None => return Some((start, end)),
            Some(past) => at = past,
        }
    }
    None
}
