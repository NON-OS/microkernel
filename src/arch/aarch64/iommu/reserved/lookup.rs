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

//! A device's reserved ranges out of the tree's entries. Pure, so the
//! proofs run it.

use crate::arch::fdt::find::reserved_dma::TreeRange;
use crate::memory::iommu::reserved::Reserved;
use crate::memory::iommu::DeviceAddress;

/// Every entry whose owner is `device`, in tree order. An entry that cannot
/// be a range makes the whole answer unresolved, so the claim is refused
/// rather than the device attached missing part of what it needs. A PCI
/// requester outside segment 0 has no entries: the tree reader keys none
/// there.
pub fn collect(entries: &[TreeRange], device: DeviceAddress) -> Reserved {
    let mut found = Reserved::NONE;
    for entry in entries {
        let named = match device {
            DeviceAddress::Pci { segment: 0, rid } => entry.owner.names_pci(rid),
            DeviceAddress::Pci { .. } => false,
            DeviceAddress::Stream(sid) => entry.owner.names_stream(sid),
        };
        if !named {
            continue;
        }
        match entry.problem {
            Some(why) => found.unresolved = Some(why),
            None => found.push(entry.range),
        }
    }
    found
}
