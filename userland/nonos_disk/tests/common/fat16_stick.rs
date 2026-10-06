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
 * A boot stick as QEMU's vvfat shows one: an MBR with the disk signature
 * vvfat writes and one FAT16 partition at sector 63, or the same volume at
 * sector zero with no table. \EFI\BOOT\BOOTX64.EFI holds `loader`; BOOT's
 * first cluster is all deleted entries, so a lookup must follow its chain.
 */

use nonos_disk::BlockSink;

use crate::fat16_sectors::{bpb, entry, mbr};
use crate::mem_disk::MemDisk;

pub const SIGNATURE: [u8; 4] = 0xBE1A_FDFAu32.to_le_bytes();
pub const START: u64 = 63;
pub const SIZE: u64 = 32_768;
const FAT: u64 = 1;
const ROOT: u64 = FAT + 2 * 32;
const DATA: u64 = ROOT + 32;

pub fn stick(partitioned: bool, loader: &[u8]) -> MemDisk {
    let base = if partitioned { START } else { 0 };
    let mut disk = MemDisk::new(START + SIZE);
    if partitioned {
        disk.write_at(0, &mbr()).unwrap();
    }
    let mut w = |lba: u64, bytes: &[u8]| {
        let mut s = vec![0u8; bytes.len().div_ceil(512) * 512];
        s[..bytes.len()].copy_from_slice(bytes);
        disk.write_at(base + lba, &s).unwrap();
    };
    w(0, &bpb());
    w(FAT, &[0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 4, 0, 0xFF, 0xFF, 6, 0, 0xFF, 0xFF]);
    let lfn = entry(b"A\0F\0I\0\0\0\0\0\0", 0x0F, 0, 0);
    w(ROOT, &[lfn, entry(b"EFI        ", 0x10, 2, 0)].concat());
    w(DATA, &entry(b"BOOT       ", 0x10, 3, 0));
    w(DATA + 4, &[0xE5u8; 2048]);
    w(DATA + 8, &entry(b"BOOTX64 EFI", 0x20, 5, loader.len() as u32));
    w(DATA + 12, loader);
    disk
}
