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

//! The frames in one bulk IN, as Linux rx_bottom walks it: each frame
//! behind a 24-byte struct rx_desc whose opts1 low 15 bits give its
//! length with the 4-byte CRC, the next descriptor at the following
//! 8-byte boundary (RX_ALIGN). A length under ETH_ZLEN or past the end of
//! the transfer ends the walk, as it does in Linux; nothing is read
//! outside the bytes that came.

/// sizeof(struct rx_desc): opts1 to opts6.
pub const RX_DESC: usize = 24;
const RX_LEN_MASK: u32 = 0x7fff;
const RX_ALIGN: usize = 8;
const ETH_ZLEN: usize = 60;
const ETH_FCS_LEN: usize = 4;

/// Hands `each` every frame in `transfer`, without its CRC; the count.
pub fn frames(transfer: &[u8], mut each: impl FnMut(&[u8])) -> usize {
    // Linux drops a transfer shorter than the least frame whole.
    if transfer.len() < ETH_ZLEN {
        return 0;
    }
    let (mut at, mut count) = (0usize, 0usize);
    while transfer.len() > at + RX_DESC {
        let Some(opts1) = transfer.get(at..at + 4) else { break };
        let opts1 = u32::from_le_bytes([opts1[0], opts1[1], opts1[2], opts1[3]]);
        let len = (opts1 & RX_LEN_MASK) as usize;
        if len < ETH_ZLEN {
            break;
        }
        // The frame and its CRC must both have come (Linux len_used).
        let end = at + RX_DESC + len;
        if end > transfer.len() {
            break;
        }
        each(&transfer[at + RX_DESC..end - ETH_FCS_LEN]);
        count += 1;
        at = end.next_multiple_of(RX_ALIGN);
    }
    count
}
