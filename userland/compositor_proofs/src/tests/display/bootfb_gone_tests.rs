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
//! bootfb let go when a GPU driver takes its scanout (G-80, L79): a LOST,
//! dark output on a driver that sets no mode moves the desktop to the best
//! other display, its LEAVING notice or not; a LOST that a driver with mode
//! setting can put back, or a blank (LOST still scanning out), does not;
//! and the desktop never goes back to the boot framebuffer, last in the
//! kernel's list, from any display it moved to.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::State;

use super::poll_tests::{link, PID};
use crate::display::link::WireLink;
use crate::display::poll_rule::{step, Step};
use crate::display::return_rule::{back, Left};
use crate::displays::DISPLAYS;

fn bootfb() -> WireLink {
    WireLink { name: "driver.bootfb0", fixed: true, ..link(false) }
}

fn state(flags: u32) -> Option<State> {
    let serial = link(false).serial;
    Some(State { serial, flags, width: 1024, height: 768, ..State::default() })
}

#[test]
fn a_fixed_driver_s_lost_dark_output_moves_the_desktop() {
    assert_eq!(step(&bootfb(), Some(PID), state(LOST)), Step::Move, "no notice needed");
    assert_eq!(step(&WireLink { stale: true, ..bootfb() }, Some(PID), state(LOST)), Step::Move);
    let blank = state(LOST | SCANNING_OUT);
    assert_eq!(step(&bootfb(), Some(PID), blank), Step::Repaint, "a blank is not a loss");
    assert_eq!(step(&bootfb(), Some(PID), state(SCANNING_OUT)), Step::Keep);
    assert_eq!(step(&link(false), Some(PID), state(LOST)), Step::Repaint, "mode set can put it back");
}

#[test]
fn the_desktop_never_goes_back_to_the_boot_framebuffer() {
    let last = DISPLAYS.len() - 1;
    assert_eq!(DISPLAYS[last], "driver.bootfb0", "the fallback is last in the kernel's list");
    let left = Some(Left { rank: last, pid: PID });
    for driving in 0..DISPLAYS.len() {
        assert!(!back(left, driving, Some(PID + 1)), "back to bootfb from rank {driving}");
    }
}
