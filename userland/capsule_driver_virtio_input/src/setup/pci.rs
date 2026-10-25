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

//! PCI command: memory decode and bus mastering on. QEMU's modern virtio
//! path does not set Bus Master by itself, and without it the device can do
//! no DMA; the broker clears it on every release, so each attempt sets it.
//! The driver polls and never reads the ISR, so the legacy pin is silenced
//! rather than left asserted on a shared line.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

pub fn enable(device_id: u64, claim_epoch: u64) -> Result<(), &'static str> {
    let bits = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
    if mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits) < 0 {
        return Err("pci command setup failed");
    }
    Ok(())
}
