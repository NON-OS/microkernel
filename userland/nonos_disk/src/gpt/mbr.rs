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

//! The protective MBR at sector 0: one partition of type 0xEE covering the
//! whole disk, so a tool that reads only MBRs sees a disk that is fully
//! used and leaves it alone.

use alloc::vec::Vec;

use crate::sink::SECTOR_SIZE;

pub fn protective(total_sectors: u64) -> Vec<u8> {
    let mut s = alloc::vec![0u8; SECTOR_SIZE];
    let e = &mut s[446..462];
    // Status 0, CHS start 0/0/2, type 0xEE, CHS end past the horizon.
    e[2] = 0x02;
    e[4] = 0xEE;
    e[5] = 0xFF;
    e[6] = 0xFF;
    e[7] = 0xFF;
    e[8..12].copy_from_slice(&1u32.to_le_bytes());
    /*
     * Size in sectors, saturated at the field's width for disks over
     * two TiB, as the specification directs. A reader that trusts this
     * number on a bigger disk still sees "all of it that I can name".
     */
    let size = (total_sectors - 1).min(0xFFFF_FFFF) as u32;
    e[12..16].copy_from_slice(&size.to_le_bytes());
    s[510] = 0x55;
    s[511] = 0xAA;
    s
}
