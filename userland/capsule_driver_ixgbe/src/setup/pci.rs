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

//! PCI phase, as rtl8169 setup/pci.rs: one write to the Command register
//! under the claim. A refusal releases the claim so the broker is clean.

use nonos_libc::{mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND};

use crate::discover::Found;

use super::pci_command::command_value;

pub fn enable(dev: Found, claim_epoch: u64) -> Result<(), &'static str> {
    let command = command_value(dev.command_bits);
    let r = mk_pci_config_write(dev.device_id, claim_epoch, MK_PCI_CFG_COMMAND, command);
    if r < 0 {
        let _ = mk_device_release(dev.device_id);
        return Err("PCI command write (bus master, memory space) refused");
    }
    Ok(())
}
