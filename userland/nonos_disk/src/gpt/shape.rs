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

//! The entry array's shape: 128 entries of 128 bytes, the minimum the UEFI
//! specification requires room for, in 32 sectors after the primary header.

use crate::sink::SECTOR_SIZE;

pub const ENTRY_COUNT: u32 = 128;
pub const ENTRY_SIZE: u32 = 128;
pub const ARRAY_SECTORS: u64 = (ENTRY_COUNT * ENTRY_SIZE) as u64 / SECTOR_SIZE as u64;
/// The first sector a partition may use: past the MBR, the header and the
/// array.
pub const FIRST_USABLE_LBA: u64 = 2 + ARRAY_SECTORS;
