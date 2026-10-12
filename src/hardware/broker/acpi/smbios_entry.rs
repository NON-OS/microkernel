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

//! An SMBIOS entry point, checked before the kernel reads the table it
//! names (DMTF DSP0134 3.x, 5.2): the 64-bit "_SM3_" form, 24 bytes or
//! more, giving the table's address and its maximum size; and the 32-bit
//! "_SM_" form, 31 bytes, whose "_DMI_" part gives a 32-bit address and the
//! table's length. Each sums to zero over its length, the "_DMI_" part over
//! its own 15 bytes too.

pub const ENTRY_MAX: usize = 32;
/// A table larger than this is not believed: real ones are a few KiB.
pub const TABLE_MAX: u32 = 1 << 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    pub table: u64,
    pub len: u32,
    pub major: u8,
    pub minor: u8,
}

fn sums_to_zero(b: &[u8]) -> bool {
    b.iter().fold(0u8, |s, x| s.wrapping_add(*x)) == 0
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

fn bits64(b: &[u8]) -> Option<Entry> {
    let len = usize::from(*b.get(6)?);
    let e = b.get(..len).filter(|e| len >= 24 && sums_to_zero(e))?;
    let table = u64::from_le_bytes(e[16..24].try_into().ok()?);
    let max = u32_at(e, 12);
    (max > 0 && max <= TABLE_MAX).then_some(Entry { table, len: max, major: e[7], minor: e[8] })
}

fn bits32(b: &[u8]) -> Option<Entry> {
    let len = usize::from(*b.get(5)?);
    let e = b.get(..len).filter(|e| len >= 0x1F && sums_to_zero(e))?;
    let dmi = e.get(0x10..0x1F).filter(|d| d.starts_with(b"_DMI_") && sums_to_zero(d))?;
    let table_len = u32::from(u16::from_le_bytes([dmi[6], dmi[7]]));
    let table = u64::from(u32_at(dmi, 8));
    (table_len > 0).then_some(Entry { table, len: table_len, major: e[6], minor: e[7] })
}

pub fn parse_entry(b: &[u8]) -> Option<Entry> {
    if b.starts_with(b"_SM3_") {
        bits64(b)
    } else if b.starts_with(b"_SM_") {
        bits32(b)
    } else {
        None
    }
}

/// The entry point's own length, for the copy to the capsule.
pub fn entry_len(b: &[u8]) -> Option<usize> {
    match b {
        [b'_', b'S', b'M', b'3', b'_', _, len, ..] => Some(usize::from(*len)),
        [b'_', b'S', b'M', b'_', _, len, ..] => Some(usize::from(*len)),
        _ => None,
    }
    .filter(|l| *l <= ENTRY_MAX)
}
