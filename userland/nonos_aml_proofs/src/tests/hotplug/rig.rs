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

//! A root port with an ejectable slot holding two functions, a dock that
//! answers `_DCK`, and a USB controller that leaves with the dock (`_EJD`).
//! `\S0ST` and `\DKST` are what the slot's and the dock's `_STA` answer.

use crate::enc::*;

const PCI0: &str = "\\_SB.PCI0";

pub fn machine() -> Vec<u8> {
    let at = |p: &str| format!("{PCI0}.{p}");
    let adr = |path: &str, a: u64| device(path, &name_obj("_ADR", &int(a)));
    let sta = |dev: &str, src: &str| method(&format!("{dev}._STA"), 0, &ret(&name(src)));
    let slot = at("RP01.SLT0");
    let dock = at("DOCK");
    cat(&[
        &name_obj("\\S0ST", &int(0x0F)),
        &name_obj("\\DKST", &int(0)),
        &device(PCI0, &name_obj("_HID", &string("PNP0A08"))),
        &adr(&at("RP01"), 0x001C_0000),
        &adr(&slot, 0),
        &sta(&slot, "\\S0ST"),
        &method(&format!("{slot}._EJ0"), 1, &store(&int(0), &name("\\S0ST"))),
        &adr(&format!("{slot}.FN1"), 1),
        &device(&dock, &name_obj("_HID", &string("PNP0C15"))),
        &sta(&dock, "\\DKST"),
        &method(
            &format!("{dock}._DCK"),
            1,
            &cat(&[&store(&arg(0), &name("\\DKST")), &ret(&int(1))]),
        ),
        &adr(&at("USB0"), 0x0014_0000),
        &name_obj(&at("USB0._EJD"), &string("\\_SB.PCI0.DOCK")),
    ])
}
