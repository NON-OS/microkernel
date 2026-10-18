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

//! The used ring, which the device writes: the index and element are its
//! word, and are taken only when they name the one request in flight.

use core::sync::atomic::{fence, Ordering};

use super::fault::Fault;
use super::layout::Queue;
use super::wipe::wipe;
use crate::constants::VQ_USED_OFFSET;

// VirtqUsed: u16 flags, u16 idx, VirtqUsedElem { u32 id, u32 len } ring[].
const USED_IDX: usize = VQ_USED_OFFSET + 2;
const USED_RING: usize = VQ_USED_OFFSET + 4;
const USED_ELEM_SIZE: usize = 8;
/// The only descriptor this queue ever posts.
const POSTED_DESC: u32 = 0;

impl Queue {
    /// The request in flight, once the device's used index has moved: the
    /// bytes it wrote, held to the buffer. `None` while it is still the
    /// device's. An index that moved by anything but the one request, or
    /// an element naming another descriptor, is the device out of turn:
    /// the queue stalls until a reset, and nothing it wrote is read.
    pub fn completion(&mut self) -> Option<Result<u32, Fault>> {
        let idx = self.ring.r16(USED_IDX);
        if idx == self.used {
            return None;
        }
        // The element is valid only once the index that covers it is seen.
        fence(Ordering::Acquire);
        let at = USED_RING + USED_ELEM_SIZE * self.ring_pos(self.used);
        let (id, len) = (self.ring.r32(at), self.ring.r32(at + 4));
        if self.posted == self.used || idx != self.posted || id != POSTED_DESC {
            self.stalled = true;
            return Some(Err(Fault::Foreign));
        }
        self.used = idx;
        let held = u32::try_from(self.buf.len()).unwrap_or(u32::MAX);
        Some(Ok(core::cmp::min(len, held)))
    }

    /// Copy the first `len` bytes the completed request wrote into `out`,
    /// as many as fit, then empty the buffer: nothing stays in it once
    /// handed on (plan/50 C8). Returns how many were copied.
    pub fn take(&self, len: u32, out: &mut [u8]) -> usize {
        let n = core::cmp::min(len as usize, core::cmp::min(out.len(), self.buf.len()));
        for (at, b) in out.iter_mut().take(n).enumerate() {
            *b = self.buf.r8(at);
        }
        wipe(self.buf);
        n
    }
}
