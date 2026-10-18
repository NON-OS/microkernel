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

//! Axes that are not a relative pointer's: an absolute wheel beside
//! relative X and Y, and a lone relative X in a keyboard's report. Neither
//! moves anything.

use super::maps::{bits_field, global, local, main};
use crate::sim::report_devices::{ready, report_keyboard_with, KEYS_IN};
use crate::sim::Run;
use nonos_bt_hogp::report::map::{parse, pointer, points, Pointer};
use nonos_bt_hogp::report::Input::*;

/// Signed octets from −127 to 127, `count` of them.
fn octets(count: u8) -> Vec<u8> {
    [global(1, 0x81), global(2, 0x7f), global(7, 8), global(9, count)].concat()
}

#[test]
fn an_absolute_wheel_beside_relative_axes_is_not_read() {
    let buttons = [global(0, 9), local(1, 1), local(2, 3), global(1, 0), global(2, 1)];
    let buttons = [buttons.concat(), global(7, 1), global(9, 3), main(8, 2)].concat();
    let pad = [global(9, 1), global(7, 5), main(8, 1)].concat();
    let axes = [global(0, 1), local(0, 0x30), local(0, 0x31), octets(2), main(8, 6)].concat();
    let wheel = [local(0, 0x38), octets(1), main(8, 2)].concat();
    let l = parse(&[buttons, pad, axes, wheel].concat()).unwrap();
    assert!(points(&l, 0));
    let p = pointer(&l, 0, &[0x01, 2, 3, 4]);
    assert_eq!(p, Some(Pointer { buttons: 1, dx: 2, dy: 3, wheel: 0 }));
}

#[test]
fn a_lone_relative_x_in_a_keyboard_report_is_not_a_pointer() {
    let x = [global(0, 1), local(0, 0x30), octets(1), main(8, 6)].concat();
    let map = [bits_field(1, 0xe0, 8), x].concat();
    let mut run = Run::new(report_keyboard_with(&map, [&[1, 1], &[9, 1], &[1, 2]]));
    ready(&mut run);
    assert_eq!(run.notify(KEYS_IN, &[0x01, 0x05]), [KeyDown(0xe0)]);
}
