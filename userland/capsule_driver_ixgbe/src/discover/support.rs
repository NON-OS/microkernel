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

use nonos_libc::{DeviceRecord, BUS_KIND_PCI};

use crate::facts::ids::{part_of, INTEL_VENDOR_ID};
use crate::facts::parts::Part;

const PCI_CLASS_NETWORK: u8 = 0x02;
const PCI_SUBCLASS_ETHERNET: u8 = 0x00;

/// The MAC of an Intel Ethernet function whose device id is in facts::ids.
/// The 82598 and the X550EM parts are not taken.
pub fn part(r: &DeviceRecord) -> Option<Part> {
    if r.vendor != INTEL_VENDOR_ID
        || r.bus_kind != BUS_KIND_PCI
        || r.pci_class != PCI_CLASS_NETWORK
        || r.pci_subclass != PCI_SUBCLASS_ETHERNET
    {
        return None;
    }
    part_of(r.device)
}
