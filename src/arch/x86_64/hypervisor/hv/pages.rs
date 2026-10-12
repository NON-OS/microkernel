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

/* The SynIC's pages: frames the kernel owns for the life of the boot. */

use crate::memory::frame_alloc::manager::allocate_frame;
use crate::memory::unified::phys_to_virt;

pub(super) const PAGE_BYTES: usize = 4096;

#[derive(Clone, Copy)]
pub(super) struct Page {
    pub phys: u64,
    pub virt: u64,
}

impl Page {
    /// A zeroed frame and its direct-map address.
    pub(super) fn new() -> Option<Self> {
        let phys = allocate_frame()?;
        let virt = phys_to_virt(phys)?.as_u64();
        // SAFETY: a frame just allocated for this module, mapped in the
        // direct map for the life of the kernel.
        unsafe { core::ptr::write_bytes(virt as *mut u8, 0, PAGE_BYTES) };
        Some(Self { phys: phys.as_u64(), virt })
    }

    pub(super) fn at(self, off: usize) -> *mut u8 {
        (self.virt as usize + off) as *mut u8
    }
}
