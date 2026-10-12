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

//! The 16-bit UUIDs this host looks for (the Bluetooth SIG's Assigned
//! Numbers) and the characteristic properties it needs (Vol 3, Part G,
//! 3.3.1.1).

/// GATT declarations (Vol 3, Part G, 3.1 and 3.3.1).
pub const PRIMARY_SERVICE: u16 = 0x2800;
pub const SECONDARY_SERVICE: u16 = 0x2801;
pub const CHARACTERISTIC: u16 = 0x2803;
/// Client Characteristic Configuration (Vol 3, Part G, 3.3.3.3).
pub const CCCD: u16 = 0x2902;

/// The services and characteristics this host serves (Vol 3, Part C, 12;
/// Vol 3, Part G, 7).
pub const GAP_SERVICE: u16 = 0x1800;
pub const GATT_SERVICE: u16 = 0x1801;
pub const DEVICE_NAME: u16 = 0x2a00;
pub const APPEARANCE: u16 = 0x2a01;

pub const HID_SERVICE: u16 = 0x1812;
pub const PROTOCOL_MODE: u16 = 0x2a4e;
pub const BOOT_KEYBOARD_INPUT: u16 = 0x2a22;
pub const BOOT_MOUSE_INPUT: u16 = 0x2a33;

/// Protocol Mode's value for boot protocol (HIDS 1.0, Protocol Mode
/// characteristic): 0 is boot,
/// 1 is report.
pub const BOOT_PROTOCOL: u8 = 0;

/// CCCD value bits: notifications, indications.
pub const NOTIFY_ON: u16 = 0x0001;

/// Characteristic properties.
pub mod prop {
    pub const READ: u8 = 0x02;
    pub const WRITE_WITHOUT_RESPONSE: u8 = 0x04;
    pub const NOTIFY: u8 = 0x10;
    pub const INDICATE: u8 = 0x20;
}
