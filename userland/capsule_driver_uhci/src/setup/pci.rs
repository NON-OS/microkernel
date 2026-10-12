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

//! The function's PCI side: its I/O decode must already be on (the broker
//! flips no I/O Space bit), bus mastering on for the schedule, its INTx pin
//! silenced since the driver polls, and firmware's legacy keyboard and SMI
//! traps in LEGSUP off, their status cleared, before the controller resets.

use nonos_libc::{mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE};

use super::claim::Claim;
use crate::facts::pci::{LEGSUP, LEGSUP_CLEAR, LEGSUP_KEEP};

const CMD_IO_SPACE: i64 = 1;

pub fn enable(c: &Claim) -> Result<(), &'static str> {
    let cmd = mk_pci_config_read(c.device_id, c.epoch, MK_PCI_CFG_COMMAND, 2);
    if cmd < 0 {
        return Err("uhci: config read refused");
    }
    if cmd & CMD_IO_SPACE == 0 {
        return Err("uhci: firmware left the controller's I/O decode off");
    }
    let want = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
    if mk_pci_config_write(c.device_id, c.epoch, MK_PCI_CFG_COMMAND, want) < 0 {
        return Err("uhci: bus master refused");
    }
    let legsup = mk_pci_config_read(c.device_id, c.epoch, LEGSUP, 2);
    if legsup < 0 {
        return Err("uhci: config read refused");
    }
    let value = (legsup as u16 & LEGSUP_KEEP) | LEGSUP_CLEAR;
    if mk_pci_config_write(c.device_id, c.epoch, LEGSUP, value) < 0 {
        return Err("uhci: LEGSUP write refused");
    }
    Ok(())
}
