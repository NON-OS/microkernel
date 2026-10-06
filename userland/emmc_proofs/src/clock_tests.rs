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

//! The SD clock divider: identification at 400 kHz or less and High Speed
//! at 52 MHz or less, from common base clocks, under version 3 and version
//! 2 rules, and never above the target.

use crate::emmc::sdhci::caps::Caps;
use crate::emmc::sdhci::clock::*;

fn v3(base_mhz: u32, target: u32) -> Divider {
    divider(base_mhz * 1_000_000, target, 2)
}

fn v2(base_mhz: u32, target: u32) -> Divider {
    divider(base_mhz * 1_000_000, target, 1)
}

/// N from a version 3 field.
fn n_of(field: u16) -> u32 {
    ((field >> 8) & 0xff) as u32 | (((field >> 6) & 3) as u32) << 8
}

#[test]
fn version_3_identification_clock() {
    assert_eq!(v3(200, IDENT_HZ), Divider { field: (250 << 8) as u16, hz: 400_000 });
    assert_eq!(v3(100, IDENT_HZ), Divider { field: (125 << 8) as u16, hz: 400_000 });
    assert_eq!(v3(50, IDENT_HZ).hz, 396_825);
    assert_eq!(n_of(v3(50, IDENT_HZ).field), 63);
}

#[test]
fn version_3_high_speed_clock() {
    assert_eq!(v3(200, HS52_HZ), Divider { field: 2 << 8, hz: 50_000_000 });
    assert_eq!(v3(100, HS52_HZ), Divider { field: 1 << 8, hz: 50_000_000 });
    assert_eq!(v3(50, HS52_HZ), Divider { field: 0, hz: 50_000_000 });
    assert_eq!(v3(52, HS52_HZ), Divider { field: 0, hz: 52_000_000 });
    assert_eq!(v3(200, HS26_HZ).hz, 25_000_000);
    assert_eq!(v3(200, LEGACY_HZ).hz, 20_000_000);
}

#[test]
fn version_3_ten_bit_divider_uses_the_upper_bits() {
    // 200 MHz to 100 kHz needs N = 1000: 0x3e8, upper bits 11b in 7:6.
    let d = v3(200, 100_000);
    assert_eq!(n_of(d.field), 1000);
    assert_eq!(d.field, 0xe8 << 8 | 0x3 << 6);
    assert_eq!(d.hz, 100_000);
    // Past the last divider the slowest clock is given.
    assert_eq!(n_of(v3(255, 1).field), MAX_N_V3);
}

#[test]
fn version_2_divides_by_powers_of_two() {
    // field = divisor / 2
    assert_eq!(v2(50, IDENT_HZ), Divider { field: 64 << 8, hz: 390_625 });
    assert_eq!(v2(48, IDENT_HZ), Divider { field: 64 << 8, hz: 375_000 });
    assert_eq!(v2(50, HS52_HZ), Divider { field: 0, hz: 50_000_000 });
    assert_eq!(v2(100, HS52_HZ), Divider { field: 1 << 8, hz: 50_000_000 });
    assert_eq!(v2(200, HS52_HZ), Divider { field: 2 << 8, hz: 50_000_000 });
    assert_eq!(v2(200, IDENT_HZ), Divider { field: 128 << 8, hz: 781_250 });
}

#[test]
fn never_above_the_target_when_the_divider_reaches_it() {
    for spec in [1u8, 2, 3, 4, 5] {
        for base in [25u32, 33, 48, 50, 52, 100, 133, 150, 192, 200, 208] {
            for target in [IDENT_HZ, LEGACY_HZ, HS26_HZ, HS52_HZ] {
                let d = divider(base * 1_000_000, target, spec);
                let slowest =
                    if spec >= 2 { base * 1_000_000 / 2046 } else { base * 1_000_000 / 256 };
                if slowest <= target {
                    assert!(d.hz <= target, "spec {spec} base {base} target {target}: {}", d.hz);
                }
            }
        }
    }
}

#[test]
fn base_clock_from_capabilities() {
    let c = |caps: u32, version: u16| Caps { caps, caps1: 0, version };
    assert_eq!(c(200 << 8, 2).base_hz(), 200_000_000);
    assert_eq!(c(0xc8 << 8, 0x1002).base_hz(), 200_000_000);
    // Version 2 reads six bits: 0xc8 & 0x3f = 8.
    assert_eq!(c(0xc8 << 8, 1).base_hz(), 8_000_000);
    // Not given: 200 MHz assumed.
    assert_eq!(c(0, 2).base_hz(), 200_000_000);
    assert!(!c(0, 2).base_given());
    assert!(c(50 << 8, 1).base_given());
}
