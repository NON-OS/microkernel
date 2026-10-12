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

//! Discovery takes a copper port before any other (the the review,
//! R15): an 82576 fiber or I350 SerDes function earlier in the device list
//! no longer shadows a copper I210 after it, and a machine with only a
//! fiber port still gets the driver, which then says why it stops.

#[path = "../../../capsule_driver_igb/src/discover/choose.rs"]
mod choose;

use crate::facts::ids::{is_copper, IGB_PARTS};
use crate::facts::parts::Media;
use choose::choose;

#[test]
fn the_review_names_these_four_as_not_copper() {
    for id in [0x10E6, 0x10E7, 0x1522, 0x1523] {
        assert!(!is_copper(id), "{id:#06x}");
    }
    for id in [0x1533, 0x1539, 0x157B, 0x1521, 0x10C9] {
        assert!(is_copper(id), "{id:#06x}");
    }
    let copper = IGB_PARTS.iter().filter(|(.., m)| *m == Media::Copper).count();
    assert_eq!(copper, 12);
}

#[test]
fn a_copper_port_later_in_the_list_is_taken_first() {
    let list = [0x10E6u16, 0x1523, 0x1533, 0x1539];
    assert_eq!(choose(list.into_iter(), |d| is_copper(*d)), Some(0x1533));
}

#[test]
fn with_no_copper_port_the_first_match_is_still_taken() {
    let list = [0x1522u16, 0x10E7];
    assert_eq!(choose(list.into_iter(), |d| is_copper(*d)), Some(0x1522));
    assert_eq!(choose(core::iter::empty::<u16>(), |d| is_copper(*d)), None);
}
