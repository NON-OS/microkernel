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

//! The disk plan's sector, in the layout the kernel's `parse_plan` reads
//! (`src/fs/blockfs_volume/plan_types.rs`): the magic, then the data
//! volume's first sector and its length, then the import count.

use alloc::vec::Vec;

use nonos_disk_map::PLAN_MAGIC;

use crate::layout::Extent;
use crate::sink::SECTOR_SIZE;

pub fn plan_sector(volume: &Extent) -> Vec<u8> {
    let mut s = alloc::vec![0u8; SECTOR_SIZE];
    s[0..8].copy_from_slice(&PLAN_MAGIC);
    s[8..16].copy_from_slice(&volume.first.to_le_bytes());
    s[16..24].copy_from_slice(&volume.sectors.to_le_bytes());
    /*
     * Bytes 24..32, the import count, stay zero: an install brings no files
     * for the kernel to import, and the volume fills the partition.
     */
    s
}
