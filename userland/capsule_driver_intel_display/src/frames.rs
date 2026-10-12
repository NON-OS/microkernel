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

//! The two scanout surfaces as the CPU writes them: DMA grants mapped
//! write-combining, so the engine, which reads them without snooping the
//! CPU's caches (unverified per generation), sees every store once fenced.
//! The surfaces own their grants: a mode set maps a new pair before the
//! old pair is given back, and the driver's end gives back what is left.

use crate::dma_pair::{map_pair, unmap, Pair};
use nonos_intel_display::display::Frames;

pub struct DmaFrames {
    pub device_id: u64,
    pub epoch: u64,
    pub now: Pair,
    /// The pair a mode set replaced, until it settles.
    pub was: Option<Pair>,
}

impl Frames for DmaFrames {
    fn put(&mut self, frame: usize, offset: usize, src: &[u8]) -> bool {
        let Some(&base) = self.now.va.get(frame) else { return false };
        if offset.checked_add(src.len()).is_none_or(|end| end > self.now.len) {
            return false;
        }
        // SAFETY: the grant is `len` bytes, mapped while it is `now`, and
        // the range was checked to lie inside it.
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), (base as *mut u8).add(offset), src.len())
        };
        true
    }

    fn fence(&mut self) {
        // SAFETY: sfence has no operands; it orders the WC stores before it.
        #[cfg(target_arch = "x86_64")]
        unsafe {
            core::arch::x86_64::_mm_sfence()
        };
    }

    fn renew(&mut self, len: usize) -> Option<[u64; 2]> {
        let next = map_pair(self.device_id, self.epoch, len as u64).ok()?;
        self.was = Some(core::mem::replace(&mut self.now, next));
        Some(self.now.iova)
    }

    fn settle(&mut self, keep: bool) {
        let Some(was) = self.was.take() else { return };
        if keep {
            unmap(&was);
        } else {
            unmap(&core::mem::replace(&mut self.now, was));
        }
    }
}
