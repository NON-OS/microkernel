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

//! The address this station would transmit under, drawn fresh every boot as
//! the RTL8821CE driver draws its own. Every access point logs the source of
//! the frames a join sends, so neither the card's factory address (which the
//! driver never reads) nor a fixed one is used for a join. The kernel's
//! randomness (`CryptoRandom`, gated on the Crypto capability) is the only
//! source; `nonos_mac` makes the bytes a locally administered unicast address.

use nonos_mac::{apply, MAC_LEN};

/// Draw a station address, or `None` when the kernel gave no randomness.
pub fn draw() -> Option<[u8; MAC_LEN]> {
    let mut mac = [0u8; MAC_LEN];
    let rc = nonos_libc::crypto_random(mac.as_mut_ptr(), MAC_LEN);
    if rc < 0 || (rc as usize) != MAC_LEN {
        return None;
    }
    apply(&mut mac);
    Some(mac)
}
