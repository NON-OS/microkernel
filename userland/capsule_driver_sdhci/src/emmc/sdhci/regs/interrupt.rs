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

//! Normal and Error Interrupt Status bits and the masks the engine enables.

// Normal Interrupt Status.
pub const INT_CMD_COMPLETE: u16 = 1 << 0;
pub const INT_TRANSFER_COMPLETE: u16 = 1 << 1;
pub const INT_ERROR: u16 = 1 << 15;

// Error Interrupt Status.
pub const ERR_CMD_TIMEOUT: u16 = 1 << 0;
pub const ERR_CMD_CRC: u16 = 1 << 1;
pub const ERR_CMD_END_BIT: u16 = 1 << 2;
pub const ERR_CMD_INDEX: u16 = 1 << 3;
pub const ERR_DATA_TIMEOUT: u16 = 1 << 4;
pub const ERR_DATA_CRC: u16 = 1 << 5;
pub const ERR_DATA_END_BIT: u16 = 1 << 6;
pub const ERR_AUTO_CMD: u16 = 1 << 8;
pub const ERR_ADMA: u16 = 1 << 9;

/// Normal statuses the engine waits on.
pub const NORMAL_ENABLE: u16 = INT_CMD_COMPLETE | INT_TRANSFER_COMPLETE;
/// Every error status the engine judges: the command and data errors,
/// Auto CMD and ADMA. Current Limit (bit 7) and tuning (bit 10) are left
/// off: neither applies to a fixed-voltage, untuned bus.
pub const ERROR_ENABLE: u16 = ERR_CMD_TIMEOUT
    | ERR_CMD_CRC
    | ERR_CMD_END_BIT
    | ERR_CMD_INDEX
    | ERR_DATA_TIMEOUT
    | ERR_DATA_CRC
    | ERR_DATA_END_BIT
    | ERR_AUTO_CMD
    | ERR_ADMA;
