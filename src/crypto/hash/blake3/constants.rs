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

pub const OUT_LEN: usize = 32;
pub const KEY_LEN: usize = 32;

pub(crate) const IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

pub(crate) const BLOCK_LEN: usize = 64;
pub(crate) const CHUNK_LEN: usize = 1024;
pub(crate) const MAX_DEPTH: usize = 54;

pub(crate) const CHUNK_START: u8 = 1 << 0;
pub(crate) const CHUNK_END: u8 = 1 << 1;
pub(crate) const PARENT: u8 = 1 << 2;
pub(crate) const ROOT: u8 = 1 << 3;
pub(crate) const KEYED_HASH: u8 = 1 << 4;
pub(crate) const DERIVE_KEY_CONTEXT: u8 = 1 << 5;
pub(crate) const DERIVE_KEY_MATERIAL: u8 = 1 << 6;

// The message word order of the next round, applied between rounds.
pub(crate) const MSG_PERMUTATION: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
