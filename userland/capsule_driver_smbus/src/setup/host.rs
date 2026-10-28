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

//! Turning the host on through HOSTC in config space: the SMBus host enabled
//! in HOSTC and its I2C mode off, kept only if the rest of the bring-up
//! succeeds. I/O decode is firmware's to have left on; the broker writes no
//! Command bit for ports.

use nonos_libc::{mk_pci_config_read, mk_pci_config_update, MK_PCI_CFG_COMMAND};

use super::claim::Claim;
use crate::hostc::{turned_on, HostcCfg, HOSTC_CHANGED};
use crate::regs::HOSTC;

const CMD_IO_SPACE: i64 = 1;

/// HOSTC is one byte: read as one, and written by the masked form, which
/// names the two bits it changes and leaves byte 0x41 as it reads.
struct Cfg<'a>(&'a Claim);

impl HostcCfg for Cfg<'_> {
    fn read(&mut self) -> Option<u8> {
        u8::try_from(mk_pci_config_read(self.0.device_id, self.0.epoch, HOSTC, 1)).ok()
    }
    fn write(&mut self, bits: u8) -> bool {
        let (id, epoch, mask) = (self.0.device_id, self.0.epoch, HOSTC_CHANGED as u16);
        mk_pci_config_update(id, epoch, HOSTC, mask, bits as u16) >= 0
    }
}

pub fn enable<T>(
    c: &Claim,
    rest: impl FnOnce() -> Result<T, &'static str>,
) -> Result<T, &'static str> {
    let cmd = mk_pci_config_read(c.device_id, c.epoch, MK_PCI_CFG_COMMAND, 2);
    if cmd < 0 {
        return Err("smbus: config read refused");
    }
    if cmd & CMD_IO_SPACE == 0 {
        return Err("smbus: firmware left the host's I/O decode off");
    }
    turned_on(&mut Cfg(c), rest)
}
