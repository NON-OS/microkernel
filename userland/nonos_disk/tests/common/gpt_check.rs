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

//! A GPT read the way the UEFI specification (2.10, section 5.3.2) tells
//! firmware to validate one, sharing no code with the writer: signature,
//! revision and size, the header CRC over 92 bytes with its own field zero,
//! the header's own LBA and its partner's, the usable range, the entry
//! array's place and CRC, and zeros past the header.

use nonos_disk::SECTOR_SIZE;

use super::mem_disk::MemDisk;

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xEDB8_8320 & (crc & 1).wrapping_neg());
        }
    }
    !crc
}

pub fn le(b: &[u8], at: usize, n: usize) -> u64 {
    b[at..at + n].iter().rev().fold(0, |v, &x| v << 8 | x as u64)
}

/// The header at `lba` checked, and its entry array: (first usable, last
/// usable, disk GUID, array).
pub fn header(disk: &MemDisk, lba: u64) -> (u64, u64, Vec<u8>, Vec<u8>) {
    let last = disk.sectors - 1;
    let h = disk.read_sectors(lba, 1);
    assert_eq!(&h[0..8], b"EFI PART");
    assert_eq!((le(&h, 8, 4), le(&h, 12, 4), le(&h, 20, 4)), (0x0001_0000, 92, 0));
    let mut z = h[..92].to_vec();
    z[16..20].fill(0);
    assert_eq!(crc32(&z) as u64, le(&h, 16, 4), "header crc at {lba}");
    assert!(h[92..SECTOR_SIZE].iter().all(|&b| b == 0), "reserved bytes at {lba}");
    let (my, alt, first, end, array_lba) =
        (le(&h, 24, 8), le(&h, 32, 8), le(&h, 40, 8), le(&h, 48, 8), le(&h, 72, 8));
    assert_eq!((my, alt), (lba, if lba == 1 { last } else { 1 }));
    assert_eq!((le(&h, 80, 4), le(&h, 84, 4)), (128, 128), "entry count and size");
    let array_sectors = 128 * 128 / SECTOR_SIZE as u64;
    let array_end = array_lba + array_sectors;
    match lba {
        1 => assert!(array_lba == 2 && array_end <= first, "primary array before usable"),
        _ => assert!(array_lba > end && array_end == last, "backup array after usable"),
    }
    assert!(first <= end && end < last);
    let array = disk.read_sectors(array_lba, array_sectors as usize);
    assert_eq!(crc32(&array) as u64, le(&h, 88, 4), "array crc at {lba}");
    (first, end, h[56..72].to_vec(), array)
}
