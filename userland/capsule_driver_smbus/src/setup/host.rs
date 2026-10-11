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

//! Turning the host on, as Linux's i801_probe does: the SMBus host enabled
//! in HOSTC and its I2C mode off. I/O decode is firmware's to have left on;
//! the broker writes no Command bit for ports.

use nonos_libc::{mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND};

use super::claim::Claim;
use crate::hostc::hostc_wanted;
use crate::regs::HOSTC;

const CMD_IO_SPACE: i64 = 1;

pub fn enable(c: &Claim) -> Result<(), &'static str> {
    let cmd = mk_pci_config_read(c.device_id, c.epoch, MK_PCI_CFG_COMMAND, 2);
    if cmd < 0 {
        return Err("smbus: config read refused");
    }
    if cmd & CMD_IO_SPACE == 0 {
        return Err("smbus: firmware left the host's I/O decode off");
    }
    let hostc = mk_pci_config_read(c.device_id, c.epoch, HOSTC, 2);
    if hostc < 0 {
        return Err("smbus: config read refused");
    }
    let want = hostc_wanted(hostc as u16);
    if want != hostc as u16 && mk_pci_config_write(c.device_id, c.epoch, HOSTC, want) < 0 {
        return Err("smbus: HOSTC write refused");
    }
    Ok(())
}
