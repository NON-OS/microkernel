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

//! A file of numbered data blocks in the packed store, closed with its
//! index block, and opened again as the kernel's read opens it.

use super::file_consts::DATA_BYTES;
use super::index_block::{decode, encode, Index};
use super::packed::{Packed, DATA};
use super::tree_reader::TreeReader;
use super::tree_writer::TreeWriter;

/// Write `blocks` numbered data blocks; the store and the file's index.
pub fn write(blocks: u64) -> (Packed, [u8; DATA_BYTES]) {
    let (mut store, mut tree) = (Packed::default(), TreeWriter::new());
    for n in 0..blocks {
        tree.push(&mut store, DATA + n).unwrap();
    }
    tree.finish(&mut store).unwrap();
    assert_eq!(tree.data_blocks, blocks);
    (store, encode(&tree.root, tree.data_blocks))
}

/// The reader the index block opens, as the kernel's read opens it.
pub fn open(index: &[u8; DATA_BYTES]) -> TreeReader {
    let Some(Index::Tree(root)) = decode(index) else { panic!("not a tree file") };
    TreeReader::new(root)
}
