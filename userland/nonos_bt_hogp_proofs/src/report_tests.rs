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

//! Boot reports (HID 1.11, B.1 and B.2) into events.

use nonos_bt_hogp::report::{Input, Keyboard, Mouse};
use Input::*;

#[test]
fn keys_and_modifiers_go_down_and_up_releases_first() {
    let mut k = Keyboard::default();
    assert_eq!(k.report(&[0x02, 0, 0x04, 0, 0, 0, 0, 0]), [KeyDown(0xe1), KeyDown(0x04)]);
    assert_eq!(k.report(&[0x02, 0, 0x05, 0x04, 0, 0, 0, 0]), [KeyDown(0x05)]);
    assert_eq!(k.report(&[0x00, 0, 0x05, 0, 0, 0, 0, 0]), [KeyUp(0xe1), KeyUp(0x04)]);
    assert_eq!(k.release_all(), [KeyUp(0x05)]);
    assert_eq!(k.release_all(), []);
}

#[test]
fn rollover_and_wrong_lengths_change_nothing() {
    let mut k = Keyboard::default();
    k.report(&[0, 0, 0x04, 0, 0, 0, 0, 0]);
    assert_eq!(k.report(&[0, 0, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]), []);
    assert_eq!(k.report(&[0, 0, 0, 0, 0, 0, 0]), []);
    assert_eq!(k.report(&[0; 9]), []);
    // The key is still held: only a real report releases it.
    assert_eq!(k.report(&[0; 8]), [KeyUp(0x04)]);
}

#[test]
fn mouse_motion_and_three_buttons() {
    let mut m = Mouse::default();
    assert_eq!(
        m.report(&[0x05, 0x80, 0x7f]),
        [Motion { dx: -128, dy: 127 }, ButtonDown(1), ButtonDown(3)]
    );
    // Bits past the third button, and octets past Y, are the device's own.
    assert_eq!(m.report(&[0xf9, 0, 0, 0x7f]), [ButtonUp(3)]);
    assert_eq!(m.report(&[0x01, 0]), []);
    assert_eq!(m.release_all(), [ButtonUp(1)]);
}
