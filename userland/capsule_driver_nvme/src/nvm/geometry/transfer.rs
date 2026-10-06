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

//! How many bytes one command may move under the controller's MDTS.

use super::super::constants::{DATA_BYTES, PAGE};

/// Bytes one command may move on a controller with this MDTS: the data
/// buffer, or less when MDTS says so. MDTS is a power of two in units of the
/// minimum page size, which enable holds at 4 KiB. Zero means no limit, and so
/// does a value too large to shift, so no MDTS takes a transfer past the
/// buffer.
pub const fn max_transfer_bytes(mdts: u8) -> u64 {
    let shift = mdts as u32 + PAGE.trailing_zeros();
    if mdts == 0 || shift >= u64::BITS {
        return DATA_BYTES;
    }
    let limit = 1u64 << shift;
    if limit < DATA_BYTES {
        limit
    } else {
        DATA_BYTES
    }
}
