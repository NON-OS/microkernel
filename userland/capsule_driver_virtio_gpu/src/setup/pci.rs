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

//! PCI command setup: bus mastering on, legacy interrupt pin off.
//!
//! The driver never waits on an interrupt: every control-queue command
//! polls the used ring. Left enabled, the device raises INTx on each
//! completion and nothing ever reads its interrupt status to lower it, so
//! it holds its level-triggered line up for good. On QEMU's q35 that line
//! is shared with the virtio disk, and every time the disk driver unmasked
//! it the interrupt fired at once, several spurious deliveries per disk
//! request. Interrupt Disable keeps the device off the line entirely.

use nonos_libc::{
    mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_INTX_DISABLE,
};

pub fn enable(device_id: u64, claim_epoch: u64) -> Result<(), &'static str> {
    let bits = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
    let rc = mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits);
    if rc >= 0 {
        return Ok(());
    }
    if mk_device_release(device_id) < 0 {
        return Err("virtio-gpu: release failed after pci setup failure");
    }
    Err("virtio-gpu: pci command setup failed")
}
