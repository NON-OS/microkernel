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

//! The bounds on setting a pad up again, each on its own: three tries with
//! no touch report between, a second apart at least; and a touch report
//! gives the tries back.

use nonos_i2cmodel::touchpad::{Touch, MODE_REPORT_ID};
use nonos_i2cmodel::Command;
use nonos_libc::mk_idle_ms;

use super::driven::frame;
use super::fixture::Rig;
use super::reset_tests::{bound_with_mouse, self_reset};
use crate::input::poll;
use crate::state::State;

/// The pad loses its mode and sends a mouse report: whether it was set up
/// again for it.
fn lose_and_report(r: &Rig, s: &mut State, dx: u8) -> bool {
    self_reset(r);
    r.pad.lock().push_input(&[3, 0, dx, 0]);
    poll(s);
    let set = |c: &Command| matches!(c, Command::SetReport { id: MODE_REPORT_ID, .. });
    r.pad.lock().commands().iter().any(set)
}

#[test]
fn a_second_reset_within_the_holdoff_waits_for_it() {
    let (r, mut s) = bound_with_mouse();
    assert!(lose_and_report(&r, &mut s, 1));
    assert!(!lose_and_report(&r, &mut s, 2), "set up twice within a second");
    mk_idle_ms(1_100);
    assert!(lose_and_report(&r, &mut s, 3), "not set up once the second had passed");
}

#[test]
fn three_tries_with_no_touch_report_between_and_no_more() {
    let (r, mut s) = bound_with_mouse();
    let tries: Vec<bool> = (1..=6)
        .map(|dx| {
            mk_idle_ms(1_100);
            lose_and_report(&r, &mut s, dx)
        })
        .collect();
    assert_eq!(tries, [true, true, true, false, false, false]);
}

#[test]
fn a_touch_report_gives_the_tries_back() {
    let (r, mut s) = bound_with_mouse();
    for dx in 1..=4 {
        mk_idle_ms(1_100);
        lose_and_report(&r, &mut s, dx);
    }
    frame(&r, &mut s, Touch::finger(500, 500));
    mk_idle_ms(1_100);
    assert!(lose_and_report(&r, &mut s, 9), "the tries stayed spent after a touch report");
}
