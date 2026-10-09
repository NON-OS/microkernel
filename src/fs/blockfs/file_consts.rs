// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! How a file's blocks are found from its index block.
//!
//! Every block is one sector sealed on its own, so a block carries
//! PLAIN_BLOCK_BYTES = 512 - 12 (nonce) - 16 (tag) = 484 bytes. The index
//! block keeps a 16-byte header and (484 - 16) / 8 = 58 slots. The first 54
//! point at data blocks; the next four point at trees of pointer blocks one,
//! two, three and four levels deep. A fifth tree, five levels deep, has its
//! root in the header. A pointer block is all pointers, 484 / 8 = 60 of
//! them: its seal binds it to its own LBA, so it needs no magic of its own.
//! A file therefore holds at most
//!
//!   direct      54                 blocks             26,136 bytes
//!   single      60                 blocks             29,040 bytes
//!   double      60^2 =       3,600 blocks          1,742,400 bytes
//!   triple      60^3 =     216,000 blocks        104,544,000 bytes
//!   quadruple   60^4 =  12,960,000 blocks      6,272,640,000 bytes
//!   quintuple   60^5 = 777,600,000 blocks    376,358,400,000 bytes
//!
//! 790,779,714 blocks, 382,737,381,576 bytes. The first four trees stop at
//! 13,179,714 blocks, 6,378,981,576 bytes, short of a 9 GB model that cannot
//! be split without changing its pinned digest; the fifth is what reaches
//! hundreds of gigabytes. The trees nest the same way whichever tree is
//! last, so a file within four levels is written exactly as before, in the
//! NONOSIX2 layout an earlier kernel reads; only a larger one is NONOSIX3,
//! which keeps the fifth root where NONOSIX2 keeps its block count. The
//! flat layout before both held 58 * 484 = 28,072 bytes.

use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

/// The index block of a file written before indirection: 58 data pointers.
pub(super) const INDEX_MAGIC_FLAT: [u8; 8] = *b"NONOSIX1";
/// The index block of a file within four levels of trees.
pub(super) const INDEX_MAGIC: [u8; 8] = *b"NONOSIX2";
/// The index block of a file that reaches the fifth level.
pub(super) const INDEX_MAGIC_DEEP: [u8; 8] = *b"NONOSIX3";
/// NONOSIX1 and NONOSIX2: the data block count, a u32.
pub(super) const INDEX_COUNT_OFFSET: usize = 8;
/// NONOSIX3: the fifth tree's root, a u64, in place of the count.
pub(super) const INDEX_DEEP_OFFSET: usize = 8;
pub(super) const INDEX_PTR_BASE: usize = 16;
pub(super) const PTR_BYTES: usize = 8;
pub(super) const DATA_BYTES: usize = PLAIN_BLOCK_BYTES;
/// Slots in the index block after its header.
pub(super) const MAX_PTRS: usize = (PLAIN_BLOCK_BYTES - INDEX_PTR_BASE) / PTR_BYTES;
/// Pointers in one pointer block.
pub(super) const FANOUT: usize = PLAIN_BLOCK_BYTES / PTR_BYTES;
/// Levels of indirection, one root for each.
pub(super) const LEVELS: usize = 5;
/// A file's roots: the index block's slots, then the fifth tree's.
pub(super) const ROOTS: usize = MAX_PTRS + 1;
pub(super) const DIRECT_SLOTS: usize = ROOTS - LEVELS;
const F: u64 = FANOUT as u64;
/// The most blocks the first four trees hold; a file this size or smaller
/// is written as NONOSIX2.
pub(crate) const FOUR_LEVEL_BLOCKS: u64 =
    DIRECT_SLOTS as u64 + F + F * F + F * F * F + F * F * F * F;
pub(crate) const MAX_FILE_BLOCKS: u64 = FOUR_LEVEL_BLOCKS + F * F * F * F * F;
pub(crate) const MAX_FILE_BYTES: u64 = MAX_FILE_BLOCKS * DATA_BYTES as u64;
