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

//! Four hex digits for the hardware report's ids.

/// The low four hex digits of `v`, lower case.
pub(super) fn hex(v: u32) -> [u8; 4] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = [b'0'; 4];
    for (i, slot) in out.iter_mut().rev().enumerate() {
        *slot = HEX[((v >> (4 * i)) & 0xF) as usize];
    }
    out
}

/// The same digits in upper case, as PNP ids are written.
pub(super) fn upper(mut digits: [u8; 4]) -> [u8; 4] {
    digits.iter_mut().for_each(|b| b.make_ascii_uppercase());
    digits
}
