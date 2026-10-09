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

//! The R1 status bits, and the ones a response reports about its command.

pub const OUT_OF_RANGE: u32 = 1 << 31;
pub const ADDRESS_ERROR: u32 = 1 << 30;
pub const BLOCK_LEN_ERROR: u32 = 1 << 29;
pub const ERASE_SEQ_ERROR: u32 = 1 << 28;
pub const ERASE_PARAM: u32 = 1 << 27;
pub const WP_VIOLATION: u32 = 1 << 26;
pub const CARD_IS_LOCKED: u32 = 1 << 25;
pub const LOCK_UNLOCK_FAILED: u32 = 1 << 24;
pub const COM_CRC_ERROR: u32 = 1 << 23;
pub const ILLEGAL_COMMAND: u32 = 1 << 22;
pub const CARD_ECC_FAILED: u32 = 1 << 21;
pub const CC_ERROR: u32 = 1 << 20;
pub const ERROR: u32 = 1 << 19;
pub const SWITCH_ERROR: u32 = 1 << 7;
pub const READY_FOR_DATA: u32 = 1 << 8;

/// The errors a command's own response reports about that command: Linux's
/// CMD_ERRORS (block.c). COM_CRC_ERROR and ILLEGAL_COMMAND are left out:
/// they describe the previous command, which got no response.
pub const CMD_ERRORS: u32 = OUT_OF_RANGE
    | ADDRESS_ERROR
    | BLOCK_LEN_ERROR
    | WP_VIOLATION
    | CARD_ECC_FAILED
    | CC_ERROR
    | ERROR;

/// The error bits a response carries about its own command.
pub const fn errors(status: u32) -> u32 {
    status & CMD_ERRORS
}
