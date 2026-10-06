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

//! BAR0 register offsets every family has, named as Linux e1000e regs.h
//! names them (`E1000_CTRL` is `REG_CTRL`). Queue 0 only: the driver runs
//! one receive and one transmit ring.

pub const REG_CTRL: usize = 0x0000;
pub const REG_STATUS: usize = 0x0008;
pub const REG_EECD: usize = 0x0010;
pub const REG_CTRL_EXT: usize = 0x0018;
pub const REG_MDIC: usize = 0x0020;
pub const REG_ICR: usize = 0x00C0;
pub const REG_IMC: usize = 0x00D8;
pub const REG_RCTL: usize = 0x0100;
pub const REG_TCTL: usize = 0x0400;
pub const REG_EXTCNF_CTRL: usize = 0x0F00;

pub const REG_RDBAL: usize = 0x2800;
pub const REG_RDBAH: usize = 0x2804;
pub const REG_RDLEN: usize = 0x2808;
pub const REG_RDH: usize = 0x2810;
pub const REG_RDT: usize = 0x2818;
pub const REG_RXDCTL: usize = 0x2828;

pub const REG_TDBAL: usize = 0x3800;
pub const REG_TDBAH: usize = 0x3804;
pub const REG_TDLEN: usize = 0x3808;
pub const REG_TDH: usize = 0x3810;
pub const REG_TDT: usize = 0x3818;
pub const REG_TXDCTL0: usize = 0x3828;
pub const REG_TXDCTL1: usize = 0x3928;
pub const REG_TARC0: usize = 0x3840;
pub const REG_TARC1: usize = 0x3940;

pub const REG_RFCTL: usize = 0x5008;
pub const REG_MTA_BASE: usize = 0x5200;
pub const MTA_ENTRY_COUNT: usize = 128;
pub const REG_RAL0: usize = 0x5400;
pub const REG_RAH0: usize = 0x5404;
pub const REG_WUC: usize = 0x5800;
pub const REG_GCR: usize = 0x5B00;
pub const REG_GCR2: usize = 0x5B64;

/// The highest offset above plus one register, rounded up: a BAR0 smaller
/// than this is not one of these parts.
pub const BAR0_MIN_BYTES: u64 = 0x6000;
