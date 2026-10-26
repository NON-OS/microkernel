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

//! The PCI Command word, through the broker. The part is reached through its
//! port BAR, and I/O decode is not a bit the broker lets a driver write: the
//! kernel's boot pass (src/bus/pci/assign/device.rs) turns it on for every
//! function, and the write here leaves it as it is. Bus Master is off from
//! the claim until the part is reset and the receive buffer and transmit
//! slots name this driver's frames, and off again before the device goes
//! back. A part left mastering by firmware or by an earlier attempt must not
//! reach memory before it is stopped, nor after its frames are freed. The
//! broker clears it on every release, so each attempt sets it.
//!
//! Memory Space is written off in every write. The same boot pass turns it
//! on, and the broker refuses any Command write that would leave it on for
//! a capsule holding no Mmio (src/hardware/broker/pci/command_rule.rs): a
//! write that named Bus Master alone was refused at every claim.
//!
//! The bits go in by the masked form of the write (`mk_pci_config_update`),
//! which names them so each takes the value given, and every other bit stays
//! as it is. The plain form ORs its bits into the register and could only
//! turn them on (src/hardware/broker/pci/change.rs).

use nonos_libc::{mk_pci_config_update, MK_PCI_CFG_COMMAND};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

/// The bits each write names: Bus Master as asked, Memory Space always off.
const COMMAND_MASK: u16 = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_MEMORY_SPACE;

pub fn command(device_id: u64, epoch: u64, bus_master: bool) -> Result<(), &'static str> {
    let value = if bus_master { MK_PCI_CMD_BUS_MASTER } else { 0 };
    if mk_pci_config_update(device_id, epoch, MK_PCI_CFG_COMMAND, COMMAND_MASK, value) < 0 {
        return Err(if bus_master {
            "pci command write refused (bus master)"
        } else {
            "pci command write refused (bus master off)"
        });
    }
    Ok(())
}
