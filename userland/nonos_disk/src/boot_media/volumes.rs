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
 * Where a FAT volume may start on a disk: at sector zero, for a medium
 * formatted whole, and at every partition the table names, GPT or MBR.
 * The first `MAX_ENTRIES` GPT entries only: an ESP is among the first
 * few on any disk a firmware boots.
 */

use alloc::vec::Vec;

use super::read::{le32, le64, sectors};
use crate::sink::{BlockSink, SECTOR_SIZE};

const MAX_ENTRIES: u64 = 128;

pub fn volume_starts(disk: &mut dyn BlockSink) -> Vec<u64> {
    let mut out = alloc::vec![0u64];
    let Some(head) = sectors(disk, 0, 2) else {
        return out;
    };
    let header = &head[SECTOR_SIZE..];
    if &header[0..8] == b"EFI PART" {
        let (array, count, size) = (le64(header, 72), le32(header, 80), le32(header, 84));
        if !(128..=4096).contains(&size) {
            return out;
        }
        let bytes = count.min(MAX_ENTRIES) * size;
        if let Some(a) = sectors(disk, array, bytes.div_ceil(SECTOR_SIZE as u64)) {
            let entries = a.chunks_exact(size as usize).take(count.min(MAX_ENTRIES) as usize);
            out.extend(entries.filter(|e| e[0..16] != [0; 16]).map(|e| le64(e, 32)));
        }
    } else if head[510] == 0x55 && head[511] == 0xAA {
        let entries = head[446..510].chunks_exact(16);
        out.extend(entries.filter(|e| e[4] != 0 && le32(e, 8) != 0).map(|e| le32(e, 8)));
    }
    out
}
