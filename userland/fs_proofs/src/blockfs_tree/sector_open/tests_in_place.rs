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

//! A range read hands each whole data block to `get_into`, straight into
//! its place in the output, and only a block it wants part of to `get`.

use crate::blockfs_tree::tree_range::read_range;
use crate::blockfs_tree::tree_reader::TreeReader;
use crate::blockfs_tree::tree_store::BlockStore;
use crate::blockfs_tree::tree_writer::TreeWriter;

use super::in_place_store::InPlace;
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES as DATA_BYTES;

fn byte_at(i: u64) -> u8 {
    (i.wrapping_mul(2_654_435_761) >> 13) as u8
}

#[test]
fn whole_blocks_open_in_place_and_only_the_ends_are_copied() {
    let size = 700 * DATA_BYTES as u64 + 5;
    let mut disk = InPlace::default();
    let mut tree = TreeWriter::new();
    for n in 0..size.div_ceil(DATA_BYTES as u64) {
        let block = core::array::from_fn(|k| byte_at(n * DATA_BYTES as u64 + k as u64));
        let lba = disk.alloc().unwrap();
        disk.put(lba, &block).unwrap();
        tree.push(&mut disk, lba).unwrap();
    }
    tree.finish(&mut disk).unwrap();
    for (offset, len, parts, wholes) in [
        (0, 10 * DATA_BYTES, 0, 10),
        (100, 10 * DATA_BYTES, 2, 9),
        (7, 20, 1, 0),
        (0, 1 << 20, 1, 700),
    ] {
        (disk.parts, disk.wholes) = (0, 0);
        let mut out = vec![0u8; len];
        let mut reader = TreeReader::new(tree.root);
        let n = read_range(&mut disk, &mut reader, size, offset, &mut out).unwrap();
        assert_eq!(n as u64, (size - offset).min(len as u64));
        assert!(out[..n].iter().enumerate().all(|(k, b)| *b == byte_at(offset + k as u64)));
        assert_eq!((disk.parts, disk.wholes), (parts, wholes), "at {offset} for {len}");
    }
}
