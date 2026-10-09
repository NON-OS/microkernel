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

//! I225/I226 register offsets in BAR0, from Linux igc_regs.h. Queue 0 sits at
//! 0xC000 (receive) and 0xE000 (transmit), not where the 8254x kept it.

pub const REG_CTRL: usize = 0x0000;
pub const REG_STATUS: usize = 0x0008;
pub const REG_EECD: usize = 0x0010;
pub const REG_CTRL_EXT: usize = 0x0018;
pub const REG_MDIC: usize = 0x0020;
pub const REG_RCTL: usize = 0x0100;
pub const REG_TCTL: usize = 0x0400;
pub const REG_ICR: usize = 0x1500;
pub const REG_IMC: usize = 0x150C;

pub const REG_RFCTL: usize = 0x5008;
pub const REG_MTA_BASE: usize = 0x5200;
/// igc_init_mac_params_base: mta_reg_count = 128.
pub const MTA_ENTRY_COUNT: usize = 128;
pub const REG_RAL0: usize = 0x5400;
pub const REG_RAH0: usize = 0x5404;
/// IGC_RAL(n) / IGC_RAH(n) step by 8; IGC_RAR_ENTRIES is 16.
pub const RAR_STRIDE: usize = 8;
pub const RAR_ENTRIES: usize = 16;

pub const REG_SWSM: usize = 0x5B50;
pub const REG_SW_FW_SYNC: usize = 0x5B5C;

/// Receive queue 0: IGC_RDBAL(0) .. IGC_RXDCTL(0).
pub const REG_RDBAL: usize = 0xC000;
pub const REG_RDBAH: usize = 0xC004;
pub const REG_RDLEN: usize = 0xC008;
pub const REG_SRRCTL: usize = 0xC00C;
pub const REG_RDH: usize = 0xC010;
pub const REG_RDT: usize = 0xC018;
pub const REG_RXDCTL: usize = 0xC028;

/// Transmit queue 0: IGC_TDBAL(0) .. IGC_TXDCTL(0).
pub const REG_TDBAL: usize = 0xE000;
pub const REG_TDBAH: usize = 0xE004;
pub const REG_TDLEN: usize = 0xE008;
pub const REG_TDH: usize = 0xE010;
pub const REG_TDT: usize = 0xE018;
pub const REG_TXDCTL: usize = 0xE028;
