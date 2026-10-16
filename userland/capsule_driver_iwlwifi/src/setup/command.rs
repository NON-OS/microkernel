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

//! The function's PCI command register. The kernel clears Bus Master when a
//! driver claims a device, so the driver turns it on itself: Memory Space
//! alone at the claim, for the register window, and Bus Master too only right
//! before the firmware boots, once the device's own bus master read back
//! stopped and the DMA it may use is the driver's own grants. Every failed
//! attempt and every teardown clears it again before a grant goes.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

/// Write the command register: Memory Space always, Bus Master when `master`.
/// Whether the broker took the write.
pub fn command(device_id: u64, claim_epoch: u64, master: bool) -> bool {
    let bus = if master { MK_PCI_CMD_BUS_MASTER } else { 0 };
    let value = MK_PCI_CMD_MEMORY_SPACE | bus;
    mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, value) >= 0
}
