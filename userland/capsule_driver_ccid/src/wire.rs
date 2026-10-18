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
//! The smart-card wire, NCCD version 1: the host wire's 20-byte header with
//! its own magic (op u16 at 6, request id u32 at 12, payload length u32 at
//! 16), then the body; a reply's payload starts with its status.

pub const MAGIC: u32 = 0x4E43_4344; // "NCCD"
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_HEALTHCHECK: u16 = 1;
/// The reader taken: `vendor u16, product u16, max_slot u8, 0, 0, 0,
/// features u32`.
pub const OP_OPEN: u16 = 2;
/// Every slot this holder powered powered off, the reader let go.
pub const OP_CLOSE: u16 = 3;
/// `slot u8`: the card's ATR.
pub const OP_POWER_ON: u16 = 4;
/// `slot u8`.
pub const OP_POWER_OFF: u16 = 5;
/// `slot u8`: `icc u8, changed u8, 0, 0`.
pub const OP_STATUS: u16 = 6;
/// `slot u8, 0, 0, 0`, then one command APDU: the response APDU.
pub const OP_TRANSMIT: u16 = 7;

pub const CHUNK: usize = 4000;
pub const REPLY_MAX: usize = HDR_LEN + 4 + CHUNK;

pub const E_IO: i32 = -5;
pub const E_NODEV: i32 = -19;
pub const E_INVAL: i32 = -22;
/// A response longer than a reply carries.
pub const E_MSGSIZE: i32 = -90;
