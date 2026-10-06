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

/*
 * With no record from the loader, the disk that holds the running loader
 * at \EFI\BOOT\BOOTX64.EFI is the boot media: on a vvfat FAT16 stick with
 * a table or without one, and on a NONOS disk's FAT32 ESP. A loader of
 * another length or other first bytes is not found, a blank disk is never
 * the boot media, and a record, when there is one, is all that is asked.
 */

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/fat16_sectors.rs"]
mod fat16_sectors;
#[path = "common/fat16_stick.rs"]
mod fat16_stick;
#[path = "common/files.rs"]
mod files;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use fat16_stick::stick;
use mem_disk::MemDisk;
use nonos_disk::{install, is_boot_media, BootEvidence, BootPartition, StoreImage};
use nonos_disk::{MIN_DISK_SECTORS, SIGNATURE_GUID, TABLE_GPT};

fn found(disk: &mut MemDisk, loader: &[u8], size: u64) -> bool {
    let head = &loader[..loader.len().min(4096)];
    is_boot_media(disk, &BootEvidence { partition: None, loader_size: size, loader_head: head })
}

#[test]
fn the_boot_stick_is_found_by_its_loader() {
    let loader: Vec<u8> = (0..3000u32).map(|i| (i * 7 + 3) as u8).collect();
    let len = loader.len() as u64;
    assert!(found(&mut stick(true, &loader), &loader, len));
    assert!(found(&mut stick(false, &loader), &loader, len));
    assert!(!found(&mut stick(true, &loader), &loader, len + 1));
    let mut other = loader.clone();
    other[100] ^= 1;
    assert!(!found(&mut stick(true, &loader), &other, len));
    assert!(!found(&mut MemDisk::new(MIN_DISK_SECTORS), &loader, len));

    let f = files::files();
    let mut disk = MemDisk::new(MIN_DISK_SECTORS);
    install(&mut disk, &files::image(&f), StoreImage::empty(), entropy::ENTROPY, &mut |_| {})
        .unwrap();
    assert!(found(&mut disk, &f.boot_efi, f.boot_efi.len() as u64));
    assert!(!found(&mut disk, &loader, len));

    let p = BootPartition {
        number: 1,
        table: TABLE_GPT,
        signature_type: SIGNATURE_GUID,
        start_lba: 2048,
        size_lba: 2048,
        signature: [7; 16],
    };
    let ev = BootEvidence { partition: Some(p), loader_size: len, loader_head: &loader };
    assert!(!is_boot_media(&mut stick(true, &loader), &ev));
}
