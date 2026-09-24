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

//! The canonical text of a GUID, upper-case hex with the four dashes, as
//! `gdisk` and firmware menus print it.

use super::Guid;

const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Byte order from on-disk to printed: the three little-endian fields
/// reversed, the two big-endian ones straight.
const ORDER: [usize; 16] = [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15];

impl Guid {
    pub fn text(&self) -> [u8; 36] {
        let mut out = [b'-'; 36];
        let mut o = 0usize;
        for (i, &src) in ORDER.iter().enumerate() {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                o += 1;
            }
            out[o] = HEX[(self.0[src] >> 4) as usize];
            out[o + 1] = HEX[(self.0[src] & 0xF) as usize];
            o += 2;
        }
        out
    }
}
