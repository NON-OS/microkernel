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

//! A pad that resets itself mid-session comes back in its power-on input
//! mode, the mouse, and says so by reporting on its mouse collection. The
//! driver sees that and sets touchpad mode again.

use nonos_i2cmodel::touchpad::{Touch, MODE_REPORT_ID, TOUCH_REPORT_LEN};
use nonos_i2cmodel::{Command, HidOverI2c};
use nonos_libc::take_events;

use super::descs::pad_with_mouse;
use super::driven::frame;
use super::fixture::{rig_with, Rig, HID_DESC_REG, PAD};
use crate::input::poll;
use crate::setup;
use crate::state::State;

const TOUCHPAD_MODE: [u8; 3] = [MODE_REPORT_ID, 3, 0b11];

pub(super) fn bound_with_mouse() -> (Rig, State) {
    let device = HidOverI2c::new(PAD, pad_with_mouse(), 2 + TOUCH_REPORT_LEN, 0x04F3, 0x3028);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    let state = setup::run().expect("setup");
    assert_eq!(r.pad.lock().feature(MODE_REPORT_ID), Some(&TOUCHPAD_MODE[..]));
    take_events();
    (r, state)
}

/// What the pad does when it resets: its input mode goes back to the mouse.
pub(super) fn self_reset(r: &Rig) {
    let mut pad = r.pad.lock();
    pad.set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID, 0, 0]);
    pad.take_commands();
}

#[test]
fn a_mouse_report_from_a_pad_in_touchpad_mode_sets_the_mode_again() {
    let (r, mut s) = bound_with_mouse();
    frame(&r, &mut s, Touch::finger(100, 100));
    self_reset(&r);
    r.pad.lock().push_input(&[3, 0, 5, 0]);
    poll(&mut s);
    let cmds = r.pad.lock().take_commands();
    let set = |c: &Command| matches!(c, Command::SetReport { ty: 3, id: MODE_REPORT_ID, .. });
    assert!(cmds.iter().any(set), "touchpad mode was not set again: {cmds:?}");
    assert_eq!(r.pad.lock().feature(MODE_REPORT_ID), Some(&TOUCHPAD_MODE[..]));
}
