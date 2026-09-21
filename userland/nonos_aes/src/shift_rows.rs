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

//! The ShiftRows step, and why the index arithmetic looks backwards.

use crate::types::BLOCK_BYTES;

/*
 * The state is column major: byte `r + 4c` is row r of column c. Row r
 * rotates left by r columns, so the index of the byte that lands at
 * `r + 4c` is `r + 4*((c + r) mod 4)`.
 */
pub(crate) fn shift_rows(block: &mut [u8; BLOCK_BYTES]) {
    let source = *block;
    for row in 1..4 {
        for column in 0..4 {
            block[row + 4 * column] = source[row + 4 * ((column + row) % 4)];
        }
    }
}
