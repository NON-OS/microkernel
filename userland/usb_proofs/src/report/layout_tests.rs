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

//! What real descriptors are walked into: QEMU's tablet, a report protocol
//! keyboard, and a gamepad and a UPS, which name nothing the driver reads.

use super::layout;
use super::vectors::{GAMEPAD, KEYBOARD, QEMU_TABLET};
use super::vectors_more::UPS;
use crate::descriptors::report::NONE;

#[test]
fn qemus_tablet_is_an_absolute_pointer_with_three_buttons_and_a_wheel() {
    let l = layout(QEMU_TABLET);
    let p = l.pointer.expect("a pointer");
    assert_eq!((l.ids, l.longest, p.buttons), (false, 6, [0, 1, 2, NONE, NONE]));
    let (x, y, w) = (p.x.unwrap(), p.y.unwrap(), p.wheel.unwrap());
    assert_eq!((x.at, x.size, x.min, x.max, x.relative), (8, 16, 0, 0x7fff, false));
    assert_eq!((y.at, y.size, y.min, y.max), (24, 16, 0, 0x7fff));
    assert_eq!((w.at, w.size, w.min, w.max, w.relative), (40, 8, -127, 127, true));
    assert!(l.keys.is_none() && l.media == [None, None]);
}

#[test]
fn a_gamepad_and_a_ups_name_nothing_the_driver_reads() {
    for (name, desc, longest) in [("gamepad", GAMEPAD, 8), ("ups", UPS, 2)] {
        let l = layout(desc);
        assert_eq!((l.keys, l.media, l.pointer), (None, [None, None], None), "{name}");
        assert_eq!(l.longest, longest, "{name}");
    }
}

#[test]
fn a_report_protocol_keyboard_has_its_modifiers_and_six_slots() {
    let l = layout(KEYBOARD);
    let k = l.keys.expect("keys");
    assert_eq!(k.mods, [0, 1, 2, 3, 4, 5, 6, 7]);
    let s = k.slots.expect("slots");
    assert_eq!((s.at, s.size, s.count, s.min, s.max, k.first), (16, 8, 6, 0, 0x65, 0));
    assert_eq!((l.ids, l.longest), (false, 8), "the LED outputs are no input");
}
