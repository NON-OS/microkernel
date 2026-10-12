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

use nonos_intel_display::display::Frames;

pub struct DmaFrames {
    pub va: [u64; 2],
    pub len: usize,
}

impl Frames for DmaFrames {
    fn put(&mut self, frame: usize, offset: usize, src: &[u8]) -> bool {
        let Some(&base) = self.va.get(frame) else { return false };
        if offset.checked_add(src.len()).is_none_or(|end| end > self.len) {
            return false;
        }
        // SAFETY: the grant is `len` bytes, mapped for this capsule's life,
        // and the range was checked to lie inside it.
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
}
