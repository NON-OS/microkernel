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

//! A tree file's index block, in the layout its size needs.
//!
//!   NONOSIX2  bytes 0..8 magic, 8..12 the data block count, 16..480 the 58
//!             slots: 54 direct, then the one- to four-level trees' roots.
//!   NONOSIX3  the same, but bytes 8..16 hold the five-level tree's root.
//!
//! A file whose fifth root is zero is written as NONOSIX2, byte for byte as
//! a kernel that knows only NONOSIX2 wrote and reads it.

use super::file_consts::{
    INDEX_COUNT_OFFSET, INDEX_DEEP_OFFSET, INDEX_MAGIC, INDEX_MAGIC_DEEP, INDEX_MAGIC_FLAT,
    INDEX_PTR_BASE, MAX_PTRS, PTR_BYTES, ROOTS,
};
use super::read_u64::read_u64;
use super::tree_store::Block;
use super::write_u32::write_u32;
use super::write_u64::write_u64;
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

/// What an index block says.
pub(crate) enum Index {
    /// NONOSIX1: the block itself, read by `read_flat`.
    Flat(Block),
    /// NONOSIX2 or NONOSIX3: the file's roots, the fifth zero for NONOSIX2.
    Tree([u64; ROOTS]),
}

/// The index block for a tree file with `root` and `data_blocks` blocks.
/// Without its fifth tree a file has at most 13,179,714 blocks, so the
/// NONOSIX2 count always fits its u32.
pub(crate) fn encode(root: &[u64; ROOTS], data_blocks: u64) -> Block {
    let mut index = [0u8; PLAIN_BLOCK_BYTES];
    let deep = root[ROOTS - 1];
    if deep == 0 {
        index[0..8].copy_from_slice(&INDEX_MAGIC);
        write_u32(&mut index, INDEX_COUNT_OFFSET, u32::try_from(data_blocks).unwrap_or(u32::MAX));
    } else {
        index[0..8].copy_from_slice(&INDEX_MAGIC_DEEP);
        write_u64(&mut index, INDEX_DEEP_OFFSET, deep);
    }
    for (i, lba) in root[..MAX_PTRS].iter().enumerate() {
        write_u64(&mut index, INDEX_PTR_BASE + i * PTR_BYTES, *lba);
    }
    index
}

/// What `index` says, or None when it is no index block this kernel knows.
pub(crate) fn decode(index: &Block) -> Option<Index> {
    let deep = match &index[0..8] {
        m if m == INDEX_MAGIC_FLAT => return Some(Index::Flat(*index)),
        m if m == INDEX_MAGIC => 0,
        m if m == INDEX_MAGIC_DEEP => read_u64(index, INDEX_DEEP_OFFSET),
        _ => return None,
    };
    let mut root = [0u64; ROOTS];
    for (i, slot) in root[..MAX_PTRS].iter_mut().enumerate() {
        *slot = read_u64(index, INDEX_PTR_BASE + i * PTR_BYTES);
    }
    root[ROOTS - 1] = deep;
    Some(Index::Tree(root))
}
