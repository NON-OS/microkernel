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

//! Recoveries in a row are bounded: a device that stalls again soon after
//! each of three recoveries is Down at its fourth stall, and one that serves
//! SERVED_MS between stalls has its count forgiven every time.

use nonos_libc::said;

use super::kit::draw;
use super::recover_kit::{flush, said_from, stall};
use super::rig::rig;
use crate::clock::pass;
use crate::server::recover::policy::SERVED_MS;
use crate::server::recover::settle;

#[test]
fn stalls_soon_after_each_of_three_recoveries_end_down_at_the_fourth() {
    let mut r = rig();
    let s = draw(&mut r, 0, 1);
    for n in 1..=3 {
        assert!(!stall(&mut r, &s, 1), "recovery {n}");
    }
    assert_eq!(r.d.tally.in_a_row, 3);
    let lines = said().len();
    assert!(stall(&mut r, &s, 1), "the fourth stall is down");
    let said = said_from(lines);
    assert!(said.iter().any(|l| l.contains("stalled after each recovery in a row")));
}

#[test]
fn a_device_that_serves_long_enough_between_stalls_is_forgiven() {
    let mut r = rig();
    let s = draw(&mut r, 0, 1);
    for n in 1..=5 {
        assert!(!stall(&mut r, &s, 1), "stall {n} recovered");
        pass(SERVED_MS);
        assert_eq!(flush(&mut r, &s), 0, "served after recovery {n}");
        assert!(!settle(&mut r.d));
        assert_eq!(r.d.tally.in_a_row, 0, "forgiven after recovery {n}");
    }
    assert_eq!(r.d.tally.recoveries, 5);
}
