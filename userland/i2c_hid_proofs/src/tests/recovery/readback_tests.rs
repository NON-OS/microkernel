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

//! Where the mode reads back, only the read-back and the ring say a pad
//! reset. A click held by a still finger the pad then reports nothing for,
//! or by a clickpad pressed with no contact reported, outlasts any run of
//! stick reports there.

use nonos_i2cmodel::touchpad::{touch_report, Touch};
use nonos_libc::{mk_idle_ms, take_events, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP};

use crate::input::poll;
use crate::state::State;
use crate::tests::fixture::{of_kind, Rig};
use crate::tests::reset_tests::bound_with_mouse;
use crate::tests::stick_tests::dragging;

/// About a second of stick reports and nothing else: the button-ups.
fn stick_alone(r: &Rig, s: &mut State) -> usize {
    let mut ups = 0;
    for dx in 1..=70u8 {
        r.pad.lock().push_input(&[3, 0, dx % 7 + 1, 0]);
        poll(s);
        ups += of_kind(&take_events(), INPUT_KIND_BUTTON_UP).len();
        mk_idle_ms(8);
    }
    ups
}

#[test]
fn a_still_finger_the_pad_reports_nothing_for_keeps_its_click() {
    let (r, mut s, _) = dragging();
    assert_eq!(stick_alone(&r, &mut s), 0, "the still finger's click was let go");
}

#[test]
fn a_clickpad_pressed_with_no_contact_keeps_its_click() {
    let (r, mut s) = bound_with_mouse();
    let nail = touch_report(&Touch { button: true, ..Touch::lifted() });
    for _ in 0..6 {
        r.pad.lock().push_input(&nail);
        poll(&mut s);
    }
    assert_eq!(of_kind(&take_events(), INPUT_KIND_BUTTON_DOWN).len(), 1, "no press held");
    assert_eq!(stick_alone(&r, &mut s), 0, "the press with no contact was let go");
}
