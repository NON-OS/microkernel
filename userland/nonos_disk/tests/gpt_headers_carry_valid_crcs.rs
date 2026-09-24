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

//! Both GPT headers check out by the specification's own arithmetic:
//! signature, header CRC over 92 bytes with its field zeroed, and the entry
//! array CRC. Firmware does exactly this before it trusts a partition.

mod common;

use nonos_disk::install;

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xEDB8_8320 & (crc & 1).wrapping_neg());
        }
    }
    !crc
}

fn le32(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

fn le64(b: &[u8], o: usize) -> u64 {
    let mut a = [0u8; 8];
    a.copy_from_slice(&b[o..o + 8]);
    u64::from_le_bytes(a)
}

fn check_header(disk: &[u8], lba: u64) {
    let h = &disk[lba as usize * 512..lba as usize * 512 + 92];
    assert_eq!(&h[0..8], b"EFI PART");
    let mut z = h.to_vec();
    z[16..20].fill(0);
    assert_eq!(crc32(&z), le32(h, 16), "header crc at lba {lba}");
    let array_lba = le64(h, 72) as usize;
    let array = &disk[array_lba * 512..array_lba * 512 + 128 * 128];
    assert_eq!(crc32(array), le32(h, 88), "array crc at lba {lba}");
    assert_eq!(le64(h, 24), lba, "header names its own lba");
}

#[test]
fn gpt_headers_carry_valid_crcs() {
    let f = common::files();
    let mut disk = common::MemDisk::new(320);
    let r = install(&mut disk, &common::image(&f), common::ENTROPY, &mut |_| {}).unwrap();
    check_header(&disk.bytes, 1);
    check_header(&disk.bytes, r.layout.backup_header_lba);
    assert_eq!(disk.bytes[510], 0x55);
    assert_eq!(disk.bytes[511], 0xAA);
    assert_eq!(disk.bytes[446 + 4], 0xEE, "protective partition type");
    assert!(disk.flushed);
}
