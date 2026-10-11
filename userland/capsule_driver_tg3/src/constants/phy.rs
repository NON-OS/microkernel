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

//! PHY registers: the standard ones from include/uapi/linux/mii.h, then the
//! Broadcom ones tg3.h names MII_TG3_*.

pub const MII_BMCR: u32 = 0x00;
pub const MII_BMSR: u32 = 0x01;
pub const MII_ADVERTISE: u32 = 0x04;
pub const MII_LPA: u32 = 0x05;
pub const MII_CTRL1000: u32 = 0x09;
pub const MII_STAT1000: u32 = 0x0A;
pub const BMCR_ANRESTART: u32 = 0x0200;
pub const BMCR_ANENABLE: u32 = 0x1000;
pub const BMCR_RESET: u32 = 0x8000;
pub const BMSR_LSTATUS: u32 = 0x0004;
pub const ADVERTISE_CSMA: u32 = 0x0001;
/// 10 and 100, half and full.
pub const ADVERTISE_ALL: u32 = 0x01E0;
pub const ADVERTISE_PAUSE_CAP: u32 = 0x0400;
pub const ADVERTISE_PAUSE_ASYM: u32 = 0x0800;
pub const ADVERTISE_1000HALF: u32 = 0x0100;
pub const ADVERTISE_1000FULL: u32 = 0x0200;

pub const MII_TG3_EXT_CTRL: u32 = 0x10;
pub const MII_TG3_EXT_CTRL_FIFO_ELASTIC: u32 = 0x0001;
pub const MII_TG3_RXR_COUNTERS: u32 = 0x14;
pub const MII_TG3_AUX_CTRL: u32 = 0x18;
pub const MII_TG3_AUXCTL_SHDWSEL_AUXCTL: u32 = 0x0000;
pub const MII_TG3_AUXCTL_ACTL_EXTPKTLEN: u32 = 0x4000;
pub const MII_TG3_AUXCTL_SHDWSEL_PWRCTL: u32 = 0x0002;
pub const MII_TG3_AUXCTL_SHDWSEL_MISCTEST: u32 = 0x0004;
pub const MII_TG3_AUXCTL_SHDWSEL_MISC: u32 = 0x0007;
pub const MII_TG3_AUXCTL_MISC_WIRESPD_EN: u32 = 0x0010;
pub const MII_TG3_AUXCTL_MISC_FORCE_AMDIX: u32 = 0x0200;
pub const MII_TG3_AUXCTL_MISC_RDSEL_SHIFT: u32 = 12;
pub const MII_TG3_AUXCTL_MISC_WREN: u32 = 0x8000;
pub const MII_TG3_AUX_STAT: u32 = 0x19;
pub const MII_TG3_AUX_STAT_SPDMASK: u32 = 0x0700;
pub const MII_TG3_ISTAT: u32 = 0x1A;
pub const MII_TG3_IMASK: u32 = 0x1B;
pub const MII_TG3_MISC_SHDW: u32 = 0x1C;
pub const MII_TG3_MISC_SHDW_WREN: u32 = 0x8000;
pub const MII_TG3_MISC_SHDW_APD_SEL: u32 = 0x2800;
pub const MII_TG3_MISC_SHDW_APD_WKTM_84MS: u32 = 0x0001;
pub const MII_TG3_MISC_SHDW_APD_ENABLE: u32 = 0x0020;
pub const MII_TG3_MISC_SHDW_SCR5_SEL: u32 = 0x1400;
/// LPED | DLPTLM | SDTL | C125OE | DLLAPD: tg3_phy_toggle_apd's SCR5 word
/// on any part but the 5784.
pub const MII_TG3_MISC_SHDW_SCR5_BITS: u32 = 0x001F;
pub const MII_TG3_TEST1: u32 = 0x1E;
pub const MII_TG3_TEST1_CRC_EN: u32 = 0x8000;
