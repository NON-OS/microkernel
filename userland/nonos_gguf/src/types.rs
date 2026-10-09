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
//!
//! The k-quants hold 256 elements. Q4_K is two f16 scales, 12 bytes of
//! sub-block scales and 128 of nibbles, 144 in all; Q6_K is 128 bytes of low
//! bits, 64 of high bits, 16 scales and an f16, 210. The 32-element quants
//! are an f16 scale (and an f16 minimum for the _1 forms) beside the packed
//! values. Checked on a real model: Qwen2.5-0.5B's tensors, sized by this
//! table, end on its last byte.

const F32: u32 = 0;
const F16: u32 = 1;
const Q4_0: u32 = 2;
const Q4_1: u32 = 3;
const Q5_0: u32 = 6;
const Q5_1: u32 = 7;
const Q8_0: u32 = 8;
const Q8_1: u32 = 9;
const Q2_K: u32 = 10;
const Q3_K: u32 = 11;
const Q4_K: u32 = 12;
const Q5_K: u32 = 13;
const Q6_K: u32 = 14;
const Q8_K: u32 = 15;
const I8: u32 = 24;
const I16: u32 = 25;
const I32: u32 = 26;
const I64: u32 = 27;
const F64: u32 = 28;
const BF16: u32 = 30;

/// (elements in a block, bytes in a block) for ggml type `ty`.
pub(crate) fn block_of(ty: u32) -> Option<(u64, u64)> {
    Some(match ty {
        F32 | I32 => (1, 4),
        F16 | BF16 | I16 => (1, 2),
        I8 => (1, 1),
        I64 | F64 => (1, 8),
        Q4_0 => (32, 18),
        Q4_1 => (32, 20),
        Q5_0 => (32, 22),
        Q5_1 => (32, 24),
        Q8_0 => (32, 34),
        Q8_1 => (32, 36),
        Q2_K => (256, 84),
        Q3_K => (256, 110),
        Q4_K => (256, 144),
        Q5_K => (256, 176),
        Q6_K => (256, 210),
        Q8_K => (256, 292),
        _ => return None,
    })
}

/// Types are counted by id up to this one.
pub const TYPE_SLOTS: usize = 31;
