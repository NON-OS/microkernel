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







use core::sync::atomic::{fence, Ordering};

use crate::constants::RING_SLOTS;
use crate::queue::RxQueue;

pub struct Frame<'a> {
    pub bytes: &'a [u8],
}








pub unsafe fn take_one(rx: &mut RxQueue) -> Option<Frame<'static>> {
    let used = rx.used_idx();
    if used == rx.last_used {
        return None;
    }
    // The element and the frame are only valid once the index is seen.
    fence(Ordering::Acquire);
    let ring_pos = rx.last_used % RING_SLOTS;
    let (desc_id, used_len) = rx.used_elem_at(ring_pos);
    rx.last_used = rx.last_used.wrapping_add(1);

    // An id past the primed buffers names no slot this queue posted. Folding
    // it into range would read a slot the device may still be writing and
    // post the wild id back to the device, so the entry is dropped and
    // nothing is refilled.
    if desc_id >= u32::from(rx.buf_count) {
        rx.pending_refill = None;
        return Some(Frame { bytes: &[] });
    }





    // The header is 10 bytes on the legacy transport and 12 under
    // VERSION_1; a frame starts after whichever this queue was set up for,
    // and never past the end of its slot.
    let hdr_len = core::cmp::min(rx.hdr_len, rx.buf_len as usize);
    let (payload_ptr, payload_len) = if used_len as usize > hdr_len {
        let raw = (used_len as usize) - hdr_len;
        let cap = (rx.buf_len as usize).saturating_sub(hdr_len);
        let len = core::cmp::min(raw, cap);
        let slot = desc_id as usize;
        let base = rx.buf_va.add(rx.buf_len as usize * slot + hdr_len);
        (base as *const u8, len)
    } else {
        (core::ptr::null::<u8>(), 0usize)
    };

    rx.pending_refill = Some(desc_id as u16);

    if payload_len == 0 {
        return Some(Frame { bytes: &[] });
    }
    let bytes: &'static [u8] = core::slice::from_raw_parts(payload_ptr, payload_len);
    Some(Frame { bytes })
}
