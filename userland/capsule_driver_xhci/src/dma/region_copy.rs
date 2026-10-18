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
//! Copying between a DMA region and a buffer of the driver's own.
use super::region::DmaRegion;

impl DmaRegion {
    /// Copy the region's first bytes into `dst`, as many as both hold;
    /// returns how many.
    pub fn copy_to(&self, dst: &mut [u8]) -> usize {
        let n = dst.len().min(self.length as usize);
        for (i, b) in dst[..n].iter_mut().enumerate() {
            *b = self.read_u8(i).unwrap_or(0);
        }
        n
    }

    /// Put `src` at the start of the region; false, writing nothing, when
    /// it does not fit.
    pub fn copy_from(&self, src: &[u8]) -> bool {
        if src.len() as u64 > self.length {
            return false;
        }
        // SAFETY: `src` fits the live grant, which no slice of ours aliases.
        unsafe { core::ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr(), src.len()) };
        true
    }
}
