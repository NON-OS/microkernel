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

//! What an install wrote, as a count and one digest, so a test pins a
//! whole disk in a line. A disk's digest is `nonos_disk_map::digest16` over
//! each sector it holds, in LBA order, as the LBA in eight little-endian
//! bytes and then the sector's 512; a write list's is over each write in
//! order, as its LBA and then its length, eight little-endian bytes each.

use nonos_disk_map::digest16;

use super::mem_disk::MemDisk;

/// How many sectors or writes, and their digest read as a big-endian number.
pub type Golden = (usize, u128);

pub fn sectors(disk: &MemDisk) -> Golden {
    let mut bytes = Vec::with_capacity(disk.written.len() * 520);
    for (lba, sector) in &disk.written {
        bytes.extend_from_slice(&lba.to_le_bytes());
        bytes.extend_from_slice(sector);
    }
    (disk.written.len(), u128::from_be_bytes(digest16(&bytes)))
}

pub fn writes(list: &[(u64, usize)]) -> Golden {
    let words = list.iter().flat_map(|&(lba, len)| [lba, len as u64]);
    let bytes: Vec<u8> = words.flat_map(u64::to_le_bytes).collect();
    (list.len(), u128::from_be_bytes(digest16(&bytes)))
}

/// `got` spelled as the pinned tables are, to paste over them once a change
/// to the bytes is signed off.
pub fn spelled(got: &[Golden]) -> String {
    got.iter().map(|(n, d)| format!("    ({n}, {d:#034x}),\n")).collect()
}
