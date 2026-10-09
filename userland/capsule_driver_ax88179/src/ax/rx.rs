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

//! The frames in one bulk IN, as ax88179_rx_fixup reads it. The transfer
//! ends in a 32-bit header: the entry count, and the offset of an array of
//! 32-bit per-packet headers. Each entry gives a packet's length (with
//! its 2-byte alignment header) and error bits; packets lie from offset 0,
//! each padded to 8 bytes; an entry of length 0 is alignment and skipped.

use alloc::vec::Vec;

use nonos_usbnet::nic::{ETH_FRAME_MAX, ETH_HEADER};

pub const RXHDR_CRC_ERR: u32 = 1 << 29;
pub const RXHDR_DROP_ERR: u32 = 1 << 31;
/// The IP alignment header AX_RX_CTL_IPE puts before each frame.
const ALIGN_HEADER: usize = 2;

/// Each good frame of `xfer` into `out` as offset and length. False, and
/// `out` empty, when the transfer is malformed: it is dropped whole. Frames
/// with an error bit, runts, and frames longer than the stack takes are
/// skipped, as Linux counts them as receive errors.
pub fn frames(xfer: &[u8], out: &mut Vec<(usize, usize)>) -> bool {
    out.clear();
    let Some(end) = xfer.len().checked_sub(4) else { return false };
    let rx_hdr = le32(xfer, end);
    let (count, hdr_off) = ((rx_hdr & 0xffff) as usize, (rx_hdr >> 16) as usize);
    // The entries lie inside the transfer, before its last word, and the
    // packets before the entries.
    if hdr_off + count * 4 > end {
        return false;
    }
    let mut at = 0;
    for i in 0..count {
        let hdr = le32(xfer, hdr_off + i * 4);
        let len = ((hdr >> 16) & 0x1fff) as usize;
        if len == 0 {
            continue;
        }
        let padded = (len + 7) & !7;
        if at + padded > hdr_off {
            out.clear();
            return false;
        }
        let bad = hdr & (RXHDR_CRC_ERR | RXHDR_DROP_ERR) != 0;
        let frame = len.saturating_sub(ALIGN_HEADER);
        if !bad && (ETH_HEADER..=ETH_FRAME_MAX).contains(&frame) {
            out.push((at + ALIGN_HEADER, frame));
        }
        at += padded;
    }
    true
}

fn le32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}
