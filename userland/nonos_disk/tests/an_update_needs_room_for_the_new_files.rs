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

//! An update never moves the boot partition, so it goes only where the new
//! release's boot files fit the one the disk has: files too large for it
//! are refused, the files the disk was installed with fit, and so does a
//! newer release's that differs from them in every file.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/keep_disk.rs"]
mod keep_disk;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/newer.rs"]
mod newer;

use nonos_disk::Keep::No;
use nonos_disk::{read_installed, NotKept, StoreImage, MIN_DISK_SECTORS};

#[test]
fn a_boot_partition_too_small_for_the_new_files_is_refused() {
    let (mut disk, found) = keep_disk::installed(MIN_DISK_SECTORS, StoreImage::empty(), 3);
    let (f, mut big) = (files::files(), files::files());
    /* Zeroed and never touched: the placement reads only its length. */
    big.kernel_bin = vec![0u8; 1100 << 20];
    let keep = read_installed(&mut disk);
    let fits = keep.for_image(&files::image(&f));
    assert_eq!(fits, nonos_disk::Keep::NeedsPrepare(found), "the files it was installed with");
    assert_eq!(keep.for_image(&files::image(&big)), No(NotKept::EspTooSmall));
    let newer = newer::newer();
    assert_eq!(keep.for_image(&files::image(&newer)), fits, "a newer release that fits");
}
