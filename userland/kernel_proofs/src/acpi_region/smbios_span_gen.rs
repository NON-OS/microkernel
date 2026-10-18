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

//! Seeded SMBIOS entry points for F-105b, both forms with their sums set
//! right, naming tables in firmware's memory, across RAM, in and out of the
//! legacy window, past the map, near the top of the address space, or
//! anywhere; and now and then a byte changed after the sums were set.

use crate::smbios::smbios_entry::TABLE_MAX;

pub struct Rng(pub u64);

impl Rng {
    pub fn below(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n.max(1)
    }
}

fn fix(b: &mut [u8], at: usize, over: core::ops::Range<usize>) {
    b[at] = 0;
    let s = b[over].iter().fold(0u8, |s, x| s.wrapping_add(*x));
    b[at] = 0u8.wrapping_sub(s);
}

/// An entry point and whether it is the 64-bit form.
pub fn entry(r: &mut Rng) -> (Vec<u8>, bool) {
    let wide = r.below(2) == 0;
    let table = super::model::place(r);
    let mut b = if wide { vec![0u8; 24] } else { vec![0u8; 0x1F] };
    if wide {
        b[..5].copy_from_slice(b"_SM3_");
        (b[6], b[7], b[8]) = (24, 3, 6);
        let max = r.below(2 * u64::from(TABLE_MAX)) as u32;
        b[12..16].copy_from_slice(&max.to_le_bytes());
        b[16..24].copy_from_slice(&table.to_le_bytes());
        fix(&mut b, 5, 0..24);
    } else {
        b[..4].copy_from_slice(b"_SM_");
        (b[5], b[6], b[7]) = (0x1F, 2, 8);
        b[0x10..0x15].copy_from_slice(b"_DMI_");
        b[0x16..0x18].copy_from_slice(&(r.below(0x1_0000) as u16).to_le_bytes());
        b[0x18..0x1C].copy_from_slice(&(table as u32).to_le_bytes());
        fix(&mut b, 0x15, 0x10..0x1F);
        fix(&mut b, 4, 0..0x1F);
    }
    if r.below(8) == 0 {
        let at = r.below(b.len() as u64) as usize;
        b[at] ^= 1 << r.below(8);
    }
    (b, wide)
}
