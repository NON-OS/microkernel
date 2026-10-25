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

//! SLP_EN, the one bit of PM1 control the ACPI capsule may not set: where it
//! lands, and every write of every width that would set it, against an oracle
//! that looks at single bits.

use super::rule::{sets_sleep_enable, sleep_enable, SleepEnable, PM1_SLP_EN, SPACE_IO};
use super::rule::{SLEEP_CONTROL_SLP_EN, SPACE_MEMORY};

#[test]
fn slp_en_lands_where_the_gas_and_the_bit_put_it() {
    let pm1 = sleep_enable(SPACE_IO, 0x604, 0, PM1_SLP_EN);
    assert_eq!(pm1, Some(SleepEnable { port: 0x605, bit: 5 }), "q35's PM1a_CNT");
    let hw = sleep_enable(SPACE_IO, 0x620, 0, SLEEP_CONTROL_SLP_EN);
    assert_eq!(hw, Some(SleepEnable { port: 0x620, bit: 5 }), "SLEEP_CONTROL");
    let shifted = sleep_enable(SPACE_IO, 0x604, 4, PM1_SLP_EN);
    assert_eq!(shifted, Some(SleepEnable { port: 0x606, bit: 1 }), "a bit offset");
    assert_eq!(sleep_enable(SPACE_MEMORY, 0x604, 0, PM1_SLP_EN), None, "memory is refused whole");
    assert_eq!(sleep_enable(SPACE_IO, 0, 0, PM1_SLP_EN), None, "absent");
}

#[test]
fn what_the_acpi_capsule_writes_goes_ahead() {
    let e = [SleepEnable { port: 0x605, bit: 5 }];
    assert!(!sets_sleep_enable(e.iter(), 0x604, 2, 1 << 2), "GBL_RLS");
    assert!(!sets_sleep_enable(e.iter(), 0x604, 2, 0x1C00 | 1), "SLP_TYP and SCI_EN");
    assert!(!sets_sleep_enable(e.iter(), 0x605, 1, 0xDF), "every bit but SLP_EN");
    assert!(!sets_sleep_enable(e.iter(), 0x604, 1, 0xFF), "a byte short of it");
}

#[test]
fn a_write_setting_slp_en_is_refused_at_every_width_and_offset() {
    let e = [SleepEnable { port: 0x605, bit: 5 }];
    let target = 0x605 * 8 + 5;
    for at in 0x600u64..0x60A {
        for width in [1u8, 2, 3, 4] {
            let bytes = if width == 3 { 4 } else { u64::from(width) };
            for bit in 0..32u64 {
                let lands = bit < bytes * 8 && at * 8 + bit == target;
                let value = 1u32 << bit;
                let got = sets_sleep_enable(e.iter(), at, width, value);
                assert_eq!(got, lands, "{value:#x} at {at:#x} width {width}");
                assert_eq!(
                    sets_sleep_enable(e.iter(), at, width, !0),
                    at <= 0x605 && 0x605 < at + bytes
                );
            }
        }
    }
}

#[test]
fn either_block_setting_slp_en_is_refused() {
    let e = [SleepEnable { port: 0x605, bit: 5 }, SleepEnable { port: 0x641, bit: 5 }];
    assert!(sets_sleep_enable(e.iter(), 0x604, 2, 1 << 13), "PM1a alone");
    assert!(sets_sleep_enable(e.iter(), 0x640, 2, 1 << 13), "PM1b alone");
    assert!(!sets_sleep_enable(e.iter(), 0x640, 2, 1 << 12), "PM1b's SLP_TYP");
}
