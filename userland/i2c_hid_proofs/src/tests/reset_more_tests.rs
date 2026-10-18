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

//! After a reset: what the pad held is let go and what it sent is posted;
//! and a pad that will not stay in touchpad mode is set up a bounded number
//! of times, then read as the mouse it stays.

use nonos_i2cmodel::touchpad::{Touch, MODE_REPORT_ID};
use nonos_i2cmodel::Command;
use nonos_libc::INPUT_KIND_POINTER_REL;
use nonos_libc::{take_events, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP};

use super::driven::frame;
use super::fixture::of_kind;
use super::reset_tests::{bound_with_mouse, self_reset};
use crate::input::poll;

#[test]
fn a_reset_lets_go_of_a_held_click_and_posts_the_mouse_report() {
    let (r, mut s) = bound_with_mouse();
    let press = Touch { button: true, ..Touch::finger(300, 300) };
    frame(&r, &mut s, press);
    frame(&r, &mut s, Touch { x: 301, ..press });
    assert_eq!(of_kind(&take_events(), INPUT_KIND_BUTTON_DOWN).len(), 1);
    self_reset(&r);
    r.pad.lock().push_input(&[3, 0, 5, 0]);
    poll(&mut s);
    let events = take_events();
    assert_eq!(of_kind(&events, INPUT_KIND_BUTTON_UP).len(), 1, "the click stays held: {events:?}");
    let motion = of_kind(&events, INPUT_KIND_POINTER_REL);
    assert_eq!(motion.iter().map(|e| e.delta_x).collect::<Vec<_>>(), [5]);
}

#[test]
fn a_pad_that_will_not_keep_touchpad_mode_is_set_up_three_times_at_most() {
    let (r, mut s) = bound_with_mouse();
    let mut sets = 0;
    for dx in 1..=30u8 {
        self_reset(&r);
        r.pad.lock().push_input(&[3, 0, dx, 0]);
        poll(&mut s);
        let mode = |c: &Command| matches!(c, Command::SetReport { id: MODE_REPORT_ID, .. });
        sets += r.pad.lock().commands().iter().filter(|c| mode(c)).count();
    }
    assert!((1..=3).contains(&sets), "touchpad mode was set {sets} times");
    let moved: i32 =
        of_kind(&take_events(), INPUT_KIND_POINTER_REL).iter().map(|e| e.delta_x).sum();
    assert_eq!(moved, (1..=30).sum::<i32>(), "a mouse report was dropped");
}
