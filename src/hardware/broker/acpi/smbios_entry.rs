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
//! names (DMTF DSP0134 3.x, 5.2): "_SM3_", 24 bytes or more, with a 64-bit
//! address and maximum size; or "_SM_", 31 bytes, whose "_DMI_" part gives
//! a 32-bit address and length. Each, and "_DMI_" alone, sums to zero.

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

fn bits64(b: &[u8]) -> Option<Entry> {
    let len = usize::from(*b.get(6)?);
    let e = b.get(..len).filter(|e| len >= 24 && sums_to_zero(e))?;
    let table = u64::from_le_bytes(e[16..24].try_into().ok()?);
    let max = u32::from_le_bytes(e[12..16].try_into().ok()?);
    (max > 0 && max <= TABLE_MAX).then_some(Entry { table, len: max, major: e[7], minor: e[8] })
}

/// A 2.1-era length of 0x1E is refused: DSP0134's text on it was not checked.
fn bits32(b: &[u8]) -> Option<Entry> {
    let len = usize::from(*b.get(5)?);
    let e = b.get(..len).filter(|e| len >= 0x1F && sums_to_zero(e))?;
    let dmi = e.get(0x10..0x1F).filter(|d| d.starts_with(b"_DMI_") && sums_to_zero(d))?;
    let table_len = u32::from(u16::from_le_bytes([dmi[6], dmi[7]]));
    let table = u64::from(u32::from_le_bytes(dmi[8..12].try_into().ok()?));
    (table_len > 0).then_some(Entry { table, len: table_len, major: e[6], minor: e[7] })
}

pub fn parse_entry(b: &[u8]) -> Option<Entry> {
    match b {
        [b'_', b'S', b'M', b'3', b'_', ..] => bits64(b),
        [b'_', b'S', b'M', b'_', ..] => bits32(b),
        _ => None,
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
