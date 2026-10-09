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

//! A MAC address from a string descriptor: twelve hexadecimal digits in
//! UTF-16LE, as CDC 1.2 section 5.2.3.16 (iMACAddress) gives it and Linux
//! usbnet_get_ethernet_addr reads it.

use super::kinds::STRING;

pub fn mac_from_string(raw: &[u8]) -> Option<[u8; 6]> {
    if raw.len() < 2 + 24 || (raw[0] as usize) < 2 + 24 || raw[1] != STRING {
        return None;
    }
    let mut mac = [0u8; 6];
    for (i, b) in mac.iter_mut().enumerate() {
        let hi = nibble(raw[2 + i * 4], raw[3 + i * 4])?;
        let lo = nibble(raw[4 + i * 4], raw[5 + i * 4])?;
        *b = hi << 4 | lo;
    }
    usable(mac).then_some(mac)
}

fn nibble(lo: u8, hi: u8) -> Option<u8> {
    if hi != 0 {
        return None;
    }
    match lo {
        b'0'..=b'9' => Some(lo - b'0'),
        b'a'..=b'f' => Some(lo - b'a' + 10),
        b'A'..=b'F' => Some(lo - b'A' + 10),
        _ => None,
    }
}

/// Not all zero and not a group address: a station address the stack can
/// put in its frames' source field.
pub fn usable(mac: [u8; 6]) -> bool {
    mac != [0; 6] && mac[0] & 0x01 == 0
}
