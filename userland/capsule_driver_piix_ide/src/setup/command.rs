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


//! The PCI Command bits the claim sets, in one masked write.

use nonos_libc::{
    mk_pci_config_update, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

/// Bus Master for the engines' reads and writes of memory; the broker
/// clears it on every release, so each attempt sets it. The PCI pin is
/// silenced as well: the driver polls, and a native-mode channel would
/// otherwise share INTx with whatever else is on the line. I/O decode is the
/// firmware's: with it off the status floats high and no channel is used.
pub const COMMAND_BITS: u16 = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
/// The bits the write sets: those, and Memory Space cleared. The function
/// decodes nothing in memory, the capsule holds no Mmio, and firmware often
/// leaves the bit on, which a write that only ORed the others in would keep.
pub const COMMAND_MASK: u16 = COMMAND_BITS | MK_PCI_CMD_MEMORY_SPACE;

/// Set them on the device the claim at `epoch` holds; negative when refused.
pub fn set(device_id: u64, epoch: u64) -> i64 {
    mk_pci_config_update(device_id, epoch, MK_PCI_CFG_COMMAND, COMMAND_MASK, COMMAND_BITS)
}
