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

//! Clock and SSC registers, and the limits rtsx_pci_switch_clock keeps.

pub const FPDCTL: u16 = 0xFC00;
pub const CLK_CTL: u16 = 0xFC02;
pub const CLK_DIV: u16 = 0xFC03;
pub const SSC_DIV_N_0: u16 = 0xFC0F;
pub const SSC_CTL1: u16 = 0xFC11;
pub const SSC_CTL2: u16 = 0xFC12;
pub const RCCTL: u16 = 0xFC14;
pub const OLT_LED_CTL: u16 = 0xFC1E;
pub const GPIO_CTL: u16 = 0xFC1F;

pub const SSC_POWER_DOWN: u8 = 0x01;
pub const OC_POWER_DOWN: u8 = 0x02;
pub const CLK_LOW_FREQ: u8 = 0x01;
pub const SSC_RSTB: u8 = 0x80;
pub const SSC_8X_EN: u8 = 0x40;
pub const SSC_SEL_4M: u8 = 0x10;
pub const SSC_DEPTH_MASK: u8 = 0x07;
pub const SSC_DEPTH_4M: u8 = 0x01;
pub const SSC_DEPTH_500K: u8 = 0x04;

pub const CLK_DIV_1: u8 = 0x01;
pub const CLK_DIV_8: u8 = 0x04;
pub const MIN_DIV_N_PCR: u8 = 80;
pub const MAX_DIV_N_PCR: u8 = 208;
