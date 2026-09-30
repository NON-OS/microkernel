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
//! A file the size of the small Qwen model, read back block by block: every
//! data block index, through all four trees, names the block written there.

use std::collections::HashMap;

use super::file_consts::DATA_BYTES;
use super::tree_reader::TreeReader;
use super::tree_store::{Block, BlockSource, BlockStore};
use super::tree_writer::TreeWriter;

/// Keeps the pointer blocks the writer puts; data blocks are only numbered.
#[derive(Default)]
struct Pointers {
    next: u64,
    kept: HashMap<u64, Block>,
}
impl BlockSource for Pointers {
    type Error = &'static str;
    fn get(&mut self, lba: u64) -> Result<Block, &'static str> {
        self.kept.get(&lba).copied().ok_or("not a pointer block")
    }
}
impl BlockStore for Pointers {
    fn alloc(&mut self) -> Result<u64, &'static str> {
        self.next += 1;
        Ok(self.next)
    }
    fn put(&mut self, lba: u64, block: &Block) -> Result<(), &'static str> {
        self.kept.insert(lba, *block);
        Ok(())
    }
}

#[test]
fn every_block_of_a_491_mb_file_reads_back_where_it_was_written() {
    let blocks = 491_400_032u64.div_ceil(DATA_BYTES as u64);
    let (mut store, mut tree) = (Pointers::default(), TreeWriter::new());
    let mut written = Vec::with_capacity(blocks as usize);
    for _ in 0..blocks {
        let lba = store.alloc().unwrap();
        tree.push(&mut store, lba).unwrap();
        written.push(lba);
    }
    tree.finish(&mut store).unwrap();
    let mut reader = TreeReader::new(tree.root);
    for (n, want) in written.iter().enumerate() {
        assert_eq!(reader.data_lba(&mut store, n as u64), Ok(*want), "block {n}");
    }
    /*
     * And backwards, as a reader seeking to earlier tensors does.
     */
    let mut reader = TreeReader::new(tree.root);
    for n in (0..blocks).rev().step_by(9973) {
        assert_eq!(reader.data_lba(&mut store, n), Ok(written[n as usize]), "block {n}");
    }
}
