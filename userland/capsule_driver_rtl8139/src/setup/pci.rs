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
//! function, and the write here leaves it as it is. So only Bus Master is
//! written: off from the claim until the part is reset and the receive
//! buffer and transmit slots name this driver's frames, and off again
//! before the device goes back. A part left mastering by firmware or
//! by an earlier attempt must not reach memory before it is stopped, nor
//! after its frames are freed. The broker clears it on every release, so
//! each attempt sets it.
//!
//! The bit goes in by the masked form of the write (`mk_pci_config_update`),
//! which names it so it takes the value given: "off" turns Bus Master off,
//! and every other bit stays as it is. The plain form ORs its bits into the
//! register and could only turn them on (src/hardware/broker/pci/change.rs).

use nonos_libc::{mk_pci_config_update, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER};

pub fn command(device_id: u64, epoch: u64, bus_master: bool) -> Result<(), &'static str> {
    let value = if bus_master { MK_PCI_CMD_BUS_MASTER } else { 0 };
    let mask = MK_PCI_CMD_BUS_MASTER;
    if mk_pci_config_update(device_id, epoch, MK_PCI_CFG_COMMAND, mask, value) < 0 {
        return Err(if bus_master {
            "pci command write refused (bus master)"
        } else {
            "pci command write refused (bus master off)"
        });
    }
    Ok(())
}
