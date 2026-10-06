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

//! The byte count of the one PRD entry a command carries. The entry names the
//! driver's data buffer, so its count is the most the HBA may move there.

use crate::constants::ata::{DATA_BUF_BYTES, PRD_MAX_BYTES, SECTOR_SIZE};

// One entry describes the whole buffer, so no command needs a second.
const _: () = assert!(DATA_BUF_BYTES <= PRD_MAX_BYTES as u64);

/// Bytes a command moving `sectors` sectors puts through the data buffer, or
/// None for no sectors or more than the buffer holds.
pub fn data_bytes(sectors: u32) -> Option<u32> {
    let bytes = u64::from(sectors) * SECTOR_SIZE as u64;
    if sectors == 0 || bytes > DATA_BUF_BYTES {
        return None;
    }
    u32::try_from(bytes).ok()
}

/// The DBC field of a PRD entry moving `bytes`: the count less one, with the
/// interrupt bit clear. None unless the count is nonzero, even (DBC bit 0
/// must read 1) and at most 4 MiB. Without the zero check the subtraction
/// would wrap to a 4 MiB count with the interrupt bit set.
pub fn prd_dbc(bytes: u32) -> Option<u32> {
    if bytes == 0 || bytes & 1 != 0 || bytes > PRD_MAX_BYTES {
        return None;
    }
    Some(bytes - 1)
}
