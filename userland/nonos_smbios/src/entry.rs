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

//! The entry points (DSP0134 5.2): "_SM3_", 24 bytes or more, with the
//! table's 64-bit address and maximum size; "_SM_", 31 bytes, whose "_DMI_"
//! part holds a 32-bit address, the table's length and its structure
//! count. Each must sum to zero over its length; the "_DMI_" part over its
//! own 15 bytes as well.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    pub major: u8,
    pub minor: u8,
    pub table: u64,
    /// The table's length (32-bit form) or its maximum size (64-bit form).
    pub len: u32,
    /// The number of structures, which only the 32-bit form gives.
    pub count: Option<u16>,
}

fn sums_to_zero(b: &[u8]) -> bool {
    b.iter().fold(0u8, |s, x| s.wrapping_add(*x)) == 0
}

fn le32(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

fn bits64(b: &[u8]) -> Option<Entry> {
    let len = usize::from(*b.get(6)?);
    let e = b.get(..len).filter(|e| len >= 24 && sums_to_zero(e))?;
    let table = u64::from_le_bytes(e[16..24].try_into().ok()?);
    Some(Entry { major: e[7], minor: e[8], table, len: le32(&e[12..]), count: None })
}

fn bits32(b: &[u8]) -> Option<Entry> {
    let len = usize::from(*b.get(5)?);
    let e = b.get(..len).filter(|e| len >= 0x1F && sums_to_zero(e))?;
    let d = e.get(0x10..0x1F).filter(|d| d.starts_with(b"_DMI_") && sums_to_zero(d))?;
    Some(Entry {
        major: e[6],
        minor: e[7],
        table: u64::from(le32(&d[8..])),
        len: u32::from(u16::from_le_bytes([d[6], d[7]])),
        count: Some(u16::from_le_bytes([d[12], d[13]])),
    })
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
