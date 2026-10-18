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

//! Generic Event Devices (ACPI 6.5, 5.6.9): how a hardware-reduced
//! platform, every Arm one among them, signals what a GPE would on a PC.
//! Each is an ACPI0013 device whose _CRS lists the interrupts it owns.

use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::object::Object;
use crate::platform::{child, hardware_id, interrupt_entries, scan, sta, Eval, Interrupt, PRESENT};
use alloc::vec::Vec;

pub const GED_HID: &[u8] = b"ACPI0013";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ged {
    pub id: Id,
    pub interrupts: Vec<Interrupt>,
}

fn is_ged(aml: &mut Aml, eval: &mut Eval<'_>, id: Id) -> bool {
    let hid = child(aml, eval, id, *b"_HID", Vec::new()).and_then(|r| r.ok());
    let named = hid.as_ref().and_then(hardware_id).is_some_and(|h| h.bytes() == GED_HID);
    named && sta(aml, eval, id).is_ok_and(|s| s & PRESENT != 0)
}

/// Present GEDs in definition order, each with the interrupts its _CRS
/// names; one whose _CRS fails or is no buffer owns none.
pub fn geds(aml: &mut Aml, eval: &mut Eval<'_>) -> Vec<Ged> {
    let devices = scan(aml, &|aml, id| aml.ns.get(id).is_some_and(|e| e.kind == Kind::Device));
    let mut out = Vec::new();
    for id in devices {
        if !is_ged(aml, eval, id) {
            continue;
        }
        let interrupts = match child(aml, eval, id, *b"_CRS", Vec::new()) {
            Some(Ok(Object::Buffer(b))) => interrupt_entries(&b.borrow()),
            _ => Vec::new(),
        };
        out.push(Ged { id, interrupts });
    }
    out
}
