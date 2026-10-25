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

//! The scanout surfaces and the cursor image as the CPU writes them: DMA
//! grants mapped write-combining, so the engine, which reads them without
//! snooping the CPU's caches (unverified), sees every store once fenced. A
//! mode set maps a new pair before the old pair goes (`frames_drop`).

use crate::dma_pair::{map_one, map_pair, Pair};
use crate::frames_drop::{sfence, unmap};
use nonos_intel_display::display::Frames;

pub struct DmaFrames {
    pub device_id: u64,
    pub epoch: u64,
    pub now: Pair,
    /// The pair a mode set replaced, until it settles.
    pub was: Option<Pair>,
    /// The cursor image's grant: address, IOVA, grant id.
    pub cursor: Option<(u64, u64, u64)>,
}

impl Frames for DmaFrames {
    fn put(&mut self, frame: usize, offset: usize, src: &[u8]) -> bool {
        let Some(&base) = self.now.va.get(frame) else { return false };
        if offset.checked_add(src.len()).is_none_or(|end| end > self.now.len) {
            return false;
        }
        // SAFETY: inside the grant, which is mapped while it is `now`.
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), (base as *mut u8).add(offset), src.len())
        };
        true
    }

    fn fence(&mut self) {
        sfence();
    }

    fn renew(&mut self, len: usize) -> Option<[u64; 2]> {
        let next = map_pair(self.device_id, self.epoch, len as u64).ok()?;
        self.was = Some(core::mem::replace(&mut self.now, next));
        Some(self.now.iova)
    }

    fn cursor_image(&mut self, img: &[u8]) -> Option<u64> {
        if self.cursor.is_none() {
            self.cursor = Some(map_one(self.device_id, self.epoch, img.len() as u64).ok()?);
        }
        let (va, iova, _) = self.cursor?;
        // SAFETY: the grant is the image's length, held mapped here.
        unsafe { core::ptr::copy_nonoverlapping(img.as_ptr(), va as *mut u8, img.len()) };
        Some(iova)
    }

    fn settle(&mut self, keep: bool) {
        if let Some(was) = self.was.take() {
            unmap(&if keep { was } else { core::mem::replace(&mut self.now, was) });
        }
    }
}
