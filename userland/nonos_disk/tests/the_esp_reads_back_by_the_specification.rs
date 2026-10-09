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

//! The ESP an install writes, read by the FAT specification
//! (`common/fat_check`) rather than by the writer: on the smallest disk
//! NONOS installs to, 2177 MiB; on one past 2 TiB, where the ESP starts
//! beyond what the boot sector's 32-bit hidden-sector field can name; and
//! with and without the kernel's approval. Every chain, every table, every
//! long name, the FSInfo counts, and every file byte for byte under the
//! name the loader opens it by.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/fat_check/mod.rs"]
mod fat_check;
#[path = "common/files.rs"]
mod files;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use std::collections::BTreeMap;

use nonos_disk::{install, NonosImage, StoreImage, MIN_DISK_SECTORS, STARTUP_NSH};

fn expected(image: &NonosImage<'_>) -> BTreeMap<String, Vec<u8>> {
    let mut want = BTreeMap::new();
    let mut put = |p: &str, d: &[u8]| want.insert(p.to_string(), d.to_vec());
    put("/EFI/BOOT/BOOTX64.EFI", image.boot_efi);
    put("/EFI/nonos/kernel.bin", image.kernel_bin);
    put("/EFI/nonos/bootloader.trailer", image.boot_trailer);
    put("/EFI/nonos/boot_root.approval", image.boot_root);
    put("/EFI/nonos/boot.cfg", image.boot_cfg);
    if let Some(a) = image.kernel_approval {
        put("/EFI/nonos/kernel.approval", a);
    }
    put("/startup.nsh", STARTUP_NSH);
    want
}

fn installed(sectors: u64, image: &NonosImage<'_>) -> fat_check::Volume {
    let mut disk = mem_disk::MemDisk::new(sectors);
    let r = install(&mut disk, image, StoreImage::empty(), entropy::ENTROPY, &mut |_| {}).unwrap();
    let v = fat_check::check(&disk, r.layout.esp.first);
    assert_eq!(v.files, expected(image), "on {sectors} sectors");
    assert_eq!(v.dirs, ["/EFI", "/EFI/BOOT", "/EFI/nonos"]);
    assert_eq!(v.boot.total, r.layout.esp.sectors, "the boot sector's total is the partition");
    assert_eq!(v.boot.clusters, r.geometry.data_clusters, "the writer counts what a driver counts");
    let hidden = u32::try_from(r.layout.esp.first).unwrap_or(u32::MAX);
    assert_eq!(v.boot.hidden, hidden, "hidden sectors");
    v
}

#[test]
fn the_esp_on_the_smallest_disk_reads_back_by_the_specification() {
    let f = files::files();
    let v = installed(MIN_DISK_SECTORS, &files::image(&f));
    assert_eq!(v.boot.sectors_per_cluster, 8, "4 KiB clusters on the one-GiB ESP");
}

#[test]
fn the_esp_past_two_tib_reads_back_with_its_hidden_sectors_saturated() {
    let f = files::files();
    let small = installed(MIN_DISK_SECTORS, &files::image(&f));
    for sectors in [(1u64 << 32) + 4_000_000, 1 << 33, 1 << 40] {
        let v = installed(sectors, &files::image(&f));
        assert_eq!(v.boot.hidden, u32::MAX, "{sectors}");
        assert_eq!((v.boot.clusters, v.used), (small.boot.clusters, small.used), "{sectors}");
    }
}

#[test]
fn the_esp_without_the_kernel_approval_reads_back_by_the_specification() {
    let f = files::files();
    let mut image = files::image(&f);
    image.kernel_approval = None;
    installed(MIN_DISK_SECTORS + 12_345, &image);
}

/*
 * Files whose sizes sit on and either side of every boundary the writer
 * rounds at: a sector, a cluster, and a long run of clusters, so a chain
 * one too short or a tail one sector too long would show.
 */
#[test]
fn files_at_every_rounding_boundary_have_exactly_their_chains() {
    let f = files::files();
    for size in [1usize, 511, 512, 513, 4095, 4096, 4097, 8192, 65_537, 1 << 20] {
        let (efi, trailer) = (vec![0xA5u8; size], vec![0x5Au8; size + 1]);
        let mut image = files::image(&f);
        image.boot_efi = &efi;
        image.boot_trailer = &trailer;
        installed(MIN_DISK_SECTORS, &image);
    }
}
