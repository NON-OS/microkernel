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

//! The values written to the MAC registers, from ax88179_178a.c.

/// PHYSICAL_LINK_STATUS: the USB speed the chip runs at.
pub const USB_SS: u8 = 0x04;
pub const USB_HS: u8 = 0x02;

/// AX_RX_CTL as ax88179_reset starts it: drop CRC errors, the 2-byte IP
/// alignment header ax88179_rx_fixup skips, start, directed (AP), all
/// multicast and broadcast. Kept bit for bit, so every receive transfer
/// has the layout ax88179_rx_fixup reads.
pub const RX_CTL_ON: u16 = 0x0100 | 0x0200 | 0x0080 | 0x0020 | 0x0002 | 0x0008;
pub const RX_CTL_STOP: u16 = 0x0000;

pub const MEDIUM_GIGAMODE: u16 = 0x01;
pub const MEDIUM_FULL_DUPLEX: u16 = 0x02;
pub const MEDIUM_EN_125MHZ: u16 = 0x08;
pub const MEDIUM_RXFLOW_CTRLEN: u16 = 0x10;
pub const MEDIUM_TXFLOW_CTRLEN: u16 = 0x20;
pub const MEDIUM_RECEIVE_EN: u16 = 0x100;
pub const MEDIUM_PS: u16 = 0x200;
/// The medium every link starts from (ax88179_link_reset).
pub const MEDIUM_BASE: u16 = MEDIUM_RECEIVE_EN | MEDIUM_TXFLOW_CTRLEN | MEDIUM_RXFLOW_CTRLEN;
/// ax88179_reset's default before a link: gigabit full duplex.
pub const MEDIUM_DEFAULT: u16 = MEDIUM_BASE | MEDIUM_FULL_DUPLEX | MEDIUM_GIGAMODE;

/// AX_MONITOR_MOD as ax88179_reset writes it: PMETYPE, PMEPOL, RWMP.
pub const MONITOR_MODE: u8 = 0x40 | 0x20 | 0x04;

pub const PHYPWR_RSTCTL_IPRL: u16 = 0x0020;
pub const PHYPWR_RSTCTL_AT: u16 = 0x1000;

pub const CLK_SELECT_BCS: u8 = 0x01;
pub const CLK_SELECT_ACS: u8 = 0x02;
pub const CLK_SELECT_ULR: u8 = 0x08;

/// The pause water levels ax88179_reset sets, low and high.
pub const PAUSE_LOW: u8 = 0x34;
pub const PAUSE_HIGH: u8 = 0x52;
