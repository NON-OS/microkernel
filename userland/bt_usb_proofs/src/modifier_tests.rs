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

//! The modifier usages and the layout chord.

use crate::modifier::{layout_chord, modifier_bit};

#[test]
fn modifier_usages_map_to_the_boot_report_bits() {
    let bits: Vec<_> = (0xe0..=0xe7).map(modifier_bit).collect();
    let want: Vec<_> = (0..8).map(|i| Some(1u8 << i)).collect();
    assert_eq!(bits, want);
    for usage in [0x00, 0x04, 0x39, 0xdf, 0xe8, 0xff] {
        assert_eq!(modifier_bit(usage), None, "usage {usage:#x}");
    }
}

#[test]
fn the_chord_is_control_and_left_alt_with_space() {
    assert!(layout_chord(0x01 | 0x04, 0x2c));
    assert!(layout_chord(0x10 | 0x04, 0x2c));
    assert!(!layout_chord(0x01 | 0x40, 0x2c), "right Alt is AltGr");
    assert!(!layout_chord(0x01 | 0x04, 0x04));
    assert!(!layout_chord(0x04, 0x2c));
}
