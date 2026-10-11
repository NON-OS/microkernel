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

//! UPIU transaction types, command flags, query functions and opcodes, and
//! the one flag this driver sets (JESD220, 10.5 to 10.7; Linux ufs.h).

pub const NOP_OUT: u8 = 0x00;
pub const COMMAND: u8 = 0x01;
pub const QUERY_REQ: u8 = 0x16;
pub const NOP_IN: u8 = 0x20;
pub const RESPONSE: u8 = 0x21;
pub const QUERY_RSP: u8 = 0x36;

/// Command UPIU flags: R and W, and the simple task attribute (0).
pub const FLAG_READ: u8 = 0x40;
pub const FLAG_WRITE: u8 = 0x20;

/// UPIU_QUERY_FUNC_STANDARD_READ_REQUEST and _WRITE_REQUEST.
pub const QUERY_READ: u8 = 0x01;
pub const QUERY_WRITE: u8 = 0x81;
/// UPIU_QUERY_OPCODE_READ_FLAG and _SET_FLAG.
pub const OP_READ_FLAG: u8 = 0x05;
pub const OP_SET_FLAG: u8 = 0x06;
/// QUERY_FLAG_IDN_FDEVICEINIT: set by the host, cleared by the device once
/// its own initialisation is done.
pub const FLAG_DEVICE_INIT: u8 = 0x01;

/// The header's response field: target success.
pub const TARGET_SUCCESS: u8 = 0x00;
