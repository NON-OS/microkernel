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
use crate::scrub::scrub;

/// A batch body, `[u32 count]` then `[u32 len][frame]` per frame, or `None`
/// when no frame is waiting. A frame is taken only while the longest one
/// take_one hands on could still fit, since a frame off the ring cannot be
/// put back; each buffer goes back to the device as its frame is copied.
/// An entry take_one dropped (its 0) is no frame and takes no place in the
/// batch, but counts toward the entries one batch may take.
pub fn fill(rx: &mut RxQueue) -> Option<Vec<u8>> {
    let mut frame = [0u8; MAX_ETHERNET_FRAME];
    let mut next = take_one(rx, &mut frame);
    next?;
    // Room for the largest batch at once: a body that grew would leave
    // copies of earlier frames behind in the memory it moved out of.
    let mut body = Vec::with_capacity(BATCH_MAX_BYTES);
    body.extend_from_slice(&0u32.to_le_bytes());
    let (mut count, mut entries) = (0u32, 0usize);
    while let Some(len) = next {
        entries += 1;
        if let Some(bytes) = frame.get(..len).filter(|b| !b.is_empty()) {
            body.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            body.extend_from_slice(bytes);
            count += 1;
        }
        let room = body.len() + RX_PAYLOAD_PREFIX_LEN + MAX_ETHERNET_FRAME <= BATCH_MAX_BYTES;
        next = if room && entries < BATCH_MAX_FRAMES { take_one(rx, &mut frame) } else { None };
    }
    scrub(&mut frame);
    if count == 0 {
        return None;
    }
    body[..4].copy_from_slice(&count.to_le_bytes());
    Some(body)
}
