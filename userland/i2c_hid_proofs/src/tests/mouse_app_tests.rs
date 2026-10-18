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

//! What the mouse layout takes: relative X and Y of a Mouse or Pointer
//! application, and nothing else. A touch screen's absolute "mouse"
//! collection is no mouse (the screen is driven by its own digitizer), nor
//! are relative axes in a vendor application.

use nonos_i2cmodel::touchpad::report_descriptor;
use nonos_i2cmodel::{Command, HidOverI2c};

use super::descs::MOUSE;
use super::fixture::{rig_with, HID_DESC_REG, PAD};
use crate::hid::{parse_mouse_layout, Surface};
use crate::setup;

/// The mouse collection touch screens carry for legacy hosts: Generic
/// Desktop Mouse, two buttons, absolute X and Y.
const ABS_MOUSE: &[u8] = &[
    0x05, 0x01, 0x09, 0x02, 0xA1, 0x01, 0x85, 0x07, 0x09, 0x01, 0xA1, 0x00, 0x05, 0x09, 0x19, 0x01,
    0x29, 0x02, 0x15, 0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x02, 0x81, 0x02, 0x95, 0x06, 0x81, 0x03,
    0x05, 0x01, 0x09, 0x30, 0x09, 0x31, 0x15, 0x00, 0x26, 0xFF, 0x0F, 0x75, 0x10, 0x95, 0x02, 0x81,
    0x02, 0xC0, 0xC0,
];

#[test]
fn a_touch_screen_with_an_absolute_mouse_collection_is_driven_as_a_screen_alone() {
    let mut ts = report_descriptor();
    ts[3] = 0x04; // Usage (Touch Screen)
    let desc = [ts, ABS_MOUSE.to_vec()].concat();
    assert!(!parse_mouse_layout(&desc).is_relative_mouse(), "absolute axes taken as a mouse");
    let r = rig_with(HidOverI2c::new(PAD, desc, 2 + 8, 0x04F3, 0x0003), Some((PAD, HID_DESC_REG)));
    let state = setup::run().expect("setup");
    assert!(!state.mouse_layout.is_relative_mouse(), "absolute axes driven as a mouse");
    assert!(state.found() && state.mt.surface == Surface::Screen, "driven, as a screen");
    assert_ne!(r.pad.lock().take_commands().last(), Some(&Command::SetPower { sleep: true }));
}

#[test]
fn relative_axes_outside_a_mouse_or_pointer_application_are_no_mouse() {
    // The mouse collection's items under Usage Page Vendor, Usage 1.
    let mut vendor = MOUSE.to_vec();
    vendor.splice(0..4, [0x06, 0x00, 0xFF, 0x09, 0x01]);
    assert!(parse_mouse_layout(MOUSE).is_relative_mouse(), "the control");
    assert!(!parse_mouse_layout(&vendor).is_relative_mouse(), "a vendor's axes taken as a mouse");
}
