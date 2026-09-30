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

//! Files past the four-level limit: one just past it, every block of it
//! found where it was written, and one of 40 GB, found at sampled blocks
//! and read by byte at offsets far past 4 GiB.

use super::deep_file::{open, write};
use super::file_consts::*;
use super::mem::byte_at;
use super::packed::DATA;
use super::tree_range::read_range;
use super::tree_store::TreeFault;

#[test]
fn a_file_that_fills_four_levels_is_still_nonosix2() {
    let (_, index) = write(FOUR_LEVEL_BLOCKS);
    assert_eq!(&index[0..8], b"NONOSIX2");
}

#[test]
fn every_block_of_a_file_just_past_four_levels_is_found() {
    let blocks = FOUR_LEVEL_BLOCKS + 3 * 60 + 7;
    let (mut store, index) = write(blocks);
    assert_eq!(&index[0..8], b"NONOSIX3");
    let mut reader = open(&index);
    for n in 0..blocks {
        assert_eq!(reader.data_lba(&mut store, n), Ok(DATA + n), "block {n}");
    }
    assert_eq!(reader.data_lba(&mut store, blocks), Err(TreeFault::Hole));
}

#[test]
fn a_40_gb_file_is_found_at_every_sampled_block_and_read_past_4_gib() {
    let size = 40_000_000_000u64;
    let blocks = size.div_ceil(DATA_BYTES as u64);
    let (mut store, index) = write(blocks);
    assert_eq!(&index[0..8], b"NONOSIX3");
    let mut reader = open(&index);
    let edges = [0, FOUR_LEVEL_BLOCKS - 1, FOUR_LEVEL_BLOCKS, blocks - 1];
    let sampled = (0..blocks).step_by(9973).chain(edges.iter().flat_map(|&e| e..e + 1));
    for n in sampled {
        assert_eq!(reader.data_lba(&mut store, n), Ok(DATA + n), "block {n}");
    }
    for at in [(4u64 << 30) + 123, 6_400_000_000, size - 1500] {
        let mut out = vec![0u8; 3000];
        let got = read_range(&mut store, &mut reader, size, at, &mut out).unwrap();
        assert_eq!(got as u64, (size - at).min(3000));
        for (k, b) in out[..got].iter().enumerate() {
            let (n, within) = ((at + k as u64) / 484, (at + k as u64) % 484);
            assert_eq!(*b, byte_at((DATA + n) * 484 + within), "byte {}", at + k as u64);
        }
    }
}
