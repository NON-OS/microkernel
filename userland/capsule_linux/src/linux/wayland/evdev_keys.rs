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

//! The keys above the ASCII range, from the driver's table
//! (capsule_driver_ps2_input keymap/set1/keycodes.rs), as evdev codes.

/// NONOS code, evdev code.
const NAMED: [(u32, u32); 33] = [
    (0x1001, 29),  // left ctrl
    (0x1002, 97),  // right ctrl
    (0x1003, 42),  // left shift
    (0x1004, 54),  // right shift
    (0x1005, 56),  // left alt
    (0x1006, 100), // right alt
    (0x1007, 125), // left meta
    (0x1008, 126), // right meta
    (0x1009, 58),  // caps lock
    (0x100A, 69),  // num lock
    (0x100B, 70),  // scroll lock
    (0x1101, 59),
    (0x1102, 60),
    (0x1103, 61),
    (0x1104, 62),
    (0x1105, 63),
    (0x1106, 64),
    (0x1107, 65),
    (0x1108, 66),
    (0x1109, 67),
    (0x110A, 68),  // F10
    (0x110B, 87),  // F11
    (0x110C, 88),  // F12
    (0x1201, 103), // up
    (0x1202, 108), // down
    (0x1203, 105), // left
    (0x1204, 106), // right
    (0x1205, 102), // home
    (0x1206, 107), // end
    (0x1207, 104), // page up
    (0x1208, 109), // page down
    (0x1209, 110), // insert
    (0x120A, 111), // delete
];

pub fn named(code: u32) -> Option<u32> {
    NAMED.iter().find(|(nonos, _)| *nonos == code).map(|(_, evdev)| *evdev)
}
