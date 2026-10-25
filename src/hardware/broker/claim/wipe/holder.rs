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

//! What a claim tells the wipe record it will leave (L78): the driver
//! service it serves, and the function's reset capabilities and address.
//! Gathered before any lock; none for a device that is not a PCI function.

use super::caps::ResetCaps;
use super::holder_rule::{name_of, Name};
use crate::drivers::pci::types::PciAddress;

#[derive(Clone, Copy, Debug)]
pub(in crate::hardware::broker) struct Fresh {
    pub(in crate::hardware::broker) by: Option<Name>,
    pub(in crate::hardware::broker) caps: ResetCaps,
    pub(in crate::hardware::broker) address: PciAddress,
}

pub(in crate::hardware::broker) fn fresh(pid: u32, device_id: u64) -> Option<Fresh> {
    let handle = crate::hardware::broker::pci_index::lookup(device_id)?;
    let served = crate::services::registry::endpoints_of(pid);
    let by = name_of(served.iter().map(|e| e.name.as_str()));
    Some(Fresh { by, caps: handle.reset, address: handle.address })
}
