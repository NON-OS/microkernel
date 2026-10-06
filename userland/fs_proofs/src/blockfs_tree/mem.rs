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

//! Stores for the tree tests: one keeping every block, one keeping none.

use super::file_consts::{DATA_BYTES, ROOTS};
use super::tree_store::{Block, BlockSource, BlockStore};
use super::tree_writer::TreeWriter;

/// Every block kept; LBA n is `blocks[n - 1]`, so 0 is never an address.
#[derive(Default)]
pub struct Mem {
    pub blocks: Vec<Block>,
    pub gets: usize,
}

impl BlockSource for Mem {
    type Error = &'static str;
    fn get(&mut self, lba: u64) -> Result<Block, &'static str> {
        self.gets += 1;
        let i = lba.checked_sub(1).ok_or("lba 0")? as usize;
        self.blocks.get(i).copied().ok_or("past the disk")
    }
}

impl BlockStore for Mem {
    fn alloc(&mut self) -> Result<u64, &'static str> {
        self.blocks.push([0u8; DATA_BYTES]);
        Ok(self.blocks.len() as u64)
    }
    fn put(&mut self, lba: u64, block: &Block) -> Result<(), &'static str> {
        let i = lba.checked_sub(1).ok_or("lba 0")? as usize;
        *self.blocks.get_mut(i).ok_or("past the disk")? = *block;
        Ok(())
    }
}

/// The byte a test file holds at `i`: never periodic in 484 or 60.
pub fn byte_at(i: u64) -> u8 {
    (i.wrapping_mul(2_654_435_761) >> 13) as u8
}

/// Write a file of `size` bytes as the kernel does: fill, seal, hang.
pub fn write_file(mem: &mut Mem, size: u64) -> ([u64; ROOTS], TreeWriter) {
    let mut tree = TreeWriter::new();
    let mut at = 0u64;
    while at < size {
        let mut block = [0u8; DATA_BYTES];
        let take = (size - at).min(DATA_BYTES as u64) as usize;
        for (k, b) in block[..take].iter_mut().enumerate() {
            *b = byte_at(at + k as u64);
        }
        let lba = mem.alloc().unwrap();
        mem.put(lba, &block).unwrap();
        tree.push(mem, lba).unwrap();
        at += take as u64;
    }
    tree.finish(mem).unwrap();
    (tree.root, tree)
}
