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

//! The bootloader keeps its own copies of the map. It copies the store
//! through the firmware's disk driver before the kernel runs
//! (`nonos-bootloader/src/loader/file/store_copy.rs` `copy_from`, `span`)
//! and reads a live plan's imports (`disk_mirror/plan.rs` `live_imports`).
//! The numbers are read out of those sources here and compared: where the
//! store begins and ends and its magic, the container's header length,
//! entry length and entry count, where the plan lies, the data floor and
//! the plan's magic, and that the loader's page holds the largest block.
//! Not the whole container: an entry's inner offsets are not read. So a
//! loader that moves the store or the plan, resizes the table, or shrinks
//! its page below a block fails this test, before an installer writes a
//! disk that loader would misread.

#[path = "lookup/defined.rs"]
mod defined;

use defined::{magic, rhs, source, value};
use nonos_disk_map::*;

const STORE_COPY: &str = "../../nonos-bootloader/src/loader/file/store_copy.rs";
const PLAN: &str = "../../nonos-bootloader/src/loader/file/disk_mirror/plan.rs";
const PAGES: &str = "../../nonos-bootloader/src/loader/file/pages.rs";

#[test]
fn the_loader_copies_the_store_from_where_the_map_puts_it() {
    let copy = source(STORE_COPY);
    assert_eq!(value(&copy, "STORE_AT"), STORE_BASE_LBA * SECTOR_SIZE as u64);
    assert_eq!(value(&copy, "PLAN_AT"), STORE_END_LBA * SECTOR_SIZE as u64);
    assert_eq!(rhs(&copy, "MAGIC"), magic(&STORE_MAGIC));
}

#[test]
fn the_loader_reads_the_container_the_map_describes() {
    let copy = source(STORE_COPY);
    assert_eq!(value(&copy, "HEADER"), HEADER_LEN as u64);
    assert_eq!(value(&copy, "TOC_ENTRY"), ENTRY_LEN as u64);
    assert_eq!(value(&copy, "MAX_ENTRIES"), MAX_ENTRIES as u64);
}

#[test]
fn the_loader_reads_the_plan_where_the_map_says() {
    let plan = source(PLAN);
    assert_eq!(value(&plan, "PLAN_LBA"), PLAN_LBA);
    assert_eq!(value(&plan, "DATA_FLOOR"), DATA_FLOOR);
    assert_eq!(rhs(&plan, "MAGIC"), magic(&PLAN_MAGIC));
}

/*
 * copy_from passes over a disk whose block does not divide the loader's
 * page, so the page must hold the largest block a NONOS disk is laid out
 * on, or the loader would find no store on such a disk.
 */
#[test]
fn the_loader_reads_the_store_on_the_largest_block() {
    assert!(source(STORE_COPY).contains("PAGE as u64 % block != 0"), "the page bounds the block");
    assert_eq!(value(&source(PAGES), "PAGE") % MAX_BLOCK_BYTES as u64, 0);
}
