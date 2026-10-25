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

//! Frames gathered into one OP_RX_BATCH body: `[u32 count]` then
//! `[u32 len][frame]` per frame, the layout net.core reads. Pure over the
//! frame source, so the host proofs drive it without a part.

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use crate::constants::MAX_ETHERNET_FRAME;

/// What one gather took. `error` is the receive error that stopped it; when
/// frames came before it, they are kept and the error is the caller's to
/// report next, never dropped.
pub struct Gathered<E> {
    pub body: Option<Vec<u8>>,
    pub error: Option<E>,
}

/// Up to `max_frames` frames from `next` (`recv_one`'s shape: `Ok(None)`
/// when nothing waits), while the largest frame would still fit in
/// `max_bytes`. Stops at the first error: the part has already moved past
/// the frame it failed, so nothing is lost by asking again later.
pub fn gather<E>(
    max_frames: usize,
    max_bytes: usize,
    mut next: impl FnMut(&mut [u8]) -> Result<Option<usize>, E>,
) -> Gathered<E> {
    let mut body = Vec::with_capacity(max_bytes.min(4 + 8 * (4 + MAX_ETHERNET_FRAME)));
    body.extend_from_slice(&0u32.to_le_bytes());
    let mut frame = vec![0u8; MAX_ETHERNET_FRAME];
    let (mut count, mut error) = (0u32, None);
    while (count as usize) < max_frames && body.len() + 4 + MAX_ETHERNET_FRAME <= max_bytes {
        match next(&mut frame) {
            Ok(Some(n)) => {
                let n = n.min(frame.len());
                body.extend_from_slice(&(n as u32).to_le_bytes());
                body.extend_from_slice(&frame[..n]);
                count += 1;
            }
            Ok(None) => break,
            Err(e) => {
                error = Some(e);
                break;
            }
        }
    }
    body[..4].copy_from_slice(&count.to_le_bytes());
    Gathered { body: (count > 0).then_some(body), error }
}
