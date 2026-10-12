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

//! The endpoint descriptor: four 32-bit words, 16-byte aligned.
//!
//! - Control: function address 6:0, endpoint 10:7, direction 12:11 (0:
//!   from each TD), low speed 13, skip 14, format 15 (0: general TDs),
//!   maximum packet size 26:16.
//! - TailP: the TD after the last to run (31:4).
//! - HeadP: the next TD to run (31:4), toggle carry bit 1, halted bit 0.
//! - NextED (31:4).

pub const ED_LEN: usize = 16;
pub const ED_EN_SHIFT: u32 = 7;
pub const ED_S: u32 = 1 << 13;
pub const ED_K: u32 = 1 << 14;
pub const ED_MPS_SHIFT: u32 = 16;
pub const ED_MPS_MASK: u32 = 0x7FF;
pub const HEAD_H: u32 = 1 << 0;
pub const HEAD_C: u32 = 1 << 1;
pub const PTR_MASK: u32 = !0xF;
