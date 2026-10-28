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

//! An installed disk as an update finds one: written by `install` with the
//! store given, then a key header of the way given, and data in the
//! volume's ring, past it and at its last sectors, as a first boot and the
//! use after it would leave them. Mounted beside `mem_disk`, `files` and
//! `entropy`.

use crate::mem_disk::MemDisk;
use crate::{entropy, files};
use nonos_disk::{install, BlockSink, Installed, StoreImage};
use nonos_disk_map::KEY_LBA;

/// A key header of `way`, with the handover's magic when `handover`.
pub fn key_header(way: u8, handover: bool) -> [u8; 512] {
    let mut s = [0u8; 512];
    s[..8].copy_from_slice(b"NONOSDK1");
    s[8] = way;
    if handover {
        s[128..136].copy_from_slice(b"NONOSHO1");
    }
    s
}

/// The disk, and what `read_installed` should find on it.
pub fn installed(total: u64, store: StoreImage, way: u8) -> (MemDisk, Installed) {
    let f = files::files();
    let mut disk = MemDisk::new(total);
    let r = install(&mut disk, &files::image(&f), store, entropy::ENTROPY, &mut |_| {}).unwrap();
    disk.write_at(KEY_LBA, &key_header(way, false)).unwrap();
    let data = r.layout.data;
    disk.write_at(data.first, &[0xD5; 256 * 512]).unwrap();
    disk.write_at(data.first + 4096, &[0x5A; 64 * 512]).unwrap();
    disk.write_at(data.end() - 8, &[0xA5; 8 * 512]).unwrap();
    (disk, Installed { layout: r.layout, disk: r.disk_guid, partitions: r.partitions })
}
