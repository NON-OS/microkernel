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

//! Compressed EISA ids (ACPI 6.5, 6.1.5): three letters of five bits each
//! and four hex digits, stored so the bytes read in order from the integer's
//! low byte.

/// "PNP0A03" from 0x030AD041.
pub fn eisa_id(v: u32) -> [u8; 7] {
    let id = v.swap_bytes();
    let letter = |shift: u32| ((id >> shift) & 0x1F) as u8 + 0x40;
    let hex = |n: u32| b"0123456789ABCDEF"[(n & 0xF) as usize];
    [letter(26), letter(21), letter(16), hex(id >> 12), hex(id >> 8), hex(id >> 4), hex(id)]
}
