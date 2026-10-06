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

//! Reads at any offset, and what a read costs in blocks.

use super::file_consts::DATA_BYTES;
use super::mem::{byte_at, write_file, Mem};
use super::tree_range::read_range;
use super::tree_reader::TreeReader;

#[test]
fn any_range_reads_the_bytes_at_that_offset_and_stops_at_the_end() {
    let size = 3715 * DATA_BYTES as u64 + 17;
    let mut mem = Mem::default();
    let (root, _) = write_file(&mut mem, size);
    let mut seed = 0x9e37_79b9_u64;
    for _ in 0..400 {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let offset = (seed >> 11) % (size + 600);
        let len = ((seed >> 40) % 3000) as usize;
        let mut out = vec![0u8; len];
        let n = read_range(&mut mem, &mut TreeReader::new(root), size, offset, &mut out).unwrap();
        assert_eq!(n as u64, size.saturating_sub(offset).min(len as u64));
        for (k, b) in out[..n].iter().enumerate() {
            assert_eq!(*b, byte_at(offset + k as u64));
        }
    }
}

#[test]
fn a_whole_read_fetches_each_pointer_block_once() {
    let size = 3715 * DATA_BYTES as u64;
    let mut mem = Mem::default();
    let (root, tree) = write_file(&mut mem, size);
    mem.gets = 0;
    let mut out = vec![0u8; size as usize];
    read_range(&mut mem, &mut TreeReader::new(root), size, 0, &mut out).unwrap();
    assert_eq!(mem.gets as u64, tree.data_blocks + tree.pointer_blocks);
}
