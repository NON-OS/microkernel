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

//! A control queue the host stalls on, recovered in place: the
//! device reset, the queue restarted on its grant, and every output shown
//! again on its own surface from the pages the driver kept, so the frame the
//! compositor last sent is on the screen again with nothing drawn anew.

use nonos_libc::{releases, said};

use super::kit::{draw, live, shown};
use super::recover_kit::{flush, said_from, stall};
use super::rig::rig;
use crate::state::Health;

#[test]
fn a_stall_is_recovered_in_place_and_the_last_frame_shown_again() {
    let mut r = rig();
    let s = draw(&mut r, 0, 0x00AB_CDEF);
    let (grants, frame, given) = (live(), shown(&r, 0), releases());
    let lines = said().len();
    assert!(!stall(&mut r, &s, 1), "recovered, not down");
    assert_eq!(r.d.server.display.health, Health::Live);
    assert_eq!((r.d.tally.recoveries, r.d.tally.in_a_row), (1, 1));
    assert_eq!(shown(&r, 0), frame, "the last frame, from the kept pages");
    assert_eq!((live(), releases()), (grants, given), "nothing given back");
    assert!(said_from(lines).iter().any(|l| l.contains("recovered in place, same surfaces")));
    let next = draw(&mut r, 0, 0x0012_3456);
    assert_eq!(flush(&mut r, &next), 0, "served again");
    assert_eq!(shown(&r, 0), Some(vec![0x0012_3456; 640 * 480]));
}

#[test]
fn a_try_the_host_stalls_on_too_is_followed_by_the_next() {
    let mut r = rig();
    let s = draw(&mut r, 0, 0x0055_5555);
    let (frame, lines) = (shown(&r, 0), said().len());
    // Dead for the flush and for the first command of the first try.
    assert!(!stall(&mut r, &s, 2));
    assert_eq!(r.d.tally.recoveries, 1, "one recovery, on its second try");
    assert!(said_from(lines).iter().any(|l| l.contains("recovering (try 2 of 3)")));
    assert_eq!(shown(&r, 0), frame);
}
