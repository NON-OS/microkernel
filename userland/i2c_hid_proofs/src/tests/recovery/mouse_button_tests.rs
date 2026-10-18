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

//! On a pad whose mode never reads back, the click a run of mouse reports
//! lets go is the gesture's alone, and it is let go once. Button 1 held down
//! by the pad's own mouse reports is theirs: it is neither pressed again nor
//! let go until they let it go.

use nonos_i2cmodel::touchpad::MODE_REPORT_ID;
use nonos_libc::{mk_idle_ms, take_events, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP};

use crate::input::poll;
use crate::state::State;
use crate::tests::fixture::{of_kind, Rig};
use crate::tests::stick_tests::dragging;

/// Twenty mouse reports with `buttons`, 8 ms apart, the mode never reading
/// back: the button-ups and the button-downs posted.
fn over(r: &Rig, s: &mut State, buttons: u8) -> (usize, usize) {
    for dx in 1..=20 {
        r.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID]);
        r.pad.lock().push_input(&[3, buttons, dx, 0]);
        poll(s);
        mk_idle_ms(8);
    }
    let events = take_events();
    (of_kind(&events, INPUT_KIND_BUTTON_UP).len(), of_kind(&events, INPUT_KIND_BUTTON_DOWN).len())
}

#[test]
fn a_run_of_mouse_reports_lets_go_of_the_click_once() {
    let (r, mut s, _) = dragging();
    assert_eq!(over(&r, &mut s, 0).0, 1, "the click was not let go once");
}

#[test]
fn button_1_the_mouse_reports_hold_stays_down_until_they_let_go() {
    let (r, mut s, _) = dragging();
    assert_eq!(over(&r, &mut s, 0b01), (0, 1), "button 1 let go, or pressed twice");
    assert_eq!(over(&r, &mut s, 0).0, 1, "not let go when the mouse reports did");
}
