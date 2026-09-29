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
use crate::fs::cryptoblock::{AAD_PREFIX, NONCE_BYTES, SECTOR_BYTES, TAG_BYTES};

#[test]
fn the_two_index_layouts_are_told_apart_before_a_pointer_is_read() {
    assert_ne!(INDEX_MAGIC, INDEX_MAGIC_FLAT);
    /*
     * The block count sits in the header, clear of the first root slot.
     */
    assert!(INDEX_COUNT_OFFSET + 4 <= INDEX_PTR_BASE);
    /*
     * 16 + 58 * 8 = 480 of 484: the 4 left over cannot hold a 59th slot.
     */
    assert_eq!(DATA_BYTES - (INDEX_PTR_BASE + MAX_PTRS * PTR_BYTES), 4);
    /*
     * A pointer block's seal binds it to its LBA, which is why it has no magic.
     */
    assert_eq!(AAD_PREFIX.len(), 16);
}

#[test]
fn the_largest_file_is_the_one_the_comment_works_out() {
    assert_eq!(DATA_BYTES, SECTOR_BYTES - NONCE_BYTES - TAG_BYTES);
    assert_eq!((DATA_BYTES, MAX_PTRS, FANOUT, DIRECT_SLOTS), (484, 58, 60, 54));
    assert_eq!(MAX_FILE_BLOCKS, 13_179_714);
    assert_eq!(MAX_FILE_BYTES, 6_378_981_576);
    /*
     * The flat layout before this held 28,072 bytes; it must never come back.
     */
    assert!(MAX_FILE_BYTES > 6_000_000_000);
}
