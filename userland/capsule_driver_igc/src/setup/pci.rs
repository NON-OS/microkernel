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

//! One write to the Command register under the claim (see `pci_command`).

use nonos_libc::{mk_pci_config_update, MK_PCI_CFG_COMMAND};

use super::pci_command::{command_value, COMMAND_MASK};

pub fn command(device_id: u64, epoch: u64, bus_master: bool) -> Result<(), &'static str> {
    let value = command_value(bus_master);
    if mk_pci_config_update(device_id, epoch, MK_PCI_CFG_COMMAND, COMMAND_MASK, value) < 0 {
        return Err(if bus_master {
            "PCI command write (bus master) refused"
        } else {
            "PCI command write (memory space) refused"
        });
    }
    Ok(())
}
