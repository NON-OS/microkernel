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

//! PCI command: memory decode, bus mastering, and the legacy interrupt pin
//! silenced. The broker leaves Bus Master to the driver and clears it on
//! every release, so each attempt sets it; without it the part reads its
//! rings as zeros and its writes go nowhere. The driver polls, so the pin
//! is quieted rather than left to hold a shared line up. The broker ORs
//! these bits into what the command register already holds. A refusal gives
//! the claim back.

use nonos_libc::{
    mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};

pub const COMMAND_BITS: u16 =
    MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;

pub fn enable(device_id: u64, claim_epoch: u64) -> Result<(), &'static str> {
    if mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, COMMAND_BITS) < 0 {
        let _ = mk_device_release(device_id);
        return Err("pci command setup failed");
    }
    Ok(())
}
