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

//! An independent FAT implementation reads the volume: mtools, the same
//! tool the USB image build uses to write one. Every file comes back byte
//! for byte through a reader that shares no code with this writer, from
//! the ESP at the end of a disk of the minimum size, saved as a sparse file.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/mem_disk.rs"]
mod mem_disk;

use std::io::{Seek, SeekFrom, Write};
use std::process::Command;

use nonos_disk::{install, StoreImage, MIN_DISK_SECTORS, SECTOR_SIZE};

fn run(tool: &str, img: &str, args: &[&str]) -> Vec<u8> {
    let out = Command::new(tool).args(["-i", img]).args(args).output().expect("mtools runs");
    assert!(out.status.success(), "{tool} {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    out.stdout
}

#[test]
fn mtools_reads_every_file_back() {
    if Command::new("mtype").arg("-V").output().is_err() {
        eprintln!("mtools not installed; skipping");
        return;
    }
    let f = files::files();
    let mut disk = mem_disk::MemDisk::new(MIN_DISK_SECTORS);
    let (image, store) = (files::image(&f), StoreImage::empty());
    let r = install(&mut disk, &image, store, entropy::ENTROPY, &mut |_| {}).unwrap();
    let dir = std::env::temp_dir().join(format!("nonos_disk_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("disk.img");
    let mut file = std::fs::File::create(&path).unwrap();
    file.set_len(MIN_DISK_SECTORS * SECTOR_SIZE as u64).unwrap();
    for (lba, sector) in &disk.written {
        file.seek(SeekFrom::Start(lba * SECTOR_SIZE as u64)).unwrap();
        file.write_all(sector).unwrap();
    }
    let img = format!("{}@@{}", path.display(), r.layout.esp.first * SECTOR_SIZE as u64);

    assert_eq!(run("mtype", &img, &["::/EFI/BOOT/BOOTX64.EFI"]), f.boot_efi);
    assert_eq!(run("mtype", &img, &["::/EFI/nonos/kernel.bin"]), f.kernel_bin);
    assert_eq!(run("mtype", &img, &["::/EFI/nonos/boot.cfg"]), f.boot_cfg);
    assert_eq!(run("mtype", &img, &["::/startup.nsh"]), nonos_disk::STARTUP_NSH);
    /*
     * The listing shows the names as written, lowercase where the image
     * spells them lowercase, and nothing else in the root.
     */
    let listing = String::from_utf8_lossy(&run("mdir", &img, &["-b", "::/"])).into_owned();
    assert!(listing.contains("::/EFI/") && listing.contains("::/startup.nsh"), "{listing}");
    std::fs::remove_dir_all(&dir).unwrap();
}
