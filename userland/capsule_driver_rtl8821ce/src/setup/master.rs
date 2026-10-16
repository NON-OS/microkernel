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

//! The function's PCI command register: Memory Space from the claim, Bus
//! Master only once the power cycle showed the MAC off; and the release of a
//! chip that must not be served, Bus Master off before the claim goes.

use nonos_libc::{
    mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_MEMORY_SPACE,
};

use super::mapped::Mapped;

/// Write the command register: Memory Space always, Bus Master when `master`.
/// Whether the broker took the write.
pub fn command(device_id: u64, epoch: u64, master: bool) -> bool {
    let bus = if master { MK_PCI_CMD_BUS_MASTER } else { 0 };
    let value = MK_PCI_CMD_MEMORY_SPACE | bus;
    mk_pci_config_write(device_id, epoch, MK_PCI_CFG_COMMAND, value) >= 0
}

/// Give a chip back that is not to be served: Bus Master off, then the claim,
/// which takes the register window with it.
pub fn release(m: &Mapped) {
    let _ = command(m.device_id, m.claim_epoch, false);
    let _ = mk_device_release(m.device_id);
}
