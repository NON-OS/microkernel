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

//! The tensor types this checker knows the size of: elements per block and
//! bytes per block, as ggml lays them out. A type not listed is refused by
//! name rather than guessed.

/// (elements in a block, bytes in a block) for ggml type `ty`.
pub(crate) fn block_of(ty: u32) -> Option<(u64, u64)> {
    Some(match ty {
        0 => (1, 4),      /* F32 */
        1 => (1, 2),      /* F16 */
        2 => (32, 18),    /* Q4_0: f16 scale, 16 bytes of nibbles */
        3 => (32, 20),    /* Q4_1 */
        6 => (32, 22),    /* Q5_0 */
        7 => (32, 24),    /* Q5_1 */
        8 => (32, 34),    /* Q8_0 */
        9 => (32, 36),    /* Q8_1 */
        10 => (256, 84),  /* Q2_K */
        11 => (256, 110), /* Q3_K */
        12 => (256, 144), /* Q4_K: 2 f16, 12 scale bytes, 128 of nibbles */
        13 => (256, 176), /* Q5_K */
        14 => (256, 210), /* Q6_K: 128 low, 64 high, 16 scales, f16 */
        15 => (256, 292), /* Q8_K */
        24 => (1, 1),     /* I8 */
        25 => (1, 2),     /* I16 */
        26 => (1, 4),     /* I32 */
        27 => (1, 8),     /* I64 */
        28 => (1, 8),     /* F64 */
        30 => (1, 2),     /* BF16 */
        _ => return None,
    })
}

/// Types are counted by id up to this one.
pub const TYPE_SLOTS: usize = 31;
