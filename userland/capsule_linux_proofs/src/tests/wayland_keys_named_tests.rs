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

//! The keys above the ASCII range, what sends nothing, and that no two
//! keys collide.

use crate::evdev::evdev_of;
use std::collections::BTreeMap;

#[test]
fn named_keys_follow_the_drivers_table() {
    let named = [
        (0x1003, 42),
        (0x1004, 54),
        (0x1001, 29),
        (0x1005, 56),
        (0x1009, 58),
        (0x1101, 59),
        (0x110A, 68),
        (0x110B, 87),
        (0x110C, 88),
        (0x1201, 103),
        (0x1202, 108),
        (0x1203, 105),
        (0x1204, 106),
        (0x120A, 111),
    ];
    named.iter().for_each(|(code, key)| assert_eq!(evdev_of(*code), Some(*key), "{code:#x}"));
}

#[test]
fn what_no_us_key_gives_sends_nothing() {
    for code in [0, 0x01, 0x7F, 0xE9, 0x100C, 0x110D, 0x1301, 0x1304, u32::MAX] {
        assert_eq!(evdev_of(code), None, "{code:#x}");
    }
}

#[test]
fn no_two_keys_collide_but_a_characters_two_cases() {
    let mut by_key: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for code in (0..0x80).chain(0x1000..0x1400) {
        if let Some(key) = evdev_of(code) {
            by_key.entry(key).or_default().push(code);
        }
    }
    for (key, codes) in by_key {
        assert!(codes.len() <= 2, "evdev {key}: {codes:x?}");
    }
}
