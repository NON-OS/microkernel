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

//! PCI command: memory decode first, bus mastering only once the device has
//! been reset and holds this driver's ring. A device left live by a previous
//! instance still holds its old ring addresses, in pages since freed, until
//! then; virtio_rng and virtio_gpu keep the same order. QEMU's modern virtio
//! path does not set Bus Master by itself, and the broker clears it on every
//! release, so each attempt sets it. The driver polls and never reads the
//! ISR, so the legacy pin is silenced rather than left asserted on a shared
//! line.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

/// Memory decode, and the legacy pin off.
pub const DECODE: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_INTX_DISABLE;
/// What a device reset and on this driver's ring may also do.
pub const MASTER: u16 = DECODE | MK_PCI_CMD_BUS_MASTER;

pub fn command(device_id: u64, claim_epoch: u64, bits: u16) -> Result<(), &'static str> {
    if mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, bits) < 0 {
        return Err("pci command setup failed");
    }
    Ok(())
}
