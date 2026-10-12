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

//! Register offsets in BAR0. Source: observed in Linux igb (facts::regs),
//! unverified against Intel's datasheets. Queue 0 is programmed at its low
//! alias, 0x2800 and 0x3800, which the part answers for its first four
//! queues as well as at 0xC000 and 0xE000 (observed, unverified).

pub const REG_CTRL: usize = 0x0000;
pub const REG_STATUS: usize = 0x0008;
pub const REG_EECD: usize = 0x0010;
pub const REG_CTRL_EXT: usize = 0x0018;
pub const REG_MDIC: usize = 0x0020;
pub const REG_ICR: usize = 0x00C0;
pub const REG_IMC: usize = 0x00D8;
pub const REG_RCTL: usize = 0x0100;
pub const REG_TCTL: usize = 0x0400;
/// PHY power management, on the 82580 and later.
pub const REG_PHPM: usize = 0x0E14;

pub const REG_MTA_BASE: usize = 0x5200;
/// The multicast table: 128 entries on every igb MAC.
pub const MTA_ENTRY_COUNT: usize = 128;
/// Receive address entries 0-15 from 0x5400, the rest from 0x54E0, 8 apart.
pub const REG_RAL0: usize = 0x5400;
pub const REG_RAL16: usize = 0x54E0;
pub const RAR_STRIDE: usize = 8;

pub const REG_SWSM: usize = 0x5B50;
pub const REG_SW_FW_SYNC: usize = 0x5B5C;

/// Receive queue 0.
pub const REG_RDBAL: usize = 0x2800;
pub const REG_RDBAH: usize = 0x2804;
pub const REG_RDLEN: usize = 0x2808;
pub const REG_SRRCTL: usize = 0x280C;
pub const REG_RDH: usize = 0x2810;
pub const REG_RDT: usize = 0x2818;
pub const REG_RXDCTL: usize = 0x2828;

/// Transmit queue 0.
pub const REG_TDBAL: usize = 0x3800;
pub const REG_TDBAH: usize = 0x3804;
pub const REG_TDLEN: usize = 0x3808;
pub const REG_TDH: usize = 0x3810;
pub const REG_TDT: usize = 0x3818;
pub const REG_TXDCTL: usize = 0x3828;

/// Receive address low of entry `i`; its high word is four bytes up.
pub const fn ral(i: usize) -> usize {
    if i < 16 {
        REG_RAL0 + i * RAR_STRIDE
    } else {
        REG_RAL16 + (i - 16) * RAR_STRIDE
    }
}
