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

use super::narrow::{u16_arg, u32_arg, u8_arg};

const EDGES: [u64; 9] = [
    0,
    1,
    0xFFFF,
    0xFFFF_FFFF,
    0x1_0000_0000,
    0x1_0000_0001,
    0x1_0000_FFFF_0005,
    0x8000_0000_0000_0000,
    u64::MAX,
];

#[test]
fn every_value_a_field_holds_is_kept_whole() {
    for raw in EDGES.into_iter().chain([0xFF, 0x100, 0x1_0000, 0x1_0100]) {
        match u32_arg(raw) {
            Some(v) => assert_eq!(u64::from(v), raw, "{raw:#x} came back as {v:#x}"),
            None => assert!(raw > u64::from(u32::MAX), "{raw:#x} fits and was refused"),
        }
        match u16_arg(raw) {
            Some(v) => assert_eq!(u64::from(v), raw, "{raw:#x} came back as {v:#x}"),
            None => assert!(raw > u64::from(u16::MAX), "{raw:#x} fits and was refused"),
        }
        match u8_arg(raw) {
            Some(v) => assert_eq!(u64::from(v), raw, "{raw:#x} came back as {v:#x}"),
            None => assert!(raw > u64::from(u8::MAX), "{raw:#x} fits and was refused"),
        }
    }
}

#[test]
fn a_value_wider_than_the_field_is_refused() {
    assert_eq!(u32_arg(u64::from(u32::MAX)), Some(u32::MAX));
    for raw in [0x1_0000_0000, 0x1_0000_0001, 0x1_0000_FFFF_0005, u64::MAX] {
        assert_eq!(u32_arg(raw), None, "{raw:#x} was cut down");
    }
    // A config write of 0x1_0006 is not a write of 6, nor BAR 256 BAR 0.
    assert_eq!(u16_arg(0xFFFF), Some(0xFFFF));
    assert_eq!(u16_arg(0x1_0006), None);
    assert_eq!(u8_arg(0xFF), Some(0xFF));
    assert_eq!(u8_arg(0x100), None);
}
