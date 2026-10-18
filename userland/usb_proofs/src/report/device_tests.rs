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

//! An interface read through its report descriptor posts what its reports
//! say: a report ID mouse its motion, wheel and buttons, QEMU's tablet its
//! position. A report under another ID touches nothing.

use super::layout;
use super::vectors::QEMU_TABLET;
use super::vectors_more::ID_MOUSE;
use crate::hid::{Device, ReportDevice};
use nonos_libc::{reset, take_posted, InputEvent};
use nonos_libc::{INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS};
use nonos_libc::{INPUT_KIND_POINTER_REL, INPUT_KIND_WHEEL};

fn device(desc: &[u8]) -> Device {
    Device::Report(Box::new(ReportDevice::new(layout(desc))))
}

fn seen() -> Vec<(u16, u32, i32, i32, i32, i32)> {
    let all: Vec<InputEvent> = take_posted();
    all.iter().map(|e| (e.kind, e.code, e.x, e.y, e.delta_x, e.delta_y)).collect()
}

#[test]
fn a_report_id_mouse_posts_its_motion_wheel_and_buttons() {
    reset();
    let mut dev = device(ID_MOUSE);
    dev.feed(&[2, 0x01, 0x05, 0x30, 0x00, 0x01]);
    dev.feed(&[1, 0x00, 0x05, 0x30, 0x00, 0x01]);
    dev.feed(&[2, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let want = vec![
        (INPUT_KIND_POINTER_REL, 0, 0, 0, 5, 3),
        (INPUT_KIND_WHEEL, 0, 0, 0, 0, 1),
        (INPUT_KIND_BUTTON_DOWN, 1, 0, 0, 0, 0),
        (INPUT_KIND_BUTTON_UP, 1, 0, 0, 0, 0),
    ];
    assert_eq!(seen(), want);
}

#[test]
fn qemus_tablet_posts_its_position_and_lets_go_when_it_leaves() {
    reset();
    let mut dev = device(QEMU_TABLET);
    dev.feed(&[0x02, 0x00, 0x40, 0xff, 0x7f, 0x00]);
    dev.release_all();
    let want = vec![
        (INPUT_KIND_POINTER_ABS, 0, 0x4000, 0x7fff, 0, 0),
        (INPUT_KIND_BUTTON_DOWN, 2, 0, 0, 0, 0),
        (INPUT_KIND_BUTTON_UP, 2, 0, 0, 0, 0),
    ];
    assert_eq!(seen(), want);
}
