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

//! The AX88179's vendor requests and the MAC registers this driver uses,
//! by the names and numbers Linux ax88179_178a.c gives them.

/// bRequest of the register access requests (ax88179_read_cmd and
/// ax88179_write_cmd): the MAC, the PHY over MDIO, and the EEPROM.
pub const ACCESS_MAC: u8 = 0x01;
pub const ACCESS_PHY: u8 = 0x02;
pub const ACCESS_EEPROM: u8 = 0x04;

/// The EEPROM word whose bit 8 asks for PHY auto detach, as
/// ax88179_auto_detach reads it (wValue 0x43, wIndex 1).
pub const EEPROM_AUTO_DETACH: u16 = 0x43;
pub const AUTO_DETACH_ON: u16 = 0x0100;

/// The USB TX FIFO state ax88179_link_reset reads before it sets the
/// medium: request 0x81, wValue 0x8c, four bytes; bit 30 while it is busy.
pub const TX_FIFO_REQUEST: u8 = 0x81;
pub const TX_FIFO_VALUE: u16 = 0x8c;
pub const TX_FIFO_BUSY: u32 = 0x4000_0000;

pub const PHYSICAL_LINK_STATUS: u16 = 0x02;
pub const RX_CTL: u16 = 0x0b;
pub const NODE_ID: u16 = 0x10;
pub const MEDIUM_STATUS_MODE: u16 = 0x22;
pub const MONITOR_MOD: u16 = 0x24;
pub const PHYPWR_RSTCTL: u16 = 0x26;
pub const RX_BULKIN_QCTRL: u16 = 0x2e;
pub const CLK_SELECT: u16 = 0x33;
pub const RXCOE_CTL: u16 = 0x34;
pub const TXCOE_CTL: u16 = 0x35;
pub const PAUSE_WATERLVL_HIGH: u16 = 0x54;
pub const PAUSE_WATERLVL_LOW: u16 = 0x55;
