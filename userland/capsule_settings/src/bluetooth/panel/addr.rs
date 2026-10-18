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

//! A device's identity address as people read it: the most significant
//! octet first, colon-separated, as the device's own label prints it.
//! BONDS carries it least significant octet first, as HCI does (Vol 4,
//! Part E, 5.2).

use nonos_bt_proto::Bond;

pub const ADDR_TEXT: usize = 17;

pub fn addr_text(addr: &[u8; 6]) -> [u8; ADDR_TEXT] {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut t = [b':'; ADDR_TEXT];
    for (i, o) in addr.iter().rev().enumerate() {
        t[3 * i] = HEX[usize::from(o >> 4)];
        t[3 * i + 1] = HEX[usize::from(o & 0xF)];
    }
    t
}

/// The line under the address: how the bond was made. Only pairing with a
/// passkey or a compared number is authenticated (Vol 3, Part H, 2.3.5.1).
pub fn made(b: &Bond) -> &'static str {
    if b.authenticated {
        "Paired with a passkey or number checked"
    } else {
        "Paired without a check: Just Works"
    }
}
