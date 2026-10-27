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

//! The chip's registers and the values this driver writes. ASIX publishes
//! no programming reference: each number here is as Linux v6.12's
//! ax88179_178a.c uses it, read for behaviour, and named here for its use.

/// The vendor requests: bRequest names the space, wValue and wIndex the
/// place in it (vendor.rs).
pub const ACCESS_MAC: u8 = 0x01;
pub const ACCESS_PHY: u8 = 0x02;
/// The PHY's address on the chip's management bus.
pub const PHY_ADDR: u16 = 0x03;

// MAC registers.
pub const RX_CONTROL: u16 = 0x0B;
pub const STATION: u16 = 0x10;
pub const GROUP_TABLE: u16 = 0x16;
pub const MEDIUM: u16 = 0x22;
pub const POWER_RESET: u16 = 0x26;
pub const BULK_IN_QUEUE: u16 = 0x2E;
pub const CLOCKS: u16 = 0x33;
pub const RX_CHECKSUM: u16 = 0x34;
pub const TX_CHECKSUM: u16 = 0x35;
pub const PAUSE_HIGH: u16 = 0x54;
pub const PAUSE_LOW: u16 = 0x55;

/// POWER_RESET: the PHY released from reset and powered.
pub const PHY_POWERED: u16 = 0x0020;
/// CLOCKS: the bus and core clocks running.
pub const CLOCKS_ON: u8 = 0x03;
/// RX_CONTROL: started (0080h), broadcast (0008h), every group (0002h),
/// each frame behind a 2-byte pad so its IP header is aligned (0200h).
/// No promiscuous bit: the device filters on the programmed address.
pub const RX_ON: u16 = 0x0080 | 0x0008 | 0x0002 | 0x0200;
pub const RX_OFF: u16 = 0x0000;

// MEDIUM bits.
pub const M_GIGA: u16 = 0x0001;
pub const M_FULL: u16 = 0x0002;
pub const M_125MHZ: u16 = 0x0008;
pub const M_RX_PAUSE: u16 = 0x0010;
pub const M_TX_PAUSE: u16 = 0x0020;
pub const M_RECEIVE: u16 = 0x0100;
pub const M_100: u16 = 0x0200;

// PHY registers: control is IEEE 802.3 clause 22's; the MMD window is
// clause 22's annex 22D; the status, page and EEE ones are the chip's.
pub const PHY_CONTROL: u16 = 0x00;
pub const PHY_MMD_CTRL: u16 = 0x0D;
pub const PHY_MMD_DATA: u16 = 0x0E;
pub const PHY_STATUS: u16 = 0x11;
pub const PHY_EEE: u16 = 0x19;
pub const PHY_PAGE: u16 = 0x1F;
/// PHY_CONTROL: autonegotiation on (1000h) and restarted (0200h).
pub const AUTONEG_RESTART: u16 = 0x1000 | 0x0200;
