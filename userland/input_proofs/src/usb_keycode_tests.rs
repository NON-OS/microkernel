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

//! A USB keyboard's navigation, function and lock keys post the PS/2 table's
//! codes, the ones app_skeleton's KEY_* constants name and every app reads.

use crate::ps2_keycodes::*;
use crate::usb_usage_keycode::usage_keycode;

#[test]
fn the_navigation_keys_post_the_codes_apps_read() {
    // The driver posted 0xE000 to 0xE008 for these, which no app reads.
    let pairs = [
        (0x52, KEYCODE_UP),
        (0x51, KEYCODE_DOWN),
        (0x50, KEYCODE_LEFT),
        (0x4f, KEYCODE_RIGHT),
        (0x4a, KEYCODE_HOME),
        (0x4d, KEYCODE_END),
        (0x4b, KEYCODE_PGUP),
        (0x4e, KEYCODE_PGDN),
        (0x49, KEYCODE_INS),
        (0x4c, KEYCODE_DEL),
    ];
    for (usage, code) in pairs {
        assert_eq!(usage_keycode(usage), Some(code), "usage {usage:#x}");
    }
}

#[test]
fn the_function_and_lock_keys_post_the_codes_apps_read() {
    let f = [
        KEYCODE_F1,
        KEYCODE_F2,
        KEYCODE_F3,
        KEYCODE_F4,
        KEYCODE_F5,
        KEYCODE_F6,
        KEYCODE_F7,
        KEYCODE_F8,
        KEYCODE_F9,
        KEYCODE_F10,
        KEYCODE_F11,
        KEYCODE_F12,
    ];
    for (i, code) in f.into_iter().enumerate() {
        assert_eq!(usage_keycode(0x3a + i as u8), Some(code), "F{}", i + 1);
    }
    assert_eq!(usage_keycode(0x39), Some(KEYCODE_CAPS));
    assert_eq!(usage_keycode(0x53), Some(KEYCODE_NUMLK));
    assert_eq!(usage_keycode(0x47), Some(KEYCODE_SCROLL));
}

#[test]
fn the_keypad_posts_its_own_characters() {
    let keypad = b"/*-+";
    for (i, &c) in keypad.iter().enumerate() {
        assert_eq!(usage_keycode(0x54 + i as u8), Some(u32::from(c)));
    }
    assert_eq!(usage_keycode(0x58), Some(0x0D));
    for (i, c) in b"1234567890.".iter().enumerate() {
        assert_eq!(usage_keycode(0x59 + i as u8), Some(u32::from(*c)));
    }
}

#[test]
fn character_keys_are_left_to_the_layout() {
    // Letters, digits, symbols, Enter, Escape, Backspace, Tab, Space and the
    // ISO key resolve through the layout.
    for usage in (0x04..=0x38).chain([0x64]) {
        assert_eq!(usage_keycode(usage), None, "usage {usage:#x}");
    }
}

#[test]
fn the_volume_and_power_usages_post_the_system_key_codes() {
    // The keyboard page's own Mute, Volume Up, Volume Down and Power, which
    // the router hands to the shell (route/shell_keys.rs).
    assert_eq!(usage_keycode(0x7f), Some(KEYCODE_MUTE));
    assert_eq!(usage_keycode(0x80), Some(KEYCODE_VOLUME_UP));
    assert_eq!(usage_keycode(0x81), Some(KEYCODE_VOLUME_DOWN));
    assert_eq!(usage_keycode(0x66), Some(KEYCODE_POWER));
    // Their neighbours (Keypad =, F13 and up, Execute to Find, Locking Caps
    // Lock) post nothing of theirs.
    for usage in (0x67..=0x7e).chain([0x65, 0x82]) {
        assert_eq!(usage_keycode(usage), None, "usage {usage:#x}");
    }
}
