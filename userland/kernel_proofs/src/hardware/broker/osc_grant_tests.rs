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

//! A root bridge's _OSC record: a grant can only be part of the offer and
//! the buses run forward; a function finds its bridge by segment and bus;
//! NONOS owns a control only when firmware offered it and NONOS took it.

use super::acpi::osc_grant::*;

fn g(segment: u16, first: u8, last: u8, offered: u32, granted: u32) -> OscGrant {
    OscGrant { segment, first, last, offered, granted }
}

#[test]
fn only_sound_records_are_kept() {
    assert!(checked(g(0, 0, 0x7F, ALL, AER | PME)).is_some());
    assert!(checked(g(0, 0, 0x7F, AER, 0)).is_some(), "offered, not taken");
    assert_eq!(checked(g(0, 0, 0x7F, AER, AER | DPC)), None, "a grant past the offer");
    assert_eq!(checked(g(0, 0x80, 0x7F, ALL, 0)), None, "buses backwards");
    assert!(checked(g(0, 5, 5, 0, 0)).is_some(), "one bus, nothing offered");
}

#[test]
fn a_function_finds_its_bridge() {
    let grants = [g(0, 0x00, 0x7F, ALL, AER), g(0, 0x80, 0xFF, ALL, 0), g(1, 0, 0xFF, PME, PME)];
    assert_eq!(grant_for(&grants, 0, 0x3A).map(|x| x.granted), Some(AER));
    assert_eq!(grant_for(&grants, 0, 0x80).map(|x| x.granted), Some(0));
    assert_eq!(grant_for(&grants, 1, 0x10).map(|x| x.offered), Some(PME));
    assert_eq!(grant_for(&grants, 2, 0x10), None, "no bridge: firmware keeps everything");
}

#[test]
fn owned_means_offered_and_taken() {
    let r = g(0, 0, 0xFF, AER | PME | DPC, AER | PME);
    assert!(r.os_owns(AER));
    assert!(r.os_owns(AER | PME));
    assert!(!r.os_owns(DPC), "offered, not taken");
    assert!(!r.os_owns(NATIVE_HOT_PLUG));
    assert!(!r.os_owns(AER | DPC));
    assert!(!r.os_owns(0));
    assert_eq!(ALL, 0b1011_1111, "bit 6 is not a control");
}
