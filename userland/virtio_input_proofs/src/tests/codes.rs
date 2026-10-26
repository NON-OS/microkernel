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

//! The virtio keyboard against the PS/2 driver's own tables: Linux key codes
//! 1 to 88 are scan set 1's make codes, and the higher ones are its E0 keys.

use crate::keyboard::codes::{is_keypad, keycode_for};
use crate::ps2::{set1, set1_e0};

#[test]
fn the_first_88_codes_post_what_ps2_posts() {
    // But 84, which Linux leaves unassigned: scan code 54 is SysRq, which
    // Linux sends as KEY_SYSRQ, 99, and the extended test holds.
    for code in (0..=0x58u16).filter(|&c| c != 84) {
        assert_eq!(keycode_for(code), set1::keycode_for(code as u8), "key {code}");
    }
    assert_eq!(keycode_for(84), None);
}

// Linux atkbd's mapping of the E0-prefixed scan codes.
#[test]
fn the_extended_codes_post_what_their_e0_scan_codes_post() {
    let pairs: [(u16, u8); 22] = [
        (96, 0x1C),
        (97, 0x1D),
        (98, 0x35),
        (99, 0x37),
        (100, 0x38),
        (102, 0x47),
        (103, 0x48),
        (104, 0x49),
        (105, 0x4B),
        (106, 0x4D),
        (107, 0x4F),
        (108, 0x50),
        (109, 0x51),
        (110, 0x52),
        (111, 0x53),
        (113, 0x20),
        (114, 0x2E),
        (115, 0x30),
        (116, 0x5E),
        (119, 0x46),
        (125, 0x5B),
        (126, 0x5C),
    ];
    for (linux, scan) in pairs {
        assert_eq!(keycode_for(linux), set1_e0::keycode_for(scan), "key {linux}");
        assert!(set1_e0::keycode_for(scan).is_some());
    }
    for code in [89u16, 95, 101, 112, 117, 124, 127, 255] {
        assert_eq!(keycode_for(code), None, "key {code}");
    }
}

#[test]
fn the_keypad_is_71_to_83_and_slash() {
    let pad: Vec<u16> = (0..=255u16).filter(|&c| is_keypad(c)).collect();
    assert_eq!(pad, [(71..=83).collect::<Vec<u16>>(), vec![98]].concat());
}
