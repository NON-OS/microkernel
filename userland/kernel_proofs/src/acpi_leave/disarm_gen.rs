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

//! Seeded FADT event blocks for the disarm proof: PM1 event blocks 8 to 32
//! bits wide, GPE blocks of every length from none to 255 bytes, in I/O or
//! memory space or absent, and now and then a hardware-reduced platform.

use super::fadt_decode::{FadtInfo, FLAG_HW_REDUCED_ACPI};
use super::gas::Gas;

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn gas(s: &mut u64, port: u64, mem: u64, bits: u8) -> Gas {
    let absent = next(s).is_multiple_of(5);
    let in_mem = next(s).is_multiple_of(3);
    let (space, address) = if in_mem { (0, mem) } else { (1, port) };
    let address = if absent { 0 } else { address };
    Gas { space, bit_width: bits, bit_offset: 0, access_size: 0, address }
}

pub fn layout(s: &mut u64) -> FadtInfo {
    let width = |s: &mut u64| [32, 32, 16, 8][(next(s) % 4) as usize];
    let len = |s: &mut u64| [0, 1, 2, 3, 4, 8, 16, 32, 255][(next(s) % 9) as usize];
    let (wa, wb, l0, l1) = (width(s), width(s), len(s), len(s));
    FadtInfo {
        pm1a_evt: gas(s, 0x400, 0xFED0_0000, wa),
        pm1b_evt: gas(s, 0x480, 0xFED1_0000, wb),
        gpe0: gas(s, 0x1000, 0xFED2_0000, 8),
        gpe1: gas(s, 0x2000, 0xFED3_0000, 8),
        gpe0_len: l0,
        gpe1_len: l1,
        flags: if next(s).is_multiple_of(8) { FLAG_HW_REDUCED_ACPI } else { 0 },
        ..FadtInfo::default()
    }
}
