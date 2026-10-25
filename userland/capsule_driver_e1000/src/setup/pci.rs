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
//! BAR0 answers. Bus Master is off from the claim until the part is reset
//! and both rings name this driver's frames, and off again before the device
//! goes back: a part left mastering by firmware or by an earlier attempt
//! must not reach memory before it is stopped, nor after its frames are
//! freed. The broker clears it on every release, so each attempt sets it.
//!
//! Only allowlisted bits are written, and every other bit stays as it is.
//! They go in by the masked form of the write (`mk_pci_config_update`),
//! which names both bits so each takes the value given: "off" turns Bus
//! Master off. The plain form ORs its bits into the register and could only
//! turn them on (src/hardware/broker/pci/change.rs).

use nonos_libc::{
    mk_pci_config_update, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};

/// The Command bits this driver sets and clears.
const MASK: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;

pub fn command(device_id: u64, epoch: u64, bus_master: bool) -> Result<(), &'static str> {
    let master = if bus_master { MK_PCI_CMD_BUS_MASTER } else { 0 };
    let value = MK_PCI_CMD_MEMORY_SPACE | master;
    if mk_pci_config_update(device_id, epoch, MK_PCI_CFG_COMMAND, MASK, value) < 0 {
        return Err(if bus_master {
            "pci command write refused (bus master)"
        } else {
            "pci command write refused (memory space)"
        });
    }
    Ok(())
}
