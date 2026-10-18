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

//! Report descriptors of HID-over-I2C devices other than the bench pad.

use nonos_i2cmodel::touchpad::report_descriptor;

/// A boot-shaped keyboard, report id 1: eight modifier bits, a reserved
/// byte, six key slots.
pub const KEYBOARD: &[u8] = &[
    0x05, 0x01, 0x09, 0x06, 0xA1, 0x01, 0x85, 0x01, // Generic Desktop Keyboard, id 1
    0x05, 0x07, 0x19, 0xE0, 0x29, 0xE7, 0x15, 0x00, 0x25, 0x01, // modifiers
    0x75, 0x01, 0x95, 0x08, 0x81, 0x02, // 8 bits, variable
    0x95, 0x01, 0x75, 0x08, 0x81, 0x03, // reserved byte
    0x95, 0x06, 0x75, 0x08, 0x25, 0x65, 0x19, 0x00, 0x29, 0x65, 0x81, 0x00, // keys
    0xC0,
];

/// The report a keyboard sends for Left Ctrl held with the A key.
pub const CTRL_A: [u8; 9] = [1, 0x01, 0, 0x04, 0, 0, 0, 0, 0];

/// The mouse collection a precision touchpad also declares, report id 3:
/// two buttons, six bits of padding, then relative X and Y of a byte each.
pub const MOUSE: &[u8] = &[
    0x05, 0x01, 0x09, 0x02, 0xA1, 0x01, 0x85, 0x03, 0x09, 0x01, 0xA1, 0x00, 0x05, 0x09, 0x19, 0x01,
    0x29, 0x02, 0x15, 0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x02, 0x81, 0x02, 0x95, 0x06, 0x81, 0x03,
    0x05, 0x01, 0x09, 0x30, 0x09, 0x31, 0x15, 0x81, 0x25, 0x7F, 0x75, 0x08, 0x95, 0x02, 0x81, 0x06,
    0xC0, 0xC0,
];

/// The bench pad's descriptor with the mouse collection ahead of it, as
/// precision touchpads ship.
pub fn pad_with_mouse() -> Vec<u8> {
    [MOUSE, &report_descriptor()].concat()
}

/// A touch pad application with one report, id 1, whose items are `body`.
pub fn touch_pad(body: &[&[u8]]) -> Vec<u8> {
    let head: &[u8] = &[0x05, 0x0D, 0x09, 0x05, 0xA1, 0x01, 0x85, 0x01];
    [&[head], body, &[&[0xC0][..]]].concat().concat()
}

/// A tip switch at bit 0 and seven bits of padding, on the Digitizer page.
pub const TIP: &[u8] = &[
    0x05, 0x0D, 0x09, 0x42, 0x15, 0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x01, 0x81, 0x02, 0x95, 0x07,
    0x81, 0x03,
];
