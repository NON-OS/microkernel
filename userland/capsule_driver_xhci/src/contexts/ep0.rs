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

//! The EP0 max packet size (USB 2.0 section 5.5.3, USB 3.2 section 9.6.1,
//! xHCI 1.2 section 4.3.3 and 4.6.5). Address Device needs one before any
//! descriptor has been read: SuperSpeed and faster is 512, high speed 64 and
//! low speed 8, each fixed by its specification. A full-speed device may use
//! 8, 16, 32 or 64, so it starts at 8, the first 8 bytes of its device
//! descriptor are read (at most one packet of any size), and bMaxPacketSize0
//! goes to the controller through Evaluate Context. Starting a 64-byte
//! device at 8 and reading more than 8 bytes is a babble on silicon.

/// Protocol speed ids with the default mapping (xHCI 1.2 table 7-13).
pub const SPEED_FULL: u8 = 1;
pub const SPEED_LOW: u8 = 2;
pub const SPEED_HIGH: u8 = 3;
pub const SPEED_SUPER: u8 = 4;

/// Whether a device at `speed` on a port of that protocol is SuperSpeed or
/// faster. A USB 3 port carries nothing slower; the speed id alone is
/// trusted only where no Supported Protocol capability says (`usb3` false).
pub fn is_superspeed(speed: u8, usb3: bool) -> bool {
    usb3 || speed >= SPEED_SUPER
}

/// The EP0 size Address Device starts with.
pub fn max_packet_for_speed(speed: u8, usb3: bool) -> u16 {
    if is_superspeed(speed, usb3) {
        return 512;
    }
    match speed {
        SPEED_HIGH => 64,
        _ => 8,
    }
}

/// Whether the starting size is a guess the device descriptor must confirm.
pub fn ep0_needs_descriptor(speed: u8, usb3: bool) -> bool {
    !is_superspeed(speed, usb3) && speed == SPEED_FULL
}

/// The EP0 size bMaxPacketSize0 names, or `None` when the value is not one
/// the speed allows (the device is then kept at the size it was addressed
/// with). SuperSpeed devices give an exponent, 9 for 512.
pub fn ep0_max_packet(speed: u8, usb3: bool, b_max_packet_size0: u8) -> Option<u16> {
    let b = b_max_packet_size0;
    if is_superspeed(speed, usb3) {
        return (b == 9).then_some(512);
    }
    match speed {
        SPEED_HIGH => (b == 64).then_some(64),
        SPEED_LOW => (b == 8).then_some(8),
        _ => matches!(b, 8 | 16 | 32 | 64).then_some(b as u16),
    }
}
