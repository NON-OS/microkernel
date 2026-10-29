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

//! What an update leaves boots and reads as the new release: the boot
//! partition read by the FAT specification holds the newer image's files,
//! the store loads by the vfs's own decoding, the receipt names the GUIDs
//! the table already had, and the disk still reads as one an update keeps,
//! so the same update can be run over it again.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/fat_check/mod.rs"]
mod fat_check;
#[path = "common/files.rs"]
mod files;
#[path = "common/keep_disk.rs"]
mod keep_disk;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/newer.rs"]
mod newer;
#[path = "common/vfs/mod.rs"]
mod vfs;

/*
 * The vfs files name `alloc`, as a no_std capsule's do.
 */
extern crate alloc;

use nonos_disk::Keep::Ready;
use nonos_disk::{read_installed, update, StoreBuilder, StoreImage, MIN_DISK_SECTORS};
use nonos_disk_map::STORE_BASE_LBA;

#[test]
fn an_updated_disk_holds_the_newer_release_and_its_store() {
    let (mut disk, found) = keep_disk::installed(MIN_DISK_SECTORS, StoreImage::empty(), 2);
    let f = newer::newer();
    let image = files::image(&f);
    let mut store = StoreBuilder::new();
    store.add("/capsules/kept.elf", &[0x42; 9_000]).unwrap();
    let store = store.finish();
    let r = update(&mut disk, &found, &image, store.clone(), entropy::ENTROPY, &mut |_| {});
    let r = r.unwrap();
    assert_eq!((r.layout, r.disk_guid, r.partitions), (found.layout, found.disk, found.partitions));
    let esp = found.layout.esp;
    let v: fat_check::Volume = fat_check::check(&disk, esp.first);
    assert_eq!((v.boot.total, v.boot.hidden as u64), (esp.sectors, esp.first), "over its extent");
    assert_eq!((v.dirs.len(), v.used > 0), (3, true), "/EFI, /EFI/BOOT, /EFI/nonos");
    let file = |p: &str| v.files.get(p).map(Vec::as_slice);
    assert_eq!(file("/EFI/nonos/kernel.bin"), Some(image.kernel_bin));
    assert_eq!(file("/EFI/BOOT/BOOTX64.EFI"), Some(image.boot_efi));
    assert_eq!(file("/EFI/nonos/boot.cfg"), Some(image.boot_cfg));
    assert_eq!(v.files.len(), 7, "every file the image has, and no other");
    let on_disk = disk.read_sectors(STORE_BASE_LBA, store.bytes().len() / 512);
    assert_eq!(vfs::load(&on_disk), [("/capsules/kept.elf".to_string(), vec![0x42; 9_000])]);
    assert_eq!(read_installed(&mut disk).for_image(&image), Ready(found), "an update again");
}
