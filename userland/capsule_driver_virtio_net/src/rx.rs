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

//! Taking one received frame off the used ring.
//!
//! Everything in a used entry is the device's word. The frame is copied out
//! of the buffer before the buffer goes back, so no borrow of memory the
//! device writes ever leaves this function; the buffer is wiped first, so
//! nothing of the frame stays behind once it is delivered.

use crate::constants::MAX_ETHERNET_FRAME;
use crate::queue::{used, zero, RxQueue};

/// Copy the next received frame into `out` and give its buffer back; the
/// frame's length, or `None` when nothing is waiting.
///
/// An entry whose id names no posted buffer is dropped and never posted
/// back. One whose length leaves no frame after the header, runs past the
/// buffer, or is longer than an Ethernet frame (or `out`) is dropped and its
/// buffer recycled. A dropped entry is `Some(0)`, so a caller that counts
/// entries still bounds its loop against a device that keeps reporting.
pub fn take_one(rx: &mut RxQueue, out: &mut [u8]) -> Option<usize> {
    if used::idx(rx.ring) == rx.last_used {
        return None;
    }
    // The entry and the frame are only valid once the index is seen.
    nonos_libc::dma_rmb();
    let (id, used_len) = used::elem(rx.ring, rx.last_used);
    rx.last_used = rx.last_used.wrapping_add(1);
    let Some(slot) = u16::try_from(id).ok().filter(|&s| s < rx.buf_count) else {
        return Some(0);
    };
    let len = frame_len(rx, used_len).filter(|&n| n <= out.len()).unwrap_or(0);
    let base = usize::from(slot) * rx.buf_len as usize;
    for (i, b) in out.iter_mut().take(len).enumerate() {
        *b = rx.bufs.r8(base + rx.hdr_len + i);
    }
    let written = core::cmp::min(used_len as usize, rx.buf_len as usize);
    zero(rx.bufs, base, written);
    rx.refill(slot);
    Some(len)
}

/// The frame an entry of `used_len` bytes carries, if it is one to hand on.
fn frame_len(rx: &RxQueue, used_len: u32) -> Option<usize> {
    let len = (used_len as usize).checked_sub(rx.hdr_len)?;
    let in_buffer = rx.hdr_len.checked_add(len)? <= rx.buf_len as usize;
    (len > 0 && len <= MAX_ETHERNET_FRAME && in_buffer).then_some(len)
}
