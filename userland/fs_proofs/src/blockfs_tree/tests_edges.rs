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

//! The triple and quadruple levels, and a hole.

use super::file_consts::*;
use super::mem::{write_file, Mem};
use super::tests::round_trip;
use super::tree_ptrs::entry;
use super::tree_range::read_range;
use super::tree_reader::TreeReader;
use super::tree_store::{BlockSource, TreeFault};

const TRIPLE_END: u64 = 219_714;

#[test]
fn a_file_crossing_from_triple_into_quadruple_reads_back_whole() {
    let edge = TRIPLE_END * DATA_BYTES as u64;
    for size in [edge - 1, edge, edge + DATA_BYTES as u64 + 1] {
        round_trip(size);
    }
}

#[test]
fn a_zeroed_pointer_under_the_size_is_a_hole_not_a_read_of_lba_zero() {
    let size = 200 * DATA_BYTES as u64;
    let mut mem = Mem::default();
    let (root, _) = write_file(&mut mem, size);
    let top = mem.get(root[DIRECT_SLOTS + 1]).unwrap();
    let leaf = entry(&top, 0) as usize;
    mem.blocks[leaf - 1] = [0u8; DATA_BYTES];
    let mut out = vec![0u8; size as usize];
    let got = read_range(&mut mem, &mut TreeReader::new(root), size, 0, &mut out);
    assert_eq!(got, Err(TreeFault::Hole));
}
