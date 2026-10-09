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
 * Whether the FAT volume at `base` holds the running loader: an
 * `\EFI\BOOT\BOOTX64.EFI` of the loader's length whose first cluster
 * reads back as the loader's first bytes. The length and the head of a PE
 * image are what differ between two builds of it.
 */

use super::fat::Fat;
use super::fat_dir::find;
use super::read::sectors;
use crate::sink::{BlockSink, SECTOR_SIZE};

pub fn holds_loader(disk: &mut dyn BlockSink, base: u64, size: u64, head: &[u8]) -> bool {
    first_cluster_matches(disk, base, size, head).unwrap_or(false)
}

fn first_cluster_matches(
    disk: &mut dyn BlockSink,
    base: u64,
    size: u64,
    head: &[u8],
) -> Option<bool> {
    let fat = Fat::open(disk, base)?;
    let efi = find(&fat, disk, 0, b"EFI        ").filter(|e| e.dir && e.cluster >= 2)?;
    let boot = find(&fat, disk, efi.cluster, b"BOOT       ").filter(|e| e.dir && e.cluster >= 2)?;
    let file = find(&fat, disk, boot.cluster, b"BOOTX64 EFI").filter(|e| !e.dir)?;
    if file.size != size || file.cluster < 2 {
        return Some(false);
    }
    let cluster_bytes = fat.per_cluster as usize * SECTOR_SIZE;
    let n = head.len().min(cluster_bytes).min(size as usize);
    if n == 0 {
        return Some(false);
    }
    let bytes = sectors(disk, fat.cluster_lba(file.cluster), n.div_ceil(SECTOR_SIZE) as u64)?;
    Some(bytes[..n] == head[..n])
}
