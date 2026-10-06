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

//! The layout's numbers, worked out in file_consts.rs and held here.

use super::file_consts::*;
use super::index_block::{decode, Index};
use crate::fs::cryptoblock::{AAD_PREFIX, NONCE_BYTES, SECTOR_BYTES, TAG_BYTES};

#[test]
fn the_three_index_layouts_are_told_apart_before_a_pointer_is_read() {
    assert_ne!(INDEX_MAGIC, INDEX_MAGIC_FLAT);
    assert_ne!(INDEX_MAGIC_DEEP, INDEX_MAGIC_FLAT);
    assert_ne!(INDEX_MAGIC_DEEP, INDEX_MAGIC);
    /*
     * The block count, or the fifth root in its place, sits in the header,
     * clear of the first slot.
     */
    const { assert!(INDEX_COUNT_OFFSET + 4 <= INDEX_PTR_BASE) };
    const { assert!(INDEX_DEEP_OFFSET + PTR_BYTES <= INDEX_PTR_BASE) };
    /*
     * 16 + 58 * 8 = 480 of 484: the 4 left over cannot hold a 59th slot,
     * which is why the fifth root lives in the header.
     */
    assert_eq!(DATA_BYTES - (INDEX_PTR_BASE + MAX_PTRS * PTR_BYTES), 4);
    assert_eq!((ROOTS, LEVELS), (MAX_PTRS + 1, 5));
    /*
     * A pointer block's seal binds it to its LBA, which is why it has no magic.
     */
    assert_eq!(AAD_PREFIX.len(), 16);
}

#[test]
fn the_largest_file_is_the_one_the_comment_works_out() {
    assert_eq!(DATA_BYTES, SECTOR_BYTES - NONCE_BYTES - TAG_BYTES);
    assert_eq!((DATA_BYTES, MAX_PTRS, FANOUT, DIRECT_SLOTS), (484, 58, 60, 54));
    assert_eq!(FOUR_LEVEL_BLOCKS, 13_179_714);
    assert_eq!(FOUR_LEVEL_BLOCKS * DATA_BYTES as u64, 6_378_981_576);
    assert_eq!(MAX_FILE_BLOCKS, 790_779_714);
    assert_eq!(MAX_FILE_BYTES, 382_737_381_576);
    /*
     * The largest official Qwen3 file is near 35 GB; one file must reach 64 GB.
     */
    const { assert!(MAX_FILE_BYTES >= 64 << 30) };
}

#[test]
fn a_flat_or_unknown_index_block_is_told_apart() {
    let mut b = [0u8; DATA_BYTES];
    b[0..8].copy_from_slice(b"NONOSIX1");
    assert!(matches!(decode(&b), Some(Index::Flat(kept)) if kept == b));
    b[0..8].copy_from_slice(b"NONOSIX4");
    assert!(decode(&b).is_none());
}
