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

//! The highest error bit an R1 status carries, by its JEDEC name.

use super::bits::*;

/// The highest error bit set in an R1 status, by its JEDEC name, for the
/// console: on a laptop with no serial port the log tail is all there is.
pub const fn first_error(status: u32) -> Option<&'static str> {
    const NAMED: [(u32, &str); 15] = [
        (OUT_OF_RANGE, "OUT_OF_RANGE"),
        (ADDRESS_ERROR, "ADDRESS_ERROR"),
        (BLOCK_LEN_ERROR, "BLOCK_LEN_ERROR"),
        (ERASE_SEQ_ERROR, "ERASE_SEQ_ERROR"),
        (ERASE_PARAM, "ERASE_PARAM"),
        (WP_VIOLATION, "WP_VIOLATION"),
        (CARD_IS_LOCKED, "CARD_IS_LOCKED"),
        (LOCK_UNLOCK_FAILED, "LOCK_UNLOCK_FAILED"),
        (COM_CRC_ERROR, "COM_CRC_ERROR"),
        (ILLEGAL_COMMAND, "ILLEGAL_COMMAND"),
        (CARD_ECC_FAILED, "CARD_ECC_FAILED"),
        (CC_ERROR, "CC_ERROR"),
        (ERROR, "ERROR"),
        (SWITCH_ERROR, "SWITCH_ERROR"),
        (READY_FOR_DATA, ""),
    ];
    let mut i = 0;
    while i < NAMED.len() {
        if status & NAMED[i].0 != 0 && !NAMED[i].1.is_empty() {
            return Some(NAMED[i].1);
        }
        i += 1;
    }
    None
}
