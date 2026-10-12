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

//! The queue element transfer descriptor: eight 32-bit words, 32-byte
//! aligned.
//!
//! - Next qTD, alternate next qTD (taken after a short IN packet).
//! - Token: status 7:0 (active 7, halted 6, buffer error 5, babble 4,
//!   transaction error 3), PID 9:8 (0 OUT, 1 IN, 2 SETUP), error counter
//!   11:10, IOC 15, total bytes 30:16 (left to move), data toggle 31.
//! - Five buffer pointers: the first with the offset into its 4 KiB page,
//!   the rest page aligned.

pub const QTD_LEN: usize = 32;
pub const TOK_ACTIVE: u32 = 1 << 7;
pub const TOK_HALTED: u32 = 1 << 6;
/// The error bits beside the halt: buffer, babble, transaction, missed
/// micro-frame (5:2).
pub const TOK_ERRORS: u32 = 0x3C;
pub const TOK_PID_SHIFT: u32 = 8;
pub const PID_OUT: u32 = 0;
pub const PID_IN: u32 = 1;
pub const PID_SETUP: u32 = 2;
pub const TOK_CERR_3: u32 = 3 << 10;
pub const TOK_BYTES_SHIFT: u32 = 16;
pub const TOK_BYTES_MASK: u32 = 0x7FFF;
pub const TOK_DT: u32 = 1 << 31;
