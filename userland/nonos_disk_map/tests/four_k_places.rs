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

//! Every place on a NONOS disk starts and ends on a 4 KiB boundary, so a
//! disk of 4096-byte logical blocks holds each at the byte offset a disk of
//! 512-byte sectors does, and the block layer's mapping reaches it in whole
//! blocks. The key header alone does not: it lies in the plan's block, and
//! nothing else does, so the only writes that patch that block are the
//! plan's and the key header's.

use nonos_disk_map::*;

/// The block, of `MAX_BLOCK_BYTES`, that sector `lba` lies in.
fn block(lba: u64) -> u64 {
    lba * SECTOR_SIZE as u64 / MAX_BLOCK_BYTES as u64
}

#[test]
fn the_largest_block_is_fat32s_largest_sector() {
    assert!(SECTOR_SIZE.is_power_of_two() && MAX_BLOCK_BYTES.is_power_of_two());
    assert_eq!(MAX_BLOCK_BYTES % SECTOR_SIZE, 0, "whole sectors");
    assert_eq!(MAX_BLOCK_BYTES, 4096, "BytsPerSec is 512, 1024, 2048 or 4096");
}

#[test]
fn every_place_starts_and_ends_on_a_block() {
    let places = [
        ("the store's first sector", STORE_BASE_LBA),
        ("the sector past the store", STORE_END_LBA),
        ("the plan", PLAN_LBA),
        ("the data floor", DATA_FLOOR),
        ("the header ring's length", HEADER_RING_SECTORS),
    ];
    for (what, lba) in places {
        let bytes = lba * SECTOR_SIZE as u64;
        assert_eq!(bytes % MAX_BLOCK_BYTES as u64, 0, "{what}, sector {lba}");
    }
}

#[test]
fn the_key_header_shares_the_plans_block_and_nothing_else_does() {
    assert_ne!(KEY_LBA, PLAN_LBA);
    assert_eq!(block(KEY_LBA), block(PLAN_LBA), "the plan's block");
    assert!(block(STORE_END_LBA - 1) < block(PLAN_LBA), "the store ends before it");
    assert!(block(DATA_FLOOR) > block(PLAN_LBA), "the data volume starts after it");
}

/*
 * The firmware reads the protective MBR, the primary GPT header and an
 * entry array of 128 entries of 128 bytes in the disk's own blocks: on
 * 4096-byte blocks they end at block 6, sector 48. The store starts past
 * them on any block the map allows.
 */
#[test]
fn the_store_starts_past_the_primary_gpt_on_the_largest_block() {
    let gpt_end = 2 * MAX_BLOCK_BYTES as u64 + 128 * 128;
    assert_eq!(gpt_end / SECTOR_SIZE as u64, 48);
    assert!(STORE_BASE_LBA * SECTOR_SIZE as u64 >= gpt_end);
}
