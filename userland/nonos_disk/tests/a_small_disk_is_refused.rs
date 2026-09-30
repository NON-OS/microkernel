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

//! A disk below the minimum is refused before a byte is written, and the
//! refusal says how big it was and how big it had to be: 2113 MiB for any
//! image that fits the one-GiB ESP, in words both installers print. A disk
//! of exactly the minimum installs.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use nonos_disk::{install, StoreImage, WriteError, MIN_DISK_SECTORS};

#[test]
fn a_small_disk_is_refused() {
    let f = files::files();
    let image = files::image(&f);
    for total in [16_384, MIN_DISK_SECTORS - 2048, MIN_DISK_SECTORS - 1] {
        let mut disk = mem_disk::MemDisk::new(total);
        let r = install(&mut disk, &image, StoreImage::empty(), entropy::ENTROPY, &mut |_| {});
        let needed_sectors = MIN_DISK_SECTORS;
        assert_eq!(
            r.err(),
            Some(WriteError::DiskTooSmall { total_sectors: total, needed_sectors })
        );
        assert!(disk.written.is_empty(), "nothing was written to {total} sectors");
    }
    let refusal =
        WriteError::DiskTooSmall { total_sectors: 16_384, needed_sectors: MIN_DISK_SECTORS };
    let words = "the disk holds 8.4 MB; NONOS needs a disk of at least 2.2 GB (2113 MiB)";
    assert_eq!(refusal.to_string(), words);
    let mut disk = mem_disk::MemDisk::new(MIN_DISK_SECTORS);
    let r = install(&mut disk, &image, StoreImage::empty(), entropy::ENTROPY, &mut |_| {});
    assert!(r.is_ok(), "{r:?}");
}
