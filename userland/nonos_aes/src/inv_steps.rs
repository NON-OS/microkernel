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

//! InvShiftRows and InvMixColumns, the inverse cipher's two linear steps.

use crate::mix_columns::mix_columns;
use crate::types::BLOCK_BYTES;
use crate::xtime::xtime;

/// Row r rotates right by r columns: the byte at `r + 4c` comes from
/// column `c - r`, taken mod 4.
pub(crate) fn inv_shift_rows(block: &mut [u8; BLOCK_BYTES]) {
    let source = *block;
    for row in 1..4 {
        for column in 0..4 {
            block[row + 4 * column] = source[row + 4 * ((column + 4 - row) % 4)];
        }
    }
}

/*
 * InvMixColumns multiplies each column by {0e 09 0d 0b}. That matrix is
 * MixColumns' {02 03 01 01} times {05 00 04 00}, so one pass that adds
 * {04} times the column's even and odd byte pairs, then MixColumns, gives
 * it (The Design of Rijndael, 4.1.3). Only xtime is used, which masks
 * rather than branches, where the general multiply tests each bit.
 */
pub(crate) fn inv_mix_columns(block: &mut [u8; BLOCK_BYTES]) {
    for column in 0..4 {
        let at = column * 4;
        let u = xtime(xtime(block[at] ^ block[at + 2]));
        let v = xtime(xtime(block[at + 1] ^ block[at + 3]));
        block[at] ^= u;
        block[at + 1] ^= v;
        block[at + 2] ^= u;
        block[at + 3] ^= v;
    }
    mix_columns(block);
}
