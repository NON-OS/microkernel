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

//! The resource descriptors the platform device proofs build templates
//! from (ACPI 6.5, 6.4.3.4, 6.4.3.5.1, 6.4.3.6).

pub fn ext_irq(flags: u8, gsi: u32) -> Vec<u8> {
    [vec![0x89, 6, 0, flags, 1], gsi.to_le_bytes().to_vec()].concat()
}

pub fn mem32(base: u32, len: u32) -> Vec<u8> {
    [vec![0x86, 9, 0, 1], base.to_le_bytes().to_vec(), len.to_le_bytes().to_vec()].concat()
}

/// Type, general and type flags, then granularity, minimum, maximum,
/// translation and length, eight bytes each.
pub fn qword(base: u64, translation: u64, len: u64) -> Vec<u8> {
    let f = [0, base, base + len - 1, translation, len];
    [vec![0x8A, 43, 0, 0, 0, 0], f.iter().flat_map(|v| v.to_le_bytes()).collect()].concat()
}

pub fn template(parts: &[Vec<u8>]) -> Vec<u8> {
    [parts.concat(), vec![0x79, 0]].concat()
}
