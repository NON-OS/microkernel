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

//! The index block in each layout: NONOSIX2 byte for byte as it was written
//! before the fifth level, NONOSIX3 only for a file that needs the fifth.

use super::file_consts::*;
use super::index_block::{decode, encode, Index};
use super::mem::{byte_at, write_file, Mem};
use super::read_u64::read_u64;
use super::tree_range::read_range;
use super::tree_reader::TreeReader;
use super::tree_store::Block;

/// The index block as the kernel wrote it before the fifth level: magic,
/// the data block count as a u32, then the 58 slots.
fn written_before(root: &[u64; ROOTS], data_blocks: u64) -> Block {
    let mut b = [0u8; DATA_BYTES];
    b[0..8].copy_from_slice(b"NONOSIX2");
    b[8..12].copy_from_slice(&(data_blocks as u32).to_le_bytes());
    for (i, lba) in root[..MAX_PTRS].iter().enumerate() {
        b[16 + i * 8..24 + i * 8].copy_from_slice(&lba.to_le_bytes());
    }
    b
}

#[test]
fn a_file_within_four_levels_gets_the_index_block_it_always_did() {
    let mut mem = Mem::default();
    let (root, tree) = write_file(&mut mem, 3715 * DATA_BYTES as u64 + 5);
    assert_eq!(root[ROOTS - 1], 0);
    assert_eq!(encode(&root, tree.data_blocks)[..], written_before(&root, tree.data_blocks)[..]);
}

#[test]
fn a_nonosix2_file_an_older_kernel_wrote_still_reads_back_whole() {
    let size = 3715 * DATA_BYTES as u64 + 17;
    let mut mem = Mem::default();
    let (root, tree) = write_file(&mut mem, size);
    let Some(Index::Tree(got)) = decode(&written_before(&root, tree.data_blocks)) else {
        panic!("a NONOSIX2 block not read as a tree file");
    };
    assert_eq!(got, root);
    let mut out = vec![0u8; size as usize];
    let n = read_range(&mut mem, &mut TreeReader::new(got), size, 0, &mut out).unwrap();
    assert_eq!(n as u64, size);
    assert!(out.iter().enumerate().all(|(i, b)| *b == byte_at(i as u64)));
}

#[test]
fn the_fifth_root_is_kept_in_the_header_of_a_nonosix3_block() {
    let root: [u64; ROOTS] = core::array::from_fn(|i| 1000 + i as u64);
    let b = encode(&root, FOUR_LEVEL_BLOCKS + 1);
    assert_eq!(&b[0..8], b"NONOSIX3");
    assert_eq!(read_u64(&b, INDEX_DEEP_OFFSET), 1000 + MAX_PTRS as u64);
    assert!(b[INDEX_PTR_BASE + MAX_PTRS * PTR_BYTES..].iter().all(|x| *x == 0));
    let Some(Index::Tree(got)) = decode(&b) else { panic!("a NONOSIX3 block not read") };
    assert_eq!(got, root);
}
