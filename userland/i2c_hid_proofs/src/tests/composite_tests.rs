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

//! A device that is a keyboard and a mouse is driven by its mouse
//! collection alone. Its key reports are dropped unread: nothing derived
//! from one stays in the driver (plan/50 C8), and none is posted.

use nonos_i2cmodel::HidOverI2c;
use nonos_libc::{take_events, INPUT_KIND_POINTER_REL};

use super::descs::{CTRL_A, KEYBOARD, MOUSE};
use super::fixture::{of_kind, rig_with, Rig, HID_DESC_REG, PAD};
use crate::input::poll;
use crate::setup;
use crate::state::State;

fn keyboard_and_mouse() -> (Rig, State) {
    let desc = [KEYBOARD, MOUSE].concat();
    let device = HidOverI2c::new(PAD, desc, 2 + CTRL_A.len() as u16, 0x04F3, 0x0004);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    let s = setup::run().expect("setup");
    assert!(s.found(), "the mouse collection was not driven");
    take_events();
    (r, s)
}

#[test]
fn a_key_report_from_a_keyboard_and_mouse_leaves_nothing_behind() {
    let (r, mut s) = keyboard_and_mouse();
    r.pad.lock().push_input(&CTRL_A);
    poll(&mut s);
    assert!(take_events().is_empty(), "a key report was posted");
    assert!(s.frame.iter().all(|&b| b == 0), "the key report is still in the buffer");
    let kept = (s.repeat.last.is_some(), s.repeat.frame_repeats);
    assert_eq!(kept, (false, 0), "something derived from the key report was kept");
    assert_eq!(s.last_buttons, 0);
    r.pad.lock().push_input(&[3, 0, 7, 0]);
    poll(&mut s);
    let motion = of_kind(&take_events(), INPUT_KIND_POINTER_REL);
    assert_eq!(motion.iter().map(|e| e.delta_x).collect::<Vec<_>>(), [7], "the mouse stopped");
}

/// A key report ends a run of verbatim repeats as any read that brought no
/// pointer report does: the same mouse report after it is fresh input.
#[test]
fn a_key_report_between_two_like_mouse_reports_ends_their_run() {
    let (r, mut s) = keyboard_and_mouse();
    let mut moved = |report: &[u8]| {
        r.pad.lock().push_input(report);
        poll(&mut s);
        of_kind(&take_events(), INPUT_KIND_POINTER_REL).len()
    };
    let step = [3, 0, 7, 0];
    assert_eq!(moved(&step), 1, "the mouse did not move");
    assert_eq!(moved(&step), 0, "a verbatim repeat was posted");
    assert_eq!(moved(&CTRL_A), 0, "a key report was posted");
    assert_eq!(moved(&step), 1, "the key report did not end the run");
}
