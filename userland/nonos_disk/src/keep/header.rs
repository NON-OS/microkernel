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

//! An installed disk's primary GPT header, held to the one this writer
//! writes (`crate::gpt::header`): its signature and size, where it says it
//! and its backup are, the usable sectors, the entry array's place and
//! shape, and both checksums. An update writes no table, so it goes only
//! where this one stands as written.

use crate::boot_media::read::{le32, le64};
use crate::crc32::crc32;
use crate::gpt::{ARRAY_SECTORS, ENTRY_COUNT, ENTRY_SIZE, FIRST_USABLE_LBA};

/// The last usable sector of a disk of `total` sectors, as the header says.
pub fn last_usable(total: u64) -> u64 {
    total.saturating_sub(2 + ARRAY_SECTORS)
}

/// `h` is the header's sector and `array` the 128 entries after it.
pub fn header_holds(total: u64, h: &[u8], array: &[u8]) -> bool {
    let mut own = [0u8; 92];
    own.copy_from_slice(&h[..92]);
    own[16..20].fill(0);
    let backup = total.saturating_sub(1);
    let at = [(24, 1), (32, backup), (40, FIRST_USABLE_LBA), (48, last_usable(total)), (72, 2)];
    let sized = [(12, 92), (80, ENTRY_COUNT as u64), (84, ENTRY_SIZE as u64)];
    h[..8] == *b"EFI PART"
        && sized.iter().all(|&(o, v)| le32(h, o) == v)
        && at.iter().all(|&(o, v)| le64(h, o) == v)
        && le32(h, 16) == crc32(&own) as u64
        && le32(h, 88) == crc32(array) as u64
}
