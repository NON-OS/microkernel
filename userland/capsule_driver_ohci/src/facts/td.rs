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

//! The general transfer descriptor: four 32-bit words, 16-byte aligned.
//!
//! - Control: buffer rounding 18 (a short packet is no error), direction
//!   20:19, delay interrupt 23:21, data toggle 25:24 (0x: the ED's carry,
//!   10: DATA0, 11: DATA1), error count 27:26, condition code 31:28.
//! - CBP: the next byte to move, 0 once all moved (or for no data).
//! - NextTD (31:4).
//! - BE: the last byte of the buffer.
//!
//! The controller moves one TD's whole buffer in packets of the ED's size.

pub const TD_LEN: usize = 16;
pub const TD_R: u32 = 1 << 18;
pub const TD_DP_SHIFT: u32 = 19;
pub const DP_SETUP: u32 = 0;
pub const DP_OUT: u32 = 1;
pub const DP_IN: u32 = 2;
/// No interrupt for this TD: the driver polls.
pub const TD_DI_NONE: u32 = 7 << 21;
pub const TD_T_DATA0: u32 = 2 << 24;
pub const TD_T_DATA1: u32 = 3 << 24;
pub const TD_CC_SHIFT: u32 = 28;

/// Condition codes.
pub const CC_NO_ERROR: u32 = 0;
pub const CC_STALL: u32 = 4;
/// Written by software: the controller has not retired the TD.
pub const CC_NOT_ACCESSED: u32 = 0xF;
