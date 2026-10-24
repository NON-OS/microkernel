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

//! The function's PCI side: memory decode and bus mastering on, its INTx
//! pin silenced since the driver polls. Firmware's hold on the controller
//! is let go in handoff.rs.

use nonos_libc::{mk_pci_config_write, MK_PCI_CFG_COMMAND};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE};

use super::claim::Claim;

pub fn enable(c: &Claim) -> Result<(), &'static str> {
    let want = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
    if mk_pci_config_write(c.device_id, c.epoch, MK_PCI_CFG_COMMAND, want) < 0 {
        return Err("ehci: command write refused");
    }
    Ok(())
}
