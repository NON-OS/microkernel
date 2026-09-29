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
//! block keeps a 16-byte header and (484 - 16) / 8 = 58 root slots. The
//! first 54 point at data blocks; the last four point at trees of pointer
//! blocks one, two, three and four levels deep. A pointer block is all
//! pointers, 484 / 8 = 60 of them: its seal binds it to its own LBA, so it
//! needs no magic of its own. A file therefore holds at most
//!
//!   direct      54               blocks        26,136 bytes
//!   single      60               blocks        29,040 bytes
//!   double      60^2 =      3,600 blocks     1,742,400 bytes
//!   triple      60^3 =    216,000 blocks   104,544,000 bytes
//!   quadruple   60^4 = 12,960,000 blocks 6,272,640,000 bytes
//!
//! 13,179,714 blocks, 6,378,981,576 bytes. Triple indirection alone stops
//! near 100 MB, short of a 400 MB model; the fourth level is what reaches
//! gigabytes. The single-level layout before this held 58 * 484 = 28,072.

use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

/// The index block of a file written before indirection: 58 data pointers.
pub(super) const INDEX_MAGIC_FLAT: [u8; 8] = *b"NONOSIX1";
/// The index block of a file with indirect trees.
pub(super) const INDEX_MAGIC: [u8; 8] = *b"NONOSIX2";
pub(super) const INDEX_COUNT_OFFSET: usize = 8;
pub(super) const INDEX_PTR_BASE: usize = 16;
pub(super) const PTR_BYTES: usize = 8;
pub(super) const DATA_BYTES: usize = PLAIN_BLOCK_BYTES;
pub(super) const MAX_PTRS: usize = (PLAIN_BLOCK_BYTES - INDEX_PTR_BASE) / PTR_BYTES;
/// Pointers in one pointer block.
pub(super) const FANOUT: usize = PLAIN_BLOCK_BYTES / PTR_BYTES;
/// Levels of indirection, one root slot for each.
pub(super) const LEVELS: usize = 4;
pub(super) const DIRECT_SLOTS: usize = MAX_PTRS - LEVELS;
const F: u64 = FANOUT as u64;
pub(crate) const MAX_FILE_BLOCKS: u64 = DIRECT_SLOTS as u64 + F + F * F + F * F * F + F * F * F * F;
pub(crate) const MAX_FILE_BYTES: u64 = MAX_FILE_BLOCKS * DATA_BYTES as u64;
