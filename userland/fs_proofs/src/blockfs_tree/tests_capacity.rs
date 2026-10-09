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

//! The whole index walked, with a store that keeps nothing.

use super::file_consts::{MAX_FILE_BLOCKS, ROOTS};
use super::tree_store::{Block, BlockSource, BlockStore, TreeFault};
use super::tree_writer::TreeWriter;

/// Allocates and forgets: enough to walk the whole index without 380 GB.
struct Null(u64);
impl BlockSource for Null {
    type Error = ();
    fn get(&mut self, _: u64) -> Result<Block, ()> {
        Err(())
    }
}
impl BlockStore for Null {
    fn alloc(&mut self) -> Result<u64, ()> {
        self.0 += 1;
        Ok(self.0)
    }
    fn put(&mut self, _: u64, _: &Block) -> Result<(), ()> {
        Ok(())
    }
}

#[test]
fn the_index_takes_exactly_its_largest_file_and_refuses_one_block_more() {
    let (mut null, mut tree) = (Null(0), TreeWriter::new());
    for lba in 1..=MAX_FILE_BLOCKS {
        tree.push(&mut null, lba).unwrap();
    }
    assert_eq!(tree.push(&mut null, 1), Err(TreeFault::TooLarge));
    tree.finish(&mut null).unwrap();
    /*
     * Every pointer block of all five trees, full, and written once:
     * single 1, double 60 + 1, triple 3600 + 60 + 1, quadruple
     * 216000 + 3600 + 60 + 1, quintuple 12960000 + 216000 + 3600 + 60 + 1.
     */
    let quadruple = 216_000 + 3600 + 60 + 1;
    let quintuple = 12_960_000 + quadruple;
    assert_eq!(tree.pointer_blocks, 1 + (60 + 1) + (3600 + 60 + 1) + quadruple + quintuple);
    assert_ne!(tree.root[ROOTS - 1], 0);
}
