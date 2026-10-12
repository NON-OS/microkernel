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

//! The PCI command a bring-up asks for: memory decode, with the legacy
//! interrupt pin off, first; bus mastering only once the device has been
//! reset. A device left live by a previous instance still holds its ring
//! addresses, in frames since freed, until it is reset, so it may not reach
//! memory before that.
//!
//! The driver never waits on an interrupt: every control-queue command
//! polls the used ring. Left enabled, the device raises INTx on each
//! completion and nothing ever reads its interrupt status to lower it, so
//! it holds its level-triggered line up for good. On QEMU's q35 that line
//! is shared with the virtio disk, and every time the disk driver unmasked
//! it the interrupt fired at once, several spurious deliveries per disk
//! request. Interrupt Disable keeps the device off the line entirely.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

/// Memory decode, and the legacy pin off. A modern function is reached only
/// through its memory BAR, and the broker does not assume firmware left
/// decoding on.
pub const DECODE: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_INTX_DISABLE;
/// What a device that has been reset may also do.
pub const MASTER: u16 = DECODE | MK_PCI_CMD_BUS_MASTER;

pub fn command(device_id: u64, claim_epoch: u64, bits: u16) -> Result<(), &'static str> {
    if mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits) < 0 {
        return Err("virtio-gpu: pci command setup failed");
    }
    Ok(())
}
