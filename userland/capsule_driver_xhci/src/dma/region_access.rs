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

//! Reading and writing a DMA region at an offset, each access held against
//! the region's length and the value's alignment first, so no caller
//! reaches past the grant with a raw pointer of its own.

use super::region::DmaRegion;

impl DmaRegion {
    /// Whether a `size`-byte value at `offset` lies inside the region at
    /// its natural alignment (the region starts on a page).
    pub(super) fn holds(&self, offset: usize, size: usize) -> bool {
        offset.is_multiple_of(size)
            && offset.checked_add(size).is_some_and(|end| end as u64 <= self.length)
    }

    pub fn read_u8(&self, offset: usize) -> Option<u8> {
        // SAFETY: `holds` put the byte inside the live grant.
        self.holds(offset, 1)
            .then(|| unsafe { self.as_mut_ptr::<u8>().add(offset).read_volatile() })
    }

    pub fn read_u32(&self, offset: usize) -> Option<u32> {
        let at = self.as_mut_ptr::<u8>().wrapping_add(offset) as *const u32;
        // SAFETY: `holds` put the aligned dword inside the live grant.
        self.holds(offset, 4).then(|| unsafe { at.read_volatile() })
    }

    /// False, writing nothing, when the dword is not inside the region.
    pub fn write_u32(&self, offset: usize, value: u32) -> bool {
        let at = self.as_mut_ptr::<u8>().wrapping_add(offset) as *mut u32;
        // SAFETY: `holds` put the aligned dword inside the live grant.
        self.holds(offset, 4).then(|| unsafe { at.write_volatile(value) }).is_some()
    }

    /// False, writing nothing, when the qword is not inside the region.
    pub fn write_u64(&self, offset: usize, value: u64) -> bool {
        let at = self.as_mut_ptr::<u8>().wrapping_add(offset) as *mut u64;
        // SAFETY: `holds` put the aligned qword inside the live grant.
        self.holds(offset, 8).then(|| unsafe { at.write_volatile(value) }).is_some()
    }
}
