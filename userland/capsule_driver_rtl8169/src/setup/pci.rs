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

use nonos_libc::{mk_pci_config_update, MK_PCI_CFG_COMMAND};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use super::command_word::command_word;

/// The Command bits the word sets and clears.
const MASK: u16 = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER;

/// Write the Command word under the claim (see `command_word`).
pub fn command(device_id: u64, epoch: u64, bus_master: bool) -> Result<(), &'static str> {
    let word = command_word(bus_master);
    if mk_pci_config_update(device_id, epoch, MK_PCI_CFG_COMMAND, MASK, word) < 0 {
        return Err(if bus_master {
            "pci command write refused (bus master)"
        } else {
            "pci command write refused (memory space)"
        });
    }
    Ok(())
}
