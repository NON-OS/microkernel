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

//! Every waiting frame off the RX ring, as one OP_RX_BATCH body:
//! `[u32 count]` then `[u32 len][frame]` per frame, the layout net.core
//! reads (`device/batch_frames.rs`) and virtio_net sends.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::RX_DESC_COUNT;
use crate::constants::MAX_ETHERNET_FRAME;

use super::rx::RxRing;

impl RxRing {
    /// The batch, or `None` when no frame was waiting. A frame is taken only
    /// while the largest one would still fit, since a slot handed back
    /// cannot be read again. Every slot taken, an errored one included,
    /// goes back through `give_back` (the RDT write) once its copy is done.
    /// No more slots are looked at than the ring holds, so a part that keeps
    /// completing errored frames cannot hold the server in here.
    pub fn take_batch(
        &mut self,
        max_frames: usize,
        max_bytes: usize,
        mut give_back: impl FnMut(u16),
    ) -> Option<Vec<u8>> {
        let mut body = Vec::with_capacity(max_bytes.min(4 + 8 * (4 + MAX_ETHERNET_FRAME)));
        body.extend_from_slice(&0u32.to_le_bytes());
        let mut count = 0u32;
        for _ in 0..RX_DESC_COUNT {
            if count as usize >= max_frames || body.len() + 4 + MAX_ETHERNET_FRAME > max_bytes {
                break;
            }
            let Some((idx, len)) = self.consume() else { break };
            if len != 0 {
                // SAFETY: slot `idx` of the RX buffer grant, RX_BUFFER_LEN
                // bytes long; consume() bounds `len` to MAX_ETHERNET_FRAME.
                let frame = unsafe {
                    core::slice::from_raw_parts(self.buffer_va(idx) as *const u8, len.into())
                };
                body.extend_from_slice(&u32::from(len).to_le_bytes());
                body.extend_from_slice(frame);
                count += 1;
            }
            // The copy out of the buffer ends before the part may write it.
            fence(Ordering::Release);
            give_back(idx);
        }
        if count == 0 {
            return None;
        }
        body[..4].copy_from_slice(&count.to_le_bytes());
        Some(body)
    }
}
