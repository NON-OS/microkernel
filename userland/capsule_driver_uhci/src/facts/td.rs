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

//! The transfer descriptor: four 32-bit words, 16-byte aligned.
//!
//! - Link: the next element's address, Vf (depth first) bit 2, Q bit 1,
//!   Terminate bit 0.
//! - Control/status: ActLen 10:0 (bytes less one, 0x7FF for none), the
//!   status bits 23:16, IOC 24, ISO 25, LS 26, the error counter 28:27,
//!   SPD 29.
//! - Token: PID 7:0, device address 14:8, endpoint 18:15, data toggle 19,
//!   MaxLen 31:21 (bytes less one, 0x7FF for none).
//! - Buffer pointer.

pub const TD_LEN: usize = 16;

pub const LINK_TERMINATE: u32 = 1 << 0;
pub const LINK_QH: u32 = 1 << 1;
pub const LINK_DEPTH: u32 = 1 << 2;

pub const STATUS_ACTIVE: u32 = 1 << 23;
pub const STATUS_STALLED: u32 = 1 << 22;
pub const STATUS_DBUF: u32 = 1 << 21;
pub const STATUS_BABBLE: u32 = 1 << 20;
pub const STATUS_CRC_TIMEOUT: u32 = 1 << 18;
pub const STATUS_BITSTUFF: u32 = 1 << 17;
/// Every status bit that ends a TD in error. Bit 19, NAK received, is not
/// one: the TD stays active and the controller asks again next frame.
pub const STATUS_ERRORS: u32 =
    STATUS_STALLED | STATUS_DBUF | STATUS_BABBLE | STATUS_CRC_TIMEOUT | STATUS_BITSTUFF;
pub const CTRL_IOC: u32 = 1 << 24;
pub const CTRL_LS: u32 = 1 << 26;
pub const CTRL_CERR_SHIFT: u32 = 27;
pub const CTRL_SPD: u32 = 1 << 29;
pub const ACTLEN_MASK: u32 = 0x7FF;

pub const PID_SETUP: u8 = 0x2D;
pub const PID_IN: u8 = 0x69;
pub const PID_OUT: u8 = 0xE1;
pub const TOKEN_ADDR_SHIFT: u32 = 8;
pub const TOKEN_EP_SHIFT: u32 = 15;
pub const TOKEN_TOGGLE: u32 = 1 << 19;
pub const TOKEN_MAXLEN_SHIFT: u32 = 21;
/// A length field's "no bytes": lengths are written less one.
pub const LEN_NONE: u32 = 0x7FF;
