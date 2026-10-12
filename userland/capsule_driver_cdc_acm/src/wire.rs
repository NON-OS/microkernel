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
//! The serial wire, NACM version 1: the host wire's 20-byte header with its
//! own magic (op u16 at 6, request id u32 at 12, payload length u32 at
//! 16), then the body; a reply's payload starts with its status.

pub const MAGIC: u32 = 0x4E41_434D; // "NACM"
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_HEALTHCHECK: u16 = 1;
/// The line taken: `vendor u16, product u16, protocol u8, capabilities u8,
/// 0, 0`, and DTR and RTS raised.
pub const OP_OPEN: u16 = 2;
/// DTR and RTS dropped, what was read and not taken forgotten.
pub const OP_CLOSE: u16 = 3;
/// 1 to CHUNK bytes: `taken u32`, what the device took.
pub const OP_WRITE: u16 = 4;
/// `most u16`: up to that many of the bytes read, none while there are none.
pub const OP_READ: u16 = 5;
/// LineCoding's seven bytes: `baud u32, stop u8, parity u8, data u8`.
pub const OP_SET_LINE: u16 = 6;
/// `lines u16`: bit 0 DTR, bit 1 RTS.
pub const OP_SET_CONTROL: u16 = 7;
/// `state u16, 0, 0, queued u32`: DCD, DSR and the events since the last.
pub const OP_STATUS: u16 = 8;
/// `ms u16`: a break that long.
pub const OP_BREAK: u16 = 9;

pub const CHUNK: usize = 4000;
pub const REPLY_MAX: usize = HDR_LEN + 4 + CHUNK;

pub const E_IO: i32 = -5;
pub const E_NODEV: i32 = -19;
pub const E_INVAL: i32 = -22;
