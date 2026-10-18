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

//! Register offsets in BAR0 for the 82599, X540 and X550. Source: as
//! observed, unverified against Intel's datasheets.

pub const REG_CTRL: usize = 0x00000;
pub const REG_STATUS: usize = 0x00008;
pub const REG_CTRL_EXT: usize = 0x00018;
pub const REG_EICR: usize = 0x00800;
pub const REG_EIMC: usize = 0x00888;
/// The extended mask-clear registers: the causes past the first sixteen
/// queues.
pub const REG_EIMC_EX0: usize = 0x00AB0;
pub const REG_EIMC_EX1: usize = 0x00AB4;
pub const REG_RDRXCTL: usize = 0x02F00;
pub const REG_RXCTRL: usize = 0x03000;
pub const REG_RSCDBU: usize = 0x03028;
pub const REG_HLREG0: usize = 0x04240;
pub const REG_AUTOC: usize = 0x042A0;
pub const REG_LINKS: usize = 0x042A4;
pub const REG_DMATXCTL: usize = 0x04A80;
pub const REG_FCTRL: usize = 0x05080;
pub const REG_SECRXCTRL: usize = 0x08D00;
pub const REG_SECRXSTAT: usize = 0x08D04;
pub const REG_EEC: usize = 0x10010;

pub const REG_MTA_BASE: usize = 0x05200;
/// 128 words of the multicast table.
pub const MTA_ENTRY_COUNT: usize = 128;
/// Receive address low of entry `i`, 8 apart; high is four bytes up.
pub const REG_RAL0: usize = 0x0A200;
pub const RAR_STRIDE: usize = 8;
/// Receive address entries on the 82599, X540 and X550.
pub const RAR_ENTRIES: usize = 128;

/// Receive queue 0, and its SRRCTL.
pub const REG_RDBAL: usize = 0x01000;
pub const REG_RDBAH: usize = 0x01004;
pub const REG_RDLEN: usize = 0x01008;
pub const REG_RDH: usize = 0x01010;
pub const REG_RDT: usize = 0x01018;
pub const REG_RXDCTL: usize = 0x01028;
pub const REG_SRRCTL: usize = 0x02100;

/// Transmit queue 0.
pub const REG_TDBAL: usize = 0x06000;
pub const REG_TDBAH: usize = 0x06004;
pub const REG_TDLEN: usize = 0x06008;
pub const REG_TDH: usize = 0x06010;
pub const REG_TDT: usize = 0x06018;
pub const REG_TXDCTL: usize = 0x06028;

pub const fn ral(i: usize) -> usize {
    REG_RAL0 + i * RAR_STRIDE
}
