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

//! The boot sector and FSInfo, field by field, as the Microsoft FAT
//! specification lays them out; and the cluster count a driver derives from
//! them, which alone decides that the volume is FAT32.

use crate::mem_disk::MemDisk;

pub struct Boot {
    pub sectors_per_cluster: u64,
    pub reserved: u64,
    pub fat_sectors: u64,
    pub total: u64,
    pub hidden: u32,
    pub clusters: u64,
    pub free: u32,
    pub next_free: u32,
}

fn le16(b: &[u8], o: usize) -> u64 {
    u16::from_le_bytes([b[o], b[o + 1]]) as u64
}

fn le32(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

impl Boot {
    pub fn read(disk: &MemDisk, first: u64) -> Boot {
        let area = disk.read_sectors(first, 8);
        let (bs, info) = (&area[..512], &area[512..1024]);
        assert!(bs[0] == 0xEB || bs[0] == 0xE9, "jump");
        assert_eq!(le16(bs, 11), 512, "bytes per sector");
        let spc = bs[13] as u64;
        assert!(spc.is_power_of_two() && spc <= 128, "sectors per cluster {spc}");
        let reserved = le16(bs, 14);
        assert!(reserved >= 8, "room for the backup boot sector and FSInfo");
        assert_eq!((bs[16], le16(bs, 17), le16(bs, 19), bs[21]), (2, 0, 0, 0xF8));
        assert_eq!(le16(bs, 22), 0, "a FAT32 volume has no 16-bit table size");
        let total = le32(bs, 32) as u64;
        let fat_sectors = le32(bs, 36) as u64;
        assert_eq!((le16(bs, 40), le16(bs, 42), le32(bs, 44)), (0, 0, 2), "flags, version, root");
        assert_eq!((le16(bs, 48), le16(bs, 50)), (1, 6), "FSInfo and backup sectors");
        assert_eq!((bs[66], &bs[82..90]), (0x29, &b"FAT32   "[..]));
        assert_eq!((bs[510], bs[511]), (0x55, 0xAA));
        assert_eq!(&area[6 * 512..7 * 512], bs, "the backup boot sector");
        assert_eq!(&area[7 * 512..8 * 512], info, "the backup FSInfo");
        assert_eq!(
            (le32(info, 0), le32(info, 484), le32(info, 508)),
            (0x4161_5252, 0x6141_7272, 0xAA55_0000)
        );
        let data = total - reserved - 2 * fat_sectors;
        let clusters = data / spc;
        assert!((65_525..=0x0FFF_FFF5).contains(&clusters), "{clusters} clusters is not FAT32");
        assert!((clusters + 2) * 4 <= fat_sectors * 512, "the table holds every cluster");
        Boot {
            sectors_per_cluster: spc,
            reserved,
            fat_sectors,
            total,
            hidden: le32(bs, 28),
            clusters,
            free: le32(info, 488),
            next_free: le32(info, 492),
        }
    }

    /// The first sector of `cluster`, from the volume's first sector.
    pub fn cluster_lba(&self, first: u64, cluster: u32) -> u64 {
        first
            + self.reserved
            + 2 * self.fat_sectors
            + (cluster as u64 - 2) * self.sectors_per_cluster
    }
}
