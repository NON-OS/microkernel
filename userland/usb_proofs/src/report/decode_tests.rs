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

//! Reports read through their layouts: a report protocol keyboard as the
//! boot report the keyboard state takes, a report ID mouse as relative
//! motion, a touchscreen as a position only while a finger is on it.

use super::layout;
use super::vectors::KEYBOARD;
use super::vectors_more::{ID_MOUSE, TOUCHSCREEN};
use crate::hid::keyboard::BootReport;
use crate::hid::mouse_event::MouseEvent;
use crate::hid::report::keys::keys_report;
use crate::hid::report::pointer::{pointer_report, Motion};
use crate::hid::tablet::TabletReport;

#[test]
fn a_report_protocol_keyboard_reads_as_its_boot_report() {
    let k = layout(KEYBOARD).keys.expect("keys");
    let raw = [0x22, 0xff, 0x04, 0x05, 0x01, 0x65, 0x66, 0xff];
    // 0x66 and 0xff lie past the logical maximum 0x65: no key.
    let want = BootReport { modifiers: 0x22, keys: [0x04, 0x05, 0x01, 0x65, 0, 0] };
    assert_eq!(keys_report(&k, &raw), Some(want));
    for len in 0..8 {
        assert_eq!(keys_report(&k, &raw[..len]), None, "{len} bytes is no report");
    }
}

fn motion(desc: &[u8], body: &[u8]) -> Option<Motion> {
    pointer_report(&layout(desc).pointer?, body)
}

#[test]
fn a_report_id_mouse_reads_twelve_bit_signed_motion() {
    // Buttons 1 and 5; X -1 (0xFFF), Y +2047 (0x7FF); wheel -2.
    let body = [0x11, 0xff, 0xff, 0x7f, 0xfe];
    let Some(Motion::Relative(ev)) = motion(ID_MOUSE, &body) else { panic!("relative") };
    assert_eq!(ev, MouseEvent { dx: -1, dy: 2047, dz: -2, buttons: 0x11 });
    assert!(motion(ID_MOUSE, &body[..4]).is_none(), "a short report is no report");
}

#[test]
fn a_touchscreen_has_a_position_only_while_touched_and_its_tip_is_button_one() {
    let at = |bits: u8| match motion(TOUCHSCREEN, &[bits, 0xff, 0x07, 0x00, 0x08]) {
        Some(Motion::Absolute(t)) => t,
        _ => panic!("absolute"),
    };
    // X 0x7FF and Y 0x800 of 0 to 0xFFF, scaled onto 0 to 0x7FFF.
    let (x, y) = (0x7ff * 0x7fff / 0xfff, 0x800 * 0x7fff / 0xfff);
    let down = TabletReport { buttons: 1, x, y, wheel: 0, at: true };
    assert_eq!(at(0b11), down, "tip and in range");
    assert_eq!(at(0b10), TabletReport { buttons: 0, ..down }, "hovering in range");
    assert_eq!(at(0b00), TabletReport { buttons: 0, at: false, ..down }, "lifted");
}
