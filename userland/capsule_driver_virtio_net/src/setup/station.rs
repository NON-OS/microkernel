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

//! The address this station transmits from, drawn per bring-up rather than
//! taken from the device. The driver never takes VIRTIO_NET_F_MAC, so the
//! address a hypervisor or a card hands out is never transmitted.

use nonos_mac::{apply, MAC_LEN};

/// Draw a station address. Fails closed: the device's address would transmit
/// the identifier this avoids.
pub fn draw() -> Result<[u8; MAC_LEN], &'static str> {
    let mut mac = [0u8; MAC_LEN];
    let rc = nonos_libc::crypto_random(mac.as_mut_ptr(), MAC_LEN);
    if rc < 0 || (rc as usize) != MAC_LEN {
        return Err("virtio-net: no entropy for a station address");
    }
    apply(&mut mac);
    Ok(mac)
}
