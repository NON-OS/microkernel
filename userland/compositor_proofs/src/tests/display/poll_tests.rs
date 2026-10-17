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

//! The 500 ms poll of a wire driver: a restart, a replaced primary, a lost
//! output, each answered with its own step and nothing else.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::State;

use crate::display::link::WireLink;
use crate::display::poll_rule::{step, Step};

pub const PID: u32 = 40;
const SERIAL: u32 = PID << 16 | 1;

pub fn link(stale: bool) -> WireLink {
    let (name, cursor_offset, leaving, nudged, fixed) = ("driver.bga0", 0, false, false, false);
    let (serial, pid, physical_mm, outputs) = (SERIAL, PID, 0, 1);
    #[rustfmt::skip]
    let l = WireLink { name, serial, pid, physical_mm, outputs, stale, cursor_offset, leaving, nudged, fixed };
    l
}

fn state(serial: u32, flags: u32) -> Option<State> {
    Some(State { serial, flags, width: 1024, height: 768, ..State::default() })
}

#[test]
fn a_driver_that_still_shows_our_primary_is_left_alone() {
    assert_eq!(step(&link(false), Some(PID), state(SERIAL, SCANNING_OUT)), Step::Keep);
    assert_eq!(step(&link(false), Some(PID), None), Step::Keep, "no answer this time");
}

#[test]
fn a_new_pid_is_a_restarted_driver_driven_from_the_start() {
    /*
     * Its primaries went with the old process; the serial the compositor
     * holds names memory that no longer exists, stale or not.
     */
    assert_eq!(step(&link(false), Some(PID + 1), None), Step::Reprime);
    assert_eq!(step(&link(true), Some(PID + 1), None), Step::Reprime);
}

#[test]
fn a_driver_gone_from_the_registry_is_waited_for() {
    assert_eq!(step(&link(true), None, None), Step::Keep);
}

#[test]
fn another_serial_or_a_stale_flush_asks_for_the_primary_again() {
    assert_eq!(step(&link(false), Some(PID), state(SERIAL + 1, SCANNING_OUT)), Step::Resurface);
    assert_eq!(step(&link(true), Some(PID), state(SERIAL, SCANNING_OUT)), Step::Resurface);
}

#[test]
fn a_lost_output_is_repainted_whole() {
    let s = state(SERIAL, SCANNING_OUT | LOST);
    assert_eq!(step(&link(false), Some(PID), s), Step::Repaint);
}
