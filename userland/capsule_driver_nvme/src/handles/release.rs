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

//! Letting a claimed device go. The release is the guard (C2): the kernel
//! clears Bus Master, reads it back, and only then drains every grant of
//! the device, its DMA frames among them (sys_device_release in
//! src/syscall/microkernel/device.rs, broker::claim::quiesce). The driver
//! never unmaps the queues' memory itself before that.

use nonos_libc::{
    mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

/// The PCI Command bits a device keeps while it is let go: its registers
/// still decode and its legacy pin stays quiet. Bus Master is not among them.
const LET_GO: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_INTX_DISABLE;

/// Write the Command register with Bus Master clear, ahead of the release.
/// Defence in depth: the broker ORs the bits it is given into what is there
/// today (src/hardware/broker/pci/command.rs), so this clears nothing until
/// the kernel's K6 (plan/54) makes MkPciConfigWrite store exactly the
/// writable bits named. The release's own clear, read back, is what stops
/// the device either way.
pub fn stop_bus_master(device_id: u64, claim_epoch: u64) {
    let _ = mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, LET_GO);
}

/// Let go of a device whose MMIO and interrupt grants were never taken.
pub fn release(device_id: u64, claim_epoch: u64) {
    stop_bus_master(device_id, claim_epoch);
    let _ = mk_device_release(device_id);
}
