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

//! The ACS groups found at boot (L72), and which requesters share one.

extern crate alloc;

use alloc::vec::Vec;

use super::acs_span::{group_of, groups, Port};
use crate::memory::iommu::DeviceAddress;

/// Set once, before any claim; empty where no pass ran (aarch64 today).
static GROUPS: spin::Once<Vec<(u8, u8)>> = spin::Once::new();

/// The groups the switch ports read at boot make, kept for every claim
/// after; the first call's stand. What was kept, for the boot log.
pub fn set_acs_groups(ports: &[Port]) -> Vec<(u8, u8)> {
    GROUPS.call_once(|| groups(ports)).clone()
}

/// Whether two requesters sit in one group: below a switch that lets them
/// reach each other past the IOMMU. Segment 0 only, the one the pass reads.
pub(super) fn peers(a: DeviceAddress, b: DeviceAddress) -> bool {
    let bus = |d: DeviceAddress| match d {
        DeviceAddress::Pci { segment: 0, rid } => Some((rid >> 8) as u8),
        _ => None,
    };
    let groups = GROUPS.get().map(Vec::as_slice).unwrap_or(&[]);
    match (bus(a), bus(b)) {
        (Some(x), Some(y)) => group_of(groups, x).is_some_and(|g| group_of(groups, y) == Some(g)),
        _ => false,
    }
}
