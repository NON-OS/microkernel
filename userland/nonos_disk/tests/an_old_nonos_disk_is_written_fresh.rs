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

//! An install over a disk that held NONOS: the old key header and the old
//! volume's header ring are replaced by a cleared header and a blank ring,
//! the plan names the new data volume, and the store header is where the
//! kernel looks for it. At the minimum, just above it, and past 2 TiB.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use nonos_disk::{install, plan_sector, BlockSink, StoreImage, MIN_DISK_SECTORS};
use nonos_disk_map::{DATA_FLOOR, KEY_LBA, PLAN_LBA, STORE_BASE_LBA};

#[test]
fn an_old_nonos_disk_is_written_fresh() {
    let f = files::files();
    for total in [MIN_DISK_SECTORS, MIN_DISK_SECTORS + 2047, (1u64 << 32) + (1 << 21)] {
        let mut disk = mem_disk::MemDisk::new(total);
        disk.write_at(KEY_LBA, &[0x4E; 512]).unwrap();
        disk.write_at(DATA_FLOOR, &[0xAB; 256 * 512]).unwrap();
        disk.write_at(PLAN_LBA, &[0x77; 512]).unwrap();
        let (image, store) = (files::image(&f), StoreImage::empty());
        let r = install(&mut disk, &image, store, entropy::ENTROPY, &mut |_| {}).unwrap();
        assert_eq!(disk.read_sectors(PLAN_LBA, 1), plan_sector(&r.layout.data), "{total}");
        assert_eq!(disk.read_sectors(KEY_LBA, 1), vec![0u8; 512], "a cleared key header");
        let ring = disk.read_sectors(r.layout.data.first, 256);
        assert!(ring.iter().all(|&b| b == 0), "a blank header ring");
        let store = disk.read_sectors(STORE_BASE_LBA, 1);
        assert_eq!(&store[..16], b"NONOSTR1\x01\0\0\0\0\0\0\0", "an empty store");
    }
}
