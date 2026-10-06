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

//! The store an install writes is one the vfs loads: its own header and
//! table decoding reads every file back, at offsets past a full table,
//! with the digests vfs checks. An empty store loads as present and empty.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/vfs/mod.rs"]
mod vfs;

/*
 * The vfs files name `alloc`, as a no_std capsule's do.
 */
extern crate alloc;

use nonos_disk::{install, StoreBuilder, StoreImage, MIN_DISK_SECTORS};
use nonos_disk_map::STORE_BASE_LBA;

#[test]
fn vfs_loads_the_store_from_the_disk() {
    let (a, b) = (vec![0x11u8; 70_001], vec![0x22u8; 512]);
    let mut store = StoreBuilder::new();
    store.add("/nonos/setup/answers", b"NSA1\0\x01\x02").unwrap();
    store.add_all(&[("/capsules/a.elf", &a), ("/capsules/a.manifest.bin", &b)]).unwrap();
    store.add("/empty", &[]).unwrap();
    let store = store.finish();
    let sectors = store.bytes().len() / 512;
    let f = files::files();
    let mut disk = mem_disk::MemDisk::new(MIN_DISK_SECTORS);
    let r = install(&mut disk, &files::image(&f), store.clone(), entropy::ENTROPY, &mut |_| {});
    assert_eq!(r.unwrap().store_files, 4);
    let on_disk = disk.read_sectors(STORE_BASE_LBA, sectors);
    assert_eq!(on_disk, store.bytes());
    let names: Vec<(String, usize)> =
        vfs::load(&on_disk).into_iter().map(|(n, d)| (n, d.len())).collect();
    let want = [
        ("/nonos/setup/answers", 7),
        ("/capsules/a.elf", 70_001),
        ("/capsules/a.manifest.bin", 512),
        ("/empty", 0),
    ];
    assert_eq!(names, want.map(|(n, l)| (n.to_string(), l)));
    assert!(vfs::load(StoreImage::empty().bytes()).is_empty());
}
