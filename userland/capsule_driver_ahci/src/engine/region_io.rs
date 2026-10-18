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

//! Copies between the driver's memory and a DMA region, held to the
//! region's length. Only the start of a region is ever copied: slot 0's one
//! PRD names the data buffer from its first byte.

use super::region::DmaRegion;

impl DmaRegion {
    /// Copy `src` to the start of the region. False, copying nothing, when
    /// it does not fit.
    pub fn put(&self, src: &[u8]) -> bool {
        if src.len() as u64 > self.len() {
            return false;
        }
        // SAFETY: the region is mapped for `len()` bytes from `user_va()`
        // while it lives, `src` is no longer, and no Rust reference points
        // into the region, so the copy aliases nothing.
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), self.user_va() as *mut u8, src.len())
        };
        true
    }

    /// Fill `dst` from the start of the region. False, copying nothing,
    /// when the region is shorter.
    pub fn get(&self, dst: &mut [u8]) -> bool {
        if dst.len() as u64 > self.len() {
            return false;
        }
        // SAFETY: as in `put`, with the copy the other way.
        unsafe {
            core::ptr::copy_nonoverlapping(self.user_va() as *const u8, dst.as_mut_ptr(), dst.len())
        };
        true
    }
}
