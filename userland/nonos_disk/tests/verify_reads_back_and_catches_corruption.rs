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

//! The read-back passes on an intact disk and covers every byte that stays
//! written, and it names the first bad sector after one byte is flipped in
//! each thing an install writes: a kernel image sector deep in the ESP, the
//! FAT's reserved area, the store header, the plan, the key header, the
//! data volume's ring, and the GPT headers and entries. A verifier that
//! cannot fail is not a verifier.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use mem_disk::MemDisk;
use nonos_disk::{install, verify, BlockSink, StoreImage, WriteError, MIN_DISK_SECTORS};
use nonos_disk_map::PLAN_LBA;

fn flip(disk: &mut MemDisk, lba: u64) {
    let mut sector = disk.read_sectors(lba, 1);
    sector[17] ^= 0x01;
    disk.write_at(lba, &sector).unwrap();
}

#[test]
fn verify_reads_back_and_catches_corruption() {
    let f = files::files();
    let mut disk = MemDisk::new(MIN_DISK_SECTORS);
    let (image, store) = (files::image(&f), StoreImage::empty());
    let r = install(&mut disk, &image, store, entropy::ENTROPY, &mut |_| {}).unwrap();
    /*
     * Everything but the five sectors of the wipe, which what follows them
     * overwrites: the old MBR and GPT header, the old backup header, and the
     * old store header and plan.
     */
    assert_eq!(verify(&mut disk, &r, &mut |_| {}), Ok(r.bytes_written - 5 * 512));
    let kernel = r.files.iter().find(|fr| fr.data.len() == f.kernel_bin.len()).unwrap();
    let l = r.layout;
    let victims = [
        (kernel.lba + 4000, "the boot partition"),
        (l.esp.first + 6, "the boot partition"),
        (256, "the store"),
        (PLAN_LBA, "the disk plan"),
        (PLAN_LBA + 1, "the key header"),
        (l.data.first + 7, "the data volume"),
        (1, "the GPT header"),
        (3, "the GPT entries"),
        (l.backup_header_lba, "the backup GPT header"),
    ];
    for (lba, what) in victims {
        flip(&mut disk, lba);
        assert_eq!(verify(&mut disk, &r, &mut |_| {}), Err(WriteError::Mismatch { lba }), "{what}");
        assert_eq!(l.what_is_at(lba), what);
        flip(&mut disk, lba);
    }
}
