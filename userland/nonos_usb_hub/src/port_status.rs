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

//! One hub port's wPortStatus and wPortChange (USB 2.0 section 11.24.2.7,
//! USB 3.2 section 10.16.2.6). The two layouts share connection, enable,
//! reset and their change bits; power and speed sit apart. Pure, so the
//! decode is proven on the host.

/// xHCI protocol speed ids with the default mapping (xHCI 1.2 table 7-13).
pub const SPEED_FULL: u8 = 1;
pub const SPEED_LOW: u8 = 2;
pub const SPEED_HIGH: u8 = 3;
pub const SPEED_SUPER: u8 = 4;

const CONNECTION: u16 = 1 << 0;
const ENABLE: u16 = 1 << 1;
const RESET: u16 = 1 << 4;
const POWER_USB2: u16 = 1 << 8;
const POWER_USB3: u16 = 1 << 9;
const LOW_SPEED: u16 = 1 << 9;
const HIGH_SPEED: u16 = 1 << 10;
const C_CONNECTION: u16 = 1 << 0;
const C_RESET: u16 = 1 << 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HubPort {
    pub connected: bool,
    pub enabled: bool,
    pub resetting: bool,
    pub powered: bool,
    /// The attached device's speed id; meaningful once the port is enabled.
    pub speed: u8,
    pub connect_changed: bool,
    pub reset_changed: bool,
}

/// The four GET_STATUS bytes of a port on a hub that is SuperSpeed or not.
/// Every device below a SuperSpeed hub is SuperSpeed: its USB 2 half is a
/// separate hub on the USB 2 root port.
pub fn decode_port_status(raw: [u8; 4], superspeed: bool) -> HubPort {
    let status = u16::from_le_bytes([raw[0], raw[1]]);
    let change = u16::from_le_bytes([raw[2], raw[3]]);
    let speed = if superspeed {
        SPEED_SUPER
    } else if status & LOW_SPEED != 0 {
        SPEED_LOW
    } else if status & HIGH_SPEED != 0 {
        SPEED_HIGH
    } else {
        SPEED_FULL
    };
    let power = if superspeed { POWER_USB3 } else { POWER_USB2 };
    HubPort {
        connected: status & CONNECTION != 0,
        enabled: status & ENABLE != 0,
        resetting: status & RESET != 0,
        powered: status & power != 0,
        speed,
        connect_changed: change & C_CONNECTION != 0,
        reset_changed: change & C_RESET != 0,
    }
}
