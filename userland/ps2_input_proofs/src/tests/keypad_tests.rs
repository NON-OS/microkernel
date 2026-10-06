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

//! The numeric keypad on every layout: what is printed on the key, never
//! what the main-row key with the same ASCII code types there.

use nonos_keymap::Layout;

use crate::keymap::keypad::character;
use crate::keymap::translate::translate;
use crate::ring::FLAG_E0_PREFIX;

/// Each keypad key with the character it prints: scan code, E0 prefix.
const KEYPAD: [(u8, bool, u8); 14] = [
    (0x47, false, b'7'),
    (0x48, false, b'8'),
    (0x49, false, b'9'),
    (0x4A, false, b'-'),
    (0x4B, false, b'4'),
    (0x4C, false, b'5'),
    (0x4D, false, b'6'),
    (0x4E, false, b'+'),
    (0x4F, false, b'1'),
    (0x50, false, b'2'),
    (0x51, false, b'3'),
    (0x52, false, b'0'),
    (0x53, false, b'.'),
    (0x35, true, b'/'),
];

fn typed(scan: u8, e0: bool, shift: bool, layout: Layout) -> u32 {
    let flags = if e0 { FLAG_E0_PREFIX } else { 0 };
    let t = translate(scan, flags).expect("a keypad key");
    character(&t, shift, false, false, layout)
}

#[test]
fn every_keypad_key_types_its_own_character_on_every_layout() {
    for i in 0..Layout::COUNT {
        let layout = Layout::from_index(i);
        for (scan, e0, want) in KEYPAD {
            assert_eq!(typed(scan, e0, false, layout), want as u32, "{layout:?} {scan:02X}");
            assert_eq!(typed(scan, e0, true, layout), want as u32, "{layout:?} shift {scan:02X}");
        }
    }
}

#[test]
fn the_main_row_still_goes_through_the_layout() {
    // Main-row 1 (scan 02) is '&' on French AZERTY, slash (E0-less 35) is
    // '-' on German: the keypad change leaves them as the layouts say.
    assert_eq!(typed(0x02, false, false, Layout::Fr), '&' as u32);
    assert_eq!(typed(0x35, false, false, Layout::De), '-' as u32);
}
