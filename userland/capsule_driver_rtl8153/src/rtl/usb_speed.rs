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

//! The bus speed the adapter came up at, which a few settings depend on.
//! driver.xhci0 does not say it; a bulk endpoint's wMaxPacketSize does
//! (1024 SuperSpeed, 512 high speed, 64 or less full speed: USB 3.2 9.6.6
//! and USB 2.0 5.8.3, as recalled, unverified).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UsbSpeed {
    Full,
    High,
    Super,
}

pub fn usb_speed(max_packet_in: u16) -> UsbSpeed {
    match max_packet_in {
        1024 => UsbSpeed::Super,
        512 => UsbSpeed::High,
        _ => UsbSpeed::Full,
    }
}

/// How long the chip may hold received data before it hands it up, in
/// nanoseconds, by bus speed (Linux v6.12's three values).
pub fn hold_ns(speed: UsbSpeed) -> u32 {
    match speed {
        UsbSpeed::Super => 85_000,
        UsbSpeed::High => 250_000,
        UsbSpeed::Full => 524_280,
    }
}
