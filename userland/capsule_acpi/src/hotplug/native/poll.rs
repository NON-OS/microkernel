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
use super::slot_wire::{LINK_STATUS, SLOT_CONTROL, SLOT_STATUS};
use super::times::POLL_MS;
use crate::boot::Acpi;
use crate::sys::acpi_slot::{slot_read, slot_write};
use crate::sys::clock::now_ms;

/// None when the port no longer answers.
pub fn read(p: &Port) -> Option<Read> {
    let status = slot_read(p.bdf, SLOT_STATUS).ok().filter(|s| *s < 0xFFFF)? as u16;
    if status & CHANGES != 0 {
        let _ = slot_write(p.bdf, SLOT_STATUS, status & CHANGES);
    }
    // A failed read skips this poll rather than look like a link gone down.
    let link = match p.dll {
        true => Some(slot_read(p.bdf, LINK_STATUS).ok()? as u16),
        false => None,
    };
    let ctrl = slot_read(p.bdf, SLOT_CONTROL).ok()? as u16;
    Some(Read::from_regs(status, p.slot_cap, link, ctrl))
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
