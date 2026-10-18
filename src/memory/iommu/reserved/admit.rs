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

//! Whether a device's reserved ranges may go into a domain (the C-34
//! ruling): each well formed, its physical span inside one of the tree's
//! reserved-memory regions, and its IOVA span clear of every other reserved
//! range and of every span the domain has already granted.

use super::range::ReservedRange;
use super::refusal::Refusal;

/// The 4 KiB granule every backend maps in.
const PAGE: u64 = 0x1000;

/// Admit `new` into a domain that already holds the IOVA spans `held`
/// (reserved and granted, `(start, end)`), with the reserved-memory regions
/// `reserved` (`(base, size)`). All or nothing.
pub fn admit(
    new: &[ReservedRange],
    held: &[(u64, u64)],
    reserved: &[(u64, u64)],
) -> Result<(), Refusal> {
    for (i, range) in new.iter().enumerate() {
        let (Some(iova_end), Some(phys_end)) = (range.iova_end(), range.phys_end()) else {
            return Err(Refusal::Malformed);
        };
        if range.len == 0 || (range.iova | range.phys | range.len) % PAGE != 0 {
            return Err(Refusal::Malformed);
        }
        let inside = |&(base, size): &(u64, u64)| {
            base.checked_add(size).is_some_and(|end| base <= range.phys && phys_end <= end)
        };
        if !reserved.iter().any(inside) {
            return Err(Refusal::OutsideReservedMemory);
        }
        // An empty span holds nothing, so it meets nothing.
        let meets =
            |&(start, end): &(u64, u64)| start < end && range.iova < end && start < iova_end;
        let others = new[..i].iter().filter_map(|r| Some((r.iova, r.iova_end()?)));
        if held.iter().any(meets) || others.into_iter().any(|span| meets(&span)) {
            return Err(Refusal::Overlap);
        }
    }
    Ok(())
}
