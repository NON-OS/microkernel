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

//! A pad that cannot read its mode back still shows a reset: four or more
//! mouse reports with no touch report among them, 100 ms from the first.
//! The click the gesture holds is let go then. A pad sending touch reports
//! at its rate, the slowest one included, never trips it.

use nonos_i2cmodel::touchpad::{touch_report, Touch, MODE_REPORT_ID};
use nonos_libc::{mk_idle_ms, mk_uptime_ms, take_events, INPUT_KIND_BUTTON_UP};

use crate::input::poll;
use crate::state::State;
use crate::tests::fixture::{of_kind, Rig};
use crate::tests::stick_tests::dragging;

/// A tenth of the 125 reports a second a precision touchpad is to send.
const SLOWEST_MS: i64 = 80;

/// A mouse report every 14 ms or so for `ms`, on a pad whose mode never
/// reads back, and the held click moving each `touch_ms` ahead of one (each
/// time when it is 0): when the first button-up came, if one did.
fn reports(r: &Rig, s: &mut State, ms: i64, touch_ms: Option<i64>) -> Option<i64> {
    let (start, mut x) = (mk_uptime_ms(), 302);
    let mut next_touch = start;
    while mk_uptime_ms() < start + ms {
        if let Some(every) = touch_ms.filter(|_| mk_uptime_ms() >= next_touch) {
            x += 1;
            let held = Touch { x, button: true, ..Touch::finger(300, 300) };
            r.pad.lock().push_input(&touch_report(&held));
            poll(s);
            next_touch += every;
        }
        r.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID]);
        r.pad.lock().push_input(&[3, 0, 1, 0]);
        poll(s);
        if !of_kind(&take_events(), INPUT_KIND_BUTTON_UP).is_empty() {
            return Some(mk_uptime_ms() - start);
        }
        mk_idle_ms(8);
    }
    None
}

#[test]
fn a_reset_mid_drag_lets_go_within_100_ms_of_mouse_reports() {
    let (r, mut s, _) = dragging();
    // The pad resets: its finger is gone and it reports as a mouse. The
    // first mouse report 100 ms into the run lets go of the click.
    let up = reports(&r, &mut s, 1_000, None);
    assert!(up.is_some_and(|ms| (90..130).contains(&ms)), "let go after {up:?} ms");
}

#[test]
fn touch_reports_at_the_pads_rate_keep_the_click() {
    for every in [0, SLOWEST_MS] {
        let (r, mut s, _) = dragging();
        let up = reports(&r, &mut s, 3_000, Some(every));
        assert_eq!(up, None, "a click held with touches every {every} ms was let go");
        assert!(s.gesture.was_button, "the click went with touches every {every} ms");
    }
}
