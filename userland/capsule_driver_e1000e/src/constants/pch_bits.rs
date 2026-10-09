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

//! Bits of the PCH-only registers, values from Linux e1000e ich8lan.h,
//! defines.h and e1000.h.

pub const FEXTNVM3_PHY_CFG_COUNTER_MASK: u32 = 0x0C00_0000;
pub const FEXTNVM3_PHY_CFG_COUNTER_50MSEC: u32 = 0x0800_0000;
pub const FEXTNVM7_SIDE_CLK_UNGATE: u32 = 1 << 2;
pub const FEXTNVM7_DISABLE_SMB_PERST: u32 = 1 << 5;
pub const FEXTNVM9_IOSFSB_CLKGATE_DIS: u32 = 1 << 11;
pub const FEXTNVM9_IOSFSB_CLKREQ_DIS: u32 = 1 << 12;
pub const FEXTNVM11_DISABLE_MULR_FIX: u32 = 1 << 13;
pub const FEXTNVM12_PHYPD_CTRL_MASK: u32 = 0x00C0_0000;
pub const FEXTNVM12_PHYPD_CTRL_P1: u32 = 0x0080_0000;
pub const KABGTXD_BGSQLBIAS: u32 = 0x0005_0000;
pub const FFLT_DBG_DONT_GATE_WAKE_DMA_CLK: u32 = 1 << 12;

/// PCICFG_DESC_RING_STATUS (e1000.h): a config-space word whose
/// FLUSH_DESC_REQUIRED bit says the I219 holds descriptors it will hang on
/// across a reset.
pub const PCICFG_DESC_RING_STATUS: u32 = 0xE4;
pub const FLUSH_DESC_REQUIRED: u16 = 0x100;
