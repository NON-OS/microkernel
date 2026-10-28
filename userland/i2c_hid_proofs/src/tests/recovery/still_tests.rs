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

//! Repeats are noted before the repeat check drops them, on a pad whose mode
//! never reads back. A still finger whose pad sends one frame again and
//! again keeps its click while a stick reports now and then; a pad that
//! reset and serves one stale mouse report on every read still lets go.

use nonos_i2cmodel::touchpad::{touch_report, Touch, MODE_REPORT_ID};
use nonos_libc::{mk_idle_ms, take_events, INPUT_KIND_BUTTON_UP};

use crate::input::poll;
use crate::tests::fixture::of_kind;
use crate::tests::stick_tests::dragging;

#[test]
fn a_still_finger_sending_one_frame_keeps_its_click_while_the_stick_moves() {
    let (r, mut s, press) = dragging();
    let still = touch_report(&Touch { x: 301, ..press });
    let mut ups = 0;
    for i in 0..300u16 {
        r.pad.lock().push_input(&still);
        poll(&mut s);
        if i % 30 == 29 {
            r.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID]);
            r.pad.lock().push_input(&[3, 0, 1 + (i % 5) as u8, 0]);
            poll(&mut s);
        }
        ups += of_kind(&take_events(), INPUT_KIND_BUTTON_UP).len();
        mk_idle_ms(8);
    }
    assert!(ups == 0 && s.gesture.was_button, "the still click was let go {ups} times");
}

#[test]
fn one_stale_mouse_report_served_on_every_read_still_lets_go() {
    let (r, mut s, _) = dragging();
    // Read back to back, the same report is past the repeat check's dozen
    // before 100 ms have gone; only the reads it never acts on count on.
    let mut ups = 0;
    for _ in 0..40 {
        r.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID]);
        r.pad.lock().push_input(&[3, 0, 1, 0]);
        poll(&mut s);
        ups += of_kind(&take_events(), INPUT_KIND_BUTTON_UP).len();
    }
    assert_eq!(ups, 1, "the click was not let go once");
}
