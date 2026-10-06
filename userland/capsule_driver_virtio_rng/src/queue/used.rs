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

//! Read the used ring after the device has filled descriptor 0.
//! The byte count comes from the used-elem the device wrote; the
//! capsule then reads up to that many bytes through the buffer
//! mapping.

use core::ptr::read_volatile;
use core::sync::atomic::{fence, Ordering};

use super::layout::Queue;
use crate::constants::VQ_USED_OFFSET;

// VirtqUsed: u16 flags, u16 idx, VirtqUsedElem ring[queue_size]
// VirtqUsedElem: u32 id, u32 len   (8 bytes; first elem at +4)
const USED_IDX_OFFSET: usize = 2;
const USED_RING_OFFSET: usize = 4;
const USED_ELEM_SIZE: usize = 8;
const USED_ELEM_LEN_OFFSET: usize = 4;
/// The only descriptor this queue ever posts.
const POSTED_DESC: u32 = 0;

impl Queue {
    /// Snapshot of the device's used-ring `idx` field. The capsule
    /// compares this to its `last_used` to detect completion.
    pub fn used_idx(&self) -> u16 {
        unsafe { read_volatile(self.region_va.add(VQ_USED_OFFSET + USED_IDX_OFFSET).cast()) }
    }

    /// The used element at ring position `idx`, as (id, len).
    pub fn used_elem(&self, idx: u16) -> (u32, u32) {
        let off = VQ_USED_OFFSET + USED_RING_OFFSET + USED_ELEM_SIZE * self.ring_pos(idx);
        unsafe {
            let id = read_volatile(self.region_va.add(off).cast::<u32>());
            let len = read_volatile(self.region_va.add(off + USED_ELEM_LEN_OFFSET).cast::<u32>());
            (id, len)
        }
    }

    /// The next completion, once the device's used index has moved past
    /// `last_used`: the bytes it wrote, held to the buffer, or an error for
    /// an element naming a descriptor this queue never posted, which is
    /// dropped rather than believed. `None` while the request is still with
    /// the device.
    ///
    /// The element is the one at this completion's own ring position. Every
    /// completion used to be read from element 0, so from the second
    /// request on the driver took the first request's length, and served
    /// stale buffer bytes whenever the device filled less than that.
    pub fn completion(&mut self) -> Option<Result<u32, &'static str>> {
        if self.used_idx() == self.last_used {
            return None;
        }
        // The element is valid only once the index that covers it is seen.
        fence(Ordering::Acquire);
        let (id, len) = self.used_elem(self.last_used);
        self.last_used = self.last_used.wrapping_add(1);
        if id != POSTED_DESC {
            return Some(Err("virtio-rng: used element names no posted descriptor"));
        }
        Some(Ok(core::cmp::min(len, self.buf_len)))
    }

    /// Borrow the entropy buffer the device wrote into. The slice
    /// is capped at `buf_len` so a misbehaving device cannot
    /// induce an out-of-bounds read.
    ///
    /// # Safety
    /// `len` must be the byte count `completion` returned for the
    /// most recent completed descriptor; the caller is responsible
    /// for not aliasing the buffer with concurrent device writes.
    /// # Safety
    ///
    /// The address the caller mapped, at an offset inside it.
    pub unsafe fn buffer(&self, len: u32) -> &[u8] {
        let n = core::cmp::min(len, self.buf_len) as usize;
        core::slice::from_raw_parts(self.buf_va, n)
    }
}
