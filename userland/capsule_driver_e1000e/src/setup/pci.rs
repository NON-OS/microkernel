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

//! The PCI Command word, through the broker. Memory Space is always on, so
//! BAR0 answers. Bus Master is off from the claim until the part is stopped
//! and reset and both rings name this driver's frames, and off again before
//! the device goes back: a part left mastering by firmware or by an earlier
//! attempt must not reach memory before it is stopped, nor after its frames
//! are freed. The broker clears it on every release, so each attempt sets
//! it; a part whose BIOS did not boot from it has it clear, and then every
//! descriptor fetch and frame write is dropped at the root port.
//!
//! Only allowlisted bits go in: the broker takes a value carrying any other
//! bit (I/O Space, which the 82574's port BAR asks for) verbatim and refuses
//! it unless every such bit already matches. It ORs allowlisted bits into
//! the register today, so "off" takes effect once it stores them as written
//! (src/hardware/broker/pci/command.rs); until then it changes nothing.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

pub fn command(device_id: u64, epoch: u64, bus_master: bool) -> Result<(), &'static str> {
    let master = if bus_master { MK_PCI_CMD_BUS_MASTER } else { 0 };
    let value = MK_PCI_CMD_MEMORY_SPACE | master;
    if mk_pci_config_write(device_id, epoch, MK_PCI_CFG_COMMAND, value) < 0 {
        return Err(if bus_master {
            "pci command write refused (bus master)"
        } else {
            "pci command write refused (memory space)"
        });
    }
    Ok(())
}
