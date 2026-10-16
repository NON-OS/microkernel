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
//! The SPI host wire, NSPI version 1: the host wire's 20-byte header with
//! its own magic (op u16 at 6, request id u32 at 12, payload length u32 at
//! 16), then the body; a reply's payload starts with its status.

pub const MAGIC: u32 = 0x4E53_5049; // "NSPI"
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_HEALTHCHECK: u16 = 1;
/// The host taken: `device u16, 0, 0, input_clock u32`.
pub const OP_OPEN: u16 = 2;
pub const OP_CLOSE: u16 = 3;
/// `mode u8, 0, 0, 0, hz u32`, then 1 to TRANSFER_MAX bytes out: as many in.
pub const OP_TRANSFER: u16 = 4;

pub const CHUNK: usize = 8 + crate::facts::TRANSFER_MAX;
pub const REPLY_MAX: usize = HDR_LEN + 4 + CHUNK;

pub const E_INVAL: i32 = -22;
