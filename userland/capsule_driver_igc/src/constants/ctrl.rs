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

//! Device control, status, NVM and receive-address bits, Linux igc_defines.h.

pub const CTRL_FD: u32 = 1 << 0;
pub const CTRL_GIO_MASTER_DISABLE: u32 = 1 << 2;
pub const CTRL_SLU: u32 = 1 << 6;
/// IGC_CTRL_SPEED_MASK, GENMASK(10, 8).
pub const CTRL_SPEED_MASK: u32 = 0x7 << 8;
pub const CTRL_FRCSPD: u32 = 1 << 11;
pub const CTRL_FRCDPX: u32 = 1 << 12;
/// IGC_CTRL_RST, the global reset igc_reset_hw_base issues.
pub const CTRL_RST: u32 = 1 << 26;

pub const STATUS_FD: u32 = 1 << 0;
pub const STATUS_LU: u32 = 1 << 1;
pub const STATUS_SPEED_100: u32 = 1 << 6;
pub const STATUS_SPEED_1000: u32 = 1 << 7;
pub const STATUS_GIO_MASTER_ENABLE: u32 = 1 << 19;
/// Set beside SPEED_1000 when the link runs at 2.5 Gb/s.
pub const STATUS_SPEED_2500: u32 = 1 << 22;

pub const EECD_AUTO_RD: u32 = 1 << 9;
pub const CTRL_EXT_DRV_LOAD: u32 = 1 << 28;
pub const RAH_AV: u32 = 1 << 31;
/// IGC_RFCTL_IPV6_EX_DIS, set by igc_rx_fifo_flush_base per hardware errata.
pub const RFCTL_IPV6_EX_DIS: u32 = 1 << 16;
