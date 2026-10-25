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

//! The sector callers count in and the most one driver request moves.

pub const SECTOR: usize = 512;

/// The smallest data buffer among the three drivers: sixty-four sectors.
pub const DATA_BYTES: u32 = 64 * SECTOR as u32;

/// The NVMe driver holds its controller at a 4 KiB memory page, and MDTS
/// counts in pages of that size.
const PAGE_SHIFT: u32 = 12;

/// Bytes one NVMe command may move: the data buffer, or less when MDTS
/// says so. Zero means no limit, and so does a value too large to shift.
pub fn max_transfer_bytes(mdts: u8) -> u32 {
    let shift = u32::from(mdts) + PAGE_SHIFT;
    if mdts == 0 || shift >= u32::BITS {
        return DATA_BYTES;
    }
    (1u32 << shift).min(DATA_BYTES)
}
