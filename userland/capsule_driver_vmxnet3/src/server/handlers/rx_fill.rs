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

//! Taking frames off the receive queue into one batch body.

extern crate alloc;

use alloc::vec::Vec;

use crate::constants::ring::RX_COMP;
use crate::constants::MAX_ETHERNET_FRAME;
use crate::protocol::{BATCH_MAX_BYTES, BATCH_MAX_FRAMES, RX_PAYLOAD_PREFIX_LEN};
use crate::queue::{Completion, RxRing};

/// A batch body, `[u32 count]` then `[u32 len][frame]` per frame, or `None`
/// when nothing is waiting. A frame is taken only while the largest one
/// could still fit, since a completion cannot be put back, and every ring 0
/// slot a completion used is handed back at once. One pass over the
/// completion ring bounds the loop. The caller writes RXPROD with `rx.fill`.
pub fn fill(rx: &mut RxRing) -> Option<Vec<u8>> {
    let mut body = Vec::with_capacity(4 + 8 * (RX_PAYLOAD_PREFIX_LEN + MAX_ETHERNET_FRAME));
    body.extend_from_slice(&0u32.to_le_bytes());
    let mut count = 0u32;
    for _ in 0..RX_COMP {
        let room = body.len() + RX_PAYLOAD_PREFIX_LEN + MAX_ETHERNET_FRAME <= BATCH_MAX_BYTES;
        if count as usize >= BATCH_MAX_FRAMES || !room {
            break;
        }
        match rx.consume() {
            None => break,
            Some(Completion::Stray) => {}
            Some(Completion::Dropped) => {
                rx.post();
            }
            Some(Completion::Frame { idx, len }) => {
                // SAFETY: slot `idx` holds `len` bytes the device wrote and
                // completed; it is the driver's until the post below.
                let src = unsafe {
                    core::slice::from_raw_parts(rx.buffer_va(idx) as *const u8, len as usize)
                };
                body.extend_from_slice(&(len as u32).to_le_bytes());
                body.extend_from_slice(src);
                rx.post();
                count += 1;
            }
        }
    }
    if count == 0 {
        return None;
    }
    body[..4].copy_from_slice(&count.to_le_bytes());
    Some(body)
}
