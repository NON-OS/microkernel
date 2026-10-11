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
//! LINKS decoded as ixgbe_check_mac_link_generic reads it, and the one line
//! per change link_status prints.

use nonos_libc::said;

use crate::constants::ctrl::*;
use crate::constants::part::Part;
use crate::link::status::changed;
use crate::link::{decode, note, LinkState};

fn st(up: bool, mbps: u16) -> LinkState {
    LinkState { up, mbps }
}

#[test]
fn links_vectors() {
    assert_eq!((LINKS_UP, LINKS_SPEED_NON_STD), (0x4000_0000, 0x0800_0000));
    assert_eq!(decode(0x7000_0000, Part::I82599), st(true, 10_000));
    assert_eq!(decode(0x6000_0000, Part::I82599), st(true, 1_000));
    assert_eq!(decode(0x5000_0000, Part::I82599), st(true, 100));
    assert_eq!(decode(0x3000_0000, Part::X540), st(false, 10_000));
    // The NBASE-T rates are the X550's alone.
    assert_eq!(decode(0x7800_0000, Part::X550), st(true, 2_500));
    assert_eq!(decode(0x5800_0000, Part::X550), st(true, 5_000));
    assert_eq!(decode(0x7800_0000, Part::I82599), st(true, 10_000));
    assert_eq!(decode(0x6800_0000, Part::X550), st(true, 1_000));
}

#[test]
fn only_a_real_change_is_a_change() {
    assert!(changed(None, st(false, 10)));
    assert!(!changed(Some(st(false, 10)), st(false, 10_000)), "down is down");
    assert!(changed(Some(st(false, 10)), st(true, 10_000)));
    assert!(!changed(Some(st(true, 10_000)), st(true, 10_000)));
    assert!(changed(Some(st(true, 10_000)), st(true, 1_000)), "renegotiated");
}

#[test]
fn one_line_per_change_in_the_owners_words() {
    let start = said().len();
    let mut last = None;
    for links in [0, 0x3000_0000, 0x7000_0000, 0x7000_0000, 0x6000_0000, 0] {
        note(&mut last, decode(links, Part::I82599));
    }
    let lines = said()[start..].to_vec();
    let want = [
        "ixgbe: link down\n",
        "ixgbe: link up 10000\n",
        "ixgbe: link up 1000\n",
        "ixgbe: link down\n",
    ];
    assert_eq!(lines, want);
}
