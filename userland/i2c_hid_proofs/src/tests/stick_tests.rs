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

//! A report on the mouse collection from a pad still in touchpad mode is a
//! pointing stick or the pad's own buttons, not a reset: the input mode is
//! read back, found as set, and nothing the pad holds is let go.

use nonos_i2cmodel::touchpad::{Touch, MODE_REPORT_ID};
use nonos_i2cmodel::Command;
use nonos_libc::INPUT_KIND_POINTER_REL;
use nonos_libc::{take_events, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP};

use super::driven::frame;
use super::fixture::{of_kind, Rig};
use super::reset_tests::bound_with_mouse;
use crate::input::poll;
use crate::state::State;

/// A pad with a mouse collection, bound, with a click held down and dragged.
pub(super) fn dragging() -> (Rig, State, Touch) {
    let (r, mut s) = bound_with_mouse();
    let press = Touch { button: true, ..Touch::finger(300, 300) };
    frame(&r, &mut s, press);
    frame(&r, &mut s, Touch { x: 301, ..press });
    assert_eq!(of_kind(&take_events(), INPUT_KIND_BUTTON_DOWN).len(), 1);
    r.pad.lock().take_commands();
    (r, s, press)
}

#[test]
fn a_drag_survives_a_stray_mouse_report_while_the_pad_is_in_touchpad_mode() {
    let (r, mut s, press) = dragging();
    r.pad.lock().push_input(&[3, 0, 5, 0]);
    poll(&mut s);
    let (events, cmds) = (take_events(), r.pad.lock().take_commands());
    assert!(
        of_kind(&events, INPUT_KIND_BUTTON_UP).is_empty(),
        "the held click was let go: {events:?}"
    );
    assert_eq!(of_kind(&events, INPUT_KIND_POINTER_REL).len(), 1, "the stick did not move");
    let set = |c: &Command| matches!(c, Command::SetReport { .. });
    assert!(!cmds.iter().any(set), "a pad still in touchpad mode was set up again: {cmds:?}");
    let read_back = |c: &Command| matches!(c, Command::GetReport { id: MODE_REPORT_ID, .. });
    assert!(cmds.iter().any(read_back), "the input mode was not read back: {cmds:?}");
    frame(&r, &mut s, Touch { x: 310, ..press });
    frame(&r, &mut s, Touch { x: 320, ..press });
    let events = take_events();
    assert!(of_kind(&events, INPUT_KIND_BUTTON_DOWN).is_empty(), "the drag was grabbed again");
    assert!(!of_kind(&events, INPUT_KIND_POINTER_REL).is_empty(), "the drag stopped");
    assert_eq!(r.pad.lock().feature(MODE_REPORT_ID), Some(&[MODE_REPORT_ID, 3, 0b11][..]));
}
