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

//! At start, with the switch on: every bridge firmware describes, and the
//! bridges found on the buses below them (switch ports), is looked at;
//! a root or downstream port with a hot-plug slot, below a root bridge
//! whose _OSC gave NONOS native hot plug, is kept and its slot read once.

use super::osc_grant::{grant_for, OscGrant, NATIVE_HOT_PLUG};
use super::regs::*;
use super::slot::Slot;
use super::walk::{bridges_below, pcie_cap, read};
use crate::boot::Acpi;
use alloc::vec::Vec;

pub type Bdf = (u16, u8, u8, u8);

pub struct Port {
    pub bdf: Bdf,
    pub cap: u16,
    pub slot: Slot,
    pub dll: bool,
    pub no_cc: bool,
    pub slot_cap: u32,
}

pub fn find(acpi: &mut Acpi, granted: &[OscGrant]) {
    if !crate::boot::NATIVE_HOT_PLUG_ON {
        return;
    }
    let mut seen: Vec<Bdf> = acpi.hotplug.bridges.iter().map(|b| b.bdf).collect();
    bridges_below(&mut seen);
    let owned = |f: Bdf| grant_for(granted, f.0, f.1).is_some_and(|g| g.os_owns(NATIVE_HOT_PLUG));
    for f in seen.into_iter().filter(|f| f.0 == 0 && owned(*f)) {
        let Some(cap) = pcie_cap(f) else { continue };
        let slot_cap = read(f, cap + SLOT_CAP, 4);
        if !hot_plug_port(read(f, cap + FLAGS, 2) as u16, slot_cap) {
            continue;
        }
        let dll = read(f, cap + LINK_CAP, 4) & DLL_REPORTING != 0;
        let (button, power) = (slot_cap & ATTENTION_BUTTON != 0, slot_cap & POWER_CONTROLLER != 0);
        let no_cc = slot_cap & NO_COMMAND_COMPLETED != 0;
        let slot = Slot::new(button, power, Default::default());
        let mut p = Port { bdf: f, cap, slot, dll, no_cc, slot_cap };
        p.slot = Slot::new(button, power, super::poll::read(&p).unwrap_or_default());
        super::act::line(f).str(b": slot ").dec(u64::from(slot_cap >> 19)).str(b", taken").end();
        acpi.hotplug.native.ports.push(p);
    }
}
