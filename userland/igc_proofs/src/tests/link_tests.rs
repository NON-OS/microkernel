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

//! STATUS decoded as igc_get_speed_and_duplex_copper reads it, 2500 among
//! them, and the one line per change link_status prints.

use nonos_libc::said;

use crate::constants::ctrl::*;
use crate::link::status::changed;
use crate::link::{decode, note, LinkState};

fn st(up: bool, mbps: u16, full: bool) -> LinkState {
    LinkState { up, mbps, full }
}

#[test]
fn status_vectors_including_2500() {
    assert_eq!((STATUS_SPEED_2500, STATUS_SPEED_1000), (0x0040_0000, 0x80));
    assert_eq!((STATUS_SPEED_100, STATUS_LU, STATUS_FD), (0x40, 0x2, 0x1));
    assert_eq!(decode(0x0040_0083), st(true, 2500, true));
    assert_eq!(decode(0x0000_0083), st(true, 1000, true));
    assert_eq!(decode(0x0000_0043), st(true, 100, true));
    assert_eq!(decode(0x0000_0003), st(true, 10, true));
    assert_eq!(decode(0x0000_0042), st(true, 100, false));
    assert_eq!(decode(0x0040_0081), st(false, 2500, true));
    assert_eq!(decode(0x0040_0002), st(true, 10, false), "2500 bit alone, as Linux");
    assert_eq!(decode(0x0040_00C3), st(true, 2500, true), "1000 wins over 100");
}

#[test]
fn only_a_real_change_is_a_change() {
    assert!(changed(None, st(false, 10, false)));
    assert!(!changed(Some(st(false, 10, false)), st(false, 2500, true)), "down is down");
    assert!(changed(Some(st(false, 10, false)), st(true, 2500, true)));
    assert!(!changed(Some(st(true, 2500, true)), st(true, 2500, true)));
    assert!(changed(Some(st(true, 2500, true)), st(true, 1000, true)), "renegotiated");
}

#[test]
fn one_line_per_change_in_the_owners_words() {
    let start = said().len();
    let mut last = None;
    note(&mut last, decode(0x0000_0000));
    note(&mut last, decode(0x0000_0080));
    note(&mut last, decode(0x0040_0083));
    note(&mut last, decode(0x0040_0083));
    note(&mut last, decode(0x0000_0042));
    note(&mut last, decode(0x0000_0000));
    let lines = said()[start..].to_vec();
    let want = [
        "igc: link down\n",
        "igc: link up 2500 full\n",
        "igc: link up 100 half\n",
        "igc: link down\n",
    ];
    assert_eq!(lines, want);
}
