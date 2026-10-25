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

//! UPIU transaction codes, flags, query functions, opcodes and response
//! field offsets. Source: JESD220 (UFS), UPIU chapter, as recalled,
//! unverified.

pub const NOP_OUT: u8 = 0x00;
pub const COMMAND: u8 = 0x01;
pub const QUERY_REQ: u8 = 0x16;
pub const NOP_IN: u8 = 0x20;
pub const RESPONSE: u8 = 0x21;
pub const QUERY_RSP: u8 = 0x36;

/// Command UPIU flags: read, write; task attribute simple (0).
pub const FLAG_READ: u8 = 0x40;
pub const FLAG_WRITE: u8 = 0x20;

/// Query functions: standard read and standard write.
pub const QUERY_READ: u8 = 0x01;
pub const QUERY_WRITE: u8 = 0x81;
pub const OP_READ_FLAG: u8 = 0x05;
pub const OP_SET_FLAG: u8 = 0x06;
/// fDeviceInit: set by the host, cleared by the device when it is ready.
pub const FLAG_DEVICE_INIT: u8 = 0x01;

/// Header response field: target success.
pub const TARGET_SUCCESS: u8 = 0x00;

/// Request UPIUs this engine sends are 32 bytes: a 12-byte header, then
/// the transaction specific fields.
pub const UPIU_LEN: usize = 32;
/// Response offsets: transaction type, response, SCSI status, the flag
/// value's last byte, sense length (big-endian) and sense data.
pub const AT_TYPE: usize = 0;
/// The request's LUN and task tag, echoed in its response's header, and a
/// query's opcode and IDN echoed in its transaction specific fields.
pub const AT_LUN: usize = 2;
pub const AT_TAG: usize = 3;
pub const AT_OPCODE: usize = 12;
pub const AT_IDN: usize = 13;
pub const AT_RESPONSE: usize = 6;
pub const AT_STATUS: usize = 7;
pub const AT_FLAG: usize = 23;
pub const AT_SENSE_LEN: usize = 32;
pub const AT_SENSE: usize = 34;
