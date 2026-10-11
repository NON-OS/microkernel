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

//! Which device records are this driver's.

use nonos_libc::{DeviceRecord, BUS_KIND_PCI};

use crate::constants::pci::{part_of, BROADCOM_VENDOR_ID};

const PCI_CLASS_NETWORK: u8 = 0x02;
const PCI_SUBCLASS_ETHERNET: u8 = 0x00;

/// A Broadcom Ethernet function whose ID is one of the 57765 class, and
/// whether it is a 10/100 part. The revision is checked again at bring-up,
/// since the ID alone does not prove the chip under it.
pub fn part(r: &DeviceRecord) -> Option<bool> {
    if r.vendor != BROADCOM_VENDOR_ID
        || r.bus_kind != BUS_KIND_PCI
        || r.pci_class != PCI_CLASS_NETWORK
        || r.pci_subclass != PCI_SUBCLASS_ETHERNET
    {
        return None;
    }
    part_of(r.device)
}
