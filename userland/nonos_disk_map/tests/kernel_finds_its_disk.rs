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

//! How the kernel picks the NONOS disk: by the store's and the plan's
//! magics, where the disk's GPT puts its NONOS partitions, or at fixed
//! places on a disk whose table names none. Both are read out of the
//! kernel's source here and compared, so a kernel that looks elsewhere fails
//! this test before an installer writes a disk that kernel would pass over.

#[path = "lookup/defined.rs"]
mod defined;

use defined::{magic, rhs, source, value};
use nonos_disk_map::*;

#[test]
fn the_kernel_picks_its_disk_by_the_same_sectors() {
    let id = source("../../src/hardware/block_device/identify.rs");
    assert_eq!(rhs(&id, "STORE_MAGIC"), magic(&STORE_MAGIC));
    assert_eq!(rhs(&id, "PLAN_MAGIC"), magic(&PLAN_MAGIC));
    // Where a disk whose table names no NONOS partitions keeps them.
    let fixed = source("../../src/hardware/block_device/gpt/fixed.rs");
    assert_eq!(value(&fixed, "STORE_BASE_LBA"), STORE_BASE_LBA);
    assert_eq!(value(&fixed, "PLAN_LBA"), PLAN_LBA);
    assert_eq!(value(&fixed, "DATA_FLOOR"), DATA_FLOOR);
}
