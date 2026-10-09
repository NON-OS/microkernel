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

//! What is not taken, and each failing step under its own name.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Bind;

use crate::chip::{found, PLA, RTL8153_IDS};
use crate::r8153::bind;

fn failure(version: u16, ids: (u16, u16)) -> Option<(&'static str, i32)> {
    let (bus, _, found) = found(version, ids);
    match bind(bus, &found) {
        Bind::Failed(what, e) => Some((what, e)),
        _ => None,
    }
}

#[test]
fn adapters_outside_the_table_are_not_taken() {
    for ids in [(0x0bda, 0x8152), (0x0bda, 0x8156), (0x0bda, 0x8155), (0x2001, 0xb301)] {
        let (bus, _, found) = found(0x5c10, ids);
        assert!(matches!(bind(bus, &found), Bind::NotOurs));
    }
}

#[test]
fn another_chip_behind_a_listed_id_is_refused_before_it_is_configured() {
    let travel_hub = (0x17ef, 0x721e);
    assert_eq!(failure(0x4c10, travel_hub), Some(("an RTL8152, not an RTL8153", -19)));
    assert_eq!(failure(0x7420, travel_hub), Some(("an RTL8156, not an RTL8153", -19)));
    let (bus, _, found) = found(0x6400, travel_hub);
    let r = bind(bus.clone(), &found);
    assert!(matches!(r, Bind::Failed("an RTL8153C, whose init this driver lacks", -19)));
    assert!(!bus.0.borrow().calls.iter().any(|c| matches!(c, Call::Configure(_))));
}

#[test]
fn a_device_without_the_vendor_configuration_is_refused() {
    let (bus, _, mut found) = found(0x5c10, RTL8153_IDS);
    found.configs.remove(1);
    let r = bind(bus, &found);
    assert!(matches!(r, Bind::Failed("no vendor configuration with bulk endpoints 1 and 2", -19)));
}

#[test]
fn a_refused_configuration_and_a_bad_address_are_named() {
    let (bus, chip, found) = crate::chip::found(0x5c10, RTL8153_IDS);
    chip.borrow_mut().refuse = Some((0x09, 2, -32));
    assert!(matches!(bind(bus, &found), Bind::Failed("SET_CONFIGURATION refused", -32)));
    let (bus, chip, found) = crate::chip::found(0x5c10, RTL8153_IDS);
    chip.borrow_mut().regs.put(PLA, 0xd000, &[0x01, 0, 0, 0, 0, 0]);
    assert!(matches!(bind(bus, &found), Bind::Failed("MAC in PLA_BACKUP not usable", -99)));
}
