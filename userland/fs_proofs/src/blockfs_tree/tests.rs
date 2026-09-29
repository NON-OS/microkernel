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

//! Where each block hangs, and a file on every boundary read back whole.

use super::file_consts::*;
use super::mem::{byte_at, write_file, Mem};
use super::tree_range::read_range;
use super::tree_reader::TreeReader;
use super::tree_shape::locate;

#[test]
fn each_block_is_placed_where_the_boundaries_say() {
    let at = |n: u64| locate(n).map(|p| (p.slot, p.depth, p.path));
    assert_eq!(at(53), Some((53, 0, [0, 0, 0, 0])));
    assert_eq!(at(54), Some((54, 1, [0, 0, 0, 0])));
    assert_eq!(at(113), Some((54, 1, [59, 0, 0, 0])));
    assert_eq!(at(114), Some((55, 2, [0, 0, 0, 0])));
    assert_eq!(at(3713), Some((55, 2, [59, 59, 0, 0])));
    /* Paths that read differently backwards: the top level's index leads. */
    assert_eq!(at(114 + 2 * 60 + 5), Some((55, 2, [2, 5, 0, 0])));
    assert_eq!(at(3714 + 3600 + 2 * 60 + 3), Some((56, 3, [1, 2, 3, 0])));
    assert_eq!(at(3714), Some((56, 3, [0, 0, 0, 0])));
    assert_eq!(at(219_713), Some((56, 3, [59, 59, 59, 0])));
    assert_eq!(at(219_714), Some((57, 4, [0, 0, 0, 0])));
    assert_eq!(at(MAX_FILE_BLOCKS - 1), Some((57, 4, [59, 59, 59, 59])));
    assert_eq!(at(MAX_FILE_BLOCKS), None);
}

/// Write `size` bytes, read them back whole, and check every byte.
pub fn round_trip(size: u64) -> Mem {
    let mut mem = Mem::default();
    let (root, tree) = write_file(&mut mem, size);
    assert_eq!(tree.data_blocks, size.div_ceil(DATA_BYTES as u64));
    let mut out = vec![0u8; size as usize + 7];
    let mut reader = TreeReader::new(root);
    let n = read_range(&mut mem, &mut reader, size, 0, &mut out).unwrap();
    assert_eq!(n as u64, size, "size {size}");
    for (i, b) in out[..n].iter().enumerate() {
        assert_eq!(*b, byte_at(i as u64), "size {size} byte {i}");
    }
    mem
}

#[test]
fn a_file_on_each_boundary_below_triple_reads_back_whole() {
    for blocks in [0u64, 1, 53, 54, 55, 113, 114, 115, 3713, 3714, 3715] {
        let edge = blocks * DATA_BYTES as u64;
        for size in [edge.saturating_sub(1), edge, edge + 1] {
            round_trip(size);
        }
    }
}
