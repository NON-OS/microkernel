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

//! The MixColumns step, written as one xtime per byte.

use crate::types::BLOCK_BYTES;
use crate::xtime::xtime;

pub(crate) fn mix_columns(block: &mut [u8; BLOCK_BYTES]) {
    for column in 0..4 {
        let at = column * 4;
        let a = [block[at], block[at + 1], block[at + 2], block[at + 3]];
        let parity = a[0] ^ a[1] ^ a[2] ^ a[3];
        for row in 0..4 {
            block[at + row] = a[row] ^ parity ^ xtime(a[row] ^ a[(row + 1) % 4]);
        }
    }
}
