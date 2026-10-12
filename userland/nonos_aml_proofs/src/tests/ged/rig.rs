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

//! The GEDs the proofs read: one with two edge interrupts and one level,
//! active-low, shared wake interrupt, whose _EVT records the number; one
//! absent; one with no _EVT; and a device that is no GED.

use crate::enc::*;

/// Interrupt(ResourceConsumer, ...) { numbers }, as 6.4.3.6 lays it out.
pub fn extended(flags: u8, numbers: &[u32]) -> Vec<u8> {
    let len = 2 + 4 * numbers.len() as u16;
    let mut t = vec![0x89, len as u8, (len >> 8) as u8, flags | 1, numbers.len() as u8];
    numbers.iter().for_each(|n| t.extend_from_slice(&n.to_le_bytes()));
    t
}

pub fn template(parts: &[Vec<u8>]) -> Vec<u8> {
    let mut t = parts.concat();
    t.extend_from_slice(&[0x79, 0x00]);
    buffer(t.len() as u64, &t)
}

pub fn machine() -> Vec<u8> {
    let crs = template(&[extended(0x02, &[0x29, 0x2A]), extended(0x1C, &[0x100])]);
    let hid = |s: &str| name_obj("_HID", &string(s));
    let evt = method("_EVT", 1, &store(&arg(0), &name("\\EVTN")));
    let crs_one = template(&[extended(0x02, &[0x30])]);
    cat(&[
        &name_obj("\\EVTN", &int(0)),
        &device("\\_SB.GED0", &cat(&[&hid("ACPI0013"), &name_obj("_CRS", &crs), &evt])),
        &device("\\_SB.GED1", &cat(&[&hid("ACPI0013"), &name_obj("_STA", &int(0))])),
        &device("\\_SB.GED2", &cat(&[&hid("ACPI0013"), &name_obj("_CRS", &crs_one)])),
        &device("\\_SB.PWRB", &hid("PNP0C0C")),
    ])
}
