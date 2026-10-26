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

//! The sectors vfs and the volume code name land where the kernel's table
//! reading puts them: on a disk the installer wrote, the store window is the
//! store partition sector for sector and the plan and key header its first
//! two; a table placing them elsewhere moves them, and none spills out.

use crate::kernel_gpt::{plan_at, store_at, Extent, Places, FIXED};
use nonos_disk_map::{KEY_LBA, PLAN_LBA, STORE_BASE_LBA, STORE_END_LBA};

const WINDOW: u64 = STORE_END_LBA - STORE_BASE_LBA;

#[test]
fn on_the_fixed_layout_every_sector_is_where_vfs_and_the_volume_name_it() {
    for lba in [STORE_BASE_LBA, STORE_BASE_LBA + 7, STORE_END_LBA - 64] {
        assert_eq!(store_at(&FIXED, lba, 64), Some(lba));
    }
    assert_eq!(store_at(&FIXED, STORE_END_LBA - 1, 1), Some(STORE_END_LBA - 1));
    assert_eq!(store_at(&FIXED, STORE_END_LBA - 1, 2), None, "into the plan");
    assert_eq!(store_at(&FIXED, STORE_BASE_LBA - 1, 1), None, "below the store");
    assert_eq!(store_at(&FIXED, u64::MAX, 1), None);
    assert_eq!((plan_at(&FIXED, 0), plan_at(&FIXED, 1)), (Some(PLAN_LBA), Some(KEY_LBA)));
}

#[test]
fn a_table_placing_the_store_elsewhere_moves_the_window_and_bounds_it() {
    let moved = Places {
        store: Extent { first: 4096, sectors: 1000 },
        plan: Extent { first: 9000, sectors: 2 },
        data: None,
    };
    assert_eq!(store_at(&moved, STORE_BASE_LBA, 1), Some(4096));
    assert_eq!(store_at(&moved, STORE_BASE_LBA + 999, 1), Some(5095));
    assert_eq!(store_at(&moved, STORE_BASE_LBA + 999, 2), None, "past a smaller store");
    assert_eq!((plan_at(&moved, 1), plan_at(&moved, 2)), (Some(9001), None));
    // A larger store partition does not widen what vfs may reach.
    let big = Places { store: Extent { first: 4096, sectors: WINDOW * 2 }, ..moved };
    assert_eq!(store_at(&big, STORE_END_LBA, 1), None);
}
