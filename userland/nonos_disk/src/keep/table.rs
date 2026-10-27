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

//! An installed disk's primary partition table, held to what this writer
//! writes (`crate::gpt`): the header as `header_holds` checks it, then the
//! four entries in `Region::ALL` order with their types, names and
//! attributes, each on the usable sectors, and nothing in the other 124.

use super::header::{header_holds, last_usable};
use super::why::NotKept;
use crate::boot_media::read::le64;
use crate::gpt::{ARRAY_SECTORS, ENTRY_SIZE, FIRST_USABLE_LBA};
use crate::guid::Guid;
use crate::layout::{Extent, Region};
use crate::sink::SECTOR_SIZE;

pub struct Table {
    pub disk: Guid,
    /// Each partition's unique GUID and extent, in `Region::ALL` order.
    pub unique: [Guid; 4],
    pub extents: [Extent; 4],
}

/// `head` is sectors 0 to 33 of a disk of `total` sectors.
pub fn read_table(total: u64, head: &[u8]) -> Result<Table, NotKept> {
    let end = (2 + ARRAY_SECTORS as usize) * SECTOR_SIZE;
    let (h, array) = (head.get(SECTOR_SIZE..2 * SECTOR_SIZE), head.get(2 * SECTOR_SIZE..end));
    let (Some(h), Some(array)) = (h, array) else { return Err(NotKept::Table) };
    if total < 2 * FIRST_USABLE_LBA || !header_holds(total, h, array) {
        return Err(NotKept::Table);
    }
    let guid = |b: &[u8]| Guid(b.try_into().unwrap_or([0; 16]));
    let (mut unique, mut extents) = ([Guid([0; 16]); 4], [Extent::new(0, 0); 4]);
    for (i, e) in array.chunks_exact(ENTRY_SIZE as usize).enumerate() {
        let Some(&region) = Region::ALL.get(i) else {
            if e[..16] != [0; 16] {
                return Err(NotKept::Table);
            }
            continue;
        };
        let (first, last) = (le64(e, 32), le64(e, 40));
        let placed = FIRST_USABLE_LBA <= first && first <= last && last <= last_usable(total);
        let own = guid(&e[..16]) == region.type_guid() && le64(e, 48) == region.attributes();
        if !placed || !own || !named(e, region) {
            return Err(NotKept::Table);
        }
        (unique[i], extents[i]) = (guid(&e[16..32]), Extent::new(first, last - first + 1));
    }
    Ok(Table { disk: guid(&h[56..72]), unique, extents })
}

/* The entry's name field holds the region's name and zeros after it. */
fn named(e: &[u8], region: Region) -> bool {
    let mut field = [0u8; 72];
    for (j, unit) in region.name().encode_utf16().enumerate() {
        field[2 * j..2 * j + 2].copy_from_slice(&unit.to_le_bytes());
    }
    e[56..128] == field
}
