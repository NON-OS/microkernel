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

//! Turn on Memory Space and Bus Master Enable once the device is claimed.
//! The e1000 capsule never did, and relied on firmware having left them on;
//! a part whose BIOS did not boot from it has Bus Master clear, and then
//! every descriptor fetch and frame write is dropped at the root port.

use nonos_libc::{
    mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_MEMORY_SPACE,
};

use crate::discover::Found;

/*
 * Only bits the broker lets a driver flip go in. It ORs a value made of
 * writable bits into the current register, but takes a value carrying any
 * other bit (I/O Space, which the 82574's port BAR asks for) verbatim, and
 * refuses it unless every non-writable bit already matches: a firmware that
 * left I/O decode off, or set SERR or parity, would fail the claim
 * (src/hardware/broker/pci/command.rs). The driver uses BAR0 only.
 */
const WRITABLE: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;

pub fn command_value(record_bits: u16) -> u16 {
    (record_bits & WRITABLE) | MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER
}

pub fn enable(dev: Found, claim_epoch: u64) -> Result<(), &'static str> {
    let value = command_value(dev.command_bits);
    let r = mk_pci_config_write(dev.device_id, claim_epoch, MK_PCI_CFG_COMMAND, value);
    if r < 0 {
        let _ = mk_device_release(dev.device_id);
        return Err("pci command write refused (bus master)");
    }
    Ok(())
}
