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
//! A slab cut into one class's blocks, and the bytes the lists hold.

use super::size_class::{CLASSES, CLASS_SIZES, SLAB_BYTES};
use super::size_lists::SizeClasses;

impl SizeClasses {
    /// Cut `slab` into blocks of `class`: the first is returned for use, the
    /// rest go on the list.
    ///
    /// # Safety
    /// `slab` is SLAB_BYTES, aligned to SLAB_ALIGN, and nothing else uses it.
    pub(crate) unsafe fn carve(&mut self, class: usize, slab: *mut u8) -> *mut u8 {
        let size = CLASS_SIZES[class];
        let mut at = SLAB_BYTES / size;
        while at > 1 {
            at -= 1;
            // SAFETY: `at * size` lies inside the slab.
            unsafe { self.push(class, slab.add(at * size)) };
        }
        slab
    }

    /// Bytes on the free lists: handed out to slabs, but not in use.
    pub(crate) fn free_bytes(&self) -> usize {
        let mut sum = 0;
        let mut i = 0;
        while i < CLASSES {
            sum += self.free[i] * CLASS_SIZES[i];
            i += 1;
        }
        sum
    }
}
