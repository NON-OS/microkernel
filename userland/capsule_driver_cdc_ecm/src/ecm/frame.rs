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

//! The ECM framing: one Ethernet frame per bulk transfer, ended by a short
//! packet (CDC ECM 1.2, section 3.3.1).

use nonos_usbnet::nic::{ETH_FRAME_MAX, ETH_HEADER};

/// The bulk OUT length for a frame of `len` bytes. A frame that fills its
/// last packet exactly would need a zero-length packet to end it; one
/// padding byte ends it instead, as Linux usbnet_start_xmit does for
/// drivers without FLAG_SEND_ZLP, and Ethernet ignores the byte. A
/// multiple of 64 is padded too: the controller driver moves a high-speed
/// or SuperSpeed pipe in 512 or 1024-byte packets whatever the descriptor
/// says, and both are multiples of 64.
pub fn padded_len(len: usize, max_packet: u16) -> usize {
    let mps = max_packet as usize;
    if len.is_multiple_of(64) || (mps != 0 && len.is_multiple_of(mps)) {
        len + 1
    } else {
        len
    }
}

/// The frame a bulk IN of `n` bytes holds: none when it is shorter than an
/// Ethernet header, all of it up to a full frame, a full frame when the
/// sender's padding byte follows one, and none for anything longer (two
/// frames run together, or junk), which is dropped rather than cut.
pub fn received_len(n: usize) -> Option<usize> {
    match n {
        n if n < ETH_HEADER => None,
        n if n <= ETH_FRAME_MAX => Some(n),
        n if n == ETH_FRAME_MAX + 1 => Some(ETH_FRAME_MAX),
        _ => None,
    }
}
