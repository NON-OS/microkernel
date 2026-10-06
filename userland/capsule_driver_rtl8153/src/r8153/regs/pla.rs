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

//! PLA (MAC) register addresses, as r8152.c defines them.

pub const IDR: u16 = 0xc000;
pub const RCR: u16 = 0xc010;
pub const RMS: u16 = 0xc016;
pub const RXFIFO_CTRL0: u16 = 0xc0a0;
pub const RXFIFO_CTRL1: u16 = 0xc0a4;
pub const RXFIFO_CTRL2: u16 = 0xc0a8;
pub const FMC: u16 = 0xc0b4;
pub const TEREDO_CFG: u16 = 0xc0bc;
pub const MAR: u16 = 0xcd00;
pub const BACKUP: u16 = 0xd000;
pub const TEREDO_TIMER: u16 = 0xd2cc;
pub const REALWOW_TIMER: u16 = 0xd2e8;
/// r8153_aldps_en polls bit 8 here after turning ALDPS off; r8152.c gives
/// the register no name.
pub const ALDPS_STATE: u16 = 0xe000;
pub const BOOT_CTRL: u16 = 0xe004;
pub const MAC_PWR_CTRL3: u16 = 0xe0cc;
pub const WDT6_CTRL: u16 = 0xe428;
pub const TCR0: u16 = 0xe610;
pub const TCR1: u16 = 0xe612;
pub const MTPS: u16 = 0xe615;
pub const TXFIFO_CTRL: u16 = 0xe618;
pub const CR: u16 = 0xe813;
pub const CRWECR: u16 = 0xe81c;
pub const OOB_CTRL: u16 = 0xe84f;
pub const CPCR: u16 = 0xe854;
pub const MISC_1: u16 = 0xe85a;
pub const OCP_GPHY_BASE: u16 = 0xe86c;
pub const SFF_STS_7: u16 = 0xe8de;
pub const PHYSTATUS: u16 = 0xe908;
