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

//! Taking frames off the receive ring into one batch body.

extern crate alloc;

use alloc::vec::Vec;

use crate::constants::MAX_ETHERNET_FRAME;
use crate::protocol::{BATCH_MAX_BYTES, BATCH_MAX_FRAMES, RX_PAYLOAD_PREFIX_LEN};
use crate::queue::RxQueue;
use crate::rx::take_one;

/// A batch body, `[u32 count]` then `[u32 len][frame]` per frame, or `None`
/// when nothing is waiting. A frame is taken only when the largest one could
/// still fit, since a frame off the ring cannot be put back; each slot is
/// refilled as soon as its frame is copied out.
///
/// # Safety
/// `rx` must be the driver's live receive queue.
pub unsafe fn fill(rx: &mut RxQueue) -> Option<Vec<u8>> {
    let mut body = Vec::with_capacity(4 + 8 * (RX_PAYLOAD_PREFIX_LEN + MAX_ETHERNET_FRAME));
    body.extend_from_slice(&0u32.to_le_bytes());
    let mut count = 0u32;
    while (count as usize) < BATCH_MAX_FRAMES
        && body.len() + RX_PAYLOAD_PREFIX_LEN + MAX_ETHERNET_FRAME <= BATCH_MAX_BYTES
    {
        let Some(frame) = take_one(rx) else { break };
        body.extend_from_slice(&(frame.bytes.len() as u32).to_le_bytes());
        body.extend_from_slice(frame.bytes);
        rx.refill_consumed();
        count += 1;
    }
    if count == 0 {
        return None;
    }
    body[..4].copy_from_slice(&count.to_le_bytes());
    Some(body)
}
