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

//! One endpoint descriptor (USB 2.0, table 9-13) taken into an interface.

use super::iface::Interface;

/// Take the endpoint `d` into `i` when it is the first bulk one of its
/// direction; that direction, so its companion is read too. A bulk
/// wMaxPacketSize has bits 15:11 reserved and a size of 1 to 1024 (USB 2.0
/// 9.6.6 and 5.8.3, USB 3.2 9.6.6, as recalled, unverified); an endpoint
/// outside that is not taken, and no pipe is built on it.
pub(super) fn endpoint(i: &mut Interface, d: &[u8]) -> Option<bool> {
    let dir_in = d[2] & 0x80 != 0;
    let raw = u16::from_le_bytes([d[4], d[5]]);
    let mps = raw & 0x07FF;
    let free = if dir_in { i.pipes.bulk_in == 0 } else { i.pipes.bulk_out == 0 };
    if d[3] & 0x03 != 0x02 || !free || raw >> 11 != 0 || !(1..=1024).contains(&mps) {
        return None;
    }
    if dir_in {
        (i.pipes.bulk_in, i.pipes.max_packet_in) = (d[2], mps);
    } else {
        (i.pipes.bulk_out, i.pipes.max_packet_out) = (d[2], mps);
    }
    Some(dir_in)
}
