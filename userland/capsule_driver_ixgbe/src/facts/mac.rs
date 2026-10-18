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

//! Control, status, EEPROM, link, MAC setting and receive-address bits.
//! Source: as observed, unverified against
//! Intel's 82599, X540 and X550 datasheets.

pub const CTRL_GIO_DIS: u32 = 1 << 2;
pub const CTRL_LNK_RST: u32 = 1 << 3;
pub const CTRL_RST: u32 = 1 << 26;
/// Either reset, link or device, still in progress.
pub const CTRL_RST_MASK: u32 = CTRL_LNK_RST | CTRL_RST;

/// A PCIe master request is outstanding.
pub const STATUS_GIO: u32 = 1 << 19;

pub const CTRL_EXT_NS_DIS: u32 = 1 << 16;
pub const CTRL_EXT_DRV_LOAD: u32 = 1 << 28;

/// The EEPROM auto-read after reset is done.
pub const EEC_ARD: u32 = 1 << 9;

pub const RAH_AV: u32 = 1 << 31;

/// Link up and the speed field, which the X550's non-standard bit turns
/// into the NBASE-T rates.
pub const LINKS_UP: u32 = 1 << 30;
pub const LINKS_SPEED_SHIFT: u32 = 28;
pub const LINKS_SPEED_MASK: u32 = 0x3;
pub const LINKS_SPEED_NON_STD: u32 = 1 << 27;

/// Restart autonegotiation.
pub const AUTOC_AN_RESTART: u32 = 1 << 12;

pub const HLREG0_TXCRCEN: u32 = 1 << 0;
pub const HLREG0_RXCRCSTRP: u32 = 1 << 1;
pub const HLREG0_JUMBOEN: u32 = 1 << 2;
pub const HLREG0_TXPADEN: u32 = 1 << 10;

/// RDRXCTL: CRC strip, which must agree with HLREG0, and bits the 82599 and
/// later want set (as observed, unverified).
pub const RDRXCTL_CRCSTRIP: u32 = 1 << 1;
pub const RDRXCTL_RSCFRSTSIZE: u32 = 0x1F << 17;
pub const RDRXCTL_RSCACKC: u32 = 1 << 25;
pub const RDRXCTL_FCOE_WRFIX: u32 = 1 << 26;
pub const RSCDBU_RSCACKDIS: u32 = 1 << 7;

pub const FCTRL_BAM: u32 = 1 << 10;
pub const RXCTRL_RXEN: u32 = 1 << 0;
pub const DMATXCTL_TE: u32 = 1 << 0;

pub const SECRXCTRL_RX_DIS: u32 = 1 << 1;
pub const SECRXSTAT_RDY: u32 = 1 << 0;
