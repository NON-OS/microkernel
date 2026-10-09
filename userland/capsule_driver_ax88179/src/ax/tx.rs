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

//! One frame out as ax88179_tx_fixup frames it: an 8-byte header, the
//! frame length then the TSO MSS word (zero, no TSO here), and the frame.

use nonos_usbnet::nic::ETH_FRAME_MAX;

pub const TX_HEADER: usize = 8;
/// Header, the largest frame, and a padding byte.
pub const TX_MAX: usize = TX_HEADER + ETH_FRAME_MAX + 1;
/// The second header word's flags when a padding byte follows the frame
/// (ax88179_tx_fixup, "Enable padding").
pub const TX_PADDING: u32 = 0x8000_8000;

/// A transfer that fills its last packet exactly would need a zero-length
/// packet to end it. Linux does not send one for this chip: usbnet adds
/// one byte and ax88179_tx_fixup sets TX_PADDING in the header so the
/// chip drops it. A multiple of 512 is padded too, whatever the pipe's
/// packet size: driver.xhci0 may move the pipe in 512-byte packets, and a
/// transfer padded that way is byte for byte what Linux sends on a
/// high-speed link.
pub fn needs_padding(total: usize, max_packet: u16) -> bool {
    let mps = max_packet as usize;
    total.is_multiple_of(512) || (mps != 0 && total.is_multiple_of(mps))
}

/// The bulk OUT for `frame` in `out`, its length returned. The frame is at
/// most ETH_FRAME_MAX bytes and `out` at least TX_MAX.
pub fn tx_transfer(frame: &[u8], max_packet: u16, out: &mut [u8]) -> usize {
    let total = TX_HEADER + frame.len();
    let pad = needs_padding(total, max_packet);
    let flags = if pad { TX_PADDING } else { 0 };
    out[..4].copy_from_slice(&(frame.len() as u32).to_le_bytes());
    out[4..8].copy_from_slice(&flags.to_le_bytes());
    out[TX_HEADER..total].copy_from_slice(frame);
    if pad {
        out[total] = 0;
        return total + 1;
    }
    total
}
