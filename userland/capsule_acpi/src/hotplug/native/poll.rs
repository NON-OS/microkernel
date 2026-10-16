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

//! Every `POLL_MS`, each port's slot is read, the change bits it latched
//! cleared, and what its phase then asks for done. No hot-plug interrupt
//! is enabled: the status bits latch whether or not one is.

use super::act::run;
use super::find::Port;
use super::regs::*;
use super::slot::{Phase, Read};
use super::times::POLL_MS;
use super::walk::read as cfg;
use crate::boot::Acpi;
use crate::sys::acpi_pci::pci_write;
use crate::sys::clock::now_ms;

/// None when the port no longer answers.
pub fn read(p: &Port) -> Option<Read> {
    let status = cfg(p.bdf, p.cap + SLOT_STATUS, 2);
    if status >= 0xFFFF {
        return None;
    }
    let status = status as u16;
    if status & CHANGES != 0 {
        let _ = pci_write(p.bdf, p.cap + SLOT_STATUS, 2, u32::from(status & CHANGES));
    }
    let link = p.dll.then(|| cfg(p.bdf, p.cap + LINK_STATUS, 2) as u16 & DLL_ACTIVE != 0);
    let ctrl = cfg(p.bdf, p.cap + SLOT_CTRL, 2) as u16;
    let powered = !p.slot.power || ctrl & POWER_OFF == 0;
    Some(Read { status, link, powered })
}

pub fn tick(acpi: &mut Acpi) {
    let now = now_ms();
    let n = &mut acpi.hotplug.native;
    if n.ports.is_empty() || now < n.next_ms {
        return;
    }
    n.next_ms = now + POLL_MS;
    for i in 0..acpi.hotplug.native.ports.len() {
        let p = &mut acpi.hotplug.native.ports[i];
        let Some(r) = read(p) else { continue };
        let counting = matches!(p.slot.phase, Phase::Button { .. });
        let act = p.slot.step(r, now);
        run(acpi, i, act, counting);
    }
}
