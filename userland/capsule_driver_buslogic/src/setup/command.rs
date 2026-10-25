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


//! The PCI Command bits the bring-up sets, in one masked write.

use nonos_libc::{
    mk_pci_config_update, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

/// Bus Master for the mailboxes and the data; the pin silenced, as the
/// driver polls. I/O decode is the firmware's.
pub const COMMAND_BITS: u16 = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
/// The bits the write sets: those, and Memory Space cleared. The capsule
/// holds no Mmio, and firmware often leaves the bit on, which a write that
/// only ORed the others in would keep.
pub const COMMAND_MASK: u16 = COMMAND_BITS | MK_PCI_CMD_MEMORY_SPACE;

/// Set them on the device the claim at `epoch` holds; negative when refused.
pub fn set(device_id: u64, epoch: u64) -> i64 {
    mk_pci_config_update(device_id, epoch, MK_PCI_CFG_COMMAND, COMMAND_MASK, COMMAND_BITS)
}
