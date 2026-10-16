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

//! The station address from the iMACAddress string (CDC 1.2, 5.2.3.16, as
//! recalled, unverified): exactly twelve hexadecimal digits in UTF-16LE, a
//! 26-byte string descriptor. Index 0, a shorter or longer string, one not
//! hexadecimal, a group address or all zeroes is refused, and no address
//! is made up in its place: the device then has no address to send from.

use nonos_usbnet::desc::{mac_from_string, STRING};
use nonos_usbnet::found::string;
use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::Bus;

const MAC_STRING_LEN: usize = 2 + 12 * 2;

pub fn station<B: Bus>(bus: &mut B, index: u8) -> Result<[u8; 6], (&'static str, i32)> {
    if index == 0 {
        return Err(("no MAC address string", E_IO));
    }
    // Room past the right length, so a longer string reads as longer.
    let mut raw = [0u8; 64];
    let n = string(bus, index, &mut raw).map_err(|e| ("MAC string unread", e))?;
    let s = &raw[..n];
    if n != MAC_STRING_LEN || s[0] as usize != MAC_STRING_LEN || s[1] != STRING {
        return Err(("MAC string not twelve digits", E_IO));
    }
    mac_from_string(s).ok_or(("MAC address refused", E_IO))
}
