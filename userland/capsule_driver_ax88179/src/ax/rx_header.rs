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

//! The chip's receive framing (as Linux v6.12 parses it, read for
//! behaviour): frames from the start of the transfer, each behind a 2-byte
//! pad and padded to 8 bytes, then a table of 32-bit entries, then a 32-bit
//! trailer at the very end: the entry count (bits 15:0) and the table's
//! offset (bits 31:16). An entry holds a frame's length with its pad (bits
//! 28:16), CRC error (bit 29) and drop (bit 31); a length of 0 is a filler.
//! Nothing here reads past the transfer: the table must lie between the
//! frames and the trailer, and each frame before the table.

use alloc::vec::Vec;

use nonos_usbnet::nic::{ETH_FRAME_MAX, ETH_HEADER};

use super::rx_span::{Span, Tally};

/// The pad before each frame (RX_ON's alignment bit).
pub const PAD: usize = 2;
/// Bytes past a full frame the chip may count (its FCS, unverified): such
/// a frame is cut to a full one.
pub const FCS: usize = 4;
const BAD: u32 = 1 << 29 | 1 << 31; // CRC error, dropped

fn word(t: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([t[at], t[at + 1], t[at + 2], t[at + 3]])
}

/// The frames of transfer `t` pushed to `out`, never past `max` of them.
pub fn parse(t: &[u8], out: &mut Vec<Span>, max: usize) -> Tally {
    let mut tally = Tally::default();
    let Some(end) = t.len().checked_sub(4) else { return Tally { lie: true, ..tally } };
    let trailer = word(t, end);
    let (count, table) = ((trailer & 0xFFFF) as usize, (trailer >> 16) as usize);
    if count * 4 + table > end {
        return Tally { lie: true, ..tally };
    }
    let mut at = 0usize;
    for i in 0..count {
        let e = word(t, table + 4 * i);
        let len = ((e >> 16) & 0x1FFF) as usize;
        if len == 0 {
            continue;
        }
        if at + len > table {
            tally.beyond = true;
            break;
        }
        if e & BAD != 0 || len < PAD + ETH_HEADER {
            tally.errors += 1;
        } else if len - PAD > ETH_FRAME_MAX + FCS {
            tally.too_long += 1;
        } else if out.len() >= max {
            tally.over += 1;
        } else {
            out.push(Span { at: (at + PAD) as u16, len: (len - PAD).min(ETH_FRAME_MAX) as u16 });
        }
        at += (len + 7) & !7;
    }
    tally
}
