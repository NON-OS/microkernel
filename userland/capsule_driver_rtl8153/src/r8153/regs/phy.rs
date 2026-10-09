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

//! PHY registers behind the OCP window and PHY SRAM addresses, as r8152.c
//! defines them.

pub const OCP_BASE_MII: u16 = 0xa400;
pub const OCP_PHY_STATUS: u16 = 0xa420;
pub const PHY_STAT_MASK: u16 = 0x0007;
pub const PHY_STAT_EXT_INIT: u16 = 2;
pub const PHY_STAT_LAN_ON: u16 = 3;
pub const PHY_STAT_PWRDN: u16 = 5;

pub const OCP_POWER_CFG: u16 = 0xa430;
pub const EN_ALDPS: u16 = 0x0004;

pub const OCP_SRAM_ADDR: u16 = 0xa436;
pub const OCP_SRAM_DATA: u16 = 0xa438;

pub const OCP_ADC_CFG: u16 = 0xbc06;
pub const CKADSEL_L: u16 = 0x0100;
pub const ADC_EN: u16 = 0x0080;
pub const EN_EMI_L: u16 = 0x0040;

pub const SRAM_LPF_CFG: u16 = 0x8012;
pub const SRAM_10M_AMP1: u16 = 0x8080;
pub const SRAM_10M_AMP2: u16 = 0x8082;
pub const SRAM_IMPEDANCE: u16 = 0x8084;
