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

//! One frame out, as Linux r8152_tx_agg_fill lays it: an 8-byte struct
//! tx_desc, opts1 the length with TX_FS and TX_LS (first and last
//! segment, r8152_tx_csum), opts2 zero with no checksum offload, then the
//! frame, at offset 0 of the transfer and so TX_ALIGN (4) aligned.
//!
//! No padding: r8152 sends a transfer whose length is a multiple of the
//! packet size as it is, without URB_ZERO_PACKET or an extra byte, since
//! the chip finds the frame's end from opts1, not from a short packet.
//! A frame under 60 bytes goes as it is too; the chip pads it on the
//! wire, and r8152 does not pad it either.

use nonos_usbnet::nic::{ETH_FRAME_MAX, ETH_HEADER};

/// sizeof(struct tx_desc).
pub const TX_DESC: usize = 8;
const TX_FS: u32 = 1 << 31;
const TX_LS: u32 = 1 << 30;

/// The transfer for `frame` in `out`, and its length; `None` for a frame
/// outside 14 to 1514 bytes or an `out` too small.
pub fn put_frame(frame: &[u8], out: &mut [u8]) -> Option<usize> {
    let n = TX_DESC + frame.len();
    if !(ETH_HEADER..=ETH_FRAME_MAX).contains(&frame.len()) || out.len() < n {
        return None;
    }
    let opts1 = frame.len() as u32 | TX_FS | TX_LS;
    out[..4].copy_from_slice(&opts1.to_le_bytes());
    out[4..TX_DESC].fill(0);
    out[TX_DESC..n].copy_from_slice(frame);
    Some(n)
}
