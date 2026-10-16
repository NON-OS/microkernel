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

//! The ports this capsule drives as native hot-plug slots: exactly the
//! ones the kernel answers to MkAcpiSlot's PORT_AT, which already checked
//! the port type, Slot Implemented, Hot-Plug Capable and `_OSC`'s grant.

use super::regs::*;
use super::slot::Slot;
use super::slot_wire::SLOT_CAPS;
use crate::boot::Acpi;
use crate::sys::acpi_slot::{port_at, slot_read};

pub use super::slot_wire::Bdf;

/// The kernel keeps no more than this many ports.
const MOST_PORTS: u64 = 64;

pub struct Port {
    pub bdf: Bdf,
    pub slot: Slot,
    pub dll: bool,
    pub no_cc: bool,
    pub slot_cap: u32,
}

pub fn find(acpi: &mut Acpi) {
    if !crate::boot::NATIVE_HOT_PLUG_ON {
        return;
    }
    for (f, dll) in (0..MOST_PORTS).map_while(port_at) {
        let Ok(slot_cap) = slot_read(f, SLOT_CAPS) else { continue };
        let (button, power) = (slot_cap & ATTENTION_BUTTON != 0, slot_cap & POWER_CONTROLLER != 0);
        let no_cc = slot_cap & NO_COMMAND_COMPLETED != 0;
        let slot = Slot::new(button, power, Default::default());
        let mut p = Port { bdf: f, slot, dll, no_cc, slot_cap };
        p.slot = Slot::new(button, power, super::poll::read(&p).unwrap_or_default());
        super::act::line(f).str(b": slot ").dec(u64::from(slot_cap >> 19)).str(b", taken").end();
        acpi.hotplug.native.ports.push(p);
    }
}
