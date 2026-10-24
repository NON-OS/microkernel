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

//! The boot framebuffer served on the wire: flushes land in the
//! framebuffer in its own layout, fenced; no mode setting, no cursor.

use super::serve::{ask, run, words, FENCES};
use crate::fb::Layout;
use nonos_gfx_wire::errno::{EINVAL, ENOSYS};
use nonos_gfx_wire::ops;
use std::sync::atomic::Ordering;

#[test]
fn a_blue_first_framebuffer_takes_the_pixels_as_they_are_and_is_fenced() {
    let mut mem = vec![0u32; 20 * 8];
    let (mut s, sf) = run(Layout::BlueFirst, &mut mem);
    let before = FENCES.load(Ordering::SeqCst);
    assert_eq!(ask(&mut s, ops::FLUSH, &words(&[sf.serial, 1, 2, 1, 3, 2])).0, 0);
    assert!(FENCES.load(Ordering::SeqCst) > before);
    assert_eq!(
        (mem[20 + 2], mem[2 * 20 + 4]),
        (0xFF11_2233, 0xFF11_2233),
        "stride 80 is 20 pixels"
    );
    assert_eq!((mem[20 + 1], mem[20 + 5], mem[3 * 20 + 2]), (0, 0, 0));
}

#[test]
fn a_red_first_framebuffer_gets_red_and_blue_swapped() {
    let mut mem = vec![0u32; 20 * 8];
    let (mut s, sf) = run(Layout::RedFirst, &mut mem);
    assert_eq!(ask(&mut s, ops::FLUSH, &words(&[sf.serial, 1, 0, 0, 16, 8])).0, 0);
    assert_eq!((mem[0], mem[7 * 20 + 15], mem[16]), (0xFF33_2211, 0xFF33_2211, 0));
}

#[test]
fn no_mode_setting_and_no_cursor() {
    let mut mem = vec![0u32; 20 * 8];
    let (mut s, sf) = run(Layout::BlueFirst, &mut mem);
    assert_eq!(ask(&mut s, ops::MODE_SET, &words(&[16, 8, 0])).0, ENOSYS);
    let c =
        nonos_gfx_wire::Cursor { flags: 1, x: 0, y: 0, hot_x: 0, hot_y: 0, width: 8, height: 8 };
    assert_eq!(ask(&mut s, ops::CURSOR, &c.encode()).0, ENOSYS);
    assert_eq!(ask(&mut s, ops::FLUSH, &words(&[sf.serial, 1, 15, 0, 2, 1])).0, EINVAL);
}
