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

//! Which controller a pad's record names, as the shipping discovery reads
//! the device list and setup pairs them, and as the shipping OP_ACPI_HID
//! handler then names the pads in reach: the one whose own record's
//! device_id it carries, on PCI and platform controllers alike and on a
//! board with several.

use nonos_i2cmodel::{Bus, Config, Designware};

use super::listed::{discovered, list, order, table};
use super::multi_sku::named;
use super::records::{hid, lpss, platform, slow};
use crate::bench::{settled_on, GEMINI_LAKE_HZ as HZ};
use crate::regs::{self, Attached};
use crate::setup::order::standard_mode;
use crate::transaction::Presence;

fn core() -> Attached {
    regs::attach(Designware::new(Config::LPSS, Bus::default()))
}

#[test]
fn pads_naming_a_platform_controller_by_its_id_are_named_there_in_turn() {
    let _core = core();
    let pads = [hid(1, 0, 0x2A), hid(2, 0, 0x10), hid(1, 0, 0x2C)];
    list(&table(&[lpss(0), platform(1), platform(2)], &pads), None);
    let (cands, pads) = discovered();
    assert_eq!((cands.len(), pads.len()), (3, 3), "the controllers and pads listed");
    let d = settled_on(&cands[1], HZ, &pads[0], Presence::Absent, &pads).expect("bind");
    assert_eq!(named(&d, 3), [0x2A, 0x2C, 0x2A], "its two pads in turn, not the other's");
}

#[test]
fn on_a_board_with_two_controllers_each_is_tried_first_for_and_reaches_its_own_pads() {
    let _core = core();
    let pads = [hid(0, 0, 0x2A), slow(hid(2, 0, 0x10)), hid(0, 0, 0x2C)];
    list(&table(&[lpss(0), platform(1), platform(2)], &pads), None);
    let (cands, pads) = discovered();
    assert_eq!(order(&cands, &pads), [0, 2, 1], "the two the pads name first");
    let speeds: Vec<_> = cands.iter().map(|c| standard_mode(c, &pads)).collect();
    assert_eq!(speeds, [false, false, true], "the slow pad's own controller alone");
    let pci = settled_on(&cands[0], HZ, &pads[0], Presence::Absent, &pads).expect("bind");
    assert_eq!(named(&pci, 3), [0x2A, 0x2C, 0x2A]);
    let acpi = settled_on(&cands[2], HZ, &pads[1], Presence::Absent, &pads).expect("bind");
    assert_eq!(named(&acpi, 2), [0x10, 0x10]);
}
