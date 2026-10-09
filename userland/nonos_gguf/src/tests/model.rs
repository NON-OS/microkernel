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

//! The well-formed model most tests start from.

use super::build::Gguf;
use alloc::vec::Vec;

/// A small, well-formed model: two keys, a Q4_K matrix and an F32 vector.
/// 512 x 4 Q4_K is 8 blocks of 144 bytes; 512 F32 is 2048 bytes.
pub fn small_model() -> Vec<u8> {
    let mut g = Gguf::header(3, 2, 2);
    g.kv_str("general.architecture", "qwen2").kv_u32("general.alignment", 32);
    g.tensor("blk.0.w", &[512, 4], 12, 0).tensor("blk.0.norm", &[512], 0, 1152);
    g.data(32, 1152 + 2048);
    g.out
}
