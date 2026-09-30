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

//! Finding a file's n-th data block through its index.
//!
//! One pointer block per level is kept, so a file read from start to end
//! reads each pointer block once and every other step is a data block.

use super::file_consts::{LEVELS, ROOTS};
use super::tree_ptrs::entry;
use super::tree_shape::locate;
use super::tree_store::{Block, BlockSource, TreeFault};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub(crate) struct TreeReader {
    root: [u64; ROOTS],
    /// The last pointer block read at each level, and its address; 0 is none.
    held: [(u64, Block); LEVELS],
}

impl TreeReader {
    pub(crate) fn new(root: [u64; ROOTS]) -> Self {
        TreeReader { root, held: [(0, [0u8; PLAIN_BLOCK_BYTES]); LEVELS] }
    }

    /// The address of data block `n`. A zero pointer on the way is a hole,
    /// which a file this layout wrote never has below its size.
    pub(crate) fn data_lba<S: BlockSource>(
        &mut self,
        s: &mut S,
        n: u64,
    ) -> Result<u64, TreeFault<S::Error>> {
        let place = locate(n).ok_or(TreeFault::TooLarge)?;
        let mut ptr = self.root[place.slot];
        for level in 0..place.depth {
            if ptr == 0 {
                return Err(TreeFault::Hole);
            }
            if self.held[level].0 != ptr {
                let block = s.get_pointer(ptr).map_err(TreeFault::Store)?;
                self.held[level] = (ptr, block);
            }
            ptr = entry(&self.held[level].1, place.path[level]);
        }
        if ptr == 0 {
            return Err(TreeFault::Hole);
        }
        Ok(ptr)
    }
}
