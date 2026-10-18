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

//! A pad that rings after a power cycle may refuse its first writes. The
//! set-up stays owed until one takes, on a pad with no mouse collection to
//! prompt another too, or until a touch report says the mode holds; and a
//! refused one spends none of the tries, however long the refusals last.

use nonos_i2cmodel::touchpad::{touch_report, Touch, MODE_REPORT_ID};
use nonos_i2cmodel::{Command, HidOverI2c};

use super::power_cycle::{quiet, rung_and_refused, PRESS};
use crate::drive::pass;
use crate::tests::descs::pad_with_mouse;
use crate::tests::fixture::{pad, MAX_INPUT, PAD};

#[test]
fn an_owed_set_up_the_pad_refuses_is_tried_again_until_it_takes() {
    let (w, mut s) = rung_and_refused(pad(), 1);
    assert!(!s.mouse_layout.is_relative_mouse(), "the pad has a mouse collection");
    quiet(&mut s, 1_100, 4_100);
    let mode = w.pad.lock().feature(MODE_REPORT_ID).map(<[u8]>::to_vec);
    assert_eq!(mode, Some(vec![MODE_REPORT_ID, 3, 0b11]), "the refused set-up was dropped");
}

#[test]
fn a_touch_report_settles_what_is_owed() {
    let (w, mut s) = rung_and_refused(pad(), 1);
    w.pad.lock().push_input(&touch_report(&Touch { x: 302, ..PRESS }));
    pass(&mut s, 1_030);
    w.pad.lock().take_commands();
    quiet(&mut s, 1_100, 4_100);
    let asked = w.pad.lock().commands().iter().any(|c| matches!(c, Command::GetReport { .. }));
    assert!(!asked, "set up again after a touch report said the mode holds");
}

#[test]
fn writes_refused_for_two_and_a_half_seconds_spend_no_tries() {
    let device = HidOverI2c::new(PAD, pad_with_mouse(), MAX_INPUT, 0x04F3, 0x3028);
    let (w, mut s) = rung_and_refused(device, u32::MAX);
    quiet(&mut s, 1_100, 3_600);
    let refused = u32::MAX - w.nacks.get();
    assert!(refused <= 3, "{refused} writes asked for in 2.5 s");
    // The pad takes writes now, and the user reaches for it: it reports as
    // the mouse it still is.
    w.nacks.set(0);
    for t in 0..15 {
        w.pad.lock().push_input(&[3, 0, 1 + t as u8, 0]);
        quiet(&mut s, 3_600 + t * 100, 3_700 + t * 100);
    }
    let mode = w.pad.lock().feature(MODE_REPORT_ID).map(<[u8]>::to_vec);
    assert_eq!(mode, Some(vec![MODE_REPORT_ID, 3, 0b11]), "a mouse for the session");
}
