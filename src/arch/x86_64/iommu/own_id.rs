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

//! Whether a device's DMA reaches the IOMMU under its own requester id
//! (L51). A conventional PCI device behind a PCIe-to-PCI bridge is seen
//! under an id the bridge supplies, so its context entry is never the one
//! its requests walk: confined by that entry, it was counted confined while
//! its DMA went through whatever the bridge's id was given. Pure, so the
//! walk and the rule run on the host too.

use alloc::vec::Vec;

/// The bus the host bridge itself hosts. A device there is its own requester.
pub(super) const ROOT_BUS: u8 = 0;

/// One bridge on the way up from a device's bus to the root bus.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Hop {
    /// A PCIe root port, or a switch's upstream or downstream port: each
    /// forwards the requester id unchanged, so the entry a device's id names
    /// is the one its DMA walks. Whether a peer below one switch is reached
    /// without the IOMMU is ACS, a fact of its own and not this one (L57).
    PciePort,
    /// Anything else: a PCIe-to-PCI bridge, a reverse bridge, a bridge with no
    /// PCIe capability, which may substitute an id of its own.
    Other,
}

/// The bridges from `bus` up to a root bus, nearest first. `is_root(b)`: bus
/// 0, or one no bridge's range holds, as a second host bridge's is (L61).
/// `parent_of(b)` names the bridge whose secondary bus is `b`, as (its own
/// bus, its kind). None when a bus has no bridge above it or a parent's bus
/// is not below its child's, which a sane numbering never gives.
pub(super) fn path_to_root(
    mut bus: u8,
    is_root: impl Fn(u8) -> bool,
    parent_of: impl Fn(u8) -> Option<(u8, Hop)>,
) -> Option<Vec<Hop>> {
    let mut path = Vec::new();
    while !is_root(bus) {
        let (up, hop) = parent_of(bus)?;
        if up >= bus {
            return None;
        }
        path.push(hop);
        bus = up;
    }
    Some(path)
}

/// Whether a device sends under its own id: on a root bus (an empty path), or
/// a PCIe function every one of whose bridges is a PCIe port. Anything else,
/// an unknown path included, is not counted as its own. Nor is a bridge that
/// may stand in for its subordinates (`stands_in`): its id is theirs too, so
/// confining it would move their DMA into its holder's domain (L54).
pub(super) fn own_requester_id(is_pcie: bool, stands_in: bool, path: Option<&[Hop]>) -> bool {
    let up = |p: &[Hop]| p.is_empty() || (is_pcie && p.iter().all(|h| *h == Hop::PciePort));
    !stands_in && path.is_some_and(up)
}
