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

//! Input reports into the keys held (HID 1.11, 6.2.2.5), by the keyboard
//! map: modifiers by bit, keys by array index, and Consumer controls
//! mapped onto the Keyboard page.

use super::maps::{bits_field, KEYBOARD};
use nonos_bt_hogp::report::map::{held, parse, HELD_MAX};

#[test]
fn modifiers_and_keys_in_report_order() {
    let l = parse(&KEYBOARD).unwrap();
    assert_eq!(held(&l, 1, &[0x22, 0, 0x04, 0x05, 0, 0, 0, 0]), Some(vec![0xe1, 0xe5, 0x04, 0x05]));
    assert_eq!(held(&l, 1, &[0; 8]), Some(vec![]));
    // Past the logical maximum (0x65) an element holds no usage.
    assert_eq!(held(&l, 1, &[0, 0, 0x66, 0xff, 0x04, 0, 0, 0]), Some(vec![0x04]));
    assert_eq!(held(&l, 1, &[0, 0, 0x04, 0x04, 0, 0, 0, 0]), Some(vec![0x04]), "once each");
}

#[test]
fn a_report_that_cannot_say_changes_nothing() {
    let l = parse(&KEYBOARD).unwrap();
    assert_eq!(held(&l, 1, &[0, 0, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]), None, "rollover");
    assert_eq!(held(&l, 1, &[0, 0, 0x04, 0x02, 0, 0, 0, 0]), None, "POSTFail");
    assert_eq!(held(&l, 1, &[0; 7]), None, "short");
    assert_eq!(held(&l, 1, &[0, 0, 0x04, 0, 0, 0, 0, 0, 0x05]), Some(vec![0x04]), "long");
    assert_eq!(held(&l, 3, &[0; 8]), None, "a report the map has not");
}

#[test]
fn volume_and_mute_reach_the_keyboard_page() {
    let l = parse(&KEYBOARD).unwrap();
    assert_eq!(held(&l, 2, &[0xe9, 0x00]), Some(vec![0x80]));
    assert_eq!(held(&l, 2, &[0xea, 0x00]), Some(vec![0x81]));
    assert_eq!(held(&l, 2, &[0xe2, 0x00]), Some(vec![0x7f]));
    assert_eq!(held(&l, 2, &[0xcd, 0x00]), Some(vec![]), "Play/Pause has no key");
}

#[test]
fn a_bitmap_keyboard_and_its_bound() {
    // One bit per key from 0x04: an n-key rollover layout.
    let l = parse(&bits_field(0, 0x04, 0x60)).unwrap();
    let mut r = [0u8; 12];
    r[0] = 0b0000_0011;
    r[11] = 0x80;
    assert_eq!(held(&l, 0, &r), Some(vec![0x04, 0x05, 0x63]));
    let all = [0xffu8; 12];
    assert_eq!(held(&l, 0, &all[..]).map(|k| k.len()), None, "more than {HELD_MAX} keys");
    r[11] = 0;
    r[..4].fill(0xff);
    assert_eq!(held(&l, 0, &r).map(|k| k.len()), Some(HELD_MAX));
    r[4] = 1;
    assert_eq!(held(&l, 0, &r), None, "one past the bound");
}
