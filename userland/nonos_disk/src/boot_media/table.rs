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
 * Whether a disk's partition table holds the partition the record names,
 * matched the way firmware matches a Hard Drive node to a disk: a GPT by
 * the partition's GUID, an MBR by the disk signature, and both by where
 * the partition starts and how long it is.
 */

use super::read::{le32, le64, sectors};
use super::record::{BootPartition, SIGNATURE_GUID, SIGNATURE_MBR, TABLE_GPT, TABLE_MBR};
use crate::sink::{BlockSink, SECTOR_SIZE};

pub fn table_names(p: &BootPartition, disk: &mut dyn BlockSink) -> bool {
    let Some(head) = sectors(disk, 0, 2) else {
        return false;
    };
    match (p.table, p.signature_type) {
        (TABLE_GPT, SIGNATURE_GUID) => gpt_names(p, disk, &head[SECTOR_SIZE..]),
        (TABLE_MBR, SIGNATURE_MBR) => mbr_names(p, &head[..SECTOR_SIZE]),
        _ => false,
    }
}

fn gpt_names(p: &BootPartition, disk: &mut dyn BlockSink, header: &[u8]) -> bool {
    let (array, count, size) = (le64(header, 72), le32(header, 80), le32(header, 84));
    let index = u64::from(p.number.wrapping_sub(1));
    if &header[0..8] != b"EFI PART" || index >= count || !(128..=4096).contains(&size) {
        return false;
    }
    let at = index * size;
    let first = at / SECTOR_SIZE as u64;
    let span = (at % SECTOR_SIZE as u64 + size).div_ceil(SECTOR_SIZE as u64);
    let Some(bytes) = sectors(disk, array.saturating_add(first), span) else {
        return false;
    };
    let e = &bytes[(at % SECTOR_SIZE as u64) as usize..];
    let (start, last) = (le64(e, 32), le64(e, 40));
    e[16..32] == p.signature
        && start == p.start_lba
        && last.wrapping_add(1) == start.wrapping_add(p.size_lba)
}

fn mbr_names(p: &BootPartition, mbr: &[u8]) -> bool {
    if mbr[510] != 0x55 || mbr[511] != 0xAA || mbr[440..444] != p.signature[..4] {
        return false;
    }
    /* A logical partition's entry is in a chain this does not walk; the
     * disk signature is what the firmware itself trusted for it. */
    if !(1..=4).contains(&p.number) {
        return p.signature[..4] != [0; 4];
    }
    let e = &mbr[446 + 16 * (p.number as usize - 1)..];
    le32(e, 8) == p.start_lba && le32(e, 12) == p.size_lba
}
