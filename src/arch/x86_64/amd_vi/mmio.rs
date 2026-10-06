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

//! One AMD IOMMU's register window, mapped uncached over the base IVRS gave.

use super::regs::WINDOW;
use crate::memory::addr::PhysAddr;

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    base_va: u64,
    base_pa: u64,
}

impl Unit {
    /// Map the unit's `WINDOW` bytes. `None` when the mapper refuses.
    pub fn map(base_pa: u64) -> Option<Self> {
        let va = crate::memory::mmio::map_device_memory(PhysAddr::new(base_pa), WINDOW).ok()?;
        Some(Self { base_va: va.as_u64(), base_pa })
    }

    pub const fn base_pa(&self) -> u64 {
        self.base_pa
    }

    /// `None` for an offset outside the window, which no caller here asks
    /// for; a wrong constant then reads as a failed register, not a stray
    /// access.
    pub fn read64(&self, offset: usize) -> Option<u64> {
        if offset > WINDOW - 8 {
            return None;
        }
        // SAFETY: eK@nonos.systems - the offset is inside the uncached window
        // `map` made over this unit's registers, which is never unmapped.
        Some(unsafe { core::ptr::read_volatile((self.base_va as usize + offset) as *const u64) })
    }

    /// # Safety
    /// Writing an IOMMU register changes how devices reach memory; the caller
    /// owns the sequencing the spec requires around it.
    pub unsafe fn write64(&self, offset: usize, value: u64) {
        if offset > WINDOW - 8 {
            return;
        }
        // SAFETY: eK@nonos.systems - inside the mapped window; the caller owns
        // what the write means.
        unsafe { core::ptr::write_volatile((self.base_va as usize + offset) as *mut u64, value) }
    }
}
