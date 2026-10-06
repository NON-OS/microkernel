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

//! Blocks in memory for the in-place test, counting data reads by kind.

use crate::blockfs_tree::tree_store::{Block, BlockSource, BlockStore};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES as DATA_BYTES;

/// Blocks in memory, LBA n at `blocks[n - 1]`, counting data reads by kind.
#[derive(Default)]
pub struct InPlace {
    blocks: Vec<Block>,
    pub parts: usize,
    pub wholes: usize,
}

impl BlockSource for InPlace {
    type Error = ();
    fn get(&mut self, lba: u64) -> Result<Block, ()> {
        self.parts += 1;
        self.get_pointer(lba)
    }
    fn get_pointer(&mut self, lba: u64) -> Result<Block, ()> {
        self.blocks.get(lba.checked_sub(1).ok_or(())? as usize).copied().ok_or(())
    }
    fn get_into(&mut self, lba: u64, out: &mut Block) -> Result<(), ()> {
        self.wholes += 1;
        *out = self.get_pointer(lba)?;
        Ok(())
    }
}

impl BlockStore for InPlace {
    fn alloc(&mut self) -> Result<u64, ()> {
        self.blocks.push([0u8; DATA_BYTES]);
        Ok(self.blocks.len() as u64)
    }
    fn put(&mut self, lba: u64, block: &Block) -> Result<(), ()> {
        *self.blocks.get_mut(lba.checked_sub(1).ok_or(())? as usize).ok_or(())? = *block;
        Ok(())
    }
}
