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
 * A FAT volume's shape, read from its boot sector: enough to walk from the
 * root to one file. FAT12, FAT16 and FAT32 all, since the medium a person
 * boots from was formatted by whatever made it, not by this crate. Only
 * 512-byte logical sectors, the size the block drivers speak.
 */

use super::read::{le16, le32, sectors};
use crate::sink::BlockSink;

pub struct Fat {
    pub bits: u32,
    pub per_cluster: u64,
    pub fat_lba: u64,
    /* The fixed root of FAT12 and FAT16; empty on FAT32. */
    pub root_lba: u64,
    pub root_sectors: u64,
    pub root_cluster: u32,
    pub data_lba: u64,
}

impl Fat {
    pub fn open(disk: &mut dyn BlockSink, base: u64) -> Option<Fat> {
        let b = sectors(disk, base, 1)?;
        let (spc, reserved, fats) = (b[13] as u64, le16(&b, 14), b[16] as u64);
        let fat_size = if le16(&b, 22) != 0 { le16(&b, 22) } else { le32(&b, 36) };
        let total = if le16(&b, 19) != 0 { le16(&b, 19) } else { le32(&b, 32) };
        let jump = b[0] == 0xEB || b[0] == 0xE9;
        let shaped = spc.is_power_of_two() && reserved > 0 && fats > 0 && fat_size > 0;
        if !jump || le16(&b, 11) != 512 || !shaped || b[510] != 0x55 || b[511] != 0xAA {
            return None;
        }
        let root_sectors = (le16(&b, 17) * 32).div_ceil(512);
        let data = reserved + fats * fat_size + root_sectors;
        let clusters = total.checked_sub(data)? / spc;
        let bits = match clusters {
            0..4085 => 12,
            4085..65525 => 16,
            _ => 32,
        };
        Some(Fat {
            bits,
            per_cluster: spc,
            fat_lba: base + reserved,
            root_lba: base + reserved + fats * fat_size,
            root_sectors,
            root_cluster: if bits == 32 { le32(&b, 44) as u32 } else { 0 },
            data_lba: base + data,
        })
    }
}
