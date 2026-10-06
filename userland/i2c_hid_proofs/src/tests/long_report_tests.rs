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

//! A full-size precision touchpad report: five finger slots, scan time and
//! contact count put the click button 20 and more bytes in, where ELAN and
//! Synaptics pads carry it. The bench pad is given 16 constant bytes ahead
//! of its button to put it there.

use nonos_i2cmodel::touchpad::{report_descriptor, touch_report, Touch, TOUCH_REPORT_LEN};
use nonos_i2cmodel::HidOverI2c;
use nonos_libc::{take_events, INPUT_KIND_BUTTON_DOWN};

use super::fixture::{of_kind, rig_with, HID_DESC_REG, PAD};
use crate::input::poll;
use crate::setup;

const PAD_BYTES: usize = 16;
/// Report Size (8), Report Count (16), Input (Constant), Report Count (1).
const PADDING: [u8; 8] = [0x75, 0x08, 0x95, PAD_BYTES as u8, 0x81, 0x03, 0x95, 0x01];

fn long_report(t: &Touch) -> Vec<u8> {
    let mut r = touch_report(t);
    let button = r.pop().unwrap_or(0);
    r.extend([0; PAD_BYTES]);
    r.push(button);
    r
}

#[test]
fn a_click_with_a_resting_finger_is_seen_past_byte_16() {
    let mut desc = report_descriptor();
    let at = desc.windows(4).position(|w| w == [0x05, 0x09, 0x09, 0x01]).expect("button page");
    desc.splice(at..at, PADDING);
    let max_input = 2 + TOUCH_REPORT_LEN + PAD_BYTES as u16;
    let r =
        rig_with(HidOverI2c::new(PAD, desc, max_input, 0x04F3, 0x3028), Some((PAD, HID_DESC_REG)));
    let mut state = setup::run().expect("setup");
    take_events();
    // The finger rests long enough for its report to count as stale, then
    // the pad is pressed under it: only the button byte changes.
    let rest = Touch::finger(600, 400);
    for _ in 0..20 {
        r.pad.lock().push_input(&long_report(&rest));
        poll(&mut state);
    }
    let press = Touch { button: true, ..rest };
    for _ in 0..8 {
        r.pad.lock().push_input(&long_report(&press));
        poll(&mut state);
    }
    let clicks = of_kind(&take_events(), INPUT_KIND_BUTTON_DOWN);
    assert_eq!(clicks.len(), 1, "a press past byte 16 was taken for a repeat of the rest");
}
