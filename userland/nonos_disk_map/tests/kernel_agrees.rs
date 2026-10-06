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

//! The kernel keeps its own copies of the map. Each is read out of its
//! source here and compared, so a kernel that moves the store, the plan,
//! the key header or the data floor fails this test before an installer
//! writes a disk that kernel would not find.

#[path = "lookup/defined.rs"]
mod defined;

use defined::{magic, rhs, source, value};
use nonos_disk_map::*;

#[test]
fn the_kernel_picks_its_disk_by_the_same_sectors() {
    let id = source("src/hardware/block_device/identify.rs");
    assert_eq!(value(&id, "STORE_LBA"), STORE_BASE_LBA);
    assert_eq!(rhs(&id, "STORE_MAGIC"), magic(&STORE_MAGIC));
    assert_eq!(rhs(&id, "PLAN_MAGIC"), magic(&PLAN_MAGIC));
}

#[test]
fn the_kernel_reads_the_plan_and_key_header_where_the_map_says() {
    let plan = source("src/fs/blockfs_volume/plan_types.rs");
    assert_eq!(value(&plan, "PLAN_LBA"), PLAN_LBA);
    assert_eq!(value(&plan, "DATA_FLOOR"), DATA_FLOOR);
    assert_eq!(value(&plan, "MIN_VOLUME"), MIN_VOLUME_SECTORS);
    assert_eq!(rhs(&plan, "MAGIC").trim_start_matches('*'), magic(&PLAN_MAGIC));
    let key = source("src/fs/blockfs_volume/key_header.rs");
    assert_eq!(rhs(&key, "KEY_LBA"), "PLAN_LBA + 1");
    assert_eq!(KEY_LBA, PLAN_LBA + 1);
    assert_eq!(rhs(&key, "MAGIC").trim_start_matches('*'), magic(&KEY_MAGIC));
    let ring = source("src/fs/blockfs/constants.rs");
    assert_eq!(value(&ring, "HEADER_RING_SECTORS"), HEADER_RING_SECTORS);
}

#[test]
fn the_kernel_serves_the_store_window_the_map_names() {
    for call in ["store_read.rs", "store_write.rs"] {
        let src = source(&format!("src/syscall/microkernel/{call}"));
        assert_eq!(value(&src, "STORE_BASE_LBA"), STORE_BASE_LBA, "{call}");
        assert!(src.contains("blockfs_volume::PLAN_LBA"), "{call} ends the store at the plan");
    }
    assert_eq!(STORE_END_LBA, PLAN_LBA);
}
