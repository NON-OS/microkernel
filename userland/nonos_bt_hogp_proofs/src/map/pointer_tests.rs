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

//! Pointer reports into buttons, motion and wheel (HID 1.11, 6.2.2.5),
//! each axis clamped to the boot report's signed octet.

use super::mice::{absolute, mouse, wide, MOUSE, MOUSE_END};
use nonos_bt_hogp::report::map::{parse, pointer, points, Pointer};
use nonos_bt_hogp::report::{Input::*, Mouse};

fn p(buttons: u8, dx: i16, dy: i16, wheel: i16) -> Option<Pointer> {
    Some(Pointer { buttons, dx, dy, wheel })
}

#[test]
fn buttons_motion_and_wheel_of_an_octet_mouse() {
    let l = parse(&mouse()).unwrap();
    assert!(points(&l, 3));
    assert_eq!(pointer(&l, 3, &[0x05, 5, 0xfb, 0x01]), p(0b101, 5, -5, 1));
    assert_eq!(pointer(&l, 3, &[0x18, 0, 0, 0]), p(0, 0, 0, 0), "buttons 4 and 5 are not read");
    assert_eq!(pointer(&l, 3, &[0x00, 0x80, 0, 0]), p(0, 0, 0, 0), "-128 is outside -127");
    assert_eq!(pointer(&l, 3, &[0x01, 1, 1]), None, "short");
    assert_eq!(pointer(&l, 3, &[0x01, 1, 1, 0, 9]), p(1, 1, 1, 0), "long");
}

#[test]
fn sixteen_bit_axes_are_clamped_to_an_octet() {
    let l = parse(&wide()).unwrap();
    assert!(points(&l, 0));
    let [x0, x1] = 1000i16.to_le_bytes();
    let [y0, y1] = (-1000i16).to_le_bytes();
    assert_eq!(pointer(&l, 0, &[0x02, x0, x1, y0, y1, 0x81]), p(0b10, 127, -128, -127));
    assert_eq!(pointer(&l, 0, &[0, 0x00, 0x80, 5, 0, 0]), p(0, 0, 5, 0), "-32768 is outside");
    assert_eq!(pointer(&l, 0, &[0, 0xff, 0x7f, 0x01, 0x80, 0]), p(0, 127, -128, 0));
}

#[test]
fn only_relative_x_and_y_make_a_pointer() {
    assert!(!points(&parse(&absolute()).unwrap(), 3), "a tablet's absolute axes");
    // X alone: the Usage Y and Wheel items dropped, two axes padding.
    let x_only = [&MOUSE[..38], &MOUSE[42..], &[0x81, 0x06], &MOUSE_END[2..]].concat();
    assert!(!points(&parse(&x_only).unwrap(), 3));
    assert!(!points(&parse(&mouse()).unwrap(), 1), "a report the map has not");
}

#[test]
fn a_pointer_moves_then_scrolls_then_presses() {
    let mut m = Mouse::default();
    assert_eq!(
        m.moved(0b11, 3, -2, -1),
        [Motion { dx: 3, dy: -2 }, Wheel(-1), ButtonDown(1), ButtonDown(2)]
    );
    assert_eq!(m.moved(0b01, 0, 0, 0), [ButtonUp(2)]);
    assert_eq!(m.moved(0b1001, 0, 0, 2), [Wheel(2)], "bit 3 is no boot button");
    assert_eq!(m.release_all(), [ButtonUp(1)]);
}
