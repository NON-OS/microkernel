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

//! Bits of the PLA registers, under r8152.c's names.

// PLA_RCR: accept all physical, physical match, multicast, broadcast.
pub const RCR_AAP: u32 = 0x01;
pub const RCR_APM: u32 = 0x02;
pub const RCR_AM: u32 = 0x04;
pub const RCR_AB: u32 = 0x08;
pub const RCR_ACPT_ALL: u32 = RCR_AAP | RCR_APM | RCR_AM | RCR_AB;
// PLA_CR
pub const CR_RST: u8 = 0x10;
pub const CR_RE: u8 = 0x08;
pub const CR_TE: u8 = 0x04;
// PLA_CRWECR: the config registers, PLA_IDR among them, unlocked.
pub const CRWECR_NORMAL: u8 = 0x00;
pub const CRWECR_CONFIG: u8 = 0xc0;
// PLA_OOB_CTRL
pub const NOW_IS_OOB: u8 = 0x80;
pub const LINK_LIST_READY: u8 = 0x02;
// PLA_MISC_1
pub const RXDY_GATED_EN: u16 = 0x0008;
// PLA_SFF_STS_7
pub const RE_INIT_LL: u16 = 0x8000;
pub const MCU_BORW_EN: u16 = 0x4000;
// PLA_CPCR
pub const CPCR_RX_VLAN: u16 = 0x0040;
// PLA_FMC
pub const FMC_FCR_MCU_EN: u16 = 0x0001;
// PLA_TEREDO_CFG and PLA_WDT6_CTRL
pub const TEREDO_SEL: u16 = 0x8000;
pub const TEREDO_RS_EVENT_MASK: u16 = 0x00fe;
pub const OOB_TEREDO_EN: u16 = 0x0001;
pub const WDT6_SET_MODE: u16 = 0x0010;
// PLA_TCR0 and PLA_TCR1
pub const TCR0_AUTO_FIFO: u16 = 0x0080;
pub const IFG_MASK: u16 = 0x0308;
pub const IFG_144NS: u16 = 0x0200;
pub const IFG_96NS: u16 = 0x0300;
// PLA_BOOT_CTRL
pub const AUTOLOAD_DONE: u16 = 0x0002;
// PLA_MAC_PWR_CTRL3
pub const PLA_MCU_SPDWN_EN: u16 = 1 << 14;
// PLA_PHYSTATUS, read as rtl8152_get_speed.
pub const FULL_DUP: u16 = 0x01;
pub const LINK_STATUS: u16 = 0x02;
pub const SPEED_10: u16 = 0x04;
pub const SPEED_100: u16 = 0x08;
pub const SPEED_1000: u16 = 0x10;
