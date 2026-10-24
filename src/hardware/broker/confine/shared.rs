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

//! Requesters a second capsule cannot take while one already holds them,
//! and the RMRR regions it may not share.

extern crate alloc;

use alloc::vec::Vec;

use super::acs_groups::peers;
use super::table::Capsule;
use crate::memory::iommu::DeviceAddress;

/// Why `address` is refused to the capsule at `pos`, if it is: another
/// capsule holds the same requester id, or a peer below one switch that
/// reaches it past the IOMMU (L72).
pub(super) fn held_elsewhere(
    all: &[Capsule],
    pos: usize,
    address: DeviceAddress,
) -> Option<&'static [u8]> {
    let others = || all.iter().enumerate().filter(|&(i, _)| i != pos).map(|(_, c)| c);
    // Drives behind one VMD share its requester id. One capsule may hold
    // several of them in its domain; a second capsule cannot take the id
    // without pulling it out from under the first.
    if others().any(|c| c.devices.iter().any(|(_, a)| *a == address)) {
        return Some(b"refused: requester id held by another capsule");
    }
    if others().any(|c| c.devices.iter().any(|(_, a)| peers(*a, address))) {
        return Some(b"refused: a peer below one switch reaches it past the IOMMU, held elsewhere");
    }
    None
}

/// Every RMRR region a capsule other than the one at `pos` holds: a region is
/// never mapped into two capsules' domains (L63).
pub(super) fn regions_elsewhere(all: &[Capsule], pos: usize) -> Vec<(u64, u64)> {
    (all.iter().enumerate().filter(|&(i, _)| i != pos))
        .flat_map(|(_, c)| c.rmrr.iter().map(|h| (h.base, h.end)))
        .collect()
}
