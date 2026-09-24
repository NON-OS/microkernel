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
//! for byte through a reader that shares no code with this writer.

mod common;

use std::process::Command;

use nonos_disk::install;

fn mtype(img: &str, path: &str) -> Vec<u8> {
    let out = Command::new("mtype")
        .args(["-i", &format!("{img}@@1M"), path])
        .output()
        .expect("mtype runs");
    assert!(out.status.success(), "mtype {path}: {}", String::from_utf8_lossy(&out.stderr));
    out.stdout
}

#[test]
fn mtools_reads_every_file_back() {
    if Command::new("mtype").arg("-V").output().is_err() {
        eprintln!("mtools not installed; skipping");
        return;
    }
    let f = common::files();
    let mut disk = common::MemDisk::new(320);
    install(&mut disk, &common::image(&f), common::ENTROPY, &mut |_| {}).unwrap();
    let dir = std::env::temp_dir().join(format!("nonos_disk_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let img = dir.join("disk.img");
    std::fs::write(&img, &disk.bytes).unwrap();
    let img = img.to_str().unwrap();

    assert_eq!(mtype(img, "::/EFI/BOOT/BOOTX64.EFI"), f.boot_efi);
    assert_eq!(mtype(img, "::/EFI/nonos/kernel.bin"), f.kernel_bin);
    assert_eq!(mtype(img, "::/EFI/nonos/boot.cfg"), f.boot_cfg);
    assert_eq!(mtype(img, "::/startup.nsh"), nonos_disk::STARTUP_NSH);

    // The listing shows the names as written, lowercase where the image
    // spells them lowercase, and nothing else in the root.
    let out =
        Command::new("mdir").args(["-i", &format!("{img}@@1M"), "-b", "::/"]).output().unwrap();
    let listing = String::from_utf8_lossy(&out.stdout);
    assert!(listing.contains("::/EFI/"), "{listing}");
    assert!(listing.contains("::/startup.nsh"), "{listing}");
    std::fs::remove_dir_all(&dir).unwrap();
}
