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

//! The ECM framing (CDC ECM 1.2, 3.3.1, as recalled, unverified): one
//! Ethernet frame per bulk transfer, ended by a short packet.

use nonos_usbnet::nic::{ETH_FRAME_MAX, ETH_HEADER};

/// The bulk OUT length for a frame of `len` bytes. A frame that fills its
/// last packet would need a zero-length packet after it, which the bus has
/// no way to ask for; one padding byte ends it with a short packet instead,
/// and the byte lies past the frame's own content. A multiple of 64 is
/// padded too: the controller driver moves a high-speed or SuperSpeed pipe
/// in 512 or 1024-byte packets whatever the descriptor says, and both are
/// multiples of 64.
pub fn padded_len(len: usize, max_packet: u16) -> usize {
    let mps = max_packet as usize;
    if len.is_multiple_of(64) || (mps != 0 && len.is_multiple_of(mps)) {
        len + 1
    } else {
        len
    }
}

/// The frame a bulk IN of `n` bytes holds: none when it is shorter than an
/// Ethernet header, and at most a full frame: a longer one is cut to 1514
/// bytes (receive.rs counts it), never read further.
pub fn received_len(n: usize) -> Option<usize> {
    (n >= ETH_HEADER).then_some(n.min(ETH_FRAME_MAX))
}
