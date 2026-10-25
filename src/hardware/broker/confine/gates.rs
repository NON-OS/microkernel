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

//! What a translated device must pass before it joins a domain.

extern crate alloc;

use alloc::vec::Vec;

use super::member::Member;
use super::refuse::Refused;
use super::say::say;
use crate::memory::iommu::DeviceAddress;

/// The device's isolation group, which join.rs holds to one capsule, and
/// the aliases its DMA carries, which join.rs attaches with it.
pub(super) fn admit(device_id: u64) -> Result<(Vec<Member>, Vec<DeviceAddress>), Refused> {
    super::interrupts::admit()?;
    // A virtio device without ACCESS_PLATFORM ignores the domain.
    if !super::virtio::may_confine(device_id) {
        say(b"refused: virtio without ACCESS_PLATFORM under an IOMMU");
        return Err(Refused);
    }
    // What it reaches without the IOMMU is its group.
    reach(device_id).ok_or_else(|| {
        say(b"refused: no isolation group, behind a VMD or a looped bus");
        Refused
    })
}

/// A device another capsule holds, as a claim beside it sees it: its
/// place, its group (None when unreadable: refuses), its DMA's requester ids.
pub(super) struct Peer {
    pub at: (u8, u8, u8),
    pub group: Option<Vec<Member>>,
    pub aliases: Vec<DeviceAddress>,
}

/// None for a device off the PCI tree, which has no group and blocks none.
pub(super) fn peer(device_id: u64) -> Option<Peer> {
    let a = crate::hardware::broker::pci_index::lookup(device_id)?.address;
    let at = (a.bus, a.device, a.function);
    Some(match reach(device_id) {
        Some((group, aliases)) => Peer { at, group: Some(group), aliases },
        None => Peer { at, group: None, aliases: Vec::new() },
    })
}

/// The device's group and its DMA's requester ids; None where the walk cannot
/// place it. The walk is x86_64's (acs_walk.rs): elsewhere nothing is reached.
fn reach(device_id: u64) -> Option<(Vec<Member>, Vec<DeviceAddress>)> {
    #[cfg(target_arch = "x86_64")]
    return super::acs_walk::acs::reach(device_id).map(|r| {
        let aliases = r.aliases.iter().map(|&(b, d, f)| DeviceAddress::pci(b, d, f)).collect();
        (r.group, aliases)
    });
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = device_id;
        Some((Vec::new(), Vec::new()))
    }
}
