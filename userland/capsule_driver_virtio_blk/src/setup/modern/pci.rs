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

//! PCI command for a modern function: memory decode and bus mastering on.
//!
//! QEMU's legacy register path turns Bus Master on by itself when a driver
//! writes ACKNOWLEDGE | DRIVER; its modern path does not, and without Bus
//! Master the disk can do no DMA and raise no MSI-X message. The broker
//! clears it on every release, so each attempt sets it again. The INTx pin
//! is left alone: this driver may be waiting on it.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

pub fn enable(device_id: u64, claim_epoch: u64) -> Result<(), &'static str> {
    let bits = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;
    if mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits) < 0 {
        return Err("virtio-blk: pci command setup failed");
    }
    Ok(())
}
