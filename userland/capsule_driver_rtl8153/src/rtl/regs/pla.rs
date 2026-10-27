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

//! The MAC side (PLA, wIndex 0100h).

pub const STATION: u16 = 0xc000;
pub const RX_CONFIG: u16 = 0xc010;
pub const RX_MAX: u16 = 0xc016;
pub const RX_FIFO_A: u16 = 0xc0a0;
pub const RX_FIFO_B: u16 = 0xc0a4;
pub const RX_FIFO_C: u16 = 0xc0a8;
pub const SPARE0: u16 = 0xc0b0;
pub const FILTER_MCU: u16 = 0xc0b4;
pub const TEREDO: u16 = 0xc0bc;
pub const MULTICAST: u16 = 0xcd00;
pub const TEREDO_TIMER: u16 = 0xd2cc;
pub const WAKE_TIMER: u16 = 0xd2e8;
pub const SUSPEND_FLAG: u16 = 0xd38a;
pub const D3_FLAG: u16 = 0xd38c;
pub const EXTRA_STATUS: u16 = 0xd398;
pub const PACKAGE: u16 = 0xdc00;
pub const LEDS: u16 = 0xdd92;
/// Read after ALDPS is turned off; Linux gives it no name.
pub const POWER_STATE: u16 = 0xe000;
pub const BOOT: u16 = 0xe004;
pub const LAN_WAKE: u16 = 0xe007;
pub const EEE_PLUS: u16 = 0xe080;
pub const MAC_POWER2: u16 = 0xe0ca;
pub const MAC_POWER3: u16 = 0xe0cc;
pub const MAC_POWER4: u16 = 0xe0ce;
pub const WATCHDOG6: u16 = 0xe428;
/// The version code is in its upper half (model.rs).
pub const TX_CONFIG: u16 = 0xe610;
pub const TX_CONFIG1: u16 = 0xe612;
pub const TX_MAX: u16 = 0xe615;
pub const TX_FIFO: u16 = 0xe618;
pub const TALLY: u16 = 0xe800;
pub const COMMAND: u16 = 0xe813;
pub const CONFIG_LOCK: u16 = 0xe81c;
pub const CONFIG34: u16 = 0xe820;
pub const PHY_POWER: u16 = 0xe84c;
pub const OOB: u16 = 0xe84f;
pub const CPLUS: u16 = 0xe854;
pub const MISC1: u16 = 0xe85a;
/// Which 4 KiB page of the PHY's registers the window at 0xb000 shows.
pub const PHY_PAGE: u16 = 0xe86c;
pub const SFF7: u16 = 0xe8de;
/// Link, speed and duplex as the MAC sees them.
pub const LINK: u16 = 0xe908;
pub const CONFIG6: u16 = 0xe90a;
