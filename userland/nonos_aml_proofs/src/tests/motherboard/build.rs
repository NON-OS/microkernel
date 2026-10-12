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

//! The descriptors a motherboard resource device's _CRS holds, laid out
//! byte by byte as ACPI 6.5, 6.4 gives them.

pub fn io(min: u16, max: u16, len: u8) -> Vec<u8> {
    let (a, b) = (min.to_le_bytes(), max.to_le_bytes());
    vec![0x47, 1, a[0], a[1], b[0], b[1], 1, len]
}

pub fn fixed_io(base: u16, len: u8) -> Vec<u8> {
    let b = base.to_le_bytes();
    vec![0x4B, b[0], b[1], len]
}

pub fn mem32_fixed(base: u32, len: u32) -> Vec<u8> {
    [vec![0x86, 9, 0, 1], base.to_le_bytes().to_vec(), len.to_le_bytes().to_vec()].concat()
}

pub fn mem32(min: u32, max: u32, len: u32) -> Vec<u8> {
    let f = [min, max, 1, len].map(u32::to_le_bytes).concat();
    [vec![0x85, 17, 0, 1], f].concat()
}

/// Minimum, maximum and length in 256-byte units.
pub fn mem24(min: u16, max: u16, len: u16) -> Vec<u8> {
    let f = [min, max, 1, len].map(u16::to_le_bytes).concat();
    [vec![0x81, 9, 0, 1], f].concat()
}

/// A QWord Address Space descriptor: kind, minimum, translation, length.
pub fn qword(kind: u8, min: u64, translation: u64, len: u64) -> Vec<u8> {
    let max = min.wrapping_add(len).wrapping_sub(1);
    let f = [0, min, max, translation, len].map(u64::to_le_bytes).concat();
    [vec![0x8A, 43, 0, kind, 0, 0], f].concat()
}

pub fn dword(kind: u8, min: u32, len: u32) -> Vec<u8> {
    let f = [0, min, min + len - 1, 0, len].map(u32::to_le_bytes).concat();
    [vec![0x87, 23, 0, kind, 0, 0], f].concat()
}

pub fn word(kind: u8, min: u16, len: u16) -> Vec<u8> {
    let f = [0, min, min + len - 1, 0, len].map(u16::to_le_bytes).concat();
    [vec![0x88, 13, 0, kind, 0, 0], f].concat()
}

/// An Extended Address Space descriptor, revision 1.
pub fn extended(kind: u8, min: u64, len: u64) -> Vec<u8> {
    let f = [0, min, min + len - 1, 0, len, 0].map(u64::to_le_bytes).concat();
    [vec![0x8B, 53, 0, kind, 0, 0, 1, 0], f].concat()
}

pub fn template(parts: &[Vec<u8>]) -> Vec<u8> {
    [parts.concat(), vec![0x79, 0]].concat()
}
