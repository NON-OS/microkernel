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

use super::dmar_rmrr::parse_rmrr;
use super::dmar_rmrr_names::{rmrr_names, rmrr_sole};
use super::dmar_scope::Cover;
use super::rmrr_tests::{rmrr, scope};
use super::scope::{SCOPE_BRIDGE, SCOPE_ENDPOINT};

// A root port at 00:1c.0 whose secondary bus is 2 and subordinate 3.
fn bridges(bus: u8, dev: u8, func: u8) -> Option<(u8, u8)> {
    ((bus, dev, func) == (0, 0x1c, 0)).then_some((2, 3))
}

#[test]
fn an_endpoint_scope_names_that_device_alone() {
    let b = rmrr(0, 0x7A00_0000, 0x7A01_FFFF, &[scope(SCOPE_ENDPOINT, 0, &[(0x14, 0)])]);
    let r = parse_rmrr(&b).expect("parsed");
    assert_eq!(rmrr_names(&r, 0, (0, 0x14, 0), &bridges), Cover::Yes);
    for other in [(0, 0x14, 1), (0, 0x02, 0), (1, 0x14, 0)] {
        assert_eq!(rmrr_names(&r, 0, other, &bridges), Cover::No, "{other:?}");
    }
    assert_eq!(rmrr_names(&r, 1, (0, 0x14, 0), &bridges), Cover::No, "another segment");
}

#[test]
fn a_bridge_scope_names_every_bus_below_it_and_no_other() {
    let b = rmrr(0, 0x8000_0000, 0x8000_FFFF, &[scope(SCOPE_BRIDGE, 0, &[(0x1c, 0)])]);
    let r = parse_rmrr(&b).expect("parsed");
    for below in [(2, 0, 0), (3, 0x1f, 7)] {
        assert_eq!(rmrr_names(&r, 0, below, &bridges), Cover::Yes, "{below:?}");
    }
    assert_eq!(rmrr_names(&r, 0, (4, 0, 0), &bridges), Cover::No);
}

#[test]
fn a_scope_that_cannot_be_resolved_or_was_dropped_is_unknown_never_yes() {
    // A bridge that does not answer: whether the device is below it is unknown.
    let b = rmrr(0, 0x8000_0000, 0x8000_FFFF, &[scope(SCOPE_BRIDGE, 0, &[(0x1d, 0)])]);
    let r = parse_rmrr(&b).expect("parsed");
    assert_eq!(rmrr_names(&r, 0, (5, 0, 0), &bridges), Cover::Unknown);
    let mut t = parse_rmrr(&rmrr(0, 0x1000, 0x1FFF, &[scope(SCOPE_ENDPOINT, 0, &[(2, 0)])]));
    let t = t.as_mut().map(|t| {
        t.truncated = true;
        *t
    });
    let t = t.expect("parsed");
    assert_eq!(rmrr_names(&t, 0, (0, 2, 0), &bridges), Cover::Yes, "a kept scope still names");
    assert_eq!(rmrr_names(&t, 0, (0, 3, 0), &bridges), Cover::Unknown, "a dropped one might");
}

#[test]
fn only_a_region_naming_one_endpoint_alone_is_its_own() {
    let one = |kind, path: &[(u8, u8)]| rmrr(0, 0x7B80_0000, 0x7F7F_FFFF, &[scope(kind, 0, path)]);
    let gpu = parse_rmrr(&one(SCOPE_ENDPOINT, &[(0x02, 0)])).expect("parsed");
    assert!(rmrr_sole(&gpu), "integrated graphics' stolen memory");
    let usb = [scope(SCOPE_ENDPOINT, 0, &[(0x14, 0)]), scope(SCOPE_ENDPOINT, 0, &[(0x1a, 0)])];
    let usb = parse_rmrr(&rmrr(0, 0x7A00_0000, 0x7A01_FFFF, &usb)).expect("parsed");
    assert!(!rmrr_sole(&usb), "a USB buffer two controllers share is neither's alone");
    let below = parse_rmrr(&one(SCOPE_BRIDGE, &[(0x1c, 0)])).expect("parsed");
    assert!(!rmrr_sole(&below), "a bridge's subtree is many devices'");
}
