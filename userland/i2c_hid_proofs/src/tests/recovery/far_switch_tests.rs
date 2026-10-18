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

//! Only the Input Mode's own report decides whether a pad is driven as the
//! mouse it reports as: a switch in a report too long to write leaves its
//! reset detection on, and a reset is still set up again.

use nonos_i2cmodel::HidOverI2c;

use crate::input::poll;
use crate::setup;
use crate::tests::bounds_tests::XY;
use crate::tests::descs::{touch_pad, MOUSE, TIP};
use crate::tests::fixture::{rig_with, HID_DESC_REG, MAX_INPUT, PAD};

/// Feature report 4, Input Mode, a byte; then feature report 5, seventy
/// bytes of padding and a Surface Switch bit past what the driver writes.
const FAR_SWITCH: &[u8] = &[
    0x85, 0x04, 0x05, 0x0D, 0x09, 0x52, 0x15, 0x00, 0x25, 0x0A, 0x75, 0x08, 0x95, 0x01, 0xB1, 0x02,
    0x85, 0x05, 0x95, 0x46, 0xB1, 0x03, 0x09, 0x57, 0x25, 0x01, 0x75, 0x01, 0x95, 0x01, 0xB1, 0x02,
    0x95, 0x07, 0xB1, 0x03,
];

#[test]
fn a_switch_too_far_to_write_leaves_reset_detection_on() {
    let desc = [MOUSE, &touch_pad(&[TIP, XY, FAR_SWITCH])].concat();
    let device = HidOverI2c::new(PAD, desc, MAX_INPUT, 0x04F3, 0x3028);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    let mut s = setup::run().expect("setup");
    assert_eq!(r.pad.lock().feature(4), Some(&[4, 3][..]), "touchpad mode was not set");
    // The pad resets: its mode reads back as the mouse's, and it reports as one.
    r.pad.lock().set_feature(4, &[4, 0]);
    r.pad.lock().push_input(&[3, 0, 5, 0]);
    poll(&mut s);
    assert_eq!(r.pad.lock().feature(4), Some(&[4, 3][..]), "the reset was not set up again");
}
