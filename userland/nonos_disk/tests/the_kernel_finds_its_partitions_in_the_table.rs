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

//! The kernel finds, in the table the installer writes, the store, plan and
//! data partitions where the layout put them, and the ESP among them.

extern crate alloc;

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/kernel_gpt/mod.rs"]
#[allow(dead_code, unused_imports)]
mod kernel_gpt;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/written.rs"]
mod written;

use kernel_gpt::{entries, header, holds, places, Entry, Extent, Places, FIXED};
use kernel_gpt::Which;
use mem_disk::MemDisk;
use nonos_disk::{Guid, MIN_DISK_SECTORS};
use written::written;

fn read(disk: &MemDisk, total: u64) -> Option<Vec<Entry>> {
    let h = header::parse_as(&disk.read_sectors(1, 1), total, Which::Primary)?;
    entries(&h, &disk.read_sectors(h.entries_lba, h.array_sectors() as usize))
}

fn ext(x: nonos_disk::Extent) -> Extent {
    Extent { first: x.first, sectors: x.sectors }
}

#[test]
fn the_kernel_finds_each_partition_where_the_layout_put_it() {
    for total in [MIN_DISK_SECTORS, (1u64 << 32) + (1 << 21) + 5] {
        let (disk, l, esp) = written(total);
        let e = read(&disk, total).expect("the table the installer wrote");
        let want = Places { store: ext(l.store), plan: ext(l.plan), data: Some(ext(l.data)) };
        assert_eq!(places(&e), Some(want), "{total}");
        assert_eq!(want.data.map(|d| d.end()), Some(l.esp.first), "data runs up to the ESP");
        // Today's layout is the fixed one, so a disk read either way agrees.
        assert_eq!((want.store, want.plan), (FIXED.store, FIXED.plan));
        assert!(holds(&e, &esp), "the ESP's GUID");
        assert!(!holds(&e, &[0x5A; 16]));
    }
}

#[test]
fn the_kernels_types_are_the_ones_written() {
    use kernel_gpt::places::{NONOS_DATA, NONOS_PLAN, NONOS_STORE};
    assert_eq!(NONOS_STORE, Guid::NONOS_STORE.0);
    assert_eq!(NONOS_PLAN, Guid::NONOS_PLAN.0);
    assert_eq!(NONOS_DATA, Guid::NONOS_DATA.0);
}
