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

//! The things the included BGA setup takes from `nonos_libc`: the device
//! list discovery reads, the claim, the PCI command register and the two
//! BAR mappings. A test says what each BAR maps to and how long the broker
//! says the mapping is, so a mapping shorter than the memory behind it shows
//! any write past its end.

mod claim;
mod mmio;
mod pci;
mod records;

pub use claim::{given_back, mk_device_claim, mk_device_release};
pub use mmio::{map_bar, mk_mmio_map, mk_mmio_unmap, MmioMapOut};
pub use pci::{
    command, mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_MEMORY_SPACE, PCI_CMD_IO_SPACE,
};
pub use records::{mk_device_list, offer, Bar, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};
