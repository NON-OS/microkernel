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

//! The version table of Linux __rtl_get_hw_ver, each value at TCR1's
//! place in the PLA_TCR0 dword.

use crate::r8153::version::{version, Version};

fn tcr0(tcr1: u16) -> u32 {
    (tcr1 as u32) << 16 | 0x0000_1234
}

#[test]
fn rtl8153_and_rtl8153b_versions_are_taken() {
    let table = [
        (0x5c00, Version::V03),
        (0x5c10, Version::V04),
        (0x5c20, Version::V05),
        (0x5c30, Version::V06),
        (0x6000, Version::V08),
        (0x6010, Version::V09),
    ];
    for (bits, v) in table {
        assert_eq!(version(tcr0(bits)), Ok(v));
    }
    assert!(Version::V08.is_8153b() && Version::V09.is_8153b() && !Version::V06.is_8153b());
}

#[test]
fn bits_outside_version_mask_are_ignored() {
    // VERSION_MASK is 0x7cf0: 0x830f are not version bits.
    assert_eq!(version(tcr0(0x5c10 | 0x830f)), Ok(Version::V04));
}

#[test]
fn other_chips_are_refused_by_name() {
    for bits in [0x4c00, 0x4c10, 0x4800] {
        assert_eq!(version(tcr0(bits)), Err("an RTL8152, not an RTL8153"));
    }
    for bits in [0x7010, 0x7020, 0x7030, 0x7400, 0x7410, 0x7420] {
        assert_eq!(version(tcr0(bits)), Err("an RTL8156, not an RTL8153"));
    }
    assert_eq!(version(tcr0(0x6400)), Err("an RTL8153C, whose init this driver lacks"));
    assert_eq!(version(tcr0(0x0000)), Err("unknown chip version"));
    assert_eq!(version(0xffff_ffff), Err("unknown chip version"));
}
