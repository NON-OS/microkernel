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

//! Frames the device reads and writes, from the allocator below 4 GiB, and
//! reached by the CPU through the direct map. The rings sat in the kernel
//! image's `.bss`, and the device was handed their virtual addresses as if
//! they were physical ones (L8).

use crate::memory::addr::PhysAddr;
use crate::memory::phys::{alloc_contiguous, free_contiguous, AllocFlags};

pub(super) struct DmaRegion {
    pub(super) phys: u64,
    pub(super) virt: u64,
    pages: usize,
}

impl DmaRegion {
    pub(super) fn new(pages: usize) -> Result<Self, &'static str> {
        let flags = AllocFlags::DMA | AllocFlags::DMA32 | AllocFlags::ZERO;
        let phys = alloc_contiguous(pages, flags).ok_or("virtio-rng: no DMA frames")?;
        let Some(virt) = crate::memory::unified::phys_to_virt(PhysAddr::new(phys)) else {
            let _ = free_contiguous(phys, pages);
            return Err("virtio-rng: DMA frames outside the direct map");
        };
        Ok(Self { phys, virt: virt.as_u64(), pages })
    }
}

impl Drop for DmaRegion {
    /// Called only once the device is reset and can write here no more.
    fn drop(&mut self) {
        // SAFETY: the region is ours, `pages` long, mapped at `virt`.
        unsafe { core::ptr::write_bytes(self.virt as *mut u8, 0, self.pages * 4096) };
        let _ = free_contiguous(self.phys, self.pages);
    }
}
