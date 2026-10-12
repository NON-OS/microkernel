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

//! Attribute types on the wire: a 16-bit UUID, or a 128-bit one least
//! significant octet first (Vol 3, Part F, 3.2.1). A 16-bit UUID stands for
//! the Bluetooth Base UUID with those bits at octets 12 and 13 (Vol 3,
//! Part B, 2.5.1).

/// 00000000-0000-1000-8000-00805F9B34FB, least significant octet first.
pub const BASE_UUID: [u8; 16] = [
    0xfb, 0x34, 0x9b, 0x5f, 0x80, 0x00, 0x00, 0x80, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// The 16-bit form of a UUID field, or `None` for a 128-bit UUID outside
/// the base range (a vendor's own type, which no profile here looks for)
/// or a field of any other length.
pub fn uuid16(field: &[u8]) -> Option<u16> {
    match field {
        &[lo, hi] => Some(u16::from_le_bytes([lo, hi])),
        long if long.len() == 16 => {
            let base = long[..12] == BASE_UUID[..12] && long[14..] == BASE_UUID[14..];
            base.then(|| u16::from_le_bytes([long[12], long[13]]))
        }
        _ => None,
    }
}
