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
 * Walking a FAT chain: where a cluster's sectors are, and which cluster
 * follows it. One or two sectors of the table per step, read as needed.
 */

use super::fat::Fat;
use super::read::{le16, le32, sectors};
use crate::sink::BlockSink;

impl Fat {
    pub fn cluster_lba(&self, cluster: u32) -> u64 {
        self.data_lba + u64::from(cluster.saturating_sub(2)) * self.per_cluster
    }

    /* The cluster after `c` in its chain, `None` at the end or on a bad link. */
    pub fn next(&self, disk: &mut dyn BlockSink, c: u32) -> Option<u32> {
        let at = u64::from(c) * u64::from(self.bits) / 8;
        let b = sectors(disk, self.fat_lba + at / 512, 2)?;
        let raw = le16(&b, (at % 512) as usize) as u32;
        let (v, end) = match self.bits {
            12 if c & 1 == 1 => (raw >> 4, 0xFF8),
            12 => (raw & 0xFFF, 0xFF8),
            16 => (raw, 0xFFF8),
            _ => (le32(&b, (at % 512) as usize) as u32 & 0x0FFF_FFFF, 0x0FFF_FFF8),
        };
        (2..end).contains(&v).then_some(v)
    }
}
