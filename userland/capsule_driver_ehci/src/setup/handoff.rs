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

//! Taking the controller from firmware through its legacy support
//! capability in config space, at the offset HCCPARAMS names: the OS-owned
//! semaphore set and the BIOS-owned one waited for, then every SMI turned
//! off and its status cleared. A controller with no such capability (a
//! VM's, often) has nothing to let go.

use nonos_libc::{mk_idle_ms, mk_pci_config_read, mk_pci_config_write, Deadline};

use super::claim::Claim;
use crate::facts::bounds::HANDOFF_MS;
use crate::facts::legsup::*;

pub const BIOS_KEPT: &str = "ehci: firmware kept the controller";

fn read(c: &Claim, off: u32) -> Result<u32, &'static str> {
    match mk_pci_config_read(c.device_id, c.epoch, off, 4) {
        v if v < 0 => Err("ehci: config read refused"),
        v => Ok(v as u32),
    }
}

fn write(c: &Claim, off: u32, v: u16) -> Result<(), &'static str> {
    match mk_pci_config_write(c.device_id, c.epoch, off, v) {
        r if r < 0 => Err("ehci: legacy support write refused"),
        _ => Ok(()),
    }
}

pub fn handoff(c: &Claim, eecp: u32) -> Result<(), &'static str> {
    if eecp < 0x40 {
        return Ok(());
    }
    let legsup = read(c, eecp)?;
    if legsup & 0xFF != LEGSUP_ID {
        return Ok(());
    }
    if legsup & BIOS_OWNED != 0 {
        write(c, eecp + 2, (legsup >> 16) as u16 | (OS_OWNED >> 16) as u16)?;
        let d = Deadline::after_ms(HANDOFF_MS);
        while read(c, eecp)? & BIOS_OWNED != 0 {
            if d.expired() {
                return Err(BIOS_KEPT);
            }
            let _ = mk_idle_ms(1);
        }
    }
    // The enables first: the status word is then the capability's alone.
    let ctl = read(c, eecp + 4)?;
    write(c, eecp + 4, ctl as u16 & !CTLSTS_ENABLES)?;
    write(c, eecp + 6, (ctl >> 16) as u16 | CTLSTS_STATUS_HI)
}
