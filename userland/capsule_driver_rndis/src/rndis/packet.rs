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

//! One frame out as one REMOTE_NDIS_PACKET_MSG (Remote NDIS 1.0, 2.2.13),
//! laid out as Linux rndis_tx_fixup does: a 44-byte header with no
//! out-of-band or per-packet data, the frame right after it.

use super::message::{put32, MSG_PACKET};

pub const PACKET_HDR: usize = 44;
/// DataOffset counts from the DataOffset field, which sits at byte 8.
pub const DATA_OFFSET: u32 = PACKET_HDR as u32 - 8;

/// The bulk OUT length for a message of `len` bytes. One that fills its
/// last packet exactly would need a zero-length packet to end it; one
/// padding byte ends it instead, as usbnet_start_xmit does without
/// FLAG_SEND_ZLP. A multiple of 64 is padded too: driver.xhci0 moves a
/// high-speed or SuperSpeed pipe in 512 or 1024-byte packets whatever the
/// descriptor says, and both are multiples of 64.
pub fn padded_len(len: usize, max_packet: u16) -> usize {
    let mps = max_packet as usize;
    if len.is_multiple_of(64) || (mps != 0 && len.is_multiple_of(mps)) {
        len + 1
    } else {
        len
    }
}

/// Write the message for `frame` into `out`, which holds PACKET_HDR, the
/// frame and a padding byte; its length. The padding byte is counted in
/// MessageLength, so a device that reads messages back to back (QEMU
/// usb_net_handle_dataout) starts the next one where it really starts.
pub fn wrap(frame: &[u8], max_packet: u16, out: &mut [u8]) -> usize {
    let n = padded_len(PACKET_HDR + frame.len(), max_packet);
    out[..PACKET_HDR].fill(0);
    put32(out, 0, MSG_PACKET);
    put32(out, 4, n as u32);
    put32(out, 8, DATA_OFFSET);
    put32(out, 12, frame.len() as u32);
    out[PACKET_HDR..PACKET_HDR + frame.len()].copy_from_slice(frame);
    out[PACKET_HDR + frame.len()..n].fill(0);
    n
}
