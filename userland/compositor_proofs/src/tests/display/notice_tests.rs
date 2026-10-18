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

//! OUTPUT_NOTICE: the driver's notice is the compositor's own envelope, read
//! by its own parser; it is taken from the driver now driven alone; and a
//! device that left moves the desktop whatever the driver still answers.

use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::notice::{notice, LEAVING, OUTPUTS_CHANGED};
use nonos_gfx_wire::State;

use super::poll_tests::{link, PID};
use crate::display::notice_rule::heed;
use crate::display::poll_rule::{step, Step};
use crate::protocol::{parse, OP_OUTPUT_NOTICE};

#[test]
fn the_notice_is_an_ncmp_request_the_compositor_parses() {
    let n = notice(LEAVING);
    let (req, body) = parse(&n).ok().expect("an NCMP request");
    assert_eq!((req.op, body), (OP_OUTPUT_NOTICE, &LEAVING.to_le_bytes()[..]));
}

#[test]
fn only_the_driver_now_driven_is_heeded() {
    let body = OUTPUTS_CHANGED.to_le_bytes();
    for sender in [0, PID + 1, 7] {
        let mut l = link(false);
        assert!(!heed(&mut l, sender, &body) && l == link(false), "sender {sender}");
    }
    let mut l = link(false);
    assert!(heed(&mut l, PID, &body) && l.nudged && !l.leaving, "a poll now");
    let mut l = link(false);
    assert!(!heed(&mut l, PID, &9u32.to_le_bytes()) && !l.nudged, "no such reason");
    assert!(!heed(&mut l, PID, &body[..3]), "short");
}

#[test]
fn a_device_that_left_moves_the_desktop_whatever_the_driver_answers() {
    let mut l = link(false);
    assert!(heed(&mut l, PID, &LEAVING.to_le_bytes()) && l.leaving && l.nudged);
    let st = Some(State { serial: l.serial, flags: SCANNING_OUT | LOST, ..State::default() });
    assert_eq!(step(&l, Some(PID), st), Step::Move, "still announced, still answering");
    assert_eq!(step(&l, None, None), Step::Move, "gone already");
    assert_eq!(step(&link(false), None, None), Step::Keep, "vanished alone: waited for");
}
